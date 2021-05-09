use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("File \"{0}\" was not found")]
    FileNotFound(PathBuf),

    #[error("\"{0}\" is not a file path")]
    NotFile(PathBuf),
}
