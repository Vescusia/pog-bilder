use crate::*;

use tokio_rusqlite::Connection;


/// create or connect to the database at `path`
///
/// If `path` `==` [`None`], then the database will be created in memory.
pub async fn create_or_connect_db(path: Option<&str>) -> Result<Connection> {
    let conn = if let Some(path) = path {
        // create the path
        let path = std::path::PathBuf::from(path);
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?
        }

        Connection::open(path).await
    }
    else {
        Connection::open_in_memory().await
    }?;

    conn.call(|conn| {
        conn.execute_batch(
            "BEGIN;
            CREATE TABLE IF NOT EXISTS senders (
                uuid INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS messages (
                timestamp REAL PRIMARY KEY,
                sender INTEGER NOT NULL,
                data BLOB NOT NULL,
                FOREIGN KEY (sender) REFERENCES senders(uuid)
            );
            COMMIT;
            "
    )}).await?;

    Ok(conn)
}


/// Warning: this function expects the [`messages::Sender`] to already be inserted into the database!
/// This will error otherwise! See [`add_sender`].
///
/// Also, no optional values (set to None) are allowed. Will panic otherwise.
pub async fn add_message(db: Connection, msg: &messages::Message) -> Result<()> {
    let timestamp = timestamp_to_real(
        msg.timestamp.as_ref().unwrap()
    );

    let sender = sender_to_u64(
        msg.sender.as_ref().unwrap()
    )?;

    let data = msg.data.as_ref().unwrap();
    let mut buf = Vec::with_capacity(data.encoded_len());
    data.encode(&mut buf);

    db.call(move |conn| {
        conn.execute(
            "INSERT INTO messages(timestamp, sender, data)
                 VALUES (?1, ?2, ?3)",
            (timestamp, sender, buf)
        )
    }).await?;

    Ok(())
}


fn timestamp_to_real(timestamp: &prost_types::Timestamp) -> f64 {
    let mut real = 0f64;
    real += timestamp.seconds as f64;
    real += (timestamp.nanos as f64) / 10f64.powf(9.0);
    real
}

fn sender_to_u64(sender: &messages::Sender) -> Result<u64> {
    let bytes = &sender.uuid;
    let (bytes, _) = bytes.split_at(8);
    Ok(u64::from_be_bytes(
        (*bytes).try_into()?
    ))
}


/// Adds a sender to the database
///
/// This function will handle adding of already existing senders gracefully and without error.
pub async fn add_sender(db: Connection, sender: &messages::Sender) -> Result<()> {
    let uuid = sender_to_u64(sender)?;
    let name = sender.name.clone();

    db.call(move |conn| {
        conn.execute(
            "INSERT OR IGNORE INTO senders(uuid, name)
                 VALUES (?1, ?2)",
            (uuid, name)
        )
    }).await?;

    Ok(())
}
