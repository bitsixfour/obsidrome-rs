use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    time::Duration,
};

use anyhow::{Context, Result};
use chrono::Local;
use reqwest::blocking::Client;
use serde::Deserialize;

const USR: &str = "nix";
const PAS: &str = "a2phaHNkZ";
/* Navidrome is read only so plaintext is ok. If you care about security you wouldn't be PFing Navidrome anyways lol. */
#[derive(Deserialize)]
struct SubsonicResponse {
    #[serde(rename = "subsonic-response")]
    inner: SubsonicInner,
}

#[derive(Deserialize)]
struct SubsonicInner {
    #[serde(rename = "nowPlaying")]
    now_playing: Option<NowPlaying>,
}

#[derive(Deserialize)]
struct NowPlaying {
    entry: Option<Vec<Entry>>,
}

#[derive(Deserialize)]
struct Entry {
    id: String,
    title: String,
    album: String,
    #[serde(rename = "displayArtist")]
    display_artist: String,
    year: Option<u32>,
    #[serde(rename = "minutesAgo")]
    minutes_ago: u64,
}


#[test]
pub async fn call_api() -> Result<SubsonicResponse, anyhow::Error {
    let url: &str = format!("http://192.168.1.20:8097/rest/getNowPlaying?u={}&p={}&v=1.16.1&c=myclient&f=json", USR, PAS);
    let req = reqwest::get(url)
        .await?
        .text()
        .await?;
    Ok(resp)
}
