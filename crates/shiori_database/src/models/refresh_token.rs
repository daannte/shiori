use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::Serialize;

use crate::schema::refresh_tokens;

/// The model representing a row in the `refresh_tokens` database table.
#[derive(Debug, HasQuery, Identifiable, Serialize)]
#[diesel(table_name = refresh_tokens)]
#[diesel(primary_key(jti))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RefreshToken {
    /// Unique identifier for the token.
    pub jti: String,
    /// Unique identifier of the user associated with this token.
    pub user_id: i32,
    /// Timestamp of when the token expires.
    pub expires_at: DateTime<Utc>,
    /// Timestamp of when the token was created.
    pub created_at: DateTime<Utc>,
    /// Timestamp of when the token was revoked.
    pub revoked_at: Option<DateTime<Utc>>,
}

impl RefreshToken {
    pub async fn find(conn: &mut AsyncPgConnection, jti: &str) -> QueryResult<RefreshToken> {
        RefreshToken::query().find(jti).first(conn).await
    }

    pub async fn revoke(conn: &mut AsyncPgConnection, jti: &str) -> QueryResult<usize> {
        diesel::update(
            refresh_tokens::table
                .filter(refresh_tokens::jti.eq(jti))
                .filter(refresh_tokens::revoked_at.is_null()),
        )
        .set(refresh_tokens::revoked_at.eq(Utc::now()))
        .execute(conn)
        .await
    }
}

/// Represents a new refresh token record insertable to the `refresh_tokens` table.
#[derive(Debug, Insertable)]
#[diesel(table_name = refresh_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewRefreshToken<'a> {
    pub jti: &'a str,
    pub user_id: i32,
    pub expires_at: DateTime<Utc>,
}

impl NewRefreshToken<'_> {
    pub async fn insert(&self, mut conn: &AsyncPgConnection) -> QueryResult<RefreshToken> {
        diesel::insert_into(refresh_tokens::table)
            .values(self)
            .returning(RefreshToken::as_returning())
            .get_result(&mut conn)
            .await
    }
}
