use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier},
    Argon2, PasswordHash,
};

use crate::errors::AuthError;

pub fn hash_password(password: &str) -> Result<String, AuthError> {
    let hash = Argon2::default()
        .hash_password(password.as_bytes())
        .map_err(|_| AuthError::Argon2Error)?
        .to_string();

    Ok(hash)
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, AuthError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|_| AuthError::Argon2Error)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_password() {
        let hash = Argon2::default()
            .hash_password(b"supercoolpass")
            .unwrap()
            .to_string();
        assert!(verify_password(&hash, "supercoolpass").unwrap());
    }
}
