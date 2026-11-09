use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Multipart, State},
    routing::post,
};
use serde::Serialize;

use crate::{
    adapters::http::{app_state::AppState, extractors::AuthUser},
    app_error::{AppError, AppResult},
    entities::form_ocr::{FormOcrField, FormOcrPage},
    use_cases::form_ocr::FormOcrUseCases,
};

pub fn router(upload_limit: usize) -> Router<AppState> {
    Router::new()
        .route("/analyze", post(analyze))
        .layer(DefaultBodyLimit::max(upload_limit))
}

#[derive(Serialize)]
struct FormOcrResponse {
    pages: Vec<PageDto>,
}

#[derive(Serialize)]
struct PageDto {
    page: usize,
    /// Field name → value, in template order.
    fields: serde_json::Map<String, serde_json::Value>,
}

#[derive(Serialize)]
struct FieldDto {
    text: String,
    lines: Vec<String>,
    confidence: Option<f32>,
    empty: bool,
}

impl From<FormOcrField> for FieldDto {
    fn from(f: FormOcrField) -> Self {
        Self {
            text: f.text,
            lines: f.lines,
            confidence: f.confidence,
            empty: f.empty,
        }
    }
}

impl From<FormOcrPage> for PageDto {
    fn from(p: FormOcrPage) -> Self {
        Self {
            page: p.page,
            fields: p
                .fields
                .into_iter()
                .map(|(name, f)| {
                    let v = serde_json::to_value(FieldDto::from(f)).unwrap_or_default();
                    (name, v)
                })
                .collect(),
        }
    }
}

/// Reads a scanned registration form (multipart field `file`).
async fn analyze(
    _user: AuthUser,
    State(ocr): State<Arc<FormOcrUseCases>>,
    mut multipart: Multipart,
) -> AppResult<Json<FormOcrResponse>> {
    tracing::info!("form OCR requested");
    let mut pdf = None;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Validation(e.body_text()))?
    {
        if field.name() == Some("file") {
            pdf = Some(
                field
                    .bytes()
                    .await
                    .map_err(|e| AppError::Validation(e.body_text()))?,
            );
            break;
        }
    }
    let pdf =
        pdf.ok_or_else(|| AppError::Validation("No field named 'file' in the request".into()))?;
    let pages = ocr.analyze(pdf).await?;
    Ok(Json(FormOcrResponse {
        pages: pages.into_iter().map(Into::into).collect(),
    }))
}
