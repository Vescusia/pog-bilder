include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
pub mod sync_reader;
mod client;


use clap::Parser;
/// The Pog Bilder Server program.
///
/// Please see https://github.com/Vescusia/pog-bilder for more info.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the Port the Server should run on
    #[arg(short, long)]
    port: u16,

    /// the address to bind to
    #[arg(long, default_value_t = String::from("0.0.0.0"))]
    bind_address: String
}


#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();

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
        println!("Client connected! {addr:?}");
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
