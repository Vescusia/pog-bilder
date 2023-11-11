use prost::Message;
use crate::*;

use tokio_rusqlite::Connection;

// TODO: swap over to the SQLx library for better performance and ergonomics


/// create or connect to the database at `path`
///
/// If `path` `==` [`None`], then the database will be created in memory.
pub async fn create_or_connect_db(path: &str) -> Result<Connection> {
    let conn = if !path.is_empty() {
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
            COMMIT;"
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

    let sender = msg.sender
        .as_ref()
        .unwrap()
        .uuid;

    // This approach has some advantages and some disadvantages.
    // The sender and timestamp field are being saved twice.
    // But the message doesn't have to be re-encoded when being sent.
    // That is the path I will choose.
    let mut buf = Vec::with_capacity(msg.encoded_len());
    msg.encode_length_delimited(&mut buf)?;

    db.call(move |conn| {
        conn.execute(
            "INSERT INTO messages(timestamp, sender, data)
                 VALUES (?1, ?2, ?3)",
            (timestamp, sender, buf)
        )
    }).await?;

    Ok(())
}


/// Adds a sender to the database
///
/// This function will handle adding of already existing senders gracefully and without error.
pub async fn add_sender(db: Connection, sender: &messages::Sender) -> Result<()> {
    let (name, uuid) = (sender.name.clone(), sender.uuid);

    db.call(move |conn| {
        conn.execute(
            "INSERT OR IGNORE INTO senders(uuid, name)
                 VALUES (?1, ?2)",
            (uuid, name)
        )
    }).await?;

    Ok(())
}


/// Selects all the [`messages::Message`]'s sent since `since`.
///
/// The [`messages::Message`]'s will be passed through a [`tokio::sync::mpsc::Receiver`] channel.
/// They are encoded in length delimited byte-form.
pub async fn select_messages_since(db: Connection, sender: &messages::Sender, since: &prost_types::Timestamp) -> Result<tokio::sync::mpsc::Receiver<Vec<u8>>> {
    let (since, sender) = (timestamp_to_real(since), sender.uuid);
    let (tx, rx) = tokio::sync::mpsc::channel(32);

    db.call(move |conn| {
        // prepare select
        let mut stmt = conn.prepare_cached("
            SELECT data
            FROM messages
            WHERE
                timestamp > ?1 AND
                sender != ?2
        ")?;

        // execute select
        let messages = stmt.query_map(
            (since, sender),
            |row| {
                row.get(0)
            }
        )?;

        // send messages
        for message in messages.flatten() {
            let _ = tx.blocking_send(message);
        }
        Ok(())
    }).await?;

    Ok(rx)
}


fn timestamp_to_real(timestamp: &prost_types::Timestamp) -> f64 {
    let mut real = 0f64;
    real += timestamp.seconds as f64;
    real += (timestamp.nanos as f64) / 10f64.powf(9.0);
    real
}
