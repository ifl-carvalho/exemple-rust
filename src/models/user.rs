use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserId {
    pub id: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserDTO {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserConditions {
    pub email: Option<String>,
}

pub type UserList = Vec<User>;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Auth {
    pub auth_token: String,
    pub refresh_token: String,
}
