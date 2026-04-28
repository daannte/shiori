use chrono::Utc;
use http::header;
use shiori_database::{
    models::{ApiToken, NewApiToken, NewRefreshToken, User},
    token::Token,
};
use shiori_jwt::JwtTokenPair;

use crate::{
    mock_request::{MockRequest, MockRequestExt},
    test_app::TestApp,
    util::{RequestHelper, req},
};

/// A type that can generate unauthenticated requests
pub struct MockAnonymousUser {
    app: TestApp,
}

impl MockAnonymousUser {
    pub fn new(app: TestApp) -> Self {
        Self { app }
    }
}

impl RequestHelper for MockAnonymousUser {
    fn request_builder(&self, method: http::Method, path: &str) -> MockRequest {
        req(method, path)
    }

    fn app(&self) -> &TestApp {
        &self.app
    }
}

/// A type that can generate JWT authenticated requests
pub struct MockJwtUser {
    app: TestApp,
    user: User,
    tokens: JwtTokenPair,
}

impl MockJwtUser {
    pub async fn new(app: TestApp, user: User) -> Self {
        let conn = app.db_conn().await;

        let tokens = JwtTokenPair::new(user.id).unwrap();

        let refresh = NewRefreshToken {
            jti: &tokens.refresh_token.jti,
            user_id: user.id,
            expires_at: tokens.refresh_token.expires_at,
        };

        refresh.insert(&conn).await.unwrap();
        Self { app, user, tokens }
    }

    /// Returns a reference to the database `User` model
    pub fn as_model(&self) -> &User {
        &self.user
    }

    /// Returns a reference to the tokens
    pub fn tokens(&self) -> &JwtTokenPair {
        &self.tokens
    }

    /// Creates a new `MockTokenUser`
    pub async fn new_token(
        &self,
        name: &str,
        expires_at: Option<chrono::DateTime<Utc>>,
    ) -> MockTokenUser {
        let conn = self.app().db_conn().await;

        let plaintoken = Token::default();

        let new_token = NewApiToken {
            user_id: self.user.id,
            name,
            key_id: plaintoken.key_id(),
            token_hash: plaintoken.hashed().hash,
            expires_at,
            last_used_at: None,
        };

        let token = new_token.insert(&conn).await.unwrap();

        MockTokenUser {
            app: self.app().clone(),
            token: Some(token),
            plaintext: plaintoken.token().to_string(),
        }
    }
}

impl RequestHelper for MockJwtUser {
    fn request_builder(&self, method: http::Method, path: &str) -> MockRequest {
        let mut request = req(method, path);
        let cookie_header = format!(
            "access_token={}; refresh_token={}",
            self.tokens.access_token.token, self.tokens.refresh_token.token
        );

        request.header(header::COOKIE, &cookie_header);
        request
    }

    fn app(&self) -> &TestApp {
        &self.app
    }
}

/// A type that can generate token authenticated requests
pub struct MockTokenUser {
    app: TestApp,
    token: Option<ApiToken>,
    plaintext: String,
}

impl RequestHelper for MockTokenUser {
    fn request_builder(&self, method: http::Method, path: &str) -> MockRequest {
        let mut request = req(method, path);
        request.header(
            header::AUTHORIZATION,
            format!("Bearer {}", &self.plaintext).as_ref(),
        );
        request
    }

    fn app(&self) -> &TestApp {
        &self.app
    }
}

impl MockTokenUser {
    pub fn with_auth_header(token: String, app: TestApp) -> Self {
        Self {
            app,
            token: None,
            plaintext: token,
        }
    }

    /// Returns a reference to the database `ApiToken` model
    pub fn as_model(&self) -> &ApiToken {
        const ERROR: &str = "Original `ApiToken` was not set on this `MockTokenUser` instance";
        self.token.as_ref().expect(ERROR)
    }

    pub fn plaintext(&self) -> &str {
        &self.plaintext
    }
}
