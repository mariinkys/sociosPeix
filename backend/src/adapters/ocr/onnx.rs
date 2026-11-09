use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use bytes::Bytes;
use pdf_analyzer::{config::FormTemplate, ocr::Engine, pdf::PdfDocument};
use tracing::{info, warn};

use crate::{
    app_error::{AppError, AppResult},
    entities::form_ocr::{FormOcrField, FormOcrPage},
    use_cases::form_ocr::FormAnalyzer,
};

/// Runs the ONNX OCR pipeline on the blocking thread pool.
///
/// the models are loaded on the first request (not at startup) and dropped again after `idle_unload` without use,
/// and a single engine behind a mutex means at most one form is processed at a time,
/// which bounds peak memory and keeps OCR from starving the API of CPU.
pub struct OnnxFormAnalyzer {
    template: Arc<FormTemplate>,
    slot: Arc<Mutex<Slot>>,
}

struct Slot {
    engine: Option<Engine>,
    last_used: Instant,
}

impl OnnxFormAnalyzer {
    pub fn new(template: FormTemplate, idle_unload: Option<Duration>) -> Self {
        let slot = Arc::new(Mutex::new(Slot {
            engine: None,
            last_used: Instant::now(),
        }));
        if let Some(idle) = idle_unload {
            spawn_idle_unloader(Arc::downgrade(&slot), idle);
        }
        Self {
            template: Arc::new(template),
            slot,
        }
    }

    /// Loads the models now instead of on the first request.
    pub fn preload(&self) {
        let (slot, template) = (self.slot.clone(), self.template.clone());
        tokio::task::spawn_blocking(move || {
            let mut slot = slot.lock().unwrap_or_else(|p| p.into_inner());
            if let Err(e) = ensure_engine(&mut slot, &template) {
                warn!(error = %e, "OCR preload failed");
            }
        });
    }
}

fn ensure_engine<'a>(slot: &'a mut Slot, template: &FormTemplate) -> anyhow::Result<&'a Engine> {
    if slot.engine.is_none() {
        let started = Instant::now();
        info!("loading OCR models (downloaded into ~/.oar on first run)");
        slot.engine = Some(Engine::new(&template.ocr)?);
        info!(
            elapsed_ms = started.elapsed().as_millis() as u64,
            "OCR models loaded"
        );
    }
    Ok(slot.engine.as_ref().expect("just initialised"))
}

fn spawn_idle_unloader(slot: std::sync::Weak<Mutex<Slot>>, idle: Duration) {
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(Duration::from_secs(60).min(idle));
        loop {
            tick.tick().await;
            let Some(slot) = slot.upgrade() else { return };
            // Never wait for a running OCR job; just check again later.
            if let Ok(mut s) = slot.try_lock()
                && s.engine.is_some()
                && s.last_used.elapsed() >= idle
            {
                s.engine = None;
                info!("OCR models unloaded after being idle");
            }
        }
    });
}

#[async_trait]
impl FormAnalyzer for OnnxFormAnalyzer {
    async fn analyze(&self, pdf: Bytes) -> AppResult<Vec<FormOcrPage>> {
        let (slot, template) = (self.slot.clone(), self.template.clone());
        tokio::task::spawn_blocking(move || -> AppResult<Vec<FormOcrPage>> {
            let doc = PdfDocument::from_bytes(pdf)
                .map_err(|e| AppError::Validation(format!("Invalid PDF: {e}")))?;

            let mut slot = slot.lock().unwrap_or_else(|p| p.into_inner());
            let engine = ensure_engine(&mut slot, &template)
                .map_err(|e| AppError::Internal(format!("OCR engine failed to load: {e:#}")))?;

            let pages = doc
                .for_each_page(template.render_scale(), None, |page| {
                    let fields = engine.extract_page(&page, &template)?;
                    Ok(FormOcrPage {
                        page: page.page_number,
                        fields: fields
                            .into_iter()
                            .map(|(name, v)| {
                                (
                                    name,
                                    FormOcrField {
                                        text: v.text,
                                        lines: v.lines,
                                        confidence: v.confidence,
                                        empty: v.empty,
                                    },
                                )
                            })
                            .collect(),
                    })
                })
                .map_err(|e| AppError::Internal(format!("OCR failed: {e:#}")));
            slot.last_used = Instant::now();
            pages
        })
        .await
        .map_err(AppError::internal)?
    }
}
