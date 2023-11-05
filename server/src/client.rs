use protobuf::Message;
use tokio::io::AsyncWriteExt;
use crate::{messages, sync_reader};

use tokio::sync::broadcast::{Sender, Receiver};

pub async fn handle_client(stream: tokio::net::TcpStream, _addr: std::net::SocketAddr, broadcast: (Sender<messages::Message>, Receiver<messages::Message>)) -> std::io::Result<()> {
    let (reader, writer) = stream.into_split();
    let mut reader = sync_reader::SyncReader::from(reader);

    // handling the "handshake"
    // get message request
    let (message_request, reader) = tokio::task::spawn_blocking(move || match messages::MessageRequest::parse_from_reader(&mut reader) {
        Ok(msg) => Ok((msg, reader)),
        Err(e) => Err(e)
    }).await??;

    // TODO: send requested messages
    let this_client = message_request.sender;


    // one, blocking, task handles the messages from the client (needs to be blocking bcs of protobuf)
    // the other task handles writing to the client
    let tasks = [
        tokio::task::spawn_blocking(|| handle_client_read(reader.into(), broadcast.0)),
        tokio::spawn(handle_client_write(writer, broadcast.1, this_client))
    ];

    for task in tasks {
        task.await??;
    }
    Ok(())
}

fn handle_client_read(mut reader: sync_reader::SyncReader, tx: Sender<messages::Message>) -> std::io::Result<()> {
    loop {
        let message = messages::Message::parse_from_reader(&mut reader)?;

        // if the broadcast fails, it means no receivers are left, which should not happen.
        if let Err(_) = tx.send(message) {
            return Err(std::io::Error::from(std::io::ErrorKind::BrokenPipe))
        }
    }
}


async fn handle_client_write(mut writer: tokio::net::tcp::OwnedWriteHalf, mut rx: Receiver<messages::Message>, this_client: protobuf::MessageField<messages::Sender>) -> std::io::Result<()> {
    loop {
        let message = loop {
            // manage lagging
            match rx.recv().await {
                Ok(msg) => break msg,
                Err(_) => {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    continue;
                }
            }
        };

        if message.sender != this_client {
            writer.write_all(&message.write_to_bytes()?).await?;
        }
    }
}
