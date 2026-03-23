use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use utoipa::ToSchema;

use crate::schema::libraries;
use serde::Serialize;

/// The model representing a row in the `library` database table.
#[derive(Debug, HasQuery, ToSchema, Serialize)]
#[diesel(table_name = libraries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Library {
    /// Unique identifier for the library.
    pub id: i32,
    /// Name of the library.
    pub name: String,
    /// File system path to the library's directory.
    pub path: String,
    /// Timestamp of when the media was created.
    pub created_at: DateTime<Utc>,
}

/// Represents a new library record insertable to the `libraries` table.
#[derive(Insertable)]
#[diesel(table_name = libraries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewLibrary<'a> {
    pub name: &'a str,
    pub path: &'a str,
}

impl NewLibrary<'_> {
    pub async fn insert(&self, conn: &mut AsyncPgConnection) -> QueryResult<Library> {
        diesel::insert_into(libraries::table)
            .values(self)
            .returning(Library::as_returning())
            .get_result(conn)
            .await
    }
}
