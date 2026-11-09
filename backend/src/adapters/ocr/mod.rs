//! Registration-form OCR, running in-process (it used to be a separate service).

#[cfg(feature = "ocr")]
mod onnx;
#[cfg(feature = "ocr")]
pub use onnx::OnnxFormAnalyzer;

use async_trait::async_trait;
use bytes::Bytes;

use crate::{
    app_error::{AppError, AppResult},
    entities::form_ocr::FormOcrPage,
    use_cases::form_ocr::FormAnalyzer,
};

/// Used when OCR is turned off (`OCR_ENABLED=false`) or compiled out.
pub struct DisabledFormAnalyzer;

#[async_trait]
impl FormAnalyzer for DisabledFormAnalyzer {
    async fn analyze(&self, _pdf: Bytes) -> AppResult<Vec<FormOcrPage>> {
        Err(AppError::Unavailable(
            "Form OCR is not enabled on this server".into(),
        ))
    }
}
