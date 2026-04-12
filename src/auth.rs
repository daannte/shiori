use argon2::{
    Argon2, PasswordHash,
    password_hash::{PasswordHasher, PasswordVerifier},
};

use crate::errors::{AppResult, server_error};

pub fn hash_password(password: &str) -> AppResult<String> {
    let hash = Argon2::default()
        .hash_password(password.as_bytes())
        .map_err(|_| server_error("Error during authentication"))?
        .to_string();

    Ok(hash)
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok(),
        Err(_) => false,
    }
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
        assert!(verify_password(&hash, "supercoolpass"));
    }
}
