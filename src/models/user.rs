use crate::models::common::{Date, Uuid};
use axum::Extension;
use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use sqlx::FromRow;
use std::sync::Arc;
use validator::Validate;

pub type UserExt = Extension<Arc<User>>;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserId {
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub id: Uuid,
    pub role: Role,
    pub email: String,
    pub password_hash: String,
    pub created_at: Date,
    pub updated_at: Option<Date>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPublicInfo {
    pub id: Uuid,
    pub role: Role,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
#[sqlx(type_name = "enum_role", rename_all = "lowercase")]
pub enum Role {
    Admin,
    User,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UserInput {
    #[validate(email)]
    pub email: String,
    pub password: String,
}
