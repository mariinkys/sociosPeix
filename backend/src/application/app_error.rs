use thiserror::Error;

/// Every error a use case can return. The HTTP adapter maps each variant to a
/// status code; the message is shown to the client unless it is internal.
#[derive(Error, Debug)]
pub enum AppError {
    /// 400 — the request is malformed or fails validation.
    #[error("{0}")]
    Validation(String),

    /// 401 — no (valid) session.
    #[error("Authentication required")]
    Unauthenticated,

    /// 401 — credentials or tokens were rejected.
    #[error("{0}")]
    Unauthorized(String),

    /// 403 — authenticated but not allowed.
    #[error("{0}")]
    Forbidden(String),

    /// 404
    #[error("{0}")]
    NotFound(String),

    /// 409 — e.g. a unique value is already taken.
    #[error("{0}")]
    Conflict(String),

    /// 429 — a quota or rate limit was hit.
    #[error("{0}")]
    TooManyRequests(String),

    /// 502 — the email provider failed or refused the message.
    #[error("{0}")]
    EmailSend(String),

    /// 503 — an optional feature is not available in this build/deployment.
    #[error("{0}")]
    Unavailable(String),

    /// 500
    #[error("Database error: {0}")]
    Database(String),

    /// 500
    #[error("Internal error: {0}")]
    Internal(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    pub fn internal(e: impl std::fmt::Display) -> Self {
        Self::Internal(e.to_string())
    }
}
