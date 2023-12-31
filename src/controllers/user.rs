use crate::error::Result;
use crate::models::auth::{Auth, RefreshInput};
use crate::models::user::{UserId, UserInput};
use crate::repositories::RepoExt;
use crate::usecases;
use axum::{debug_handler, Extension, Json};

#[debug_handler]
pub async fn login(
    Extension(repo): RepoExt,
    Json(new_user): Json<UserInput>,
) -> Result<Json<Auth>> {
    let auth = usecases::user::login(repo.clone(), &new_user).await?;
    Ok(Json(auth))
}

#[debug_handler]
pub async fn register(
    Extension(repo): RepoExt,
    Json(new_user): Json<UserInput>,
) -> Result<Json<UserId>> {
    let user_id = usecases::user::register(repo.clone(), &new_user).await?;
    Ok(Json(user_id))
}

#[debug_handler]
pub async fn refresh(
    Extension(repo): RepoExt,
    Json(refresh_input): Json<RefreshInput>,
) -> Result<Json<Auth>> {
    let auth = usecases::user::refresh(repo.clone(), &refresh_input).await?;
    Ok(Json(auth))
}
