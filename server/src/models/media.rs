use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use utoipa::ToSchema;

use crate::schema::media;

/// The model representing a row in the `media` database table.
#[derive(Debug, HasQuery, ToSchema)]
#[diesel(table_name = crate::schema::media)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Media {
    /// Unique identifier for the media item.
    pub id: i32,
    /// Name of the media file, excluding extension.
    pub name: String,
    /// Size of the media file in bytes.
    pub size: i32,
    /// File system path where the media is stored.
    pub path: String,
    /// The file extension of the media.
    pub extension: String,
    /// The timestamp when the media was created.
    pub created_at: DateTime<Utc>,
}

/// Represents a new media record insertable to the `media` table
#[derive(Insertable)]
#[diesel(table_name = crate::schema::media)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMedia<'a> {
    pub name: &'a str,
    pub size: i32,
    pub path: &'a str,
    pub extension: &'a str,
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
