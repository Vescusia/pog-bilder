use pog_bilder::*;


use clap::Parser;

/// The Pog Bilder Server program.
///
/// Please see https://github.com/Vescusia/pog-bilder for more info.
///
/// For any `String` arguments, the `%BIN_DIR%` and `%PORT%` variables are available
/// and will be replaced with their respective values.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct EnvArgs {
    /// the Port the Server should run on
    #[arg(short, long)]
    pub port: u16,

    /// the address to bind to
    #[arg(long, default_value_t = String::from("0.0.0.0"))]
    pub bind_address: String,

    /// the Path at which the Messages database should be created
    ///
    /// No path means that it will be created in memory.
    #[arg(long, default_value_t = String::from("%BIN_DIR%/db/%PORT%.db"))]
    pub db_path: String,

    /// A "welcome message" that any first-time connecting clients will receive from the server
    #[arg(long)]
    pub welcome_message: Option<String>,

    /// The directory in which the big files will be saved.
    #[arg(long, default_value_t = String::from("%BIN_DIR%/files/%PORT%/"))]
    pub big_file_directory: String,
}


/// These Args are essentially the immutable state of the server which every thread needs.
pub struct ServerArgs{
    pub env_args: EnvArgs,
    pub default_sender: Option<messages::Sender>,
}


pub fn parse_args() -> ServerArgs {
    // parse args
    let mut env_args = EnvArgs::parse();

    // replace vars
    env_args.big_file_directory = replace_string_vars(&env_args.big_file_directory, &env_args);
    if let Some(welcome_msg) = env_args.welcome_message.as_ref() {
        env_args.welcome_message = Some(replace_string_vars(welcome_msg, &env_args));
    }
    env_args.db_path = replace_string_vars(&env_args.db_path, &env_args);

    // return ServerArgs
    ServerArgs{
        default_sender: Some(messages::Sender{
            name: "Server".to_owned(),
            uuid: env_args.port as u64,
        }),
        env_args,
    }
}


fn replace_string_vars(to_replace: &str, env_args: &EnvArgs) -> String {
    // replace %PORT%
    let mut to_replace = to_replace.replace("%PORT%", &env_args.port.to_string());

    // replace %BIN_PATH%
    let bin_dir: std::path::PathBuf = std::env::args().next()
        .unwrap()
        .parse()
        .unwrap();
    to_replace = to_replace.replace(
        "%BIN_DIR%",
        bin_dir.parent()
            .unwrap()
            .to_str()
            .unwrap()
    );

    to_replace
}


impl ServerArgs {
    pub fn generate_big_file_path(&self, ts: &prost_types::Timestamp) -> String {
        self.env_args.big_file_directory.clone() + &ts.to_string().replace(':', ";")
    }
}
