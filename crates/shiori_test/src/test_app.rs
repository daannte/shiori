use std::{env, path::PathBuf, rc::Rc, sync::Arc};

use diesel_async::AsyncPgConnection;
use shiori::auth::hash_password;
use shiori_core::{App, db};
use shiori_database::models::{NewLibrary, NewUser};

use crate::{
    mock_users::{MockAnonymousUser, MockJwtUser, MockTokenUser},
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
    pub async fn init_empty() -> (TestApp, MockAnonymousUser) {
        let test_database = TestDatabase::new();

        let app = Arc::new(App {
            pool: Arc::new(db::create_pool(test_database.url())),
            base_path: PathBuf::from(
                env::var("APP_BASE_DIR").unwrap_or_else(|_| "/data".to_string()),
            ),
        });

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

    pub async fn init_with_user() -> (TestApp, MockAnonymousUser, MockJwtUser) {
        let (test_app, anon) = TestApp::init_empty().await;
        let user = test_app.new_user("shinei", true).await;
        (test_app, anon, user)
    }

    pub async fn init_with_token() -> (TestApp, MockAnonymousUser, MockJwtUser, MockTokenUser) {
        let (test_app, anon) = TestApp::init_empty().await;
        let user = test_app.new_user("shinei", true).await;
        let token = user.new_token("test", None).await;
        (test_app, anon, user, token)
    }

    pub fn as_inner(&self) -> &App {
        &self.0.app
    }

    pub fn router(&self) -> &axum::Router {
        &self.0.router
    }

    pub async fn db_conn(&self) -> AsyncPgConnection {
        self.0.test_database.async_connect().await
    }

    /// Create new user
    pub async fn new_user(&self, username: &str, is_server_owner: bool) -> MockJwtUser {
        let conn = self.db_conn().await;
        let new_user = NewUser {
            username,
            hashed_password: &hash_password("supercoolpass").unwrap(),
            is_server_owner,
        };
        let user = new_user.insert(&conn).await.unwrap();
        MockJwtUser::new(self.clone(), user).await
    }

    /// Create empty library
    pub async fn new_library(&self, name: &str, path: &str) {
        let conn = self.db_conn().await;
        let new_library = NewLibrary { name, path };
        new_library.insert(&conn).await.unwrap();
    }
}
