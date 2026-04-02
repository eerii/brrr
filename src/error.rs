use std::env::VarError;

use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Not in a browser directory")]
    NotInBrowserDir,
    #[error("No {0} command configured")]
    NoCommand(String),
    #[error("Config error: {0}")]
    Config(#[from] toml::de::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),
    #[error("Shell expansion error: {0}")]
    ShellExpand(#[from] shellexpand::LookupError<VarError>),
}

pub type Result<T> = std::result::Result<T, Error>;
