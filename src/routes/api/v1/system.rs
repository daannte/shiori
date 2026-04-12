use axum::{Json, extract::State};
use shiori_api_types::EncodableMeta;
use shiori_database::models::User;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{config::state::AppState, errors::AppResult, routes::openapi::tags};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(meta))
}

/// Info about the server.
#[utoipa::path(
    get,
    path = "/meta",
    tag = tags::SYSTEM,
    responses(
        (status = 200, description = "Successfully retrieved server metadata", body = inline(EncodableMeta)),
        (status = 500, description = "Internal server error")
    )
)]
async fn meta(State(app): State<AppState>) -> AppResult<Json<EncodableMeta>> {
    let mut conn = app.db().await?;
    let initialized = User::count(&mut conn).await? > 0;

    Ok(Json(EncodableMeta { initialized }))
}
