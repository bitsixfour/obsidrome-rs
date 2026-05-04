use anyhow::{anyhow, Context, Result};
use csv::{Reader, StringRecord};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration, Instant};

const VAULT_ROOT: &str = "/home/will/Documents/Obsidian Vault";
const MUSICBRAINZ_USER_AGENT: &str = "obsidianfm/0.1.0 (https://wngyn.net)";
const API_REQUEST_INTERVAL: Duration = Duration::from_secs(1);

pub async fn csv_md<R: Read>(reader: &mut Reader<R>) -> Result<()> {
    let library = collect_library_stats(reader)?;
    let writer = VaultWriter::new(PathBuf::from(VAULT_ROOT))?;

    for stats in library.artist_stats.values() {
        writer.write_artist_note(stats)?;
    }

    for stats in library.album_stats.values() {
        writer.write_album_note(stats).await?;
    }

    Ok(())
}

fn collect_library_stats<R: Read>(reader: &mut Reader<R>) -> Result<LibraryStats> {
    let mut album_stats = HashMap::new();
    let mut artist_stats = HashMap::new();

    for result in reader.records() {
        let record = result?;
        let scrobble = ScrobbleRecord::from_csv(&record)?;

        println!(
            "importing scrobble: {} from {} scrobbled on {}",
            scrobble.artist, scrobble.album, scrobble.played_at
        );

        let album_key = (scrobble.artist.to_string(), scrobble.album.to_string());
        album_stats
            .entry(album_key)
            .or_insert_with(|| AlbumStats::new(scrobble.artist, scrobble.album))
            .record_scrobble(scrobble.song, scrobble.played_at);

        artist_stats
            .entry(scrobble.artist.to_string())
            .or_insert_with(|| ArtistStats::new(scrobble.artist))
            .record_scrobble(scrobble.album, scrobble.song, scrobble.played_at);
    }

    Ok(LibraryStats {
        album_stats,
        artist_stats,
    })
}

struct LibraryStats {
    album_stats: HashMap<(String, String), AlbumStats>,
    artist_stats: HashMap<String, ArtistStats>,
}

struct ScrobbleRecord<'a> {
    artist: &'a str,
    album: &'a str,
    song: &'a str,
    played_at: &'a str,
}

impl<'a> ScrobbleRecord<'a> {
    fn from_csv(record: &'a StringRecord) -> Result<Self> {
        Ok(Self {
            artist: get_field(record, 0, "artist")?,
            album: get_field(record, 1, "album")?,
            song: get_field(record, 2, "song")?,
            played_at: get_field(record, 3, "played_at")?,
        })
    }
}

struct VaultWriter {
    vault_root: PathBuf,
    musicbrainz: RateLimitedClient,
}

impl VaultWriter {
    fn new(vault_root: PathBuf) -> Result<Self> {
        Ok(Self {
            vault_root,
            musicbrainz: RateLimitedClient::new()?,
        })
    }

    fn write_artist_note(&self, stats: &ArtistStats) -> Result<()> {
        let path = self.note_path("artist", &stats.artist);
        let body = render_artist_note(stats);
        self.write_note(&path, &body)
    }

    async fn write_album_note(&self, stats: &AlbumStats) -> Result<()> {
        let path = self.note_path("album", &stats.album);
        let metadata = self.resolve_album_metadata(&path, stats).await?;

        self.write_genre_notes(&metadata.genres)?;

        let body = render_album_note(stats, &metadata);
        self.write_note(&path, &body)
    }

    async fn resolve_album_metadata(
        &self,
        album_path: &Path,
        stats: &AlbumStats,
    ) -> Result<AlbumMetadata> {
        if let Some(metadata) = load_existing_album_metadata(album_path)? {
            return Ok(metadata);
        }

        match self.fetch_album_metadata(stats).await {
            Ok(metadata) => Ok(metadata),
            Err(error) => {
                eprintln!(
                    "failed to fetch MusicBrainz metadata for {} - {}: {}",
                    stats.artist, stats.album, error
                );
                Ok(AlbumMetadata::default())
            }
        }
    }

