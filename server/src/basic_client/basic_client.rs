use std::io::{Read, Write};
use pog_bilder::messages;

use std::net::TcpStream;

use clap::Parser;
use prost::Message;

/// Basic test client
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the address to connect to
    address: String,
    /// The message to send
    message: String
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut stream = TcpStream::connect(args.address)?;
    println!("Connected to server!");

    let sender = Some({
        let mut s = messages::Sender::default();
        s.uuid = {
            let micros = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros();
            Vec::from(micros.to_be_bytes())
        };
        s.name = "Vescusia".to_owned();
        s
    });

    // send MessageRequest
    let mut msg = messages::MessageRequest::default();
    msg.sender = sender.clone();
    msg.since =  Some(prost_types::Timestamp::from(std::time::SystemTime::now()));
    let bytes = msg.encode_length_delimited_to_vec();
    println!("{bytes:?}");
    stream.write_all(&bytes)?;
    let new_message = messages::MessageRequest::decode_length_delimited(bytes.as_slice())?;
    println!("MessageRequest sent! {}, {} <> {}", msg == new_message, bytes.len(), msg.encoded_len());

    // no replies
    let mut msg = messages::Message::default();
    msg.sender = sender.clone().into();
    msg.timestamp = Some(prost_types::Timestamp::from(std::time::SystemTime::now()));
    msg.data = Some(messages::message::Data::Text({
        let mut text = String::with_capacity(300);
        for _ in 0..300 {
            text.push('a')
        }
        text
    }));
    println!("Message1 sent: {:?} = {}", msg, msg.encoded_len());
    stream.write_all(&msg.encode_length_delimited_to_vec())?;
    stream.flush()?;

    let mut msg = messages::Message::default();
    msg.sender = sender.into();
    msg.timestamp = Some(prost_types::Timestamp::from(std::time::SystemTime::now()));
    msg.data = Some(messages::message::Data::Text(args.message));
    println!("Message2 sent: {:?} = {}", msg, msg.encoded_len());
    stream.write_all(&msg.encode_length_delimited_to_vec())?;

    // check reply
    let mut buf = vec![0u8; 1024];
    stream.read(&mut buf)?;
    let mut buf = bytes::BytesMut::from(buf.as_slice());
    let msg = Message::decode_length_delimited(&mut buf)?;
    println!("message received: {msg:?}");

    Ok(())
}