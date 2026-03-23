use axum::extract::{Path, State};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::state::AppState,
    errors::{APIError, APIResult},
};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_libraries, create_library))
        .routes(routes!(get_library))
}

#[utoipa::path(
    get,
    path = "/libraries",
    tag = "library",
    responses(
        (status = 200, description = "Successfully fetched libraries"),
        (status = 500, description = "Internal server error")
    )
)]
async fn list_libraries(State(_app): State<AppState>) -> APIResult<()> {
    Err(APIError::NotImplemented)
}

#[utoipa::path(
    post,
    path = "/libraries",
    tag = "library",
    responses(
        (status = 200, description = "Successfully created library"),
        (status = 500, description = "Internal server error")
    )
)]
async fn create_library(State(_app): State<AppState>) -> APIResult<()> {
    Err(APIError::NotImplemented)
}

#[utoipa::path(
    get,
    path = "/libraries/{id}",
    tag = "library",
    params(
        ("id" = i32, Path, description = "Id of the library")
    ),
    responses(
        (status = 200, description = "Successfully fetched library"),
        (status = 404, description = "Library not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn get_library(Path(_library_id): Path<i32>, State(_app): State<AppState>) -> APIResult<()> {
    Err(APIError::NotImplemented)
}
