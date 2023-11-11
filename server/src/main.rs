use pog_bilder::*;

mod client;
mod args;


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = args::parse_args();

    // create file directory
    tokio::fs::create_dir_all(&args.env_args.big_file_directory).await?;

    // create database connection
    let db = db::create_or_connect_db(&args.env_args.db_path).await?;

    // create broadcast channel
    let (tx, rx) = tokio::sync::broadcast::channel(32);

    // create listener
    let addr: std::net::SocketAddr = std::net::SocketAddr::new(
        args.env_args.bind_address.parse()?,
        args.env_args.port
    );
    let listener = tokio::net::TcpListener::bind(addr).await?;

    // "start" server
    let args = std::sync::Arc::new(args);
    println!("Server is listening!");
    loop {
        let (socket, addr) = listener.accept().await?;
        // spawn client handler
        tokio::spawn(
            client::handle_client(
                socket,
                addr,
                (tx.clone(), rx.resubscribe()),
                db.clone(),
                args.clone()
            )
        );
    }
}
