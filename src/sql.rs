use csv::ReaderBuilder;
use rusqlite::{params, Connection};
use anyhow::{anyhow, Result};

pub fn import_csv(conn: &mut Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS tracks (
            id        INTEGER PRIMARY KEY AUTOINCREMENT,
            artist    TEXT NOT NULL,
            song     TEXT NOT NULL,
            album     TEXT NOT NULL,
            played_at TEXT NOT NULL
        ) STRICT",
    )?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .flexible(false)
        .from_path("data.csv")?;

    let transaction = conn.transaction()?;
    {
        let mut stmt = transaction.prepare(
            "INSERT INTO tracks (artist, song, album, played_at) VALUES (?1, ?2, ?3, ?4)",
        )?;

        for result in rdr.records() {
            let record = result?;
            println!("add data entry");
            let artist = record.get(0).ok_or_else(|| anyhow!("missing artist"))?;
            let song = record.get(1).ok_or_else(|| anyhow!("missing name"))?;
            let album = record.get(2).ok_or_else(|| anyhow!("missing title"))?;
            let played_at = record
                .get(3)
                .ok_or_else(|| anyhow!("missing played_at"))?;

            stmt.execute(params![artist, song, album, played_at])?;
        }
    }

    transaction.commit()?;
    Ok(())
}
