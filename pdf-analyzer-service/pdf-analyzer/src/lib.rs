//! Extract fixed-layout form data from a PDF into structured JSON.
//!
//! ```no_run
//! use pdf_form_ocr::{config::FormTemplate, extract};
//! use std::path::Path;
//!
//! # fn main() -> anyhow::Result<()> {
//! let template = FormTemplate::load(Path::new("template.toml"))?;
//! let entries = extract(Path::new("scans.pdf"), &template, false)?;
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

/// Render every page of `pdf_path`, read the templated fields, and return one
/// [`Entry`] per page in document order.
///
/// The OCR pipeline is built once and reused for all pages — model loading is
/// by far the most expensive part of a run, so this matters for multi-page
/// documents.
pub fn extract(pdf_path: &Path, template: &FormTemplate, flat: bool) -> Result<Vec<Entry>> {
    let engine = ocr::Engine::new(&template.ocr)?;
    extract_with(&engine, pdf_path, template, flat)
}

/// Same as [`extract`], but reuses an engine you already built. Use this when
/// processing many PDFs in one process.
pub fn extract_with(
    engine: &ocr::Engine,
    pdf_path: &Path,
    template: &FormTemplate,
    flat: bool,
) -> Result<Vec<Entry>> {
    let doc = pdf::PdfDocument::open(pdf_path)?;
    let pages = doc.render_all(template.render_scale())?;

    let mut entries = Vec::with_capacity(pages.len());
    for page in &pages {
        tracing::info!(page = page.page_number, "extracting");
        let fields = engine.extract_page(page, template)?;
        entries.push(model::build_entry(page.page_number, fields, flat));
    }
    Ok(entries)
}
