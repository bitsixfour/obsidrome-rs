
use anyhow::Result;
use csv::Reader;
use csv::StringRecord;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::path::Path;

pub fn csv_md<R: Read>(album: &mut Reader<R>) -> Result<()> {
    for result in album.records() {
        let record: ::csv::StringRecord = result?;
        print!("importing scrobble: {} from  ", &record[0]);
        print!("{}", &record[1]);
        println!(" scrobbled on {}", &record[3]);
        write_md(&record)?;
    }
    Ok(())
}

pub fn write_md(data: &StringRecord) -> Result<()> {
    let artist = &data[0];
    let album = &data[1];
    let song = &data[2];
    let played_at = &data[3];

    println!("writing file for artist '{}' album '{}'", artist, album);

    let artist_file = format!("{}.md", sanitize_path_component(artist));
    let album_file = format!("{}.md", sanitize_path_component(album));
    let artist_path = Path::new("vault").join("artist").join(artist_file);
    let album_path = Path::new("vault").join("album").join(album_file);

    if let Some(parent) = artist_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    if let Some(parent) = album_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut artist_note = File::create(artist_path)?;
    writeln!(artist_note, "# {}", artist)?;
    writeln!(artist_note)?;
    writeln!(artist_note, "- artist: {}", artist)?;
    writeln!(artist_note, "- latest_album: {}", album)?;
    writeln!(artist_note, "- last_song: {}", song)?;
    writeln!(artist_note, "- played_at: {}", played_at)?;

    let mut album_note = File::create(album_path)?;
    writeln!(album_note, "# {}", album)?;
    writeln!(album_note)?;
    writeln!(album_note, "- artist: {}", artist)?;
    writeln!(album_note, "- album: {}", album)?;
    writeln!(album_note, "- last_song: {}", song)?;
    writeln!(album_note, "- played_at: {}", played_at)?;
    Ok(())
}

fn sanitize_path_component(value: &str) -> String {
    value.replace('/', "-")
}
