use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use shiori_database::models::{Library, Media, MediaMetadata};

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[schema(as = Library)]
pub struct EncodableLibrary {
    /// Unique identifier for the library.
    #[schema(example = 86)]
    pub id: i32,

    /// Name of the library.
    #[schema(example = "Light Novels")]
    pub name: String,

    /// File system path to the library's directory.
    #[schema(example = "/data/books/light_novels")]
    pub path: String,

    /// Timestamp of when the media was created.
    #[schema(example = "2024-11-08T17:23:41Z")]
    pub created_at: DateTime<Utc>,
}

impl From<Library> for EncodableLibrary {
    fn from(library: Library) -> Self {
        Self {
            id: library.id,
            name: library.name,
            path: library.path,
            created_at: library.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[schema(as = Media)]
pub struct EncodableMedia {
    /// Unique identifier for the media item.
    #[schema(example = 86)]
    pub id: i32,

    /// Name of the media file, excluding extension.
    #[schema(example = "86_Volume_1")]
    pub name: String,

    /// Size of the media file in bytes.
    #[schema(example = "102400")]
    pub size: i64,

    /// File system path where the media is stored.
    #[schema(example = "/data/books/light_novels/86_Volume_1.epub")]
    pub path: String,

    /// File extension of the media.
    #[schema(example = "epub")]
    pub extension: String,

    /// Timestamp of when the media was created.
    #[schema(example = "2026-03-23T12:00:00Z")]
    pub created_at: DateTime<Utc>,

    /// Library this media belongs to.
    #[schema(example = 2)]
    pub library_id: i32,

    /// Endpoint where the cover is stored.
    #[schema(example = "/api/v1/media/4/cover")]
    pub cover_path: Option<String>,
}

impl From<Media> for EncodableMedia {
    fn from(media: Media) -> Self {
        Self {
            id: media.id,
            name: media.name,
            size: media.size,
            path: media.path,
            extension: media.extension,
            created_at: media.created_at,
            library_id: media.library_id,
            cover_path: media
                .cover_path
                .map(|_| format!("/api/v1/media/{}/cover", media.id)),
        }
    }
}

#[derive(Default, Serialize, Deserialize, utoipa::ToSchema)]
#[schema(as = MediaMetadata)]
pub struct EncodableMetadata {
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

    /// Description of the media item.
    #[schema(example = "The San Magnolia Republic...")]
    pub description: Option<String>,

    /// List of genres associated with the media item.
    #[schema(example = json!(["Light Novel", "War"]))]
    pub genres: Vec<String>,
}

impl From<MediaMetadata> for EncodableMetadata {
    fn from(metadata: MediaMetadata) -> Self {
        Self {
            authors: metadata.authors,
            publisher: metadata.publisher,
            isbn: metadata.isbn,
            language: metadata.language,
            published_at: metadata.published_at,
            description: metadata.description,
            genres: metadata.genres,
        }
    }
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct EncodableMediaWithMetadata {
    #[serde(flatten)]
    pub media: EncodableMedia,
    pub metadata: Option<EncodableMetadata>,
}

#[derive(Default, Serialize, Deserialize, utoipa::ToSchema)]
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

    /// URL of the cover image associated with the media.
    #[schema(example = "https://example.com/cover.jpg")]
    pub cover_url: Option<String>,

    /// Description of the media item.
    #[schema(example = "The San Magnolia Republic...")]
    pub description: Option<String>,

    /// List of genres associated with the media item.
    #[schema(example = json!(["Light Novel", "War"]))]
    pub genres: Vec<String>,
}
