use bytes::BufMut;
use tokio::io::{AsyncRead, AsyncReadExt};

pub struct ReaderBuffer<S: AsyncReadExt + Unpin> {
    bytes: bytes::BytesMut,
    reader: S
}

impl<S: AsyncReadExt + Unpin + AsyncRead> ReaderBuffer<S> {
    /// Creates a new [`ReaderBuffer`] with the `cap` amount of bytes reallocated.
    /// You should (basically) always use this constructor.
    pub fn with_capacity(cap: usize, reader: S) -> Self {
        Self{
            bytes: bytes::BytesMut::with_capacity(cap),
            reader
        }
    }


    /// Reads a length delimiter from the wire and then returns as many [`bytes::Bytes`] as delimited.
    ///
    /// It's important, that the bytes are encoded in variable width unsigned integers, like with [protobuf](https://protobuf.dev/programming-guides/encoding/#varints)
    ///
    /// You won't have to call [`Self::confirm_read`] after this method, obviously.
    /// In fact, it will panic, because the Buffer is empty.
    ///
    /// **Also, the delimiter-bytes are not included within the returned Bytes!!!!**
    pub async fn read_delimited(&mut self) -> std::io::Result<bytes::Bytes> {
        let msg_length = self.read_delimited_length().await?;

        // read msg_length amount of bytes (very dirty, but i hope it gets optimized away in release O:)
        for _ in 0..msg_length {
            self.bytes.put_u8(self.reader.read_u8().await?)
        }

        Ok(
            self.bytes.split_to(msg_length).freeze()
        )
    }


    /// Reads the delimited message length from the stream.
    /// This will only work with variable width integers, like in [protobuf](https://protobuf.dev/programming-guides/encoding/#varints)
    async fn read_delimited_length(&mut self) -> std::io::Result<usize> {
        const CONT_BIT_MASK: u8 = 0b10000000;

        // read size of message
        let mut delimiter = 0u64;

        // read variable amount of bytes
        for i in 0..std::mem::size_of_val(&delimiter) {
            let mut byte = self.reader.read_u8().await?;
            let has_continuation_bit = (byte & CONT_BIT_MASK) > 0;

            byte &= !CONT_BIT_MASK;  // set continuation bit to 0
            delimiter |= (byte as u64) << (7 * i) as u64;  // concatenate with 7 bit big endian

            if !has_continuation_bit {
                break;
            }
        }
        Ok(delimiter as usize)
    }
}


impl<S: AsyncReadExt + Unpin> From<S> for ReaderBuffer<S> {
    fn from(value: S) -> Self {
        Self{
            bytes: bytes::BytesMut::with_capacity(128),
            reader: value,
        }
    }
}
