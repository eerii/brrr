use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Not in a browser directory")]
    NotInBrowserDir,
    #[error("Config error: {0}")]
    Config(#[from] toml::de::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
