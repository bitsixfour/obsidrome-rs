use anyhow::Result;

mod csv;
mod sql;
mod lastfm;
use lastfm::Navidrome;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Importing CSV data");
    let mut album = ::csv::Reader::from_path("data.csv")?;
    csv::csv_md(&mut album).await;
    Navidrome::new();

    Ok(())

}
