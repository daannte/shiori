use axum::{
    extract::{FromRequestParts, Request},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use shiori_database::models::User;
use shiori_jwt::AccessToken;

use crate::{
    config::state::AppState,
    errors::{APIError, APIResult},
};

pub async fn auth_middleware(mut req: Request, next: Next) -> APIResult<Response> {
    let jar = CookieJar::from_headers(req.headers());

    if let Some(cookie) = jar.get("access_token") {
        let user_id_str =
            AccessToken::decode(cookie.value()).map_err(|_| APIError::Unauthorized)?;

        let user_id = user_id_str
            .parse::<i32>()
            .map_err(|_| APIError::Unauthorized)?;

        req.extensions_mut().insert(AuthContext::UserId(user_id));
        return Ok(next.run(req).await);
    }

    Err(APIError::Unauthorized)
}

#[derive(Clone)]
pub enum AuthContext {
    UserId(i32),
}

pub struct CurrentUser(pub User);

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = APIError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth = parts
            .extensions
            .get::<AuthContext>()
            .ok_or(APIError::Unauthorized)?;

        let user_id = match auth {
            AuthContext::UserId(id) => *id,
        };

        let mut conn = state.db().await?;

        let user = User::find(&mut conn, user_id).await?;

        Ok(CurrentUser(user))
    }
}
