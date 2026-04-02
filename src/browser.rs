use crate::{
    Error,
    config::{BrowserConfig, Config},
    error::Result,
};

use git2::Repository;

use std::path::PathBuf;

pub struct BrowserContext {
    pub browser: String,
    pub config: BrowserConfig,
    pub repo: Option<Repository>,
    pub root: PathBuf,
}

impl BrowserContext {
    pub fn detect() -> Result<Self> {
        let cwd = std::env::current_dir()?;
        let config = Config::load()?;

        let root = PathBuf::from(shellexpand::full(&config.root)?.into_owned());
        let tentative_browser = cwd
            .strip_prefix(&root)
            .ok()
            .and_then(|path| path.components().next())
            .ok_or(Error::NotInBrowserDir)?;
        let (browser, browser_config) = config
            .browsers
            .into_iter()
            .find(|(name, _)| *tentative_browser.as_os_str() == **name)
            .ok_or(Error::NotInBrowserDir)?;

        Ok(BrowserContext {
            config: browser_config,
            repo: Repository::discover(cwd).ok(),
            root: root.join(&browser),
            browser,
        })
    }

    pub fn fetch_remote(&self) -> Result<()> {
        let Some(remote) = &self.config.remote else {
            println!("There is no configured remote for {}", self.browser);
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
        self.root.join(format!("{}-main", self.browser))
    }
}
