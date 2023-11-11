pub use anyhow::Result;

pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/pog_bilder.rs"));
}
pub mod reader_buffer;
pub mod db;
pub mod big_file;

#[inline(always)]
pub fn log_error<T, F: std::fmt::Debug>(val: Result<T, F>) -> Result<T, F> {
    val.map_err(|e| {
        println!("{e:?}");
        e
    })
}
