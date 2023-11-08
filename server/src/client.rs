use prost::Message;
use pog_bilder::*;


use tokio::sync::broadcast::{Sender, Receiver};
use tokio::io::AsyncWriteExt;

pub async fn handle_client(stream: tokio::net::TcpStream, _addr: std::net::SocketAddr, broadcast: (Sender<messages::Message>, Receiver<messages::Message>), db: tokio_rusqlite::Connection) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader: reader_buffer::ReaderBuffer<_> = reader.into();

    // handling the "handshake"
    // get message request
    let message_request = messages::MessageRequest::decode(
        reader.read_delimited().await?
    )?;
    println!("{message_request:?} connected!");


    // send all requested messages
    let this_client = message_request.sender;
    if let Some(client) = &this_client {
        db::add_sender(db.clone(), client).await?;

        let mut msgs_sent = 0;
        let since = message_request.since.as_ref().unwrap();
        let mut messages = db::select_messages_since(db.clone(), client, since).await?;
        while let Some(msg) = messages.recv().await {
            writer.write_all(&msg).await?;
            msgs_sent += 1;
        }
        println!("client requested (and received) {msgs_sent} messages.")
    }


    // one, blocking, task handles the messages from the client (needs to be blocking bcs of protobuf)
    // the other task handles writing to the client
    let tasks = [
        tokio::spawn(handle_client_read(reader, broadcast.0, db)),
        tokio::spawn(handle_client_write(writer, broadcast.1, this_client))
    ];

    for task in tasks {
        task.await??;
    }
    Ok(())
}

async fn handle_client_read(mut reader: reader_buffer::ReaderBuffer<tokio::net::tcp::OwnedReadHalf>, tx: Sender<messages::Message>, db: tokio_rusqlite::Connection) -> Result<()> {
    loop {
        let message = Message::decode(
            reader.read_delimited().await?
        )?;

        db::add_message(db.clone(), &message).await?;

        // if the broadcast fails, it means no receivers are left, which should not happen.
        tx.send(message)?;
    }
}


async fn handle_client_write(mut writer: tokio::net::tcp::OwnedWriteHalf, mut rx: Receiver<messages::Message>, this_client: Option<messages::Sender>) -> Result<()> {
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

        // send message
        if message.sender != this_client {
            message.encode_length_delimited(&mut buf)?;
            writer.write_all(&buf).await?;
            buf.clear();
        }
    }
}
