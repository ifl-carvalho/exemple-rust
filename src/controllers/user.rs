use crate::error::Result;
use crate::models::user::{Auth, UserDTO, UserId};
use crate::repositories::RepoExt;
use crate::usecases;
use axum::{debug_handler, Extension, Json};

#[debug_handler]
pub async fn sign_in(
    Extension(repo): RepoExt,
    Json(new_user): Json<UserDTO>,
) -> Result<Json<Auth>> {
    let user = usecases::users::sign_in(repo.clone(), &new_user).await?;
    Ok(Json(user))
}

#[debug_handler]
pub async fn sign_up(
    Extension(repo): RepoExt,
    Json(new_user): Json<UserDTO>,
) -> Result<Json<UserId>> {
    let user_id = usecases::users::sign_up(repo.clone(), &new_user).await?;
    Ok(Json(user_id))
}
