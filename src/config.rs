use crate::error::{Error, Result};

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub browsers: HashMap<String, BrowserConfig>,
    pub root: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BrowserConfig {
    // pub build_dir: String,
    #[serde(default)]
    pub container_bootstrap: Option<String>,
    #[serde(default)]
    pub container_packages: Vec<String>,
    #[serde(default)]
    pub remote: Option<String>,
    #[serde(default)]
    pub use_wkdev: bool,
    #[serde(default)]
    pub use_gclient: bool,
    #[serde(default)]
    pub use_sccache: bool,
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

        toml::from_str(include_str!("../resources/config.toml")).map_err(Error::Config)
    }
}
