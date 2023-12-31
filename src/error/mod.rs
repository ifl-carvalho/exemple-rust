use axum::extract::rejection::FormRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

pub type Result<T, E = AppError> = core::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("Not Found: {0}")]
    NotFound(&'static str),
    #[error("Conflict: {0}")]
    Conflict(&'static str),
    #[error("Unauthorized: {0}")]
    Unauthorized(&'static str),
    #[error("Invalid params: {0:?}")]
    InvalidParams(Vec<&'static str>),
    #[error("Bad Request: {0:?}")]
    BadRequest(#[from] FormRejection),
    #[error("Unprocessable Entity: {0:?}")]
    UnprocessableEntity(#[from] ValidationErrors),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            AppError::Other(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Conflict(_) => (StatusCode::CONFLICT, self.to_string()),
            AppError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::InvalidParams(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::UnprocessableEntity(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, self.to_string())
            }
        };
        let body = Json(json!({
            "error": err_msg,
        }));
        (status, body).into_response()
    }
}
