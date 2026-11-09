pub mod auth;
pub mod catalog;
pub mod emails;
pub mod form_ocr;
pub mod interests;
pub mod members;
pub mod statistics;
pub mod system;
pub mod users;

use axum::Router;

use crate::adapters::http::app_state::AppState;

/// Everything under `/api`. `upload_limit` caps multipart bodies (emails with
/// attachments, scanned forms); JSON endpoints keep axum's 2 MB default.
pub fn router(state: &AppState, upload_limit: usize) -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::router(state))
        .nest("/users", users::router())
        .nest("/members", members::router())
        .nest("/interests", interests::router())
        .nest("/genders", catalog::genders_router())
        .nest("/countries", catalog::countries_router())
        .nest("/emails", emails::router(upload_limit))
        .nest("/statistics", statistics::router())
        .nest("/formOcr", form_ocr::router(upload_limit))
        .nest("/system", system::router())
}
