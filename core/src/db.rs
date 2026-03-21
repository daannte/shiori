use std::env;

use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool},
};
use dotenvy::dotenv;

pub fn create_pool() -> Pool<AsyncPgConnection> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = AsyncDieselConnectionManager::new(db_url);

    let pool = Pool::builder(manager).build().unwrap();

    pool
}
