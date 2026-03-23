use chrono::NaiveDate;
use diesel::prelude::*;
use utoipa::ToSchema;

use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{models::Media, schema::media_metadata};
use serde::Serialize;

/// The model representing a row in the `media_metadata` database table.
#[derive(Debug, HasQuery, ToSchema, Serialize, Associations)]
#[diesel(table_name = media_metadata)]
#[diesel(belongs_to(Media))]
#[diesel(check_for_backend(diesel::pg::Pg))]
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
    pub published_at: Option<NaiveDate>,
}

/// Represents a new media_metadata record insertable to the `media_metadata` table.
#[derive(Debug, Insertable)]
#[diesel(table_name = media_metadata)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMediaMetadata<'a> {
    pub authors: Vec<&'a str>,
    pub publisher: Option<&'a str>,
    pub isbn: Option<&'a str>,
    pub language: Option<&'a str>,
    pub published_at: Option<NaiveDate>,
}

impl NewMediaMetadata<'_> {
    pub async fn insert(&self, conn: &mut AsyncPgConnection) -> QueryResult<MediaMetadata> {
        diesel::insert_into(media_metadata::table)
            .values(self)
            .returning(MediaMetadata::as_returning())
            .get_result(conn)
            .await
    }
}
