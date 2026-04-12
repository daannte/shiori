use axum_extra::extract::cookie::{Cookie, SameSite};
use base64::{Engine, engine::general_purpose};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use once_cell::sync::Lazy;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Generates a random 64-byte secret and encodes it as Base64.
fn generate_secret() -> String {
    if cfg!(debug_assertions) {
        "dev-secret".to_string()
    } else {
        let mut bytes = [0u8; 64];
        rand::rng().fill_bytes(&mut bytes);
        general_purpose::STANDARD.encode(bytes)
    }
}

// I dont expect the server to crash every 2 seconds, so I'll keep it like this for now.
// This will invalidate all tokens on server restart
static ACCESS_TOKEN_SECRET: Lazy<String> = Lazy::new(generate_secret);
static REFRESH_TOKEN_SECRET: Lazy<String> = Lazy::new(generate_secret);

#[derive(Debug, Serialize, Deserialize)]
struct AccessTokenClaims {
    iat: i64,
    exp: i64,
    sub: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RefreshTokenClaims {
    iat: i64,
    exp: i64,
    sub: String,
    jti: String,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct JwtTokenPair {
    /// Short-lived access token used for authenticated requests.
    pub access_token: AccessToken,

    /// Long-lived refresh token used to obtain new access tokens.
    pub refresh_token: RefreshToken,
}

impl JwtTokenPair {
    /// Creates a new access and refresh token pair for a user.
    pub fn new(user_id: i32) -> Result<JwtTokenPair, jsonwebtoken::errors::Error> {
        Ok(Self {
            access_token: AccessToken::new(user_id)?,
            refresh_token: RefreshToken::new(user_id)?,
        })
    }
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct AccessToken {
    /// JWT access token
    pub token: String,

    /// Token expiry time
    pub expires_at: DateTime<Utc>,
}

impl AccessToken {
    /// Creates a signed JWT access token valid for 15 minutes.
    pub(crate) fn new(user_id: i32) -> Result<AccessToken, jsonwebtoken::errors::Error> {
        let times = JwtTimes::new(Duration::minutes(15));

        let claims = AccessTokenClaims {
            iat: times.iat,
            exp: times.exp,
            sub: user_id.to_string(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(ACCESS_TOKEN_SECRET.as_bytes()),
        )?;

        Ok(Self {
            token,
            expires_at: times.exp_dt,
        })
    }

    /// Decode a signed JWT access token and return the user ID.
    pub fn decode(token: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let data = decode::<AccessTokenClaims>(
            &token,
            &DecodingKey::from_secret(ACCESS_TOKEN_SECRET.as_bytes()),
            &Validation::default(),
        )?;

        Ok(data.claims.sub)
    }

    pub fn to_cookie(self) -> Cookie<'static> {
        create_cookie("access_token", self.token, self.expires_at)
    }

    pub fn remove_cookie() -> Cookie<'static> {
        create_cookie("access_token", String::new(), Utc::now())
    }
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct RefreshToken {
    /// JWT refresh token
    pub token: String,

    /// JWT ID used for tracking/revocation.
    #[serde(skip_serializing)]
    pub jti: String,

    /// Token expiry time
    pub expires_at: DateTime<Utc>,
}

impl RefreshToken {
    /// Creates a signed JWT refresh token valid for 7 days.
    pub(crate) fn new(user_id: i32) -> Result<RefreshToken, jsonwebtoken::errors::Error> {
        let times = JwtTimes::new(Duration::days(7));
        let jti = Uuid::new_v4().to_string();

        let claims = RefreshTokenClaims {
            iat: times.iat,
            exp: times.exp,
            sub: user_id.to_string(),
            jti: jti.clone(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(REFRESH_TOKEN_SECRET.as_bytes()),
        )?;

        Ok(Self {
            token,
            jti,
            expires_at: times.exp_dt,
        })
    }

    /// Decode a signed JWT refresh token and return the user ID and jti.
    pub fn decode(token: &str) -> Result<(String, String), jsonwebtoken::errors::Error> {
        let data = decode::<RefreshTokenClaims>(
            &token,
            &DecodingKey::from_secret(REFRESH_TOKEN_SECRET.as_bytes()),
            &Validation::default(),
        )?;

        Ok((data.claims.sub, data.claims.jti))
    }

    pub fn to_cookie(self) -> Cookie<'static> {
        create_cookie("refresh_token", self.token, self.expires_at)
    }

    pub fn remove_cookie() -> Cookie<'static> {
        create_cookie("refresh_token", String::new(), Utc::now())
    }
}

/// JWT timestamp metadata.
pub struct JwtTimes {
    /// Issued-at timestamp (seconds since epoch)
    pub iat: i64,

    /// Expiration timestamp (seconds since epoch)
    pub exp: i64,

    /// Expiration as a UTC datetime
    pub exp_dt: DateTime<Utc>,
}

impl JwtTimes {
    /// Creates JWT timestamp data for a given duration.
    pub fn new(duration: Duration) -> Self {
        let now = Utc::now();
        let exp_dt = now + duration;

        Self {
            iat: now.timestamp(),
            exp: exp_dt.timestamp(),
            exp_dt,
        }
    }
}

fn create_cookie(name: &str, value: String, expires_at: DateTime<Utc>) -> Cookie<'static> {
    let offset = time::OffsetDateTime::from_unix_timestamp(expires_at.timestamp()).unwrap();
    Cookie::build((name.to_string(), value))
        .path("/")
        .secure(cfg!(not(debug_assertions)))
        .http_only(true)
        .expires(offset)
        .same_site(SameSite::Lax)
        .build()
}
