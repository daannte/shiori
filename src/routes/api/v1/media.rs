use std::path;

use axum::extract::{Path, State};
use shiori_database::models::Media;
use shiori_filesystem::common::get_cover;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::state::AppState,
    errors::{APIError, APIResult},
};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(get_media_cover))
}

/// Fetch media cover.
#[utoipa::path(
    get,
    path = "/media/{id}/cover",
    tag = "media",
    params(
        ("id" = i32, Path, description = "Id of the media item")
    ),
    responses(
        (status = 200, description = "Successfully fetched media cover"),
        (status = 404, description = "Media or cover not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn get_media_cover(
    Path(media_id): Path<i32>,
    State(app): State<AppState>,
) -> APIResult<Vec<u8>> {
    let mut conn = app.db().await?;

    let media = Media::find(&mut conn, media_id).await?;

    let path = media
        .cover_path
        .ok_or_else(|| APIError::NotFound("Media does not have a cover".to_string()))?;

    let data = get_cover(&path::Path::new(&path))
        .await
        .map_err(|_| APIError::InternalServerError("Failed to get cover".to_string()))?;

    Ok(data)
}
