use chrono::NaiveDate;
use serde::Serialize;

#[derive(Default, Serialize, utoipa::ToSchema)]
pub struct EncodableMetadataSearch {
    /// List of authors associated with the media item.
    #[schema(example = json!(["Asato Asato"]))]
    pub authors: Vec<String>,

    /// Name of the publisher or publishing organization.
    #[schema(example = "Yen On")]
    pub publisher: Option<String>,

    /// International Standard Book Number (ISBN).
    /// Typically used for books.
    #[schema(example = "1975303121")]
    pub isbn: Option<String>,

    /// Language of the media content.
    #[schema(example = "English")]
    pub language: Option<String>,

    /// Date the media was published.
    #[schema(example = "2019-03-26")]
    pub published_at: Option<NaiveDate>,
}
