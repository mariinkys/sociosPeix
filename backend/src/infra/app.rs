use std::time::Duration;

use axum::{
    Router,
    http::{HeaderName, HeaderValue, Method, Request, StatusCode, header},
    response::IntoResponse,
};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};

use crate::{
    adapters::http::{app_error_impl::error_response, app_state::AppState, routes},
    infra::config::AppConfig,
};

const REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");

pub fn create_app(state: AppState, config: &AppConfig) -> Router {
    let origins: Vec<HeaderValue> = config
        .cors_allowed_origins
        .iter()
        .filter_map(|o| o.parse().ok())
        .collect();
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600));

    let mut app = Router::new()
        .nest("/api", routes::router(&state, config.max_upload_bytes))
        .fallback(not_found)
        .with_state(state)
        .layer(cors)
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-store"),
        ));

    if config.cookie_secure {
        // Only meaningful when served over HTTPS.
        app = app.layer(SetResponseHeaderLayer::overriding(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=31536000; includeSubDomains"),
        ));
    }

    app.layer(
        TraceLayer::new_for_http().make_span_with(|req: &Request<_>| {
            let request_id = req
                .headers()
                .get(&REQUEST_ID)
                .and_then(|v| v.to_str().ok())
                .unwrap_or_default();
            tracing::info_span!("http", method = %req.method(), uri = %req.uri().path(), request_id)
        }),
    )
    .layer(PropagateRequestIdLayer::new(REQUEST_ID))
    .layer(SetRequestIdLayer::new(REQUEST_ID, MakeRequestUuid))
}

async fn not_found() -> impl IntoResponse {
    error_response(StatusCode::NOT_FOUND, "Not found")
}
