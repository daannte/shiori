use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{models::Library, schema::media};
use serde::Serialize;

/// The model representing a row in the `media` database table.
#[derive(HasQuery, Serialize, Associations)]
#[diesel(table_name = media)]
#[diesel(belongs_to(Library))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Media {
    /// Unique identifier for the media item.
    pub id: i32,
    /// Name of the media file, excluding extension.
    pub name: String,
    /// Size of the media file in bytes.
    pub size: i64,
    /// File system path where the media is stored.
    pub path: String,
    /// File extension of the media.
    pub extension: String,
    /// Timestamp of when the media was created.
    pub created_at: DateTime<Utc>,
    /// Foreign key reference to the `libraries` table,
    /// indicating the library to which this media belongs.
    pub library_id: i32,
}

impl Media {
    pub async fn find_by_library_id(
        conn: &mut AsyncPgConnection,
        library_id: i32,
    ) -> QueryResult<Vec<Media>> {
        Media::query()
            .filter(media::library_id.eq(library_id))
            .load(conn)
            .await
    }
}

/// Represents a new media record insertable to the `media` table.
#[derive(Insertable)]
#[diesel(table_name = media)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMedia<'a> {
    pub name: &'a str,
    pub size: i64,
    pub path: &'a str,
    pub extension: &'a str,
    pub library_id: i32,
}

impl NewMedia<'_> {
    pub async fn insert(&self, conn: &mut AsyncPgConnection) -> QueryResult<Media> {
        diesel::insert_into(media::table)
            .values(self)
            .returning(Media::as_returning())
            .get_result(conn)
            .await
    }
}
