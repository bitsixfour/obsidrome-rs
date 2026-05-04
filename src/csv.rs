use anyhow::{anyhow, Result};
use csv::Reader;
use csv::StringRecord;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::path::{Path, PathBuf};

const VAULT_ROOT: &str = "/home/will/Documents/Obsidian Vault";

pub fn csv_md<R: Read>(album: &mut Reader<R>) -> Result<()> {
    let mut album_stats: HashMap<(String, String), AlbumStats> = HashMap::new();
    let mut artist_stats: HashMap<String, ArtistStats> = HashMap::new();

    for result in album.records() {
        let record = result?;
        let artist = get_field(&record, 0, "artist")?;
        let album_name = get_field(&record, 1, "album")?;
        let song = get_field(&record, 2, "song")?;
        let played_at = get_field(&record, 3, "played_at")?;

        println!(
            "importing scrobble: {} from {} scrobbled on {}",
            artist, album_name, played_at
        );

        let album_key = (artist.to_string(), album_name.to_string());
        let album_entry = album_stats
            .entry(album_key)
            .or_insert_with(|| AlbumStats::new(artist, album_name));
        album_entry.scrobble_count += 1;
        album_entry.last_song = song.to_string();
        album_entry.last_played_at = played_at.to_string();

        let artist_entry = artist_stats
            .entry(artist.to_string())
            .or_insert_with(|| ArtistStats::new(artist));
        artist_entry.scrobble_count += 1;
        artist_entry.last_album = album_name.to_string();
        artist_entry.last_song = song.to_string();
        artist_entry.last_played_at = played_at.to_string();
        artist_entry.albums.insert(album_name.to_string(), ());
    }

    for stats in artist_stats.values() {
        write_artist_md(stats)?;
    }

    for stats in album_stats.values() {
        write_album_md(stats)?;
    }

    Ok(())
}

fn write_artist_md(stats: &ArtistStats) -> Result<()> {
    let artist_file = format!("{}.md", sanitize_path_component(&stats.artist));
    let artist_path = Path::new(VAULT_ROOT).join("artist").join(artist_file);

    ensure_parent_dir(&artist_path)?;

    let mut artist_note = File::create(artist_path)?;
    writeln!(artist_note, "# {}", stats.artist)?;
    writeln!(artist_note)?;
    writeln!(artist_note, "- artist: {}", stats.artist)?;
    writeln!(artist_note, "- scrobble_count: {}", stats.scrobble_count)?;
    writeln!(artist_note, "- album_count: {}", stats.albums.len())?;
    writeln!(artist_note, "- latest_album: [[{}]]", stats.last_album)?;
    writeln!(artist_note, "- last_song: {}", stats.last_song)?;
    writeln!(artist_note, "- played_at: {}", stats.last_played_at)?;
    Ok(())
}

fn write_album_md(stats: &AlbumStats) -> Result<()> {
    let album_file = format!("{}.md", sanitize_path_component(&stats.album));
    let album_path = Path::new(VAULT_ROOT).join("album").join(album_file);

    ensure_parent_dir(&album_path)?;

    let mut album_note = File::create(album_path)?;
    writeln!(album_note, "# {}", stats.album)?;
    writeln!(album_note)?;
    writeln!(album_note, "- artist: [[{}]]", stats.artist)?;
    writeln!(album_note, "- album: {}", stats.album)?;
    writeln!(album_note, "- scrobble_count: {}", stats.scrobble_count)?;
    writeln!(album_note, "- last_song: {}", stats.last_song)?;
    writeln!(album_note, "- played_at: {}", stats.last_played_at)?;
    Ok(())
}

fn ensure_parent_dir(path: &PathBuf) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn get_field<'a>(record: &'a StringRecord, index: usize, name: &str) -> Result<&'a str> {
    record
        .get(index)
        .ok_or_else(|| anyhow!("missing {} at column {}", name, index))
}

fn sanitize_path_component(value: &str) -> String {
    value.replace('/', "-")
}

pub struct AlbumStats {
    pub artist: String,
    pub album: String,
    scrobble_count: usize,
    last_song: String,
    last_played_at: String,
}

impl AlbumStats {
    fn new(artist: &str, album: &str) -> Self {
        Self {
            artist: artist.to_string(),
            album: album.to_string(),
            scrobble_count: 0,
            last_song: String::new(),
            last_played_at: String::new(),
        }
    }
}

struct ArtistStats {
    artist: String,
    scrobble_count: usize,
    last_album: String,
    last_song: String,
    last_played_at: String,
    albums: HashMap<String, ()>,
}

impl ArtistStats {
    fn new(artist: &str) -> Self {
        Self {
            artist: artist.to_string(),
            scrobble_count: 0,
            last_album: String::new(),
            last_song: String::new(),
            last_played_at: String::new(),
            albums: HashMap::new(),
        }
    }
}
