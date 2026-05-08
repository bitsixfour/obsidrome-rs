use anyhow::Result;

mod config;
mod csv;
mod lastfm;

#[tokio::main]
async fn main() -> Result<()> {
    let app_config = config::AppConfig::from_env()?;

    println!("Importing CSV data");
    csv::sync_markdown_from_csv(&app_config.csv_path, &app_config.vault_root).await?;
    lastfm::run(&app_config).await?;

    Ok(())
}
