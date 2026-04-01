use crate::error::{Error, Result};

use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub browsers: HashMap<String, BrowserConfig>,
    pub paths: PathConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BrowserConfig {
    pub container: String,
    pub build_dir: String,
    pub main_worktree: String,
    #[serde(default)]
    pub use_wkdev: bool,
    #[serde(default)]
    pub use_gclient: bool,
    #[serde(default)]
    pub use_sccache: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PathConfig {
    pub root: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| "~/.config".into())
            .join("bx")
            .join("config.toml");

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            return toml::from_str(&content).map_err(Error::Config);
        }

        toml::from_str(include_str!("../resources/default_config.toml")).map_err(Error::Config)
    }
}