    async fn fetch_album_metadata(&self, stats: &AlbumStats) -> Result<AlbumMetadata> {
        let query = format!("release:\"{}\" AND artist:\"{}\"", stats.album, stats.artist);

        let search: ReleaseSearch = self
            .musicbrainz
            .get_json(
                "https://musicbrainz.org/ws/2/release/",
                &[("query", query.as_str()), ("fmt", "json"), ("limit", "1")],
            )
            .await?;

        let release = match search.releases.first() {
            Some(release) => release,
            None => return Ok(AlbumMetadata::default()),
        };

        let detail: ReleaseDetail = self
            .musicbrainz
            .get_json(
                &format!("https://musicbrainz.org/ws/2/release/{}", release.id),
                &[("inc", "genres+tags+release-groups"), ("fmt", "json")],
            )
            .await?;

        let release_group_id = detail.release_group.as_ref().map(|group| group.id.clone());
        let cover_art_url = match resolve_cover_art_url(
            &self.musicbrainz,
            release.id.as_str(),
            release_group_id.as_deref(),
        )
        .await
        {
            Ok(url) => url,
            Err(error) => {
                eprintln!(
                    "failed to resolve cover art for {} - {}: {}",
                    stats.artist, stats.album, error
                );
                None
            }
        };

        let release_genres = normalize_genres(detail.genres, detail.tags);

        let genres = if let Some(release_group_id) = release_group_id.as_deref() {
            match self
                .musicbrainz
                .get_json::<ReleaseGroupDetail>(
                    &format!("https://musicbrainz.org/ws/2/release-group/{}", release_group_id),
                    &[("inc", "genres+tags"), ("fmt", "json")],
                )
                .await
            {
                Ok(release_group_detail) => {
                    let release_group_genres =
                        normalize_genres(release_group_detail.genres, release_group_detail.tags);
                    if release_group_genres.is_empty() {
                        release_genres
                    } else {
                        release_group_genres
                    }
                }
                Err(error) => {
                    eprintln!(
                        "failed to fetch release-group genres for {} - {}: {}",
                        stats.artist, stats.album, error
                    );
                    release_genres
                }
            }
        } else {
            release_genres
        };

        Ok(AlbumMetadata {
            release_url: Some(format!("https://musicbrainz.org/release/{}", release.id)),
            cover_art_url,
            cover_art_page_url: Some(format!(
                "https://coverartarchive.org/release/{}/",
                release.id
            )),
            genres,
        })
    }

    fn write_genre_notes(&self, genres: &[Genre]) -> Result<()> {
        for genre in genres {
            let path = self.note_path("genre", &genre.name);
            let body = render_genre_note(genre);
            self.write_note(&path, &body)?;
        }

        Ok(())
    }

    fn note_path(&self, directory: &str, name: &str) -> PathBuf {
        let file_name = format!("{}.md", sanitize_path_component(name));
        self.vault_root.join(directory).join(file_name)
    }

    fn write_note(&self, path: &Path, body: &str) -> Result<()> {
        ensure_parent_dir(path)?;

        let mut note = File::create(path)
            .with_context(|| format!("failed to create note at {}", path.display()))?;
        note.write_all(body.as_bytes())
            .with_context(|| format!("failed to write note at {}", path.display()))?;
        Ok(())
    }
}

fn render_artist_note(stats: &ArtistStats) -> String {
    let mut body = String::new();
    writeln!(&mut body, "# {}", stats.artist).unwrap();
    writeln!(&mut body).unwrap();
    writeln!(&mut body, "- artist: {}", stats.artist).unwrap();
    writeln!(&mut body, "- scrobble_count: {}", stats.scrobble_count).unwrap();
    writeln!(&mut body, "- album_count: {}", stats.albums.len()).unwrap();
    writeln!(&mut body, "- latest_album: [[{}]]", stats.last_album).unwrap();
    writeln!(&mut body, "- last_song: {}", stats.last_song).unwrap();
    writeln!(&mut body, "- played_at: {}", stats.last_played_at).unwrap();
    body
}

