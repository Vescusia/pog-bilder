use clap::Parser;

/// The Pog Bilder Server program.
///
/// Please see https://github.com/Vescusia/pog-bilder for more info.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// the Port the Server should run on
    #[arg(short, long)]
    pub port: u16,

    /// the address to bind to
    #[arg(long, default_value_t = String::from("0.0.0.0"))]
    pub bind_address: String
}