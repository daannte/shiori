use axum::{extract::State, middleware};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::state::AppState, errors::AppResult, middleware::auth::auth_middleware,
    routes::openapi::tags,
};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create_token, delete_current_token, list_tokens))
        .routes(routes!(delete_token))
        .layer(middleware::from_fn(auth_middleware))
}

/// Create new api token.
#[utoipa::path(
    post,
    path = "/tokens",
    tag = tags::TOKENS,
    security(
        ("cookie" = [])
    ),
    responses(
        (status = 200, description = "Successfully created api token"),
        (status = 500, description = "Internal server error")
    )
)]
async fn create_token(State(_app): State<AppState>) -> AppResult<()> {
    Ok(())
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
        (status = 500, description = "Internal server error")
    )
)]
async fn delete_token(State(_app): State<AppState>) -> AppResult<()> {
    Ok(())
}
