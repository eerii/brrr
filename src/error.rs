use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Not in a browser directory")]
    NotInBrowserDir,
    #[error("Browser not supported: {0}")]
    UnsupportedBrowser(String),
    #[error("Config error: {0}")]
    Config(#[from] toml::de::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
