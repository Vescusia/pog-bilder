use pog_bilder::*;

use std::sync::Arc;

use tokio::sync::{self, broadcast::{Sender, Receiver}};
use tokio::io::AsyncWriteExt;
use prost::Message as MessageTrait;


pub async fn handle_client(
    stream: tokio::net::TcpStream,
    _addr: std::net::SocketAddr,
    broadcast: (Sender<Arc<messages::Message>>, Receiver<Arc<messages::Message>>),
    db: tokio_rusqlite::Connection,
    args: Arc<crate::args::ServerArgs>
) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader: reader_buffer::ReaderBuffer<_> = reader.into();


    // handling the "handshake"
    // get message request
    let message_request = messages::MessageRequest::decode(
        reader.read_delimited().await?
    )?;
    println!("{message_request:?} connected!");


    // send welcome message (if wanted)
    if let Some(text) = &args.env_args.welcome_message {
        writer.write_all(&messages::Message {
            sender: args.default_sender.clone(),
            data: Some(messages::message::Data::Text(text.clone())),
            ..std::default::Default::default()
        }.encode_length_delimited_to_vec()).await?;
    }


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


    // the paradigm is that this channel will forward any messages that the client-writer should send to the client
    // because of this, the reader thread can also send to the client (BigFileAccepts)
    // and managing file reader threads is simple
    // but we need a separate thread to simple forward any messages received on the broadcast (._.)
    let (tx, rx) = sync::mpsc::channel(32);


    // one, blocking, task handles the messages from the client (needs to be blocking bcs of protobuf)
    // the other task handles writing to the client
    let tasks = [
        tokio::spawn(handle_client_read(reader, broadcast.0, db, tx.clone(), args.clone())),
        tokio::spawn(handle_client_write(writer, rx)),
        tokio::spawn(forward_broadcast_msgs(tx, broadcast.1, this_client))
    ];

    for task in tasks {
        task.await??;
    }
    Ok(())
}


/// This thread handles reading the clients [`messages::Message`]'s
/// To communicate with the writer thread, there is the `writer_tx`
async fn handle_client_read(
    mut reader: reader_buffer::ReaderBuffer<tokio::net::tcp::OwnedReadHalf>,
    broadcast_tx: Sender<Arc<messages::Message>>,
    db: tokio_rusqlite::Connection,
    writer_tx: sync::mpsc::Sender<Arc<messages::Message>>,
    args: Arc<crate::args::ServerArgs>
) -> Result<()> {
    use messages::message::Data;

    let mut big_file_writers = std::collections::HashMap::new();

    loop {
        let message = messages::Message::decode(
            reader.read_delimited().await?
        )?;

        // handle messages differently
        match message.data.as_ref().unwrap() {
            // respond to BigFileOffer
            Data::BigFileOffer(offer) => {
                // arbitrary accept clause
                if offer.byte_amount > 0 {
                    let ts = message.timestamp.as_ref().unwrap().clone();

                    // respond with accept
                    writer_tx.send(Arc::new(
                        messages::Message {
                            sender: args.default_sender.clone(),
                            timestamp: Some(prost_types::Timestamp::default()),
                            data: Some(
                                Data::BigFileAccept(ts.clone())
                            ),
                        }
                    )).await?;

                    // add file writer to hashmap
                    big_file_writers.insert(ts.clone(),
                                            (
                                                    big_file::BigFileWriter::new(
                                                        args.generate_big_file_path(&ts),
                                                        ts,
                                                        offer.byte_amount as usize,
                                                    ).await?,
                                                    message
                                                )
                    );
                }
            }

            // write bytes to file
            Data::BigFileData(data) => {
                if let Some(id) = &data.file_id {
                    if let Some((writer, _)) = big_file_writers.get_mut(id) {
                        writer.write_bytes(&data.bytes).await?;

                        // if the writer is done!
                        if writer.is_done() {
                            // remove it from the writers
                            let (_, message) = big_file_writers.remove(id).unwrap();

                            // broadcast and save the original BigFileOffer!
                            db::add_message(db.clone(), &message).await?;
                            broadcast_tx.send(Arc::new(message))?;
                        }
                    }
                }
            }

            // start file reader
            Data::BigFileAccept(ts) => {
                tokio::task::spawn(
                    big_file_reader(args.generate_big_file_path(ts), writer_tx.clone(), ts.clone())
                );
            }

            // handle normal messages
            _ => {
                db::add_message(db.clone(), &message).await?;

                broadcast_tx.send(Arc::new(message))?;
            }
        }
    }
}



/// This thread manages all writing to the client
/// The [`messages::Message`]'s that should be sent are read from the `messages_to_send` receiver
async fn handle_client_write(mut writer: tokio::net::tcp::OwnedWriteHalf, mut messages_to_send: sync::mpsc::Receiver<Arc<messages::Message>>) -> Result<()> {
    let mut buf = Vec::with_capacity(128);

    loop {
        // get message from channel
        let message = match messages_to_send.recv().await {
            Some(msg) => msg,
            None => return Err(anyhow::Error::msg("Messages mpsc dried out (?)"))
        };

        // send message
        // the threads sending us messages should make sure that we don't
        // send this client messages, that they should not receive (their own messages mainly :D)
        message.encode_length_delimited(&mut buf)?;
        writer.write_all(&buf).await?;
        buf.clear();
    }
}


/// This thread forwards any (except the ones sent by `this_client`) [`messages::Message`]'s received on the `broadcast_rx` channel
async fn forward_broadcast_msgs(writer_tx: sync::mpsc::Sender<Arc<messages::Message>>, mut broadcast_rx: Receiver<Arc<messages::Message>>, this_client: Option<messages::Sender>) -> Result<()> {
    loop {
        let message = loop {
            // manage lagging
            match broadcast_rx.recv().await {
                Ok(msg) => break msg,
                Err(_) => {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    continue;
                }
            }
        };

        // send to client writer if this message has not been sent by us ^^
        if message.sender != this_client {
            writer_tx.send(message).await?;
        }
    }
}


/// This thread reads the file at `path` and sends the bytes as [`messages::message::Data::BigFileData`] to the client
async fn big_file_reader<P: AsRef<std::path::Path>>(path: P, writer_tx: sync::mpsc::Sender<Arc<messages::Message>>, ts: prost_types::Timestamp) -> Result<()> {
    // create reader with section size of 4kiB
    let mut reader = big_file::FileReader::<{ 1024 * 4 }>::open(path).await?;

    // read and send all sections in the file
    while let Some(bytes) = reader.read_section().await? {
        let msg = messages::Message{
            data: Some(messages::message::Data::BigFileData(
                messages::BigFileData{
                    file_id: Some(ts.clone()),
                    bytes: bytes.to_owned(),
                }
            )),
            ..std::default::Default::default()
        };

        writer_tx.send(Arc::new(msg)).await?
    }

    Ok(())
}
