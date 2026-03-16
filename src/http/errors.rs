use axum::http::StatusCode;
use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("request is bad: {0}")]
    BadRequest(String),
    #[error("Not found")]
    NotFound,
    #[error("Not found")]
    NoContent,
    #[error("UNAUTHORIZED")]
    UNAUTHORIZED,
}
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Page not found").into_response(),
            AppError::NoContent => (StatusCode::NOT_FOUND, "Content not found").into_response(),
            AppError::UNAUTHORIZED => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
        }
    }
}
