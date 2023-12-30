use crate::error::Result;
use crate::models::auth::Auth;
use crate::models::user::{PublicUserInfo, UserId, UserInput};
use crate::repositories::{user::UserRepo, Repositories};
use crate::utils::hash::{hash_password, verify_password};
use crate::utils::jwt::create_jwt;
use chrono::{Duration, Utc};
use std::sync::Arc;
use validator::Validate;

pub async fn login<R: Repositories>(repo: Arc<R>, credentials: &UserInput) -> Result<Auth> {
    let user = repo.user().find_by_email(&credentials.email).await?;

    verify_password(credentials.password.clone(), user.password_hash).await?;

    let access_token = create_jwt(user.id, user.role.clone(), Duration::hours(1)).await?;

    let refresh_token = create_jwt(user.id, user.role.clone(), Duration::days(7)).await?;

    let public_user = PublicUserInfo {
        id: user.id,
        role: user.role,
        email: user.email,
    };

    let auth = Auth {
        user: public_user,
        access_token,
        refresh_token,
        expires_in: Utc::now() + Duration::hours(1),
    };

    Ok(auth)
}

pub async fn register<R: Repositories>(repo: Arc<R>, user_input: &UserInput) -> Result<UserId> {
    let new_user = UserInput {
        email: user_input.email.clone(),
        password: hash_password(user_input.password.clone()).await?,
    };

    user_input.validate()?;

    let user_id = repo.user().add(&new_user).await?;

    Ok(user_id)
}
