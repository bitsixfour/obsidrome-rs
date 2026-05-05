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
/* Navidrome is read only so plaintext is ok. Even if you PF your Navidrome a "threat actor" can't really do shit. */

#[derive(Debug, Deserialize)]
struct SubsonicResponse {
    inner: SubsonicInner,
}

#[derive(Debug, Deserialize)]
struct SubsonicInner {
    album_list2: Option<AlbumList2>,
}

#[derive(Debug, Deserialize)]
struct AlbumList2 {
    album: Option<Vec<Album>>,
}
