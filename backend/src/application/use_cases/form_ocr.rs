use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use tracing::{info, instrument};

use crate::{
    app_error::{AppError, AppResult},
    entities::form_ocr::FormOcrPage,
};

/// Reads the handwritten registration form out of a scanned PDF.
#[async_trait]
pub trait FormAnalyzer: Send + Sync {
    async fn analyze(&self, pdf: Bytes) -> AppResult<Vec<FormOcrPage>>;
}

pub struct FormOcrUseCases {
    analyzer: Arc<dyn FormAnalyzer>,
}

impl FormOcrUseCases {
    pub fn new(analyzer: Arc<dyn FormAnalyzer>) -> Self {
        Self { analyzer }
    }

    #[instrument(skip_all, fields(bytes = pdf.len()))]
    pub async fn analyze(&self, pdf: Bytes) -> AppResult<Vec<FormOcrPage>> {
        if pdf.is_empty() {
            return Err(AppError::Validation("Uploaded file is empty".into()));
        }
        let pages = self.analyzer.analyze(pdf).await?;
        info!(pages = pages.len(), "registration form analysed");
        Ok(pages)
    }
}
