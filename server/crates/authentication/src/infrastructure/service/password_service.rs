use crate::domain::authentication_error::AuthenticationError;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub fn hash(password: String) -> Result<String, AuthenticationError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|error| AuthenticationError::DatabaseError(error.to_string()))?;

    Ok(hash.to_string())
}

pub fn compare(password: String, hash: String) -> Result<bool, AuthenticationError> {
    let parsed_hash = PasswordHash::new(&hash)
        .map_err(|error| AuthenticationError::DatabaseError(error.to_string()))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
