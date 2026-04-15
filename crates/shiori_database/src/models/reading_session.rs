use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::Serialize;

use crate::{
    models::{Media, User},
    schema::reading_sessions,
};

/// The model representing a row in the `reading_sessions` database table.
#[derive(Debug, HasQuery, Identifiable, Associations, Serialize)]
#[diesel(table_name = reading_sessions)]
#[diesel(belongs_to(User), belongs_to(Media))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReadingSession {
    /// Unique identifier for the session.
    pub id: i32,
    /// Unique identifier of the user associated with this session.
    pub user_id: i32,
    /// Unique identifier of the media associated with this session.
    pub media_id: i32,
    /// Unique identifier of the device associated with this session.
    pub device_id: Option<String>,
    /// Current position in the book.
    pub koreader_progress: Option<String>,
    /// Reading progress as a percentage, indicating how much of the book has been read.
    pub percentage_completed: Option<BigDecimal>,
    /// Timestamp of when the session was created.
    pub created_at: DateTime<Utc>,
    /// Timestamp of when the session was updated.
    pub updated_at: DateTime<Utc>,
}

/// Represents a new reading session record insertable to the `reading_session` table.
#[derive(Debug, Insertable)]
#[diesel(table_name = reading_sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewReadingSession<'a> {
    pub user_id: i32,
    pub media_id: i32,
    pub device_id: Option<&'a str>,
    pub koreader_progress: Option<&'a str>,
    pub percentage_completed: Option<BigDecimal>,
    pub updated_at: DateTime<Utc>,
}

impl NewReadingSession<'_> {
    pub async fn insert(&self, mut conn: &AsyncPgConnection) -> QueryResult<ReadingSession> {
        diesel::insert_into(reading_sessions::table)
            .values(self)
            .returning(ReadingSession::as_returning())
            .get_result(&mut conn)
            .await
    }
}
