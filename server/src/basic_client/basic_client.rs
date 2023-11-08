use std::io::{Read, Write};
use pog_bilder::*;

use std::net::TcpStream;
use std::time::Duration;

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

    let sender = Some(messages::Sender{
        uuid: {
            let micros = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros();
            micros as u64
        },
        name: "Vescusia".to_owned()
    });

    // send MessageRequest
    let msg = messages::MessageRequest{
        sender: sender.clone(),
        since:  Some(prost_types::Timestamp::from(std::time::UNIX_EPOCH))
    };
    let bytes = msg.encode_length_delimited_to_vec();
    println!("{bytes:?}");
    stream.write_all(&bytes)?;
    let new_message = messages::MessageRequest::decode_length_delimited(bytes.as_slice())?;
    println!("MessageRequest sent! {}, {} <> {}", msg == new_message, bytes.len(), msg.encoded_len());

    // no replies
    let msg = messages::Message{
        sender: sender.clone(),
        timestamp: Some(prost_types::Timestamp::from(std::time::SystemTime::now())),
        data: Some(messages::message::Data::Text({
            let mut text = String::with_capacity(300);
            for _ in 0..300 {
                text.push('a')
            }
            text
        }))
    };
    stream.write_all(&msg.encode_length_delimited_to_vec())?;
    println!("Message1 sent: {:?} = {}", msg, msg.encoded_len());

    let msg = messages::Message{
        sender,
        timestamp: Some(prost_types::Timestamp::from(std::time::SystemTime::now())),
        data: Some(messages::message::Data::Text(args.message))
    };
    stream.write_all(&msg.encode_length_delimited_to_vec())?;
    println!("Message2 sent: {:?} = {}", msg, msg.encoded_len());


    // check reply
    std::thread::sleep(Duration::from_secs_f32(0.1));
    let mut buf = vec![0u8; 1024];
    let amount = stream.read(&mut buf)?;
    let buf = bytes::Bytes::from(buf);
    let msg = messages::Message::decode_length_delimited(buf)?;
    println!("message received: {msg:?} <> {amount}");

    Ok(())
}