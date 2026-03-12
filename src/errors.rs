use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("user not found")]
    UserNotFound,
    #[error("meeting not found")]
    MeetingNotFound,
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("internal error")]
    Internal,
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            AppError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AppError::UserNotFound | AppError::MeetingNotFound => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = Json(ErrorBody {
            error: self.to_string(),
        });
        (status, body).into_response()
    }
}
