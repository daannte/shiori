use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use shiori_database::models::{ApiToken, Library, Media, MediaMetadata, ReadingProgress, User};

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[schema(as = Library)]
pub struct EncodableLibrary {
    /// Unique identifier for the library.
    #[schema(examples(86))]
    pub id: i32,

    /// Name of the library.
    #[schema(examples("Light Novels"))]
    pub name: String,

    /// File system path to the library's directory.
    #[schema(examples("/data/books/light_novels"))]
    pub path: String,

    /// Timestamp of when the media was created.
    #[schema(examples("2024-11-08T17:23:41Z"))]
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
    #[schema(examples(86))]
    pub id: i32,

    /// Name of the media file, excluding extension.
    #[schema(examples("86_Volume_1"))]
    pub name: String,

    /// Size of the media file in bytes.
    #[schema(examples("102400"))]
    pub size: i64,

    /// File system path where the media is stored.
    #[schema(examples("/data/books/light_novels/86_Volume_1.epub"))]
    pub path: String,

    /// File extension of the media.
    #[schema(examples("epub"))]
    pub extension: String,

    /// Timestamp of when the media was created.
    #[schema(examples("2026-03-23T12:00:00Z"))]
    pub created_at: DateTime<Utc>,

    /// Library this media belongs to.
    #[schema(examples(2))]
    pub library_id: i32,

    /// Endpoint where the cover is stored.
    #[schema(examples("/api/v1/media/4/cover"))]
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
    #[schema(examples(json!(["Asato Asato"])))]
    pub authors: Vec<String>,

    /// Name of the publisher or publishing organization.
    #[schema(examples("Yen On"))]
    pub publisher: Option<String>,

    /// International Standard Book Number (ISBN).
    /// Typically used for books.
    #[schema(examples("1975303121"))]
    pub isbn: Option<String>,

    /// Language of the media content.
    #[schema(examples("English"))]
    pub language: Option<String>,

    /// Date the media was published.
    #[schema(examples("2019-03-26"))]
    pub published: Option<NaiveDate>,

    /// Description of the media item.
    #[schema(examples("The San Magnolia Republic..."))]
    pub description: Option<String>,

    /// List of genres associated with the media item.
    #[schema(examples(json!(["Light Novel", "War"])))]
    pub genres: Vec<String>,
}

impl From<MediaMetadata> for EncodableMetadata {
    fn from(metadata: MediaMetadata) -> Self {
        Self {
            authors: metadata.authors,
            publisher: metadata.publisher,
            isbn: metadata.isbn,
            language: metadata.language,
            published: metadata.published,
            description: metadata.description,
            genres: metadata.genres,
        }
    }
}

#[derive(Default, Serialize, Deserialize, utoipa::ToSchema)]
#[schema(as = User)]
pub struct EncodableUser {
    /// Unique identifier for the user.
    #[schema(examples(86))]
    pub id: i32,

    /// Username of the user.
    #[schema(examples("Reaper"))]
    pub username: String,

    /// Indicates whether the user is the owner of the server.
    #[schema(examples(false))]
    pub is_server_owner: bool,

    /// Timestamp of when the user was created.
    #[schema(examples("2025-07-25T12:45:19Z"))]
    pub created_at: DateTime<Utc>,
}

impl From<User> for EncodableUser {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            username: u.username,
            is_server_owner: u.is_server_owner,
            created_at: u.created_at,
        }
    }
}

#[derive(Default, Serialize, Deserialize, utoipa::ToSchema)]
#[schema(as = ApiToken)]
pub struct EncodableApiToken {
    /// Unique identifier for the token.
    #[schema(examples(86))]
    pub id: i32,

    /// Name for the token.
    #[schema(examples("Koreader Sync"))]
    pub name: String,

    /// Short key id for the token.
    #[schema(examples("1a2b3c4d"))]
    pub key_id: String,

    /// Timestamp of when the token expires.
    #[schema(examples("2026-12-31T00:00:00Z"))]
    pub expires_at: Option<DateTime<Utc>>,

    /// Timestamp of when the token was created.
    #[schema(examples("2025-07-25T12:45:19Z"))]
    pub created_at: DateTime<Utc>,

    /// Timestamp of when the token was last used.
    #[schema(examples("2026-03-24T16:33:19Z"))]
    pub last_used_at: Option<DateTime<Utc>>,
}

