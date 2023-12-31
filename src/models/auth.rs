use crate::models::common::{Date, Uuid};
use crate::models::user::{Role, UserPublicInfo};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: Date,
    pub user: UserPublicInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: Uuid,
    pub iat: i64,
    pub exp: i64,
    pub scope: Role,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshInput {
    pub token: String,
}
