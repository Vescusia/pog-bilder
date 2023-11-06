mod client;
mod args;


use clap::Parser;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = args::Args::parse();

    // create listener
    let addr: std::net::SocketAddr = std::net::SocketAddr::new(
        args.bind_address.parse().expect("could not parse given Ip Address!"),
        args.port
    );
    let listener = tokio::net::TcpListener::bind(addr).await.expect("could not bind to address!");

    // create broadcast channel
    let (tx, rx) = tokio::sync::broadcast::channel(32);

    println!("Server is listening!");
    loop {
        let (socket, addr) = listener.accept().await.expect("accepting client failed!");
        // spawn client handler
        tokio::spawn(
            client::handle_client(
                socket,
                addr,
                (tx.clone(), rx.resubscribe())
            )
        );
    }
}
