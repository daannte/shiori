use http::header;
use shiori_database::models::User;
use shiori_jwt::JwtTokenPair;

use crate::{
    mock_request::MockRequestExt,
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
    fn request_builder(
        &self,
        method: http::Method,
        path: &str,
    ) -> crate::mock_request::MockRequest {
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
}

impl MockJwtUser {
    pub fn new(app: TestApp, user: User) -> Self {
        Self { app, user }
    }

    /// Returns a reference to the database `User` model
    pub fn as_model(&self) -> &User {
        &self.user
    }
}

impl RequestHelper for MockJwtUser {
    fn request_builder(
        &self,
        method: http::Method,
        path: &str,
    ) -> crate::mock_request::MockRequest {
        let tokens = JwtTokenPair::new(self.user.id).unwrap();

        let mut request = req(method, path);
        let cookie_header = format!(
            "access_token={}; refresh_token={}",
            tokens.access_token.token, tokens.refresh_token.token
        );

        request.header(header::COOKIE, &cookie_header);
        request
    }

    fn app(&self) -> &TestApp {
        &self.app
    }
}
