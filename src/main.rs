use anyhow::Result;

mod csv;
mod lastfm;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Importing CSV data");
    csv::sync_markdown_from_csv("data.csv").await?;
    lastfm::run().await?;

    Ok(())
}
