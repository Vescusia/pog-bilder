use crate::*;

use std::hash::{Hash, Hasher};
use std::path::Path;
use tokio::{fs, io};
use io::AsyncWriteExt;
use tokio::io::AsyncReadExt;

#[derive(Debug)]
pub struct BigFileWriter {
    file_handle: fs::File,
    file_path: std::path::PathBuf,  // only for incomplete drops
    id: prost_types::Timestamp,
    pub bytes_remaining: usize,
}

impl PartialEq for BigFileWriter {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for BigFileWriter {}

impl Hash for BigFileWriter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}


impl BigFileWriter {
    /// Create a new [`BigFileWriter`] at the specified path with the `id` of `time_stamp`
    /// and the promised size of `byte_amount`
    pub async fn new<P: AsRef<Path>>(path: P, time_stamp: prost_types::Timestamp, byte_amount: usize) -> Result<Self> {
        let file_path = path.as_ref().to_path_buf();
        let file_handle = fs::File::open(&file_path).await?;

        Ok(Self{
            file_handle,
            file_path,
            id: time_stamp,
            bytes_remaining: byte_amount
        })
    }


    /// Write some bytes to the file.
    ///
    /// The internal file handle will automatically shutdown when [`Self::is_done`] `==` `true`.
    /// Trying to write any more bytes after that will result in an error.
    pub async fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.file_handle.write_all(bytes).await?;
        self.bytes_remaining -= bytes.len();

        if self.is_done() {
            self.file_handle.shutdown().await?;
        }

        Ok(())
    }

    /// Checks if all bytes have been read.
    pub fn is_done(&self) -> bool {
        self.bytes_remaining == 0
    }
}


impl Drop for BigFileWriter {
    fn drop(&mut self) {
        if !self.is_done() {
            // try to remove (incomplete) file
            let _ = log_error(std::fs::remove_file(&self.file_path));
        }
    }
}



#[derive(Debug)]
pub struct FileReader<const SECTION_SIZE: usize> {
    file: fs::File,
    buf: bytes::BytesMut
}


impl<const SECTION_SIZE: usize> FileReader<SECTION_SIZE> {
    /// Opens a new File at `path` without creating if not exists.
    pub async fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(false)
            .open(path).await?;

        Ok(Self{
            file,
            buf: bytes::BytesMut::with_capacity(SECTION_SIZE)
        })
    }

    /// Read [`SECTION_SIZE`] amount of bytes from the file (or less if it becomes empty)
    /// or nothing if trying to read a finished file.
    pub async fn read_section(&mut self) -> Result<Option<&[u8]>> {
        self.buf.clear();
        match self.file.read_exact(&mut self.buf).await {
            Ok(_) => {
                Ok(Some(&self.buf))
            }

            Err(e) => match e.kind() {
                // file is done
                io::ErrorKind::UnexpectedEof => {
                    Ok(
                        if self.buf.is_empty() {
                            None
                        } else {
                            Some(&self.buf)
                        }
                    )
                }

                // any other error gets passed through
                _ => Err(e.into())
            }
        }
    }
}
