pub mod common;
pub mod image;
pub mod media;

#[derive(Debug, thiserror::Error)]
pub enum FilesystemError {
    #[error("I/O error occurred: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid file extension in URL: {0}")]
    InvalidExtension(String),
    #[error("Request failed: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}
