use crate::error::{Error, Result};

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use toml::{Table, Value};

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    pub commands: HashMap<String, String>,
    pub container_packages: Vec<String>,
    pub env: Vec<String>,
    pub remote: Option<String>,
    pub use_wkdev: bool,
}

pub struct Browser {
    pub name: String,
    pub config: Config,
    pub root: PathBuf,
}

const SOURCES: &[&'static str] = &[
    include_str!("../resources/servo.toml"),
    include_str!("../resources/firefox.toml"),
    include_str!("../resources/webkit.toml"),
    include_str!("../resources/chromium.toml"),
];

impl Browser {
    fn root() -> Result<PathBuf> {
        std::env::var("WORK_DIR")
            .ok()
            .map(PathBuf::from)
            .or(dirs::home_dir())
            .ok_or(Error::NoWorkDir)
    }

    fn name() -> Result<String> {
        let cwd = std::env::current_dir()?;
        let browser = cwd
            .strip_prefix(&Self::root()?)
            .ok()
            .and_then(|path| path.components().next())
            .ok_or(Error::NotInBrowserDir)?;
        Ok(browser.as_os_str().to_string_lossy().into())
    }

    fn get_configs(root: &Path, name: &str) -> Result<Vec<String>> {
        let mut configs = Vec::new();

        for &source in SOURCES {
            configs.push(source.to_string());
        }

        if let Some(config_dir) = dirs::config_dir()
            .map(|d| d.join("brrr"))
            .filter(|d| d.exists())
        {
            for entry in fs::read_dir(config_dir)?.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        configs.push(content);
                    }
                }
            }
        }

        let project_config = root.join(name).join(".brrr.toml");
        if project_config.exists() {
            if let Ok(content) = fs::read_to_string(&project_config) {
                configs.push(content);
            }
        }

        Ok(configs)
    }

    fn merge_tables(target: &mut toml::Table, source: &toml::Table) {
        for (key, value) in source {
            if let (Some(Value::Table(target_table)), Value::Table(source_table)) =
                (target.get_mut(key), value)
            {
                Self::merge_tables(target_table, source_table);
            } else {
                target.insert(key.clone(), value.clone());
            }
        }
    }

    pub fn discover() -> Result<Self> {
        let root = Self::root()?;
        let name = Self::name()?;

        let mut table = Table::new();
        for config_str in &Self::get_configs(&root, &name)? {
            if let Ok(toml::Value::Table(source_table)) = toml::from_str(config_str) {
                Self::merge_tables(&mut table, &source_table);
            }
        }

        let all_configs: HashMap<String, Config> = toml::Value::Table(table)
            .try_into()
            .map_err(Error::Config)?;

        let config = all_configs
            .get(&name)
            .cloned()
            .ok_or(Error::NotInBrowserDir)?;

        let root = root.join(&name);

        Ok(Browser {
            name,
            config,
            root,
        })
    }

    pub fn fetch_remote(&self) -> Result<()> {
        let Some(remote) = &self.config.remote else {
            println!("There is no configured remote for {}", self.name);
            return Ok(());
        };

        if remote.starts_with("git@") || remote.starts_with("https://") {
            let path = self.main_worktree();
            println!("Cloning {} into {:?}", remote, &path);
            std::process::Command::new("git")
                .args(["clone", remote, &path.to_string_lossy()])
                .spawn()?
                .wait()?;
            return Ok(());
        }

        println!("Fetch remote: {}", remote);
        std::process::Command::new("sh")
            .args(["-c", remote])
            .spawn()?
            .wait()?;

        Ok(())
    }

    pub fn main_worktree(&self) -> PathBuf {
        self.root.join(format!("{}-main", self.name))
    }
}
