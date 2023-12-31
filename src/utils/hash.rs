use crate::error::{AppError, Result};
use anyhow::{anyhow, Context};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use tokio::task;

pub async fn hash_password(password: String) -> Result<String> {
    let hash = task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2.hash_password(password.as_bytes(), &salt);
        hash.map(|hash| hash.to_string())
    })
    .await
    .context("Error on password hashing Blocking Thread")?;

    match hash {
        Ok(hash) => Ok(hash),
        Err(_) => Err(AppError::Other(anyhow!("Error hashing password"))),
    }
}

pub async fn verify_password(password: String, password_hash: String) -> Result<()> {
    let result = task::spawn_blocking(move || {
        let parsed_hash = PasswordHash::new(&password_hash)?;
        let argon2 = Argon2::default();
        argon2.verify_password(&password.as_bytes(), &parsed_hash)
    })
    .await
    .context("Error on password verification Blocking Thread")?;

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(AppError::Unauthorized("Invalid email or password")),
    }
}
