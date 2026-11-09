use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::{adapters::http::dto::utc_timestamp, app_error::AppError};

/// The JSON error body the frontend expects: `{status, message, timestamp}`.
#[derive(Serialize)]
pub struct ErrorBody {
    pub status: u16,
    pub message: String,
    pub timestamp: String,
}

pub fn error_response(status: StatusCode, message: impl Into<String>) -> Response {
    let body = ErrorBody {
        status: status.as_u16(),
        message: message.into(),
        timestamp: utc_timestamp(chrono::Utc::now().naive_utc()),
    };
    (status, Json(body)).into_response()
}

impl AppError {
    pub fn status(&self) -> StatusCode {
        match self {
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthenticated | AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            AppError::EmailSend(_) => StatusCode::BAD_GATEWAY,
            AppError::Unavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::Database(_) | AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status();
        let message = match &self {
            // Never leak internals to the client.
            AppError::Database(_) | AppError::Internal(_) => {
                tracing::error!(error = %self, "request failed");
                "An unexpected error occurred".to_owned()
            }
            AppError::EmailSend(_) => {
                tracing::warn!(error = %self, "email sending failed");
                self.to_string()
            }
            _ => {
                tracing::debug!(error = %self, %status, "request rejected");
                self.to_string()
            }
        };
        error_response(status, message)
    }
}
