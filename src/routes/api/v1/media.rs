// use crate::{config::state::AppState, errors::APIResult};
// use axum::{
//     Json,
//     extract::{Path, State},
// };
// use diesel::prelude::*;
// use diesel_async::RunQueryDsl;
// use shiori_api_types::EncodableMedia;
// use shiori_database::models::media::Media;
// use shiori_database::schema::media;
// use utoipa_axum::{router::OpenApiRouter, routes};
//
// pub fn mount() -> OpenApiRouter<AppState> {
//     OpenApiRouter::new().routes(routes!(get_media))
// }

// #[derive(TryFromMultipart, ToSchema)]
// struct NewMediaRequest {
//     /// An array of files to upload.
//     #[schema(value_type = Vec<Object>)]
//     files: Vec<FieldData<NamedTempFile>>,
// }
//
// #[utoipa::path(
//     post,
//     path = "/media",
//     tag = "media",
//     request_body(
//         content = NewMediaRequest,
//         content_type = "multipart/form-data"
//     ),
//     responses(
//         (status = 200, description = "Successfully added media", body = Vec<EncodableMedia>),
//         (status = 400, description = "Invalid media file"),
//         (status = 500, description = "Internal server error")
//     )
// )]
// // TODO: Refactor all this
// async fn upload_media(
//     State(app): State<AppState>,
//     TypedMultipart(files): TypedMultipart<NewMediaRequest>,
// ) -> APIResult<Json<Vec<EncodableMedia>>> {
//     let mut conn = app.db().await?;
//
//     let mut uploaded = Vec::new();
//
//     let temp_file_dir = env::var("FILE_DIR").expect("FILE_DIR must be set");
//     for f in files.files {
//         let file_name = f.metadata.file_name.as_deref().ok_or_else(|| {
//             APIError::BadRequest("Uploaded media must have a filename.".to_string())
//         })?;
//
//         let ext = path::Path::new(file_name)
//             .extension()
//             .and_then(OsStr::to_str)
//             .map(str::to_ascii_lowercase)
//             .ok_or_else(|| {
//                 APIError::BadRequest("Uploaded media must have an extension.".to_string())
//             })?;
//
//         let new_media = NewMedia {
//             name: file_name,
//             size: f
//                 .contents
//                 .as_file()
//                 .metadata()
//                 .unwrap()
//                 .len()
//                 .try_into()
//                 .unwrap_or_else(|_| {
//                     println!("Failed to conver to i64");
//                     0
//                 }),
//             path: &format!("{}/{}", temp_file_dir, file_name),
//             extension: &ext,
//         };
//
//         let media = new_media.insert(&mut conn).await?;
//         uploaded.push(media);
//     }
//
//     Ok(Json(uploaded))
// }

// #[utoipa::path(
//     get,
//     path = "/media/{id}",
//     tag = "media",
//     params(
//         ("id" = i32, Path, description = "Id of the media")
//     ),
//     responses(
//         (status = 200, description = "Successfully fetched media", body = EncodableMedia),
//         (status = 404, description = "Media not found"),
//         (status = 500, description = "Internal server error")
//     )
// )]
// async fn get_media(
//     Path(media_id): Path<i32>,
//     State(app): State<AppState>,
// ) -> APIResult<Json<EncodableMedia>> {
//     let mut conn = app.db().await?;
//
//     let media = Media::query()
//         .filter(media::id.eq(media_id))
//         .first::<Media>(&mut conn)
//         .await?;
//
//     Ok(Json(()))
// }
