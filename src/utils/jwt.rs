use crate::error::{AppError, Result};
use crate::models::auth::Claims;
use crate::models::user::Role;
use anyhow::Context;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;
use tokio::task;
use uuid::Uuid;

pub async fn create_jwt(sub: Uuid, scope: Role, duration: Duration) -> Result<String> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token = task::spawn_blocking(move || {
        let claims = Claims {
            sub,
            scope,
            iat: Utc::now().timestamp(),
            exp: (Utc::now() + duration).timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
    })
    .await
    .context("Error on JWT encoding Blocking Thread")?;

    match token {
        Ok(token) => Ok(token),
        Err(_) => Err(AppError::Unauthorized("Failed to create token")),
    }
}

pub async fn verify_jwt(token: String) -> Result<Claims> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token = task::spawn_blocking(move || {
        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
    })
    .await
    .context("Error on JWT decoding Blocking Thread")?;

    match token {
        Ok(token) => Ok(token.claims),
        Err(_) => Err(AppError::Unauthorized("Invalid token")),
    }
}
