use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::Serialize;

use crate::schema::users;

/// The model representing a row in the `users` database table.
#[derive(Debug, HasQuery, Identifiable, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    /// Unique identifier for the user.
    pub id: i32,
    /// Username of the user.
    pub username: String,
    /// Hashed password of the user.
    pub hashed_password: String,
    /// Indicates whether the user is the owner of the server.
    pub is_server_owner: bool,
    /// Timestamp of when the user was created.
    pub created_at: DateTime<Utc>,
}

impl User {
    pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<User> {
        User::query().find(id).first(conn).await
    }

    pub async fn find_by_username(
        conn: &mut AsyncPgConnection,
        username: &str,
    ) -> QueryResult<User> {
        User::query()
            .filter(users::username.eq(username.to_lowercase()))
            .first(conn)
            .await
    }

    pub async fn count(conn: &mut AsyncPgConnection) -> QueryResult<i64> {
        users::table.count().get_result(conn).await
    }
}

/// Represents a new user record insertable to the `users` table.
#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub hashed_password: &'a str,
    pub is_server_owner: bool,
}

impl NewUser<'_> {
    pub async fn insert(&self, mut conn: &AsyncPgConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(self)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .await
    }
}
