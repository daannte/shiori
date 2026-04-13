use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::Serialize;

use crate::schema::api_tokens;

/// The model representing a row in the `api_tokens` database table.
#[derive(Debug, HasQuery, Identifiable, Serialize)]
#[diesel(table_name = api_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ApiToken {
    /// Unique identifier for the token.
    pub id: i32,
    /// Unique identifier of the user associated with this token.
    pub user_id: i32,
    /// Name for the token.
    pub name: String,
    /// Short key id for the token.
    pub key_id: String,
    /// Hash of the token.
    pub token_hash: Vec<u8>,
    /// Timestamp of when the token expires.
    pub expires_at: Option<DateTime<Utc>>,
    /// Timestamp of when the token was created.
    pub created_at: DateTime<Utc>,
    /// Timestamp of when the token was last used.
    pub last_used_at: Option<DateTime<Utc>>,
}

impl ApiToken {
    pub async fn all(conn: &mut AsyncPgConnection) -> QueryResult<Vec<ApiToken>> {
        ApiToken::query().load(conn).await
    }
}

/// Represents a new api token record insertable to the `api_tokens` table.
#[derive(Debug, Insertable)]
#[diesel(table_name = api_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewApiToken<'a> {
    pub user_id: i32,
    pub name: &'a str,
    pub key_id: &'a str,
    pub token_hash: Vec<u8>,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
}

impl NewApiToken<'_> {
    pub async fn insert(&self, mut conn: &AsyncPgConnection) -> QueryResult<ApiToken> {
        diesel::insert_into(api_tokens::table)
            .values(self)
            .returning(ApiToken::as_returning())
            .get_result(&mut conn)
            .await
    }
}
