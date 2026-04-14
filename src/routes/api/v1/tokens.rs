use axum::{
    Json,
    extract::{Path, State},
    middleware,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use shiori_api_types::{EncodableApiToken, EncodableApiTokenWithToken};
use shiori_database::{
    models::{ApiToken, NewApiToken},
    token::Token,
};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::state::AppState,
    errors::{AppResult, bad_request, not_found},
    middleware::auth::{AuthUser, auth_middleware},
    routes::openapi::tags,
};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create_token, list_tokens))
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
    AuthUser(auth): AuthUser,
    Json(body): Json<TokenRequest>,
) -> AppResult<Json<EncodableApiTokenWithToken>> {
    if body.name.is_empty() {
        return Err(bad_request("name must have a value"));
    }

    if auth.is_token() {
        return Err(bad_request(
            "API token cannot be used to create another API token",
        ));
    }

    let conn = app.db().await?;

    let token = Token::default();

    let new_token = NewApiToken {
        user_id: auth.user().id,
        name: &body.name,
        key_id: token.key_id(),
        token_hash: token.hashed().hash,
        expires_at: body.expires_at,
        last_used_at: None,
    };

    let t = new_token.insert(&conn).await?;

    let api_token = EncodableApiTokenWithToken {
        token: t.into(),
        plaintoken: token.token().to_owned(),
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
        (status = 200, description = "Successfully fetched api tokens", body = inline(Vec<EncodableApiToken>)),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
async fn list_tokens(
    State(app): State<AppState>,
    AuthUser(auth): AuthUser,
) -> AppResult<Json<Vec<EncodableApiToken>>> {
    let mut conn = app.db().await?;

    let tokens = ApiToken::all(&mut conn, auth.user())
        .await?
        .into_iter()
        .map(Into::into)
        .collect::<Vec<_>>();

    Ok(Json(tokens))
}

/// Delete api token with key id.
#[utoipa::path(
    delete,
    path = "/tokens/{key_id}",
    tag = tags::TOKENS,
    security(
        ("cookie" = []),
        ("api_token" = [])
    ),
    params(
        ("key_id" = String, Path, description = "Key id of the api token")
    ),
    responses(
        (status = 204, description = "Successfully deleted api token"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn delete_token(
    State(app): State<AppState>,
    AuthUser(auth): AuthUser,
    Path(key_id): Path<String>,
) -> AppResult<()> {
    let mut conn = app.db().await?;

    let num_deleted = ApiToken::delete(&mut conn, key_id, auth.user()).await?;

    if num_deleted != 1 {
        return Err(not_found());
    }

    Ok(())
}
