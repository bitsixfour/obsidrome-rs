const APIKEY: &str = "10200f41b2cdb8a4ec376f891db1f18b";
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ReleaseSearch {
    releases: Vec<Release>,
}

#[derive(Debug, Deserialize)]
struct Release {
    id: String,
    title: String,
}

#[derive(Debug, Deserialize)]
struct ReleaseDetail {
    genres: Option<Vec<Genre>>,
    tags: Option<Vec<Tag>>,
}

#[derive(Debug, Deserialize)]
struct Genre {
    name: String,
    count: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct Tag {
    name: String,
    count: Option<u32>,
}

#[tokio::main]
async fn main(stats: &AlbumStats) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = Client::new();

    let search: ReleaseSearch = client.get("https://musicbrainz.org/ws/2/release/").query(&[("query", "release:OK Computer AND artist:Radiohead"),
              ("fmt", "json"),]).header("User-Agent", "your-app/0.1.0 (you@example.com)")
          .send().await?.json().await?;

    let release_id = &search.releases[0].id;

    let detail: ReleaseDetail = client
        .get(format!("https://musicbrainz.org/ws/2/release/{}", release_id))
        .query(&[
            ("inc", "genres+tags"),
            ("fmt", "json"),
        ]).header("User-Agent", "your-app/0.1.0 (you@example.com)")
        .send().await?
        .json().await?;


  }
