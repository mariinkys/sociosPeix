use std::path::Path;
use std::sync::{Arc, Mutex};

use anyhow::Context;
use axum::extract::{DefaultBodyLimit, Multipart, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use axum::routing::post;
use axum::Router;

use pdf_analyzer::config::FormTemplate;
use pdf_analyzer::ocr::Engine;
use tower_http::cors::CorsLayer;

const TEMPLATE_PATH: &str = "pdf-analyzer/template.toml";

struct AppState {
    engine: Mutex<Engine>,
    template: FormTemplate,
}


struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": self.0.to_string() })),
        )
            .into_response()
    }
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(e: E) -> Self {
        AppError(e.into())
    }
}

async fn extract(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut pdf_bytes: Option<Vec<u8>> = None;
    while let Some(field) = multipart.next_field().await? {
        if field.name() == Some("file") {
            pdf_bytes = Some(field.bytes().await?.to_vec());
            break;
        }
    }

    let pdf_bytes = pdf_bytes
        .ok_or_else(|| anyhow::anyhow!("no field named 'file' in the request"))?;

    if pdf_bytes.is_empty() {
        return Err(anyhow::anyhow!("uploaded file is empty").into());
    }

    // write to a temp file, hayro needs a path on disk
    let tmp = tempfile::Builder::new()
        .suffix(".pdf")
        .tempfile()
        .context("creating temp file")?;
    std::fs::write(tmp.path(), &pdf_bytes).context("writing temp file")?;
    let path = tmp.path().to_path_buf();

    let entries = tokio::task::spawn_blocking(move || -> anyhow::Result<_> {
        let engine = state
            .engine
            .lock()
            .map_err(|_| anyhow::anyhow!("engine lock poisoned"))?;
        pdf_analyzer::extract_with(&engine, &path, &state.template, false)
    })
    .await
    .context("task panicked")??;

    Ok(Json(serde_json::to_value(entries)?))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "warn,pdf_analyzer_service=info".into()),
        )
        .init();

    let bind = std::env::var("BIND").unwrap_or_else(|_| "0.0.0.0:8080".into());

    tracing::info!("loading template from {TEMPLATE_PATH}");
    let template = FormTemplate::load(Path::new(TEMPLATE_PATH))?;

    tracing::info!("building OCR engine (models download on first run into ~/.oar)");
    let engine = Engine::new(&template.ocr)?;

    let state = Arc::new(AppState {
        engine: Mutex::new(engine),
        template,
    });

    let app = Router::new()
        .route("/extract", post(extract))
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024)) //50mb
        .layer(CorsLayer::permissive())
        .with_state(state);

    tracing::info!("listening on {bind}");
    let listener = tokio::net::TcpListener::bind(&bind).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
