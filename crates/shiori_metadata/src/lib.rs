pub use self::goodreads::GoodreadsProvider;

mod client;
mod goodreads;
pub mod provider;

#[derive(Debug, thiserror::Error)]
pub enum MetadataError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("missing tag: {0}")]
    MissingTag(String),
    #[error("failed to parse JSON: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("no book info found in apollo state")]
    MissingBookInfo,
    #[error("{0}")]
    Other(String),
}

pub fn is_isbn(input: &str) -> bool {
    let digits = input.chars().filter(|c| c.is_ascii_digit()).count();
    digits == 10 || digits == 13
}
