use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub error: String,
}

impl ErrorOutput {
    pub fn new<E: Into<String>>(error: E) -> Self {
        Self {
            error: error.into(),
        }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("conflict: {0}")]
    ConflictError(String),
    #[error("permission deny: {0}")]
    PermissionDenyError(String),
    #[error("invalid: {0}")]
    InvalidError(String),
    #[error("sql error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("passwd hash error: {0}")]
    PasswdHashError(#[from] argon2::password_hash::Error),
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("any error: {0}")]
    AnyError(#[from] anyhow::Error),
    #[error("general error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("general error: {0}")]
    GeneralError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::ConflictError(_) => StatusCode::CONFLICT,
            AppError::PermissionDenyError(_) => StatusCode::FORBIDDEN,
            AppError::InvalidError(_) => StatusCode::BAD_REQUEST,
            AppError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PasswdHashError(_) => StatusCode::UNAUTHORIZED,
            AppError::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::AnyError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SerdeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::GeneralError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(json!(ErrorOutput::new(self.to_string())))).into_response()
    }
}
