use base64::{Engine, engine::general_purpose};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use once_cell::sync::Lazy;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Generates a random 64-byte secret and encodes it as Base64.
fn generate_secret() -> String {
    let mut bytes = [0u8; 64];
    rand::rng().fill_bytes(&mut bytes);
    general_purpose::STANDARD.encode(bytes)
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
        let times = JwtTimes::new(Duration::days(15));

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
