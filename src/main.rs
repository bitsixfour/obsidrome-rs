use csv::ReaderBuilder;
use rusqlite::{params, Connection};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use time::OffsetDateTime;
use anyhow::{Context, Result};

mod sql;
use sql::import_csv;



#[tokio::main]
async fn main() -> Result<(), anyhow::Error>   {
    println!("Importing CSV data... \n
        I read /obsidianfm/data/*");
    let mut conn = Connection::open("music.db")?;
    sql::import_csv(&mut conn);
    Ok(())
}
