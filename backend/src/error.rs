use axum::response::{IntoResponse, Response};
use axum::Json;
use http::StatusCode;
use serde_json::json;
use sqlx::Error;

#[derive(Debug)]
pub enum AppError {
    Question(QuestionError),
    Database(sqlx::Error),
    MissingCredentials,
    UserDoesNotExist,
    UserAlreadyExists,
    InvalidToken,
    InternalServerError,
    #[allow(dead_code)]
    Any(anyhow::Error),
    MissingContent,
}

#[derive(derive_more::Display, Debug)]
pub enum QuestionError {
    InvalidId,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Question(err) => match err {
                QuestionError::InvalidId => (StatusCode::NOT_FOUND, err.to_string()),
            },
            AppError::Database(err) => (StatusCode::SERVICE_UNAVAILABLE, err.to_string()),
            AppError::Any(err) => {
                let message = format!("Internal server error! {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            AppError::MissingCredentials => (
                StatusCode::UNAUTHORIZED,
                "Your credentials were missing or otherwise incorrect".to_string(),
            ),
            AppError::UserDoesNotExist => (
                StatusCode::UNAUTHORIZED,
                "Your account does not exist!".to_string(),
            ),
            AppError::UserAlreadyExists => (
                StatusCode::UNAUTHORIZED,
                "There is already an account with that email address in the system".to_string(),
            ),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid Token".to_string()),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something terrible happened".to_string(),
            ),
            AppError::MissingContent => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Comment missing content".to_string(),
            ),
        };

        let body = Json(json!({"error": error_message}));
        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: Error) -> Self {
        AppError::Database(value)
    }
}
