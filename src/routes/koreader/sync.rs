use axum::{
    Json,
    extract::{Path, State},
};
use serde::Deserialize;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{config::state::AppState, errors::AppResult, routes::openapi::tags};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(authorize))
        .routes(routes!(get_progress))
        .routes(routes!(update_progress))
}

/// Authorize koreader sync user.
#[utoipa::path(
    get,
    path = "/users/auth",
    tag = tags::KOREADER,
    responses(
        (status = 200, description = "Successfully authorized user"),
        (status = 401, description = "Unauthorized"),
    )
)]
async fn authorize() -> AppResult<()> {
    tracing::debug!("Received koreader auth request");
    Ok(())
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
#[allow(unused)]
struct PutProgressRequest {
    /// Hash of the book in the Koreader system.
    document: String,
    /// Current position in the book. There are two types of progress measurement:
    ///     - A page number (for paginated books).
    ///     - An x-pointer (for DOM-based books, using their scrolling reader).
    progress: String,
    /// Reading progress as a percentage, indicating how much of the book has been read.
    percentage: f32,
    /// Name of the Koreader device that the progress is being tracked on.
    device: String,
    /// Unique device identifier (UUID) assigned by Koreader to the specific device.
    device_id: String,
}

/// Save koreader progress.
#[utoipa::path(
    put,
    path = "/syncs/progress",
    tag = tags::KOREADER,
    request_body = inline(PutProgressRequest),
    responses(
        (status = 200, description = "Successfully saved progress"),
        (status = 401, description = "Unauthorized"),
    )
)]
async fn update_progress(
    State(_app): State<AppState>,
    Json(body): Json<PutProgressRequest>,
) -> AppResult<()> {
    tracing::debug!(progress = ?body, "Put progress request");
    Ok(())
}

#[derive(Deserialize, utoipa::ToSchema)]
#[allow(unused)]
struct GetProgressResponse {
    device: String,
    device_id: String,
    percentage: String,
    progress: String,
    /// Timestamp of the progress update, in Unix epoch time.
    timestamp: u64,
}

/// Retrieve koreader progress.
#[utoipa::path(
    get,
    path = "/syncs/progress/{document}",
    tag = tags::KOREADER,
    params(
        ("document" = String, Path, description = "Hash of the book in the Koreader system")
    ),
    responses(
        (status = 200, description = "Successfully retrieved progress", body = inline(GetProgressResponse)),
        (status = 401, description = "Unauthorized"),
    )
)]
async fn get_progress(State(_app): State<AppState>, Path(document): Path<String>) -> AppResult<()> {
    tracing::debug!(document, "Get progress request");
    Ok(())
}
