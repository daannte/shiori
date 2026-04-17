use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::{dsl::sql, prelude::*, query_dsl::methods::FilterDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::Serialize;

use crate::{
    models::{Media, User},
    schema::reading_progress,
};

/// The model representing a row in the `reading_progress` database table.
#[derive(Debug, HasQuery, Identifiable, Associations, Serialize)]
#[diesel(table_name = reading_progress)]
#[diesel(belongs_to(User), belongs_to(Media))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReadingProgress {
    /// Unique identifier for the reading progress.
    pub id: i32,
    /// Unique identifier of the user associated with this progress.
    pub user_id: i32,
    /// Unique identifier of the media associated with this progress.
    pub media_id: i32,
    /// Unique identifier of the device associated with this progress.
    pub device_id: Option<String>,
    /// Current koreader position in the book.
    pub koreader_progress: Option<String>,
    /// Reading progress as a percentage of completion.
    pub percentage_completed: Option<BigDecimal>,
    /// Timestamp of when this progress started.
    pub started_at: DateTime<Utc>,
    /// Timestamp of when this progrses was updated.
    pub updated_at: DateTime<Utc>,
    /// Indicates whether this media has been fully read by the user.
    pub completed: bool,
    /// Timestamp of when the media was completed.
    pub completed_at: Option<DateTime<Utc>>,
}

/// Represents a reading progress for an upsert into the `reading_progress` table.
#[derive(Debug, Insertable)]
#[diesel(table_name = reading_progress)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateReadingProgress<'a> {
    pub user_id: i32,
    pub media_id: i32,
    pub device_id: Option<&'a str>,
    pub koreader_progress: Option<&'a str>,
    pub percentage_completed: Option<BigDecimal>,
    pub updated_at: DateTime<Utc>,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
}

impl UpdateReadingProgress<'_> {
    pub async fn upsert(&self, mut conn: &AsyncPgConnection) -> QueryResult<ReadingProgress> {
        diesel::insert_into(reading_progress::table)
            .values(self)
            .on_conflict((reading_progress::media_id, reading_progress::user_id))
            .do_update()
            .set((
                reading_progress::device_id.eq(self.device_id),
                reading_progress::koreader_progress.eq(self.koreader_progress),
                reading_progress::percentage_completed.eq(self.percentage_completed.clone()),
                reading_progress::updated_at.eq(self.updated_at),
                reading_progress::completed.eq(self.completed),
                reading_progress::completed_at.eq(sql(
                    "COALESCE(reading_progress.completed_at, EXCLUDED.completed_at)",
                )),
            ))
            .filter(reading_progress::completed.eq(false))
            .returning(ReadingProgress::as_returning())
            .get_result(&mut conn)
            .await
    }
}
