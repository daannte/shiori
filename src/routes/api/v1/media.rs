use std::{ffi::OsStr, path};

use axum::{
    Json,
    extract::{Path, State},
};
use chrono::NaiveDate;
use diesel_async::{AsyncConnection, scoped_futures::ScopedFutureExt};
use serde::Deserialize;
use shiori_api_types::{EncodableMediaWithMetadata, EncodableMetadata};
use shiori_database::models::{Media, PatchMedia, UpdateMediaMetadata};
use shiori_filesystem::{
    common::move_file,
    image::cover::{download_cover, get_cover},
};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::state::AppState,
    errors::{APIError, APIResult},
};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_media_cover))
        .routes(routes!(get_media, patch_media))
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

    let data = get_cover(path::Path::new(&path))
        .await
        .map_err(|_| APIError::InternalServerError("Failed to get cover".to_string()))?;

    Ok(data)
}

/// Fetch media item.
#[utoipa::path(
    get,
    path = "/media/{id}",
    tag = "media",
    params(
        ("id" = i32, Path, description = "Id of the media item")
    ),
    responses(
        (status = 200, description = "Successfully fetched media cover", body = inline(EncodableMediaWithMetadata)),
        (status = 404, description = "Media not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn get_media(
    Path(media_id): Path<i32>,
    State(app): State<AppState>,
) -> APIResult<Json<EncodableMediaWithMetadata>> {
    let mut conn = app.db().await?;

    let (media, metadata) = Media::with_metadata(&mut conn, media_id).await?;

    let res = EncodableMediaWithMetadata {
        media: media.into(),
        metadata: metadata.map(|m| m.into()),
    };

    Ok(Json(res))
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

// I don't like this but its whatever
impl PatchMetadata {
    pub fn is_empty(&self) -> bool {
        self.authors.is_none()
            && self.publisher.is_none()
            && self.isbn.is_none()
            && self.language.is_none()
            && self.published_at.is_none()
            && self.genres.is_none()
            && self.description.is_none()
    }
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
    tag = "media",
    params(
        ("id" = i32, Path, description = "Id of the media item")
    ),
    request_body = inline(PatchRequest),
    responses(
        (status = 200, description = "Successfully updated media information", body = inline(EncodableMediaWithMetadata)),
        (status = 404, description = "Media not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn patch_media(
    Path(media_id): Path<i32>,
    State(app): State<AppState>,
    Json(body): Json<PatchRequest>,
) -> APIResult<Json<EncodableMediaWithMetadata>> {
    let mut conn = app.db().await?;

    let downloaded_cover =
        if let Some(cover_url) = &body.cover_url {
            Some(download_cover(cover_url).await.map_err(|_| {
                APIError::InternalServerError("Failed to download cover".to_string())
            })?)
        } else {
            None
        };

    conn.transaction(|conn| {
        async move {
            let mut m = Media::find(conn, media_id).await?;

            let mut new_path: Option<String> = None;

            if let Some(name) = &body.name {
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
                if metadata.is_empty() {
                    None
                } else {
                    let changes = UpdateMediaMetadata {
                        authors: metadata.authors.clone(),
                        publisher: metadata.publisher.clone(),
                        isbn: metadata.isbn.clone(),
                        language: metadata.language.clone(),
                        published_at: metadata.published_at,
                        description: metadata.description.clone(),
                        genres: metadata.genres.clone(),
                    };

                    Some(changes.upsert(conn, m.id).await?)
                }
            } else {
                None
            };

            let res = EncodableMediaWithMetadata {
                media: m.into(),
                metadata: metadata.map(EncodableMetadata::from),
            };

            Ok(Json(res))
        }
        .scope_boxed()
    })
    .await
}
