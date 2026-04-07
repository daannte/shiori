use thiserror::Error;

/// Error type for metadata providers
#[derive(Debug, Error)]
pub enum MetadataError {
    /// Network or HTTP request failed
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    /// Failed to parse HTML from the provider
    #[error("failed to parse HTML")]
    HtmlParse,

    /// Missing expected tag
    #[error("missing tag: {0}")]
    MissingTag(String),

    /// Failed to parse JSON from the provider
    #[error("failed to parse JSON: {0}")]
    JsonParse(#[from] serde_json::Error),

    /// Apollo state was missing expected book info
    #[error("no book info found in apollo state")]
    MissingBookInfo,

    /// Generic or unexpected error
    #[error("{0}")]
    Other(String),
}
