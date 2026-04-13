use axum::{Json, extract::State, middleware};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use shiori_api_types::EncodableApiTokenWithToken;
use shiori_database::{models::NewApiToken, token::Token};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::state::AppState,
    errors::AppResult,
    middleware::auth::{CurrentUser, auth_middleware},
    routes::openapi::tags,
};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create_token, delete_current_token, list_tokens))
        .routes(routes!(delete_token))
        .layer(middleware::from_fn(auth_middleware))
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct TokenRequest {
    /// Name of the token
    #[schema(examples("Koreader Sync"))]
    name: String,

    /// Timestamp of when the token expires.
    #[schema(examples("2026-12-31T00:00:00Z"))]
    expires_at: Option<DateTime<Utc>>,
}

/// Create new api token.
#[utoipa::path(
    post,
    path = "/tokens",
    tag = tags::TOKENS,
    security(
        ("cookie" = [])
    ),
    request_body = inline(TokenRequest),
    responses(
        (status = 200, description = "Successfully created api token", body = inline(EncodableApiTokenWithToken)),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
async fn create_token(
    State(app): State<AppState>,
    CurrentUser(user): CurrentUser,
    Json(body): Json<TokenRequest>,
) -> AppResult<Json<EncodableApiTokenWithToken>> {
    // TODO: once i add token verification to middleware, this endpoint
    // shouldnt be access wtih a token
    let conn = app.db().await?;

    let token = Token::new();

    let new_token = NewApiToken {
        user_id: user.id,
        name: &body.name,
        key_id: token.key_id(),
        token_hash: token.hashed().hash,
        expires_at: body.expires_at,
        last_used_at: None,
    };

    let t = new_token.insert(&conn).await?;

    let api_token = EncodableApiTokenWithToken {
        token: t.into(),
        plaintoken: token.secret().to_owned(),
    };

    Ok(Json(api_token))
}

/// List all api tokens of authenticated user.
#[utoipa::path(
    get,
    path = "/tokens",
    tag = tags::TOKENS,
    security(
        ("cookie" = []),
        ("api_token" = [])
    ),
    responses(
        (status = 200, description = "Successfully fetched api tokens"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
async fn list_tokens(State(_app): State<AppState>) -> AppResult<()> {
    Ok(())
}

/// Delete current api token.
#[utoipa::path(
    delete,
    path = "/tokens",
    tag = tags::TOKENS,
    security(
        ("api_token" = [])
    ),
    responses(
        (status = 200, description = "Successfully deleted current api token"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
async fn delete_current_token(State(_app): State<AppState>) -> AppResult<()> {
    Ok(())
}

/// Delete api token with id.
#[utoipa::path(
    delete,
    path = "/tokens/{id}",
    tag = tags::TOKENS,
    security(
        ("cookie" = []),
        ("api_token" = [])
    ),
    params(
        ("id" = String, Path, description = "Id of the api token")
    ),
    responses(
        (status = 200, description = "Successfully deleted api token"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
async fn delete_token(State(_app): State<AppState>) -> AppResult<()> {
    Ok(())
}
