use std::ffi::OsStr;
use std::{env, path};

use crate::{
    config::state::AppState,
    errors::{APIError, APIResult},
};
use axum::{
    Json,
    extract::{Path, State},
};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use shiori_database::models::media::{Media, NewMedia};
use shiori_database::schema::media;
use tempfile::NamedTempFile;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_media, upload_media))
        .routes(routes!(get_media))
}

// TODO: Pagination
#[utoipa::path(
    get,
    path = "/media",
    tag = "media",
    responses(
        (status = 200, description = "Successfully fetched media"),
        (status = 500, description = "Internal server error")
    )
)]
async fn list_media(State(_app): State<AppState>) -> APIResult<()> {
    Err(APIError::NotImplemented)
}

#[derive(TryFromMultipart, ToSchema)]
struct MediaRequest {
    /// An array of files to upload.
    #[schema(value_type = Vec<Object>)]
    files: Vec<FieldData<NamedTempFile>>,
}

#[utoipa::path(
    post,
    path = "/media",
    tag = "media",
    request_body(
        content = MediaRequest,
        content_type = "multipart/form-data"
    ),
    responses(
        (status = 200, description = "Successfully added media", body = Vec<Media>),
        (status = 400, description = "Invalid media file"),
        (status = 500, description = "Internal server error")
    )
)]
// TODO: Refactor all this
async fn upload_media(
    State(app): State<AppState>,
    TypedMultipart(files): TypedMultipart<MediaRequest>,
) -> APIResult<Json<Vec<Media>>> {
    let mut conn = app.db().await?;

    let mut uploaded = Vec::new();

    let temp_file_dir = env::var("FILE_DIR").expect("FILE_DIR must be set");
    for f in files.files {
        let file_name = f.metadata.file_name.as_deref().ok_or_else(|| {
            APIError::BadRequest("Uploaded media must have a filename.".to_string())
        })?;

        let ext = path::Path::new(file_name)
            .extension()
            .and_then(OsStr::to_str)
            .map(str::to_ascii_lowercase)
            .ok_or_else(|| {
                APIError::BadRequest("Uploaded media must have an extension.".to_string())
            })?;

        let new_media = NewMedia {
            name: file_name,
            size: f
                .contents
                .as_file()
                .metadata()
                .unwrap()
                .len()
                .try_into()
                .unwrap_or_else(|_| {
                    println!("Failed to conver to i64");
                    0
                }),
            path: &format!("{}/{}", temp_file_dir, file_name),
            extension: &ext,
        };

        let media = new_media.insert(&mut conn).await?;
        uploaded.push(media);
    }

    Ok(Json(uploaded))
}

#[utoipa::path(
    get,
    path = "/media/{id}",
    tag = "media",
    params(
        ("id" = i32, Path, description = "Id of the media")
    ),
    responses(
        (status = 200, description = "Successfully fetched media", body = Media),
        (status = 404, description = "Media not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn get_media(
    Path(media_id): Path<i32>,
    State(app): State<AppState>,
) -> APIResult<Json<Media>> {
    let mut conn = app.db().await?;

    let media = Media::query()
        .filter(media::id.eq(media_id))
        .first::<Media>(&mut conn)
        .await?;

    Ok(Json(media))
}
