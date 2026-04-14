use rand::distr::{Alphanumeric, SampleString};
use sha2::{Digest, Sha256};

const TOKEN_LENGTH: usize = 32;
const TOKEN_PREFIX: &str = "shiori";
const KEY_ID_LENGTH: usize = 8;

#[derive(Debug, thiserror::Error)]
#[error("Invalid token")]
pub struct InvalidTokenError;

pub struct HashedToken {
    pub key_id: String,
    pub hash: Vec<u8>,
}

impl HashedToken {
    fn hash(secret: &str) -> Vec<u8> {
        Sha256::digest(secret.as_bytes()).to_vec()
    }

    pub fn parse(token: &str) -> Result<Self, InvalidTokenError> {
        let parts: Vec<&str> = token.split('_').collect();

        if parts.len() != 3 {
            return Err(InvalidTokenError);
        }

        let prefix = parts[0];
        let key_id = parts[1];
        let secret = parts[2];

        if prefix != TOKEN_PREFIX {
            return Err(InvalidTokenError);
        }

        Ok(Self {
            key_id: key_id.to_string(),
            hash: Self::hash(secret),
        })
    }
}

pub struct Token {
    key_id: String,
    secret: String,
    full: String,
}

impl Default for Token {
    fn default() -> Self {
        let key_id = Alphanumeric.sample_string(&mut rand::rng(), KEY_ID_LENGTH);
        let secret = Alphanumeric.sample_string(&mut rand::rng(), TOKEN_LENGTH);

        let full = format!("{}_{}_{}", TOKEN_PREFIX, key_id, secret);

        Self {
            key_id,
            secret,
            full,
        }
    }
}

impl Token {
    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    pub fn secret(&self) -> &str {
        &self.secret
    }

    pub fn token(&self) -> &str {
        &self.full
    }

    pub fn hashed(&self) -> HashedToken {
        HashedToken {
            key_id: self.key_id.clone(),
            hash: HashedToken::hash(&self.secret),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_format_is_valid() {
        let token = Token::default();
        let parts: Vec<&str> = token.token().split('_').collect();

        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], TOKEN_PREFIX);
        assert_eq!(parts[1], token.key_id());
    }

    #[test]
    fn token_contains_correct_key_id() {
        let token = Token::default();

        assert!(token.token().contains(token.key_id()));
    }

    #[test]
    fn parse_valid_token() {
        let token = Token::default();
        let parsed = HashedToken::parse(token.token());

        assert!(parsed.is_ok());

        let parsed = parsed.unwrap();
        assert_eq!(parsed.key_id, token.key_id());
    }

    #[test]
    fn parse_rejects_invalid_prefix() {
        let bad_token = "wrong_ABC123_secret123";

        let result = HashedToken::parse(bad_token);

        assert!(result.is_err());
    }

    #[test]
    fn parse_rejects_wrong_format() {
        let bad_token = "shiori_onlytwoparts";

        let result = HashedToken::parse(bad_token);

        assert!(result.is_err());
    }

    #[test]
    fn parsed_hash_matches_secret() {
        let token = Token::default();

        let parsed = HashedToken::parse(token.token()).unwrap();
        let expected = HashedToken::hash(token.secret());

        assert_eq!(parsed.hash, expected);
    }
}
