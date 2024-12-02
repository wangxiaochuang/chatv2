use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

use crate::error::AppError;

pub mod token;

pub fn hash_passwd(passwd: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let hasher = Argon2::default();
    let passwd_hash = hasher.hash_password(passwd.as_bytes(), &salt)?.to_string();
    Ok(passwd_hash)
}

pub fn verify_passwd(passwd: &str, passwd_hash: &str) -> Result<(), AppError> {
    let hasher = Argon2::default();
    let password_hash = PasswordHash::new(passwd_hash)?;
    hasher.verify_password(passwd.as_bytes(), &password_hash)?;
    Ok(())
}
