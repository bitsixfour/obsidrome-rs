use std::env;
use std::path::PathBuf;

use anyhow::{Context, Result};

const DEFAULT_CSV_PATH: &str = "data.csv";
const DEFAULT_VAULT_ROOT: &str = "/home/will/Documents/Obsidian Vault";
const DEFAULT_NAVIDROME_BASE_URL: &str = "http://192.168.1.20:8097";
const DEFAULT_NAVIDROME_USER: &str = "nix";
const DEFAULT_NAVIDROME_PASSWORD: &str = "2008";
const DEFAULT_POLL_SECS: u64 = 30;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub csv_path: PathBuf,
    pub vault_root: PathBuf,
    pub navidrome_base_url: String,
    pub navidrome_user: String,
    pub navidrome_password: String,
    pub poll_secs: u64,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            csv_path: env_path("OBSIDIANFM_CSV_PATH")
                .unwrap_or_else(|| PathBuf::from(DEFAULT_CSV_PATH)),
            vault_root: env_path("OBSIDIANFM_VAULT_ROOT")
                .unwrap_or_else(|| PathBuf::from(DEFAULT_VAULT_ROOT)),
            navidrome_base_url: env_string("OBSIDIANFM_NAVIDROME_BASE_URL")
                .unwrap_or_else(|| DEFAULT_NAVIDROME_BASE_URL.to_string())
                .trim_end_matches('/')
                .to_string(),
            navidrome_user: env_string("OBSIDIANFM_NAVIDROME_USER")
                .unwrap_or_else(|| DEFAULT_NAVIDROME_USER.to_string()),
            navidrome_password: env_string("OBSIDIANFM_NAVIDROME_PASSWORD")
                .unwrap_or_else(|| DEFAULT_NAVIDROME_PASSWORD.to_string()),
            poll_secs: env_u64("OBSIDIANFM_POLL_SECS")?.unwrap_or(DEFAULT_POLL_SECS),
        })
    }
}

fn env_string(key: &str) -> Option<String> {
    env::var(key).ok().filter(|value| !value.trim().is_empty())
}

fn env_path(key: &str) -> Option<PathBuf> {
    env::var_os(key).filter(|value| !value.is_empty()).map(PathBuf::from)
}

fn env_u64(key: &str) -> Result<Option<u64>> {
    match env_string(key) {
        Some(value) => value
            .parse::<u64>()
            .with_context(|| format!("{} must be a positive integer", key))
            .map(Some),
        None => Ok(None),
    }
}
