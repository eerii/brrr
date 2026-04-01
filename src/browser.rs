use crate::{
    Error,
    config::{BrowserConfig, Config},
    error::Result,
};

use git2::Repository;

use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Browser {
    Firefox,
    WebKit,
    Servo,
    Chromium,
}

impl Browser {
    const VARIANTS: [Browser; 4] = [
        Browser::Firefox,
        Browser::WebKit,
        Browser::Servo,
        Browser::Chromium,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Browser::Firefox => "firefox",
            Browser::WebKit => "webkit",
            Browser::Servo => "servo",
            Browser::Chromium => "chromium",
        }
    }

    fn from_path(path: &Path) -> Result<Self> {
        Browser::VARIANTS
            .into_iter()
            .find(|b| {
                path.components()
                    .find(|dir| dir.as_os_str() == b.name())
                    .is_some()
            })
            .ok_or(Error::NotInBrowserDir)
    }
}

pub struct BrowserContext {
    pub browser: Browser,
    pub config: BrowserConfig,
    pub repo: Option<Repository>,
    pub root: PathBuf,
}

impl BrowserContext {
    pub fn detect() -> Result<Self> {
        let cwd = std::env::current_dir()?;
        let browser = Browser::from_path(&cwd)?;
        let mut config = Config::load()?;
        Ok(BrowserContext {
            config: config
                .browsers
                .remove(browser.name())
                .expect("Browser should exist"),
            repo: Repository::discover(cwd).ok(),
            root: config.paths.root.join(browser.name()),
            browser,
        })
    }

    pub fn bootstrap(&self) -> Result<()> {
        if let Some(remote) = &self.config.bootstrap.remote {
            let path = self.root.join(format!("{}-main", self.browser.name()));
            println!("Cloning {} into {:?}", remote, &path);
            std::process::Command::new("git")
                .args(["clone", remote, &path.to_string_lossy()])
                .spawn()?
                .wait()?;
        }

        if let Some(command) = &self.config.bootstrap.command {
            println!("Bootstrap command:\n{}", command);
            std::process::Command::new("sh")
                .args(["-c", command])
                .spawn()?
                .wait()?;
        }

        Ok(())
    }
}
