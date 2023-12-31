use crate::error::{AppError, Result};
use crate::models::auth::{Auth, RefreshInput};
use crate::models::user::{UserId, UserInput, UserPublicInfo};
use crate::repositories::{user::UserRepo, Repositories};
use crate::utils::hash::{hash_password, verify_password};
use crate::utils::jwt::{decode_jwt, encode_jwt};
use chrono::{Duration, Utc};
use std::sync::Arc;
use validator::Validate;

pub async fn login<R: Repositories>(repo: Arc<R>, credentials: &UserInput) -> Result<Auth> {
    let user = repo
        .user()
        .find_by_email(&credentials.email)
        .await
        .map_err(|_| AppError::Unauthorized("Invalid email or password"))?;

    verify_password(credentials.password.clone(), user.password_hash).await?;

    let access_token = encode_jwt(user.id, user.role.clone(), Duration::hours(1)).await?;
    let refresh_token = encode_jwt(user.id, user.role.clone(), Duration::days(7)).await?;

    let user_public_info = UserPublicInfo {
        id: user.id,
        role: user.role,
        email: user.email,
    };

    let auth = Auth {
        user: user_public_info,
        access_token,
        refresh_token,
        expires_in: Utc::now() + Duration::hours(1),
    };

    Ok(auth)
}

pub async fn register<R: Repositories>(repo: Arc<R>, user_input: &UserInput) -> Result<UserId> {
    user_input.validate()?;

    let new_user = UserInput {
        email: user_input.email.clone(),
        password: hash_password(user_input.password.clone()).await?,
    };

    let user_id = repo.user().add(&new_user).await?;

    Ok(user_id)
}

pub async fn refresh<R: Repositories>(repo: Arc<R>, refresh_input: &RefreshInput) -> Result<Auth> {
    let claim = decode_jwt(refresh_input.token.clone()).await?;

    let user = repo
        .user()
        .find_by_id(&claim.sub)
        .await
        .map_err(|_| AppError::Unauthorized("Invalid token"))?;

    let access_token = encode_jwt(user.id, user.role.clone(), Duration::hours(1)).await?;
    let refresh_token = encode_jwt(user.id, user.role.clone(), Duration::days(7)).await?;

    let user_public_info = UserPublicInfo {
        id: user.id,
        role: user.role,
        email: user.email,
    };

    let auth = Auth {
        user: user_public_info,
        access_token,
        refresh_token,
        expires_in: Utc::now() + Duration::hours(1),
    };

    Ok(auth)
}
