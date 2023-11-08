pub use anyhow::Result;

pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/pog_bilder.rs"));
}
pub mod reader_buffer;
pub mod db;
