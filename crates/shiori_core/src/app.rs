use std::sync::Arc;

use diesel_async::{
    AsyncPgConnection,
    pooled_connection::deadpool::{Object, Pool, PoolError},
};

use crate::db;

type DeadpoolResult = Result<Object<AsyncPgConnection>, PoolError>;

#[derive(Clone)]
pub struct App {
    pub pool: Arc<Pool<AsyncPgConnection>>,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> App {
        let pool = Arc::new(db::create_pool());
        App { pool }
    }

    pub async fn db(&self) -> DeadpoolResult {
        self.pool.get().await
    }
}