impl From<ApiToken> for EncodableApiToken {
    fn from(t: ApiToken) -> Self {
        Self {
            id: t.id,
            name: t.name,
            key_id: t.key_id,
            expires_at: t.expires_at,
            created_at: t.created_at,
            last_used_at: t.last_used_at,
        }
    }
}

#[derive(Default, Serialize, Deserialize, utoipa::ToSchema)]
#[schema(as = ReadingProgress)]
pub struct EncodableReadingProgress {
    /// Reading progress as a percentage of completion.
    #[schema(value_type = f64, examples(0.986))]
    pub percentage_completed: Option<BigDecimal>,

    /// Timestamp of when this progress started.
    #[schema(examples("2026-03-24T16:33:19Z"))]
    pub started_at: DateTime<Utc>,

    /// Timestamp of when this progrses was updated.
    #[schema(examples("2026-05-24T17:21:12Z"))]
    pub updated_at: DateTime<Utc>,

    /// Indicates whether this media has been fully read by the user.
    #[schema(examples(false))]
    pub completed: bool,

    /// Timestamp of when the media was completed.
    #[schema(examples("2026-05-24T17:21:12Z"))]
    pub completed_at: Option<DateTime<Utc>>,
}

impl From<ReadingProgress> for EncodableReadingProgress {
    fn from(r: ReadingProgress) -> Self {
        Self {
            percentage_completed: r.percentage_completed,
            started_at: r.started_at,
            updated_at: r.updated_at,
            completed: r.completed,
            completed_at: r.completed_at,
        }
    }
}

#[derive(Default, Serialize, Deserialize, utoipa::ToSchema)]
pub struct EncodableApiTokenWithToken {
    /// The token information.
    #[serde(flatten)]
    pub token: EncodableApiToken,

    /// The plaintext representation of the API token.
    #[schema(examples("shiori_1a2b3c4d_1a2b3c4d5e6f7g8h9i0j1234567890ab"))]
    pub plaintoken: String,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct EncodableMediaDetails {
    #[serde(flatten)]
    pub media: EncodableMedia,
    pub metadata: Option<EncodableMetadata>,
    pub progress: Option<EncodableReadingProgress>,
}

impl From<(Media, Option<ReadingProgress>)> for EncodableMediaDetails {
    fn from((media, progress): (Media, Option<ReadingProgress>)) -> Self {
        Self {
            media: media.into(),
            progress: progress.map(|p| p.into()),
            metadata: None,
        }
    }
}

#[derive(Default, Serialize, Deserialize, utoipa::ToSchema)]
pub struct EncodableMetadataSearch {
    /// Title of the media item.
    #[schema(examples("86—EIGHTY-SIX, Vol. 1"))]
    pub title: String,

    /// List of authors associated with the media item.
    #[schema(examples("Asato Asato"))]
    pub authors: Vec<String>,

    /// Name of the publisher or publishing organization.
    #[schema(examples("Yen On"))]
    pub publisher: Option<String>,

    /// International Standard Book Number (ISBN).
    /// Typically used for books.
    #[schema(examples("1975303121"))]
    pub isbn: Option<String>,

    /// Language of the media content.
    #[schema(examples("English"))]
    pub language: Option<String>,

    /// Date the media was published.
    #[schema(examples("2019-03-26"))]
    pub published: Option<NaiveDate>,

    /// URL of the cover image associated with the media.
    #[schema(examples("https://example.com/cover.jpg"))]
    pub cover_url: Option<String>,

    /// Description of the media item.
    #[schema(examples("The San Magnolia Republic..."))]
    pub description: Option<String>,

    /// List of genres associated with the media item.
    #[schema(examples("War"))]
    pub genres: Vec<String>,

    /// Provider id of the media item.
    #[schema(examples(41825371))]
    pub provider_id: u32,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct EncodableDirectories {
    /// Parent directory of the given path.
    #[schema(examples(json!(null)), required)]
    pub parent: Option<String>,

    /// Immediate subdirectories of the given path.
    #[schema(examples("light_novels"))]
    pub directories: Vec<String>,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct EncodableMeta {
    /// Whether the server has been initialized and an owner account exists.
    #[schema(examples(true))]
    pub initialized: bool,
}
