use std::{
    fs,
    io,
    path::{Path, PathBuf},
};

use crate::models::config::AppConfig;

pub struct ConfigStore;

impl ConfigStore {
    pub fn load() -> AppConfig {
        let path = Self::config_path();

        match fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str::<AppConfig>(&content).unwrap_or_default(),
            Err(_) => AppConfig::default(),
        }
    }

    pub fn save(config: &AppConfig) -> io::Result<()> {
        let path = Self::config_path();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(config)
            .map_err(|error| io::Error::other(error.to_string()))?;

        fs::write(path, content)
    }

    fn config_path() -> PathBuf {
        workspace_data_dir().join("neko-config.json")
    }
}

fn workspace_data_dir() -> PathBuf {
    Path::new(".").join(".local").join("config")
}
