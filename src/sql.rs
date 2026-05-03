use csv::ReaderBuilder;
use rusqlite::{params, Connection};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use time::OffsetDateTime;
use std::io;
use anyhow::{Context, Result, anyhow};

/* add proper error handling later */

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

    let mut stmt = conn.prepare(
        "INSERT INTO tracks (artist, song, album, played_at) VALUES (?1, ?2, ?3, ?4)",
    )?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .flexible(false)
        .from_path("data.csv")?;
    conn.execute("BEGIN", [])?;

    for result in rdr.records() {
        let record = result?;
        println!("add data entry");
        let artist = record.get(0).ok_or_else(|| anyhow!("missing artist"))?;
        let song = record.get(1).ok_or_else(|| anyhow!("missing name"))?;
        let album = record.get(2).ok_or_else(|| anyhow!("missing title"))?;
        let played_at = record.get(3)
            .ok_or_else(|| anyhow!("missing played_at"))?;

        stmt.execute(params![artist, song, album, played_at])?;
    }

    conn.execute("COMMIT", [])?;
    Ok(())
}
