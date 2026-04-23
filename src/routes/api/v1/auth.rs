use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;
use serde::Deserialize;
use shiori_api_types::EncodableUser;
use shiori_database::models::RefreshToken as RefreshModel;
use shiori_jwt::{AccessToken, JwtTokenPair, RefreshToken};
use utoipa_axum::{router::OpenApiRouter, routes};

use shiori_database::models::{NewRefreshToken, NewUser, User};

use crate::{
    auth::{hash_password, verify_password},
    config::state::AppState,
    errors::{AppResult, bad_request, server_error, unauthorized},
    middleware::auth::AuthUser,
    routes::openapi::tags,
};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(logout))
        .routes(routes!(me))
}

pub fn mount_public() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(login))
        .routes(routes!(register))
        .routes(routes!(refresh_token))
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
                ("set-cookie" = String, description = "Sets access_token and refresh_token cookies")
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
) -> AppResult<impl IntoResponse> {
    let mut conn = app.db().await?;

    let user = User::find_by_username(&mut conn, &body.username).await?;

    // TODO: Maybe lock account after 3 or 5 attempts?
    let valid = verify_password(&user.hashed_password, &body.password);

    if !valid {
        return Err(unauthorized("Invalid credentials"));
    }

    let tokens =
        JwtTokenPair::new(user.id).map_err(|_| server_error("Error during authentication"))?;

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
) -> AppResult<Json<EncodableUser>> {
    let mut conn = app.db().await?;

    let has_users = User::count(&mut conn).await? > 0;
    let is_server_owner = !has_users;

    if body.password.len() < 8 {
        return Err(bad_request("Password must be at least 8 characters"));
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
    params(
        ("refresh_token" = String, Cookie, description = "Refresh token")
    ),
    responses(
        (status = 204, description = "Successfully refreshed JWT token", headers(
                ("set-cookie" = String, description = "Sets access_token and refresh_token HttpOnly cookies")
            )
        ),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
async fn refresh_token(
    State(app): State<AppState>,
    jar: CookieJar,
) -> AppResult<(StatusCode, CookieJar)> {
    let mut conn = app.db().await?;
    let cookie = jar
        .get("refresh_token")
        .ok_or_else(|| bad_request("missing refresh token"))?;

    let (user_id_str, jti) =
        RefreshToken::decode(cookie.value()).map_err(|_| unauthorized("invalid refresh token"))?;

    let user_id = user_id_str
        .parse::<i32>()
        .map_err(|_| unauthorized("invalid refresh token"))?;

    let token = RefreshModel::find(&mut conn, &jti).await?;

    if token.revoked_at.is_some() || token.user_id != user_id {
        return Err(unauthorized("invalid refresh token"));
    }

    RefreshModel::revoke(&mut conn, &jti).await?;

    let tokens = JwtTokenPair::new(user_id).map_err(|_| server_error("internal server error"))?;

    let rt = NewRefreshToken {
        jti: &tokens.refresh_token.jti,
        user_id,
        expires_at: tokens.refresh_token.expires_at,
    };

    rt.insert(&conn).await?;

    let jar = jar
        .add(tokens.access_token.to_cookie())
        .add(tokens.refresh_token.to_cookie());

    Ok((StatusCode::NO_CONTENT, jar))
}

/// Logout
#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = tags::AUTH,
    security(
        ("cookie" = [])
    ),
    responses(
        (status = 204, description = "Successfully logged out", headers(
                ("set-cookie" = String, description = "Removes access_token and refresh_token cookies")
            )
        ),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
async fn logout(State(app): State<AppState>, jar: CookieJar) -> AppResult<(StatusCode, CookieJar)> {
    let mut conn = app.db().await?;

    let cookie = jar
        .get("refresh_token")
        .ok_or_else(|| bad_request("missing refresh token"))?;

    let (_, jti) =
        RefreshToken::decode(cookie.value()).map_err(|_| unauthorized("invalid refresh token"))?;

    RefreshModel::revoke(&mut conn, &jti).await?;

    let jar = jar
        .remove(RefreshToken::remove_cookie())
        .remove(AccessToken::remove_cookie());

    Ok((StatusCode::NO_CONTENT, jar))
}

/// Currently authenticated user
#[utoipa::path(
    get,
    path = "/auth/me",
    tag = tags::AUTH,
    security(
        ("cookie" = []),
        ("api_token" = [])
    ),
    responses(
        (status = 200, description = "Successfully retrieved current user", body = inline(EncodableUser)),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
async fn me(AuthUser(auth): AuthUser) -> AppResult<Json<EncodableUser>> {
    Ok(Json(EncodableUser::from(auth.user().to_owned())))
}
