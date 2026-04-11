use axum::{Json, extract::State, middleware, response::IntoResponse};
use axum_extra::extract::CookieJar;
use serde::Deserialize;
use shiori_api_types::EncodableUser;
use shiori_jwt::JwtTokenPair;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use shiori_database::models::{NewRefreshToken, NewUser, User};

use crate::{
    auth::{hash_password, verify_password},
    config::state::AppState,
    errors::{APIError, APIResult},
    middleware::auth::auth_middleware,
    routes::openapi::tags,
};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(login))
        .routes(routes!(register))
        .routes(routes!(refresh_token))
        .routes(routes!(logout).layer(middleware::from_fn(auth_middleware)))
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct AuthRequest {
    /// Username of the account.
    #[schema(examples("Reaper"))]
    username: String,

    /// Password associated with the account.
    #[schema(examples("supercoolpass"))]
    password: String,
}

/// Login
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = tags::AUTH,
    request_body = inline(AuthRequest),
    responses(
        (status = 200, description = "Successfully logged in", body = inline(EncodableUser),
            headers(
                ("set-cookie" = String, description = "Sets access_token and refresh_token HttpOnly cookies")
            )
        ),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
async fn login(
    State(app): State<AppState>,
    jar: CookieJar,
    Json(body): Json<AuthRequest>,
) -> APIResult<impl IntoResponse> {
    let mut conn = app.db().await?;

    let user = User::find_by_username(&mut conn, &body.username)
        .await?
        .ok_or_else(|| APIError::Unauthorized)?;

    // TODO: Maybe lock account after 3 or 5 attempts?
    let valid = verify_password(&user.hashed_password, &body.password)
        .map_err(|_| APIError::Unauthorized)?;

    if !valid {
        return Err(APIError::Unauthorized);
    }

    let tokens = JwtTokenPair::new(user.id).map_err(|_| {
        APIError::InternalServerError("Failed to generate authentication tokens".to_string())
    })?;

    let rt = NewRefreshToken {
        jti: &tokens.refresh_token.jti,
        user_id: user.id,
        expires_at: tokens.refresh_token.expires_at,
    };

    rt.insert(&conn).await?;

    let jar = jar
        .add(tokens.access_token.to_cookie())
        .add(tokens.refresh_token.to_cookie());

    Ok((jar, Json(EncodableUser::from(user))))
}

/// Register
#[utoipa::path(
    post,
    path = "/auth/register",
    tag = tags::AUTH,
    request_body = inline(AuthRequest),
    responses(
        (status = 200, description = "Successfully registered", body = inline(EncodableUser)),
        (status = 400, description = "Bad request payload"),
        (status = 409, description = "Username already taken"),
        (status = 500, description = "Internal server error")
    )
)]
async fn register(
    State(app): State<AppState>,
    Json(body): Json<AuthRequest>,
) -> APIResult<Json<EncodableUser>> {
    let mut conn = app.db().await?;

    let has_users = User::count(&mut conn).await? > 0;
    let is_server_owner = !has_users;

    if body.password.len() < 8 {
        return Err(APIError::BadRequest(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    let hash = hash_password(&body.password)?;

    let new_user = NewUser {
        username: &body.username,
        hashed_password: &hash,
        is_server_owner,
    };

    let user = new_user.insert(&conn).await?;

    Ok(Json(user.into()))
}

/// Refresh JWT token
#[utoipa::path(
    post,
    path = "/auth/refresh-token",
    tag = tags::AUTH,
    responses(
        (status = 200, description = "Successfully refreshed JWT token"),
        (status = 500, description = "Internal server error")
    )
)]
async fn refresh_token() {}

/// Logout
#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = tags::AUTH,
    security(
        ("bearerAuth" = [])
    ),
    responses(
        (status = 200, description = "Successfully logged out"),
        (status = 500, description = "Internal server error")
    )
)]
async fn logout() {}