fn render_album_note(stats: &AlbumStats, metadata: &AlbumMetadata) -> String {
    let mut body = String::new();
    writeln!(&mut body, "# {}", stats.album).unwrap();
    writeln!(&mut body).unwrap();

    if let Some(cover_art_url) = metadata.cover_art_url.as_deref() {
        writeln!(&mut body, "![{} cover]({})", stats.album, cover_art_url).unwrap();
        writeln!(&mut body).unwrap();
    }

    writeln!(&mut body, "- artist: [[{}]]", stats.artist).unwrap();
    writeln!(&mut body, "- album: {}", stats.album).unwrap();
    writeln!(&mut body, "- scrobble_count: {}", stats.scrobble_count).unwrap();
    writeln!(&mut body, "- last_song: {}", stats.last_song).unwrap();
    writeln!(&mut body, "- played_at: {}", stats.last_played_at).unwrap();
    writeln!(&mut body, "- genres: {}", format_genre_links(&metadata.genres)).unwrap();

    if let Some(release_url) = metadata.release_url.as_deref() {
        writeln!(&mut body, "- musicbrainz: [release]({})", release_url).unwrap();
    }

    if let Some(cover_art_page_url) = metadata.cover_art_page_url.as_deref() {
        writeln!(
            &mut body,
            "- cover_art: [Cover Art Archive]({})",
            cover_art_page_url
        )
        .unwrap();
    }

    body
}

fn render_genre_note(genre: &Genre) -> String {
    let mut body = String::new();
    writeln!(&mut body, "# {}", genre.name).unwrap();
    writeln!(&mut body).unwrap();
    writeln!(&mut body, "- genre: {}", genre.name).unwrap();

    if let Some(count) = genre.count {
        writeln!(&mut body, "- musicbrainz_count: {}", count).unwrap();
    }

    body
}

fn load_existing_album_metadata(path: &Path) -> Result<Option<AlbumMetadata>> {
    if !path.exists() {
        return Ok(None);
    }

    let note = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read existing note at {}", path.display()))?;

    let metadata = AlbumMetadata {
        cover_art_url: note.lines().find_map(parse_image_embed_target),
        release_url: note
            .lines()
            .find_map(|line| parse_markdown_link_target(line, "- musicbrainz: [release](")),
        cover_art_page_url: note.lines().find_map(|line| {
            parse_markdown_link_target(line, "- cover_art: [Cover Art Archive](")
        }),
        genres: note.lines().find_map(parse_genres_line).unwrap_or_default(),
    };

    if metadata.is_empty() {
        Ok(None)
    } else {
        Ok(Some(metadata))
    }
}

fn parse_markdown_link_target(line: &str, prefix: &str) -> Option<String> {
    Some(line.strip_prefix(prefix)?.strip_suffix(')')?.to_string())
}

fn parse_image_embed_target(line: &str) -> Option<String> {
    if !line.starts_with("![") {
        return None;
    }

    let (_, url) = line.split_once("](")?;
    Some(url.strip_suffix(')')?.to_string())
}

fn parse_genres_line(line: &str) -> Option<Vec<Genre>> {
    let raw = line.strip_prefix("- genres: ")?;
    if raw == "none" {
        return Some(Vec::new());
    }

    Some(raw.split(", ").filter_map(parse_genre_item).collect())
}

fn parse_genre_item(raw: &str) -> Option<Genre> {
    if raw.is_empty() || raw == "none" {
        return None;
    }

    if let Some((_, display)) = raw.split_once('|') {
        return Some(Genre {
            name: display.strip_suffix("]]")?.to_string(),
            count: None,
        });
    }

    let name = if let Some(wikilink) = raw.strip_prefix("[[").and_then(|value| value.strip_suffix("]]"))
    {
        wikilink.rsplit('/').next().unwrap_or(wikilink).to_string()
    } else {
        raw.to_string()
    };

    Some(Genre { name, count: None })
}

fn normalize_genres(genres: Option<Vec<Genre>>, tags: Option<Vec<Tag>>) -> Vec<Genre> {
    let mut genres = genres.unwrap_or_default();
    if genres.is_empty() {
        genres = tags
            .unwrap_or_default()
            .into_iter()
            .map(|tag| Genre {
                name: tag.name,
                count: tag.count,
            })
            .collect();
    }

    genres.sort_by(|left, right| {
        right
            .count
            .unwrap_or_default()
            .cmp(&left.count.unwrap_or_default())
            .then_with(|| left.name.cmp(&right.name))
    });
    genres.dedup_by(|left, right| left.name.eq_ignore_ascii_case(&right.name));
    genres
}

