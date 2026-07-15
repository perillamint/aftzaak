use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("user already exists")]
    UserExists,

    #[error("unauthorized")]
    Unauthorized,

    #[error("item not found")]
    ItemNotFound,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("password hash error: {0}")]
    PasswordHash(String),

    #[error("jwt error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::InvalidCredentials | AppError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, self.to_string())
            }
            AppError::UserExists => (StatusCode::CONFLICT, self.to_string()),
            AppError::ItemNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            ),
        };
        (status, Json(ErrorResponse { error: message })).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
