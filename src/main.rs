use csv::ReaderBuilder;
use rusqlite::{params, Connection};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use time::OffsetDateTime;
use anyhow::{Context, Result};
use csv::Reader;

mod sql;
use sql::import_csv;



#[tokio::main]
async fn main() -> Result<(), anyhow::Error>   {
    /* may be depricated */
    println!("Importing CSV data... \n
        I read /obsidianfm/data/*");
    let mut conn = Connection::open("music.db")?;
    sql::import_csv(&mut conn);

    let mut album = csv::Reader::from_reader(io::stdin());
    for result in album.records() {
        println!("test");
    }
    Ok(())
}
