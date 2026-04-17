use std::{ffi::OsStr, path};

use axum::{
    Json,
    extract::{Path, State},
    middleware,
};
use chrono::NaiveDate;
use diesel_async::{AsyncConnection, scoped_futures::ScopedFutureExt};
use serde::Deserialize;
use shiori_api_types::{EncodableMediaDetails, EncodableMetadata};
use shiori_database::models::{Media, PatchMedia, UpdateMediaMetadata};
use shiori_filesystem::{
    common::{delete_file, move_file},
    image::cover::{download_cover, get_cover},
};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::state::AppState,
    errors::{AppResult, not_found},
    middleware::auth::{AuthUser, auth_middleware},
    routes::openapi::tags,
};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_media_cover))
        .routes(routes!(get_media, patch_media, delete_media))
        .layer(middleware::from_fn(auth_middleware))
}

/// Fetch media cover.
#[utoipa::path(
    get,
    path = "/media/{id}/cover",
    tag = tags::MEDIA,
    security(
        ("cookie" = []),
        ("api_token" = [])
    ),
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
) -> AppResult<Vec<u8>> {
    let mut conn = app.db().await?;

    let media = Media::find(&mut conn, media_id).await?;

    let path = media.cover_path.ok_or_else(not_found)?;

    let data = get_cover(path::Path::new(&path)).await?;

    Ok(data)
}

/// Fetch media item.
#[utoipa::path(
    get,
    path = "/media/{id}",
    tag = tags::MEDIA,
    security(
        ("cookie" = []),
        ("api_token" = [])
    ),
    params(
        ("id" = i32, Path, description = "Id of the media item")
    ),
    responses(
        (status = 200, description = "Successfully fetched media", body = inline(EncodableMediaDetails)),
        (status = 404, description = "Media not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn get_media(
    Path(media_id): Path<i32>,
    State(app): State<AppState>,
    AuthUser(auth): AuthUser,
) -> AppResult<Json<EncodableMediaDetails>> {
    let mut conn = app.db().await?;

    let (media, metadata, progress) =
        Media::with_details(&mut conn, media_id, auth.user().id).await?;

    let res = EncodableMediaDetails {
        media: media.into(),
        metadata: metadata.map(|m| m.into()),
        progress: progress.map(|p| p.into()),
    };

    Ok(Json(res))
}

/// Delete a media item.
#[utoipa::path(
    delete,
    path = "/media/{id}",
    tag = tags::MEDIA,
    security(
        ("cookie" = []),
        ("api_token" = [])
    ),
    params(
        ("id" = i32, Path, description = "Id of the media item")
    ),
    responses(
        (status = 204, description = "Successfully delete media"),
        (status = 404, description = "Media not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn delete_media(Path(media_id): Path<i32>, State(app): State<AppState>) -> AppResult<()> {
    let mut conn = app.db().await?;

    let media = Media::delete(&mut conn, media_id).await?;

    if let Some(cover_path) = media.cover_path {
        delete_file(path::Path::new(&cover_path)).await?
    }

    delete_file(path::Path::new(&media.path)).await?;

    Ok(())
}

#[derive(Default, Deserialize, utoipa::ToSchema)]
pub struct PatchMetadata {
    /// List of authors associated with the media item.
    #[schema(examples(json!(["Asato Asato"])))]
    pub authors: Option<Vec<String>>,

    /// Name of the publisher or publishing organization.
    #[schema(examples("Yen On"))]
    pub publisher: Option<String>,

    /// International Standard Book Number (ISBN).
    /// Typically used for books.
    #[schema(examples("1975303121"))]
    pub isbn: Option<String>,

    /// Language of the media content.
    #[schema(examples("English"))]
    pub language: Option<String>,

    /// Date the media was published.
    #[schema(examples("2019-03-26"))]
    pub published_at: Option<NaiveDate>,

    /// Description of the media item.
    #[schema(examples("The San Magnolia Republic..."))]
    pub description: Option<String>,

    /// List of genres associated with the media item.
    #[schema(examples(json!(["Light Novel", "War"])))]
    pub genres: Option<Vec<String>>,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct PatchRequest {
    /// Name of the media item.
    #[schema(examples("86—EIGHTY-SIX, Vol. 1"))]
    pub name: Option<String>,

    /// URL of the cover image associated with the media.
    #[schema(examples("https://example.com/cover.jpg"))]
    pub cover_url: Option<String>,

    /// Optional metadata to update for the media item.
    pub metadata: Option<PatchMetadata>,
}

/// Update media information.
#[utoipa::path(
    patch,
    path = "/media/{id}",
    tag = tags::MEDIA,
    security(
        ("cookie" = []),
        ("api_token" = [])
    ),
    params(
        ("id" = i32, Path, description = "Id of the media item")
    ),
    request_body = inline(PatchRequest),
    responses(
        (status = 200, description = "Successfully updated media information", body = inline(EncodableMediaDetails)),
        (status = 404, description = "Media not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn patch_media(
    Path(media_id): Path<i32>,
    State(app): State<AppState>,
    Json(body): Json<PatchRequest>,
) -> AppResult<Json<EncodableMediaDetails>> {
    let mut conn = app.db().await?;

    let downloaded_cover = if let Some(cover_url) = &body.cover_url {
        let new_cover_path = download_cover(cover_url, &app.base_path, &media_id).await?;

        Some(new_cover_path)
    } else {
        None
    };

    conn.transaction(|conn| {
        async move {
            let mut m = Media::find(conn, media_id).await?;

            let mut new_path: Option<String> = None;

            if let Some(name) = &body.name {
                tracing::debug!("Changing name of media to: {}", name);

                let source = path::Path::new(&m.path);

                let ext = source.extension().and_then(OsStr::to_str).unwrap_or("");

                let filename = if ext.is_empty() {
                    name.clone()
                } else {
                    format!("{}.{}", name, ext)
                };

                let mut dest = path::PathBuf::from(&m.path);
                dest.set_file_name(filename);

                if source != dest {
                    tracing::debug!(
                        "Moving file from {} to {}",
                        source.display(),
                        dest.display()
                    );
                    move_file(source, &dest).await?;
                    new_path = Some(dest.to_string_lossy().to_string());
                }
            }

            let changes = PatchMedia {
                name: body.name.as_deref(),
                cover_path: downloaded_cover.as_deref(),
                path: new_path.as_deref(),
            };

            if !changes.is_empty() {
                m = changes.update(conn, m).await?;
            }

            let metadata = if let Some(metadata) = &body.metadata {
                let changes = UpdateMediaMetadata {
                    authors: metadata.authors.clone(),
                    publisher: metadata.publisher.clone(),
                    isbn: metadata.isbn.clone(),
                    language: metadata.language.clone(),
                    published_at: metadata.published_at,
                    description: metadata.description.clone(),
                    genres: metadata.genres.clone(),
                };

                tracing::debug!(%media_id, ?changes, "Updating metadata");
                Some(changes.upsert(conn, m.id).await?)
            } else {
                None
            };

            let res = EncodableMediaDetails {
                media: m.into(),
                metadata: metadata.map(EncodableMetadata::from),
                progress: None,
            };

            Ok(Json(res))
        }
        .scope_boxed()
    })
    .await
}
