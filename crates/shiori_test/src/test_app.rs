use std::{rc::Rc, sync::Arc};

use diesel_async::AsyncPgConnection;
use shiori_core::{App, ShioriCore};
use shiori_database::models::NewUser;

use crate::{
    mock_users::{MockAnonymousUser, MockJwtUser},
    test_app,
    test_db::TestDatabase,
};

struct TestAppInner {
    app: Arc<App>,
    router: axum::Router,
    test_database: TestDatabase,
}

/// A representation of the app and its database transaction
#[derive(Clone)]
pub struct TestApp(Rc<TestAppInner>);

impl TestApp {
    /// Obtain a reference to the inner `App` value
    pub fn as_inner(&self) -> &App {
        &self.0.app
    }

    /// Obtain a reference to the axum Router
    pub fn router(&self) -> &axum::Router {
        &self.0.router
    }

    /// Obtain an async database connection from the primary database pool.
    pub async fn db_conn(&self) -> AsyncPgConnection {
        self.0.test_database.async_connect().await
    }

    /// Create a new use in the database
    /// This method updates the database directly
    pub async fn db_new_user(&self, username: &str) -> MockJwtUser {
        let conn = self.db_conn().await;

        let new_user = NewUser {
            username,
            hashed_password: "supercoolpass",
            is_server_owner: false,
        };

        let user = new_user.insert(&conn).await.unwrap();

        MockJwtUser::new(self.clone(), user)
    }
}

pub struct TestAppBuilder;

impl TestAppBuilder {
    /// Create a `TestApp` with an empty database
    pub async fn empty(mut self) -> (TestApp, MockAnonymousUser) {
        // Run each test inside a fresh database schema, deleted at the end of the test,
        // The schema will be cleared up once the app is dropped.
        let test_database = TestDatabase::new();

        let core = ShioriCore::new();
        let app = Arc::new(core.get_app());
        let router = shiori::build_axum_router(app.clone());

        let inner = TestAppInner {
            app,
            router,
            test_database,
        };
        let test_app = TestApp(Rc::new(inner));
        let anon = MockAnonymousUser::new(test_app.clone());

        (test_app, anon)
    }

    pub async fn with_user(self) -> (TestApp, MockAnonymousUser, MockJwtUser) {
        let (app, anon) = self.empty().await;
        let user = app.db_new_user("shinei").await;
        (app, anon, user)
    }
}
