use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    models::{Library, MediaMetadata, ReadingProgress},
    schema::{media, media_metadata, reading_progress},
};
use serde::Serialize;

/// The model representing a row in the `media` database table.
#[derive(Debug, HasQuery, Identifiable, Serialize, Associations)]
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
    /// File system path where the cover is stored.
    pub cover_path: Option<String>,
    /// Hash of the book in the Koreader system.
    pub koreader_hash: Option<String>,
}

impl Media {
    pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<Media> {
        Media::query().find(id).first(conn).await
    }

    pub async fn find_by_library_with_progress(
        conn: &mut AsyncPgConnection,
        library_id: i32,
        user_id: i32,
    ) -> QueryResult<Vec<(Media, Option<ReadingProgress>)>> {
        media::table
            .left_join(
                reading_progress::table.on(reading_progress::media_id
                    .eq(media::id)
                    .and(reading_progress::user_id.eq(user_id))),
            )
            .filter(media::library_id.eq(library_id))
            .select((Media::as_select(), Option::as_select()))
            .load(conn)
            .await
    }

    pub async fn find_by_koreader_hash(
        conn: &mut AsyncPgConnection,
        koreader_hash: &str,
    ) -> QueryResult<Media> {
        Media::query()
            .filter(media::koreader_hash.eq(koreader_hash))
            .first(conn)
            .await
    }

    pub async fn with_metadata(
        conn: &mut AsyncPgConnection,
        id: i32,
    ) -> QueryResult<(Media, Option<MediaMetadata>)> {
        media::table
            .left_join(media_metadata::table)
            .filter(media::id.eq(id))
            .select((Media::as_select(), Option::as_select()))
            .first(conn)
            .await
    }

    pub async fn with_details(
        conn: &mut AsyncPgConnection,
        id: i32,
        user_id: i32,
    ) -> QueryResult<(Media, Option<MediaMetadata>, Option<ReadingProgress>)> {
        media::table
            .left_join(media_metadata::table)
            .left_join(
                reading_progress::table.on(reading_progress::media_id
                    .eq(media::id)
                    .and(reading_progress::user_id.eq(user_id))),
            )
            .filter(media::id.eq(id))
            .select((Media::as_select(), Option::as_select(), Option::as_select()))
            .first(conn)
            .await
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<Media> {
        diesel::delete(media::table.find(id)).get_result(conn).await
    }
}

/// Represents a new media record insertable to the `media` table.
#[derive(Debug, Insertable)]
#[diesel(table_name = media)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMedia<'a> {
    pub name: &'a str,
    pub size: i64,
    pub path: &'a str,
    pub extension: &'a str,
    pub library_id: i32,
    pub cover_path: Option<&'a str>,
    pub koreader_hash: Option<&'a str>,
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

/// Represents a PATCH update for the `media` table.
#[derive(Debug, Default, AsChangeset)]
#[diesel(table_name = media)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchMedia<'a> {
    pub name: Option<&'a str>,
    pub cover_path: Option<&'a str>,
    pub path: Option<&'a str>,
}

impl PatchMedia<'_> {
    pub async fn update(&self, conn: &mut AsyncPgConnection, media: Media) -> QueryResult<Media> {
        diesel::update(&media).set(self).get_result(conn).await
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.cover_path.is_none()
    }
}
