use git2::Repository;

use std::path::Path;

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

    fn name(&self) -> &'static str {
        match self {
            Browser::Firefox => "firefox",
            Browser::WebKit => "webkit",
            Browser::Servo => "servo",
            Browser::Chromium => "chromium",
        }
    }

    fn from_path(path: &Path) -> Option<Self> {
        Browser::VARIANTS.into_iter().find(|b| {
            path.components()
                .find(|dir| dir.as_os_str() == b.name())
                .is_some()
        })
    }

    pub fn container_name(&self) -> String {
        format!("{}-dev", self.name())
    }
}

pub struct BrowserContext {
    pub browser: Browser,
    pub repo: Repository,
}

impl BrowserContext {
    pub fn new() -> Option<Self> {
        let cwd = std::env::current_dir().ok()?;
        let browser = Browser::from_path(&cwd)?;
        let repo = Repository::discover(cwd).ok()?;
        Some(BrowserContext { browser, repo })
    }
}
