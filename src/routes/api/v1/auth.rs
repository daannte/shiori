use utoipa_axum::{router::OpenApiRouter, routes};

use crate::config::state::AppState;

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(login))
        .routes(routes!(register))
        .routes(routes!(refresh_token))
        .routes(routes!(logout))
}

/// Login
#[utoipa::path(
    get,
    path = "/auth/login",
    tag = "auth",
    responses(
        (status = 200, description = "Successfully logged in"),
        (status = 500, description = "Internal server error")
    )
)]
async fn login() {}

/// Register
#[utoipa::path(
    get,
    path = "/auth/register",
    tag = "auth",
    responses(
        (status = 200, description = "Successfully registered"),
        (status = 500, description = "Internal server error")
    )
)]
async fn register() {}

/// Refresh JWT token
#[utoipa::path(
    get,
    path = "/auth/refresh-token",
    tag = "auth",
    responses(
        (status = 200, description = "Successfully refreshed JWT token"),
        (status = 500, description = "Internal server error")
    )
)]
async fn refresh_token() {}

/// Logout
#[utoipa::path(
    get,
    path = "/auth/logout",
    tag = "auth",
    responses(
        (status = 200, description = "Successfully logged out"),
        (status = 500, description = "Internal server error")
    )
)]
async fn logout() {}
