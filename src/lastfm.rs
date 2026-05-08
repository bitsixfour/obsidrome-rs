use std::fs::OpenOptions;
use std::time::Duration;

use anyhow::{Context, Result};
use serde::Deserialize;
use tokio::time;

use crate::{
    config::AppConfig,
    csv::{self, NewScrobble},
};

#[derive(Debug, Deserialize)]
pub struct NavidromeResponse {
    #[serde(rename = "subsonic-response")]
    subsonic_response: SubsonicResponse,
}

#[derive(Debug, Deserialize)]
struct SubsonicResponse {
    #[serde(rename = "nowPlaying")]
    now_playing: Option<NowPlaying>,
}

#[derive(Debug, Deserialize)]
struct NowPlaying {
    entry: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    #[serde(rename = "played")]
    scrobble_time: String,
    title: String,
    album: String,
    artist: String,
}

impl Entry {
    fn as_scrobble(&self) -> NewScrobble {
        NewScrobble {
            artist: self.artist.clone(),
            album: self.album.clone(),
            song: self.title.clone(),
            played_at: self.scrobble_time.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ScrobbleKey {
    artist: String,
    album: String,
    song: String,
    played_at: String,
}

impl From<&NewScrobble> for ScrobbleKey {
    fn from(scrobble: &NewScrobble) -> Self {
        Self {
            artist: scrobble.artist.clone(),
            album: scrobble.album.clone(),
            song: scrobble.song.clone(),
            played_at: scrobble.played_at.clone(),
        }
    }
}

async fn fetch_now_playing(config: &AppConfig) -> Result<NavidromeResponse> {
    let url = format!(
        "{}/rest/getNowPlaying?u={}&p={}&v=1.16.1&c=scrobbler&f=json",
        config.navidrome_base_url, config.navidrome_user, config.navidrome_password
    );

    reqwest::get(url)
        .await
        .context("request to Navidrome failed")?
        .json::<NavidromeResponse>()
        .await
        .context("failed to deserialize Navidrome response")
}

fn csv_contains_scrobble(config: &AppConfig, scrobble: &NewScrobble) -> Result<bool> {
    let file = match OpenOptions::new().read(true).open(&config.csv_path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => {
            return Err(error).with_context(|| {
                format!(
                    "failed to open {} for duplicate check",
                    config.csv_path.display()
                )
            })
        }
    };

    let mut reader = ::csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    Ok(reader.records().any(|result| {
        result
            .ok()
            .and_then(|record| {
                Some(
                    record.get(0)? == scrobble.artist
                        && record.get(1)? == scrobble.album
                        && record.get(2)? == scrobble.song
                        && record.get(3)? == scrobble.played_at,
                )
            })
            .unwrap_or(false)
    }))
}

fn is_duplicate(
    config: &AppConfig,
    scrobble: &NewScrobble,
    last_scrobble: Option<&ScrobbleKey>,
) -> Result<bool> {
    let key = ScrobbleKey::from(scrobble);
    if last_scrobble == Some(&key) {
        return Ok(true);
    }

    csv_contains_scrobble(config, scrobble)
}

async fn tick(config: &AppConfig, last_scrobble: &mut Option<ScrobbleKey>) -> Result<()> {
    let navidrome = fetch_now_playing(config).await?;
    let entry = match navidrome
        .subsonic_response
        .now_playing
        .as_ref()
        .and_then(|now_playing| now_playing.entry.first())
    {
        Some(entry) => entry,
        None => {
            println!("nothing playing");
            return Ok(());
        }
    };

    let scrobble = entry.as_scrobble();
    let key = ScrobbleKey::from(&scrobble);
    if is_duplicate(config, &scrobble, last_scrobble.as_ref())? {
        println!(
            "duplicate -> skipping ({} / {} / {} / {})",
            scrobble.artist, scrobble.album, scrobble.song, scrobble.played_at
        );
        *last_scrobble = Some(key);
        return Ok(());
    }

    csv::append_scrobble(&scrobble, &config.csv_path)?;
    csv::write_scrobble_markdown(&config.csv_path, &config.vault_root).await?;
    *last_scrobble = Some(key);
    println!(
        "scrobbled -> {},{},{},{}",
        scrobble.artist, scrobble.album, scrobble.song, scrobble.played_at
    );

    Ok(())
}

pub async fn run(config: &AppConfig) -> Result<()> {
    println!("scrobbler started; polling every {}s", config.poll_secs);
    let mut interval = time::interval(Duration::from_secs(config.poll_secs));
    let mut last_scrobble = None;

    loop {
        interval.tick().await;
        if let Err(error) = tick(config, &mut last_scrobble).await {
            eprintln!("scrobbler error: {:#}", error);
        }
    }
}
