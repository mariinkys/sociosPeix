use axum::{Router, routing::get};

use crate::adapters::http::app_state::AppState;

/// Public (no session needed).
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/version", get(version))
        .route("/health", get(health))
}

/// The backend version, as plain text (the frontend's version chip shows it).
async fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Liveness probe for container healthchecks.
async fn health() -> &'static str {
    "ok"
}