fn format_genre_links(genres: &[Genre]) -> String {
    if genres.is_empty() {
        return "none".to_string();
    }

    genres
        .iter()
        .map(|genre| {
            format!(
                "[[genre/{}|{}]]",
                sanitize_path_component(&genre.name),
                genre.name
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn ensure_parent_dir(path: &Path) -> Result<()> {
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
    value.replace('/', "-").replace('\\', "-")
}

struct AlbumStats {
    artist: String,
    album: String,
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

    fn record_scrobble(&mut self, song: &str, played_at: &str) {
        self.scrobble_count += 1;
        self.last_song = song.to_string();
        self.last_played_at = played_at.to_string();
    }
}

struct ArtistStats {
    artist: String,
    scrobble_count: usize,
    last_album: String,
    last_song: String,
    last_played_at: String,
    albums: HashSet<String>,
}

impl ArtistStats {
    fn new(artist: &str) -> Self {
        Self {
            artist: artist.to_string(),
            scrobble_count: 0,
            last_album: String::new(),
            last_song: String::new(),
            last_played_at: String::new(),
            albums: HashSet::new(),
        }
    }

    fn record_scrobble(&mut self, album: &str, song: &str, played_at: &str) {
        self.scrobble_count += 1;
        self.last_album = album.to_string();
        self.last_song = song.to_string();
        self.last_played_at = played_at.to_string();
        self.albums.insert(album.to_string());
    }
}

#[derive(Debug, Deserialize)]
struct ReleaseSearch {
    releases: Vec<Release>,
}

#[derive(Debug, Deserialize)]
struct Release {
    id: String,
}

#[derive(Debug, Deserialize)]
struct ReleaseDetail {
    genres: Option<Vec<Genre>>,
    tags: Option<Vec<Tag>>,
    #[serde(rename = "release-group")]
    release_group: Option<ReleaseGroup>,
}

#[derive(Debug, Deserialize)]
struct ReleaseGroup {
    id: String,
}

#[derive(Debug, Deserialize)]
struct ReleaseGroupDetail {
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

#[derive(Default)]
struct AlbumMetadata {
    release_url: Option<String>,
    cover_art_url: Option<String>,
    cover_art_page_url: Option<String>,
    genres: Vec<Genre>,
}

impl AlbumMetadata {
    fn is_empty(&self) -> bool {
        self.release_url.is_none()
            && self.cover_art_url.is_none()
            && self.cover_art_page_url.is_none()
            && self.genres.is_empty()
    }
}

async fn resolve_cover_art_url(
    client: &RateLimitedClient,
    release_id: &str,
    release_group_id: Option<&str>,
) -> Result<Option<String>> {
    let mut candidates = Vec::new();

    if let Some(release_group_id) = release_group_id {
        candidates.push(format!(
            "https://coverartarchive.org/release-group/{}/front-500",
            release_group_id
        ));
    }

    candidates.push(format!(
        "https://coverartarchive.org/release/{}/front-500",
        release_id
    ));

    for candidate in candidates {
        if client.head_exists(&candidate).await? {
            return Ok(Some(candidate));
        }
    }

    Ok(None)
}

struct RateLimitedClient {
    client: Client,
    last_request_at: Mutex<Option<Instant>>,
}

impl RateLimitedClient {
    fn new() -> Result<Self> {
        Ok(Self {
            client: Client::builder()
                .user_agent(MUSICBRAINZ_USER_AGENT)
                .build()?,
            last_request_at: Mutex::new(None),
        })
    }

    async fn get_json<T>(&self, url: &str, query: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.throttle().await;

        Ok(self
            .client
            .get(url)
            .query(query)
            .send()
            .await?
            .error_for_status()?
            .json::<T>()
            .await?)
    }

    async fn head_exists(&self, url: &str) -> Result<bool> {
        self.throttle().await;
        let response = self.client.head(url).send().await?;
        Ok(response.status().is_success() || response.status().is_redirection())
    }

    async fn throttle(&self) {
        let mut last_request_at = self.last_request_at.lock().await;
        if let Some(previous_request) = *last_request_at {
            let elapsed = previous_request.elapsed();
            if elapsed < API_REQUEST_INTERVAL {
                sleep(API_REQUEST_INTERVAL - elapsed).await;
            }
        }
        *last_request_at = Some(Instant::now());
    }
}
