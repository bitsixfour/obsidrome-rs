use csv::ReaderBuilder;
use rusqlite::{params, Connection};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use time::OffsetDateTime;
use anyhow::{Context, Result};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// CSV File to read
    #[clap(short, long, default_value = "sample.csv")]
    file: String,

    /// Table name to create (default if file name of the CSV)
    #[clap(short, long)]
    table: Option<String>,

    /// Database name (default: data.db)
    #[clap(short, long, default_value = "data.db")]
    db: String,

    /// Batch size.
    ///
    /// When debugging: Reduce to 1 to identify the row that is causing the error.
    #[clap(short, long, default_value = "10000")]
    batch_size: usize,

    /// Dry run
    #[clap(long, default_value = "false")]
    dry_run: bool,

    /// Do not auto-detect types.
    /// All columns will be treated as strings.
    /// This is useful rows have mixed data types.
    #[clap(long, default_value = "false")]
    no_auto_detect_types: bool,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error>   {
    println!("Importing CSV data... \n I read /obsidianfm/data/*");
    Ok(())
}
