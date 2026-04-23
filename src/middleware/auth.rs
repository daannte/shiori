use axum::{
    extract::{FromRequestParts, Path, Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use shiori_database::{
    models::{ApiToken, User},
    token::HashedToken,
};
use shiori_jwt::AccessToken;

use crate::{
    config::state::AppState,
    errors::{AppResult, BoxedAppError, unauthorized},
};

#[derive(Clone)]
pub enum Auth {
    Cookie(User),
    Token(User),
}

impl Auth {
    pub fn is_token(&self) -> bool {
        matches!(self, Auth::Token(_))
    }

    pub fn user(&self) -> &User {
        match self {
            Auth::Token(user) | Auth::Cookie(user) => user,
        }
    }
}

pub async fn auth_middleware(
    State(app): State<AppState>,
    mut req: Request,
    next: Next,
) -> AppResult<Response> {
    let source = extract_auth_source(&req).ok_or_else(|| unauthorized("Unauthorized"))?;
    let auth = resolve_auth(&app, source).await?;

    req.extensions_mut().insert(auth);

    Ok(next.run(req).await)
}

pub async fn url_auth_middleware(
    State(app): State<AppState>,
    Path(api_token): Path<String>,
    mut req: Request,
    next: Next,
) -> AppResult<Response> {
    let source = AuthSource::Url(api_token);
    let auth = resolve_auth(&app, source).await?;

    req.extensions_mut().insert(auth);

    Ok(next.run(req).await)
}

enum AuthSource {
    Header(String),
    Cookie(String),
    Url(String),
}

fn extract_auth_source(req: &Request) -> Option<AuthSource> {
    if let Some(auth_header) = req.headers().get(header::AUTHORIZATION)
        && let Ok(value) = auth_header.to_str()
    {
        return Some(AuthSource::Header(value.to_string()));
    }

    let jar = CookieJar::from_headers(req.headers());
    if let Some(cookie) = jar.get("access_token") {
        return Some(AuthSource::Cookie(cookie.value().to_string()));
    }

    None
}

async fn resolve_auth(app: &AppState, source: AuthSource) -> AppResult<Auth> {
    match source {
        AuthSource::Header(value) => {
            let (scheme, token) = value.split_once(' ').unwrap_or(("", value.as_str()));

            if !scheme.eq_ignore_ascii_case("bearer") {
                return Err(unauthorized("Invalid auth scheme"));
            }

            let token = HashedToken::parse(token).map_err(|_| unauthorized("Invalid API token"))?;

            let user_id = update_last_used_at(app, &token.hash).await?;
            let user = fetch_user(app, user_id).await?;

            Ok(Auth::Token(user))
        }

        AuthSource::Cookie(value) => {
            let user_id_str =
                AccessToken::decode(&value).map_err(|_| unauthorized("Invalid access token"))?;

            let user_id = user_id_str
                .parse::<i32>()
                .map_err(|_| unauthorized("Invalid access token"))?;

            let user = fetch_user(app, user_id).await?;

            Ok(Auth::Cookie(user))
        }

        AuthSource::Url(token_str) => {
            let token =
                HashedToken::parse(&token_str).map_err(|_| unauthorized("Invalid API token"))?;

            let user_id = update_last_used_at(app, &token.hash).await?;
            let user = fetch_user(app, user_id).await?;

            Ok(Auth::Token(user))
        }
    }
}

pub struct AuthUser(pub Auth);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = BoxedAppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth = parts
            .extensions
            .get::<Auth>()
            .cloned()
            .ok_or_else(|| unauthorized("Unauthorized"))?;

        Ok(AuthUser(auth))
    }
}

async fn fetch_user(app: &AppState, user_id: i32) -> AppResult<User> {
    let mut conn = app.db().await?;
    Ok(User::find(&mut conn, user_id).await?)
}

async fn update_last_used_at(app: &AppState, token: &[u8]) -> AppResult<i32> {
    let mut conn = app.db().await?;
    let api_token = ApiToken::find_by_hash(&mut conn, token)
        .await
        .map_err(|_| unauthorized("Invalid API token"))?;

    if let Err(e) = api_token.update_last_used(&mut conn).await {
        tracing::error!(
            api_token = api_token.key_id,
            error = %e,
            "Failed to update last used time"
        );
    };

    Ok(api_token.user_id)
}
