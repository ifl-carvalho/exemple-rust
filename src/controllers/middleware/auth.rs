use crate::error::AppError;
use crate::repositories::user::UserRepo;
use crate::repositories::{RepoExt, Repositories};
use crate::utils::jwt::decode_jwt;
use axum::{extract::Request, middleware::Next, response::Response, Extension};
use std::sync::Arc;

pub async fn guard(
    Extension(repo): RepoExt,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or(AppError::Unauthorized("Missing Authorization header"))?;

    let token = auth_header
        .to_str()
        .map_err(|_| AppError::Unauthorized("Invalid Authorization header"))?
        .replace("Bearer ", "");

    let claims = decode_jwt(token).await?;

    let user = repo
        .user()
        .find_by_id(&claims.sub)
        .await
        .map_err(|_| AppError::Unauthorized("Invalid Authorization header"))?;

    let user = Arc::new(user);

    req.extensions_mut().insert(Extension(user));

    Ok(next.run(req).await)
}
