use chrono::NaiveDate;
use diesel::prelude::*;

use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{models::Media, schema::media_metadata};
use serde::Serialize;

/// The model representing a row in the `media_metadata` database table.
#[derive(Debug, HasQuery, Identifiable, Serialize, Associations)]
#[diesel(table_name = media_metadata)]
#[diesel(belongs_to(Media))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(media_id))]
pub struct MediaMetadata {
    /// Foreign key reference to the `media` table,
    /// indicating the media to which this metadata belongs.
    pub media_id: i32,
    /// List of authors associated with the media item.
    pub authors: Vec<String>,
    /// Name of the publisher or publishing organization.
    pub publisher: Option<String>,
    /// International Standard Book Number (ISBN).
    /// Typically used for books.
    pub isbn: Option<String>,
    /// Language of the media content.
    pub language: Option<String>,
    /// Date the media was published.
    pub published: Option<NaiveDate>,
    /// Description of the media item.
    pub description: Option<String>,
    /// List of genres associated with the media item.
    pub genres: Vec<String>,
}

impl MediaMetadata {
    pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<MediaMetadata> {
        MediaMetadata::query().find(id).first(conn).await
    }
}

/// Represents a new media_metadata record insertable to the `media_metadata` table.
#[derive(Debug, Insertable)]
#[diesel(table_name = media_metadata)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMediaMetadata {
    pub media_id: i32,
    pub authors: Vec<String>,
    pub publisher: Option<String>,
    pub isbn: Option<String>,
    pub language: Option<String>,
    pub published: Option<NaiveDate>,
    pub description: Option<String>,
    pub genres: Vec<String>,
}

/// Represents a PATCH update for the `media_metadata` table.
#[derive(Debug, Default, AsChangeset)]
#[diesel(table_name = media_metadata)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateMediaMetadata {
    pub authors: Option<Vec<String>>,
    pub publisher: Option<String>,
    pub isbn: Option<String>,
    pub language: Option<String>,
    pub published: Option<NaiveDate>,
    pub description: Option<String>,
    pub genres: Option<Vec<String>>,
}

impl UpdateMediaMetadata {
    pub async fn upsert(
        &self,
        conn: &mut AsyncPgConnection,
        media_id: i32,
    ) -> QueryResult<MediaMetadata> {
        let new = NewMediaMetadata {
            media_id,
            authors: self.authors.clone().unwrap_or_default(),
            publisher: self.publisher.clone(),
            isbn: self.isbn.clone(),
            language: self.language.clone(),
            published: self.published,
            description: self.description.clone(),
            genres: self.genres.clone().unwrap_or_default(),
        };

        diesel::insert_into(media_metadata::table)
            .values(new)
            .on_conflict(media_metadata::media_id)
            .do_update()
            .set(self)
            .returning(MediaMetadata::as_returning())
            .get_result(conn)
            .await
    }
}
