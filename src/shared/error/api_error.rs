use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Not found")]
    NotFound,

    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),
    // #[error("Unauthorized")]
    // Unauthorized,
    #[error("Internal server error")]
    InternalServerError,
    // #[error("Bad request: {0}")]
    // BadRequest(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            // ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            // ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
        };

        #[derive(Serialize)]
        struct ErrorResponse {
            error: String,
        }

        let body = ErrorResponse {
            error: self.to_string(),
        };

        (status, axum::Json(body)).into_response()
    }
}

// Conversion from diesel::result::Error to ApiError
impl From<diesel::result::Error> for ApiError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => ApiError::NotFound,
            _ => ApiError::DatabaseError(error.to_string()),
        }
    }
}
