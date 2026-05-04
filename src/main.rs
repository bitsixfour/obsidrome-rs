use anyhow::Result;
use rusqlite::Connection;

mod csv;
mod sql;

#[tokio::main]
async fn main() -> Result<()> {
    /* may be depricated */
    println!("Importing CSV data");
    let mut _conn = Connection::open("data.db")?;
    // sql::import_csv(&mut conn);

    let mut album = ::csv::Reader::from_path("data.csv")?;
    csv::csv_md(&mut album)?;
    Ok(())
}
