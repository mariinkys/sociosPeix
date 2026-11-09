//! PostgreSQL implementations of the application's persistence ports.

use sqlx::PgPool;

use crate::app_error::AppError;

pub mod catalog;
pub mod email;
pub mod interest;
pub mod member;
pub mod password_reset;
pub mod refresh_token;
pub mod statistics;
pub mod user;

#[derive(Clone)]
pub struct PostgresPersistence {
    pool: PgPool,
}

impl PostgresPersistence {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::Database(value.to_string())
    }
}

/// Maps a unique-constraint violation to a 409 with `message`; anything else
/// stays a database error.
pub(crate) fn conflict_on_unique(e: sqlx::Error, message: impl FnOnce() -> String) -> AppError {
    match &e {
        sqlx::Error::Database(db) if db.is_unique_violation() => AppError::Conflict(message()),
        _ => e.into(),
    }
}

/// `%term%` for a case-insensitive `ILIKE`, with LIKE wildcards in the user's
/// input escaped so they match literally.
pub(crate) fn like_pattern(term: &str) -> String {
    let mut out = String::with_capacity(term.len() + 2);
    out.push('%');
    for c in term.chars() {
        if matches!(c, '%' | '_' | '\\') {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('%');
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn like_pattern_escapes_wildcards() {
        assert_eq!(super::like_pattern("a_b%"), r"%a\_b\%%");
    }
}
