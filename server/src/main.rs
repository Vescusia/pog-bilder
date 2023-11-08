use pog_bilder::*;

mod client;
mod args;


use clap::Parser;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = args::Args::parse();


    // create database connection
    let db = db::create_or_connect_db(args.db_path.as_deref()).await?;


    // create broadcast channel
    let (tx, rx) = tokio::sync::broadcast::channel(32);

    // create listener
    let addr: std::net::SocketAddr = std::net::SocketAddr::new(
        args.bind_address.parse().expect("could not parse given Ip Address!"),
        args.port
    );
    let listener = tokio::net::TcpListener::bind(addr).await?;

    // "start" server
    println!("Server is listening!");
    loop {
        let (socket, addr) = listener.accept().await?;
        // spawn client handler
        tokio::spawn(
            client::handle_client(
                socket,
                addr,
                (tx.clone(), rx.resubscribe()),
                db.clone()
            )
        );
    }
}
