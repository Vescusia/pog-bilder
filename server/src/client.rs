use prost::Message;
use pog_bilder::{messages, reader_buffer};


use tokio::sync::broadcast::{Sender, Receiver};
use tokio::io::AsyncWriteExt;

pub async fn handle_client(stream: tokio::net::TcpStream, _addr: std::net::SocketAddr, broadcast: (Sender<messages::Message>, Receiver<messages::Message>)) -> std::io::Result<()> {
    let (reader, writer) = stream.into_split();
    let mut reader: reader_buffer::ReaderBuffer<_> = reader.into();

    // handling the "handshake"
    // get message request
    let message_request = messages::MessageRequest::decode(
        reader.read_delimited().await?
    )?;

    // TODO: send requested messages
    println!("{message_request:?} connected!");
    let this_client = message_request.sender;

    // one, blocking, task handles the messages from the client (needs to be blocking bcs of protobuf)
    // the other task handles writing to the client
    let tasks = [
        tokio::spawn(handle_client_read(reader.into(), broadcast.0)),
        tokio::spawn(handle_client_write(writer, broadcast.1, this_client))
    ];

    for task in tasks {
        task.await??;
    }
    Ok(())
}

async fn handle_client_read(mut reader: reader_buffer::ReaderBuffer<tokio::net::tcp::OwnedReadHalf>, tx: Sender<messages::Message>) -> std::io::Result<()> {
    loop {
        let message = Message::decode(
            reader.read_delimited().await?
        )?;

        // if the broadcast fails, it means no receivers are left, which should not happen.
        if let Err(_) = tx.send(message) {
            return Err(std::io::Error::from(std::io::ErrorKind::BrokenPipe))
        }
    }
}


async fn handle_client_write(mut writer: tokio::net::tcp::OwnedWriteHalf, mut rx: Receiver<messages::Message>, this_client: Option<messages::Sender>) -> std::io::Result<()> {
    let mut buf = Vec::with_capacity(128);

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
            message.encode_length_delimited(&mut buf)?;
            writer.write_all(&buf).await?;
            buf.clear();
        }
    }
}
