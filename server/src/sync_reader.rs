use tokio::net::tcp::OwnedReadHalf;

#[derive(Debug)]
pub struct SyncReader {
    reader: tokio::net::tcp::OwnedReadHalf
}

impl std::io::Read for SyncReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        loop {
            match self.reader.try_read(buf) {
                Ok(n) => return Ok(n),
                Err(e) => match e.kind() {
                    std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(std::time::Duration::from_secs_f32(0.5))
                    },
                    _ => return Err(e)
                }
            }

        }
    }
}

impl From<OwnedReadHalf> for SyncReader {
    fn from(value: OwnedReadHalf) -> Self {
        Self{
            reader: value
        }
    }
}
