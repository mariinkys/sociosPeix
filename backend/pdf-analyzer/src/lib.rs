//! Extract fixed-layout form data from a PDF into structured JSON.
//!
//! ```no_run
//! use pdf_analyzer::{config::FormTemplate, ocr::Engine, extract_bytes};
//! use std::path::Path;
//!
//! # fn main() -> anyhow::Result<()> {
//! let template = FormTemplate::load(Path::new("template.toml"))?;
//! let engine = Engine::new(&template.ocr)?;
//! let pdf = std::fs::read("scans.pdf")?;
//! let entries = extract_bytes(&engine, pdf, &template, false)?;
//! println!("{}", serde_json::to_string_pretty(&entries)?);
//! # Ok(())
//! # }
//! ```

pub mod config;
pub mod model;
pub mod ocr;
pub mod pdf;

use anyhow::Result;
use std::path::Path;

use crate::config::FormTemplate;
use crate::model::Entry;

/// The registration-form template bundled with the crate, so deployments do
/// not need to ship it as a separate file.
pub const BUNDLED_TEMPLATE: &str = include_str!("../template.toml");

/// Read the templated fields from every page of an in-memory PDF and return
/// one [`Entry`] per page, in document order.
///
/// Reuse one [`ocr::Engine`] across calls: model loading is by far the most
/// expensive part of a run.
pub fn extract_bytes(
    engine: &ocr::Engine,
    pdf: impl AsRef<[u8]> + Send + Sync + 'static,
    template: &FormTemplate,
    flat: bool,
) -> Result<Vec<Entry>> {
    let doc = pdf::PdfDocument::from_bytes(pdf)?;
    extract_document(engine, &doc, template, flat)
}

/// Same as [`extract_bytes`], reading the PDF from disk.
pub fn extract_with(
    engine: &ocr::Engine,
    pdf_path: &Path,
    template: &FormTemplate,
    flat: bool,
) -> Result<Vec<Entry>> {
    let doc = pdf::PdfDocument::open(pdf_path)?;
    extract_document(engine, &doc, template, flat)
}

/// Extract from an already-parsed document.
pub fn extract_document(
    engine: &ocr::Engine,
    doc: &pdf::PdfDocument,
    template: &FormTemplate,
    flat: bool,
) -> Result<Vec<Entry>> {
    doc.for_each_page(template.render_scale(), None, |page| {
        tracing::debug!(page = page.page_number, "extracting");
        let fields = engine.extract_page(&page, template)?;
        Ok(model::build_entry(page.page_number, fields, flat))
    })
}
