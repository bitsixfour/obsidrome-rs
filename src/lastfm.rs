use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    time::Duration,
};
use csv::StringRecord;
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
pub struct SubsonicResponse {
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



struct ScnrData {
    name: String,
    song: String,
    artist: String,
    date: String,
}
impl ScnrData {

    pub fn new(rec: &StringRecord) -> Result<ScnrData> {
        Ok(Self {
            name: "Str".to_string(),
            song: "str".to_string(),
            artist: "str".to_string(),
            date: "str".to_string(),
        })

    }
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
            let compare =  ScnrData::new(&record);

        }

        false
    }


}
