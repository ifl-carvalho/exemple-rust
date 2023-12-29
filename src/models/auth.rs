use crate::models::date::Date;
use crate::models::user::PublicUserInfo;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user::Role;

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: Date,
    pub user: PublicUserInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: Uuid,
    pub iat: i64,
    pub exp: i64,
    pub scope: Role,
}
