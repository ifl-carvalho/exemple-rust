use std::sync::Arc;

use crate::error::Result;
use crate::models::user::{Auth, UserConditions, UserDTO, UserId, UserList};
use crate::repositories::{user::UserRepo, Repositories};

pub async fn search<R: Repositories>(
    repo: Arc<R>,
    conditions: &UserConditions,
) -> Result<UserList> {
    let users = repo.user().find_all(conditions).await?;
    Ok(users)
}

pub async fn sign_in<R: Repositories>(repo: Arc<R>, user_dto: &UserDTO) -> Result<Auth> {
    let _user = repo.user().find_by_email(&user_dto.email).await?;

    let auth = Auth {
        auth_token: "".to_string(),
        refresh_token: "".to_string(),
    };
    Ok(auth)
}

pub async fn sign_up<R: Repositories>(repo: Arc<R>, new_user: &UserDTO) -> Result<UserId> {
    let user_id = repo.user().add(&new_user).await?;
    Ok(user_id)
}
