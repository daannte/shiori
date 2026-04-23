use axum::{
    Json,
    extract::{Path, State},
};
use bigdecimal::BigDecimal;
use chrono::Utc;
use serde::Deserialize;
use shiori_database::models::{Media, UpdateReadingProgress};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::state::AppState,
    errors::{AppResult, bad_request},
    middleware::auth::AuthUser,
    routes::openapi::tags,
};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(authorize))
        .routes(routes!(get_progress))
        .routes(routes!(update_progress))
}

/// Authorize koreader sync user.
#[utoipa::path(
    get,
    path = "/{token}/users/auth",
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
    /// Reading progress as a percentage of completion.
    #[schema(value_type = f64)]
    percentage: BigDecimal,
    /// Name of the Koreader device that the progress is being tracked on.
    device: String,
    /// Unique device identifier (UUID) assigned by Koreader to the specific device.
    device_id: String,
}

/// Save koreader progress.
#[utoipa::path(
    put,
    path = "/{token}/syncs/progress",
    tag = tags::KOREADER,
    request_body = inline(PutProgressRequest),
    responses(
        (status = 204, description = "Successfully saved progress"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Media not found"),
    )
)]
async fn update_progress(
    State(app): State<AppState>,
    AuthUser(auth): AuthUser,
    Json(body): Json<PutProgressRequest>,
) -> AppResult<()> {
    tracing::debug!(progress = ?body, "Progress request update");

    if body.percentage < 0 || body.percentage > 1 {
        tracing::error!("Invalid percentage provided for progress request");
        return Err(bad_request("Invalid percentage"));
    }

    let mut conn = app.db().await?;
    let user = auth.user();
    let percentage = body.percentage.with_scale(4);

    let media = Media::find_by_koreader_hash(&mut conn, &body.document).await?;

    let completed = percentage == 1;

    let progress = UpdateReadingProgress {
        user_id: user.id,
        media_id: media.id,
        device_id: Some(&body.device_id),
        koreader_progress: Some(&body.progress),
        percentage_completed: Some(percentage),
        updated_at: Utc::now(),
        completed_at: if completed { Some(Utc::now()) } else { None },
        completed,
    };

    // NOTE: I'll leave it to 404 for now if the book is complete
    let saved = progress.upsert(&conn).await.inspect_err(|e| {
        tracing::error!(error = ?e, "Assuming book is complete");
    })?;

    tracing::debug!(progress = ?saved, "Updated reading progress");

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
    path = "/{token}/syncs/progress/{document}",
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
