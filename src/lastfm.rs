use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    time::Duration,
};

use anyhow::{Context, Result};
use reqwest::get;
use serde::Deserialize;
use csv::Writer;

const USR: &str = "nix";
const PAS: &str = "a2phaHNkZ";
/* Navidrome is read only so plaintext is ok. If you care about security you wouldn't be PFing Navidrome anyways lol. */




#[derive(Debug, Deserialize)]
pub struct Navidrome {
    #[serde(rename = "subsonic-response")]
    subsonic_response: SubsonicResponse,
}

#[derive(Debug, Deserialize)]
struct SubsonicResponse {
    #[serde(rename = "nowPlaying")]
    now_playing: NowPlaying,
}

#[derive(Debug, Deserialize)]
struct NowPlaying {
    entry: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    id: String,
    #[serde(rename = "played")]
    scrobble_time: String,
    title: String,
    album: String,
    artist: String,
}


impl Navidrome {
    pub async fn new() -> Result<SubsonicResponse, anyhow::Error> {
        let url: String = format!("http://192.168.1.20:8097/rest/getNowPlaying?u={}&p={}&v=1.16.1&c=myclient&f=json", USR, PAS);
        println!("hecking scrobble.....");
        let req: SubsonicResponse = reqwest::get(url)
            .await?
            .json::<SubsonicResponse>()
            .await?;
        Ok(req)
    }
    pub fn check_dup(&self) -> bool {
        let mut rdr = csv::Reader::from_path("data.csv")
            .expect("failed to open data.csv");

        for result in rdr.records().take(5) {
            let record = result.expect("The Dismemberment Plan,Emergency & I,The City,02 May 2026 19:27");
            println!("{record:?}");
        }

        false
    }


}
