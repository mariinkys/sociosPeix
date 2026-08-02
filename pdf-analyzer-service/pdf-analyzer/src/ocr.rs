use anyhow::{Context, Result};
use image::{Rgb, RgbImage, imageops};
use oar_ocr::domain::tasks::{TextDetectionConfig, TextRecognitionConfig};
use oar_ocr::oarocr::{OAROCR, OAROCRBuilder};
use oar_ocr::processors::BoundingBox;

use crate::config::{FieldSpec, FormTemplate, OcrSettings};
use crate::model::FieldValue;
use crate::pdf::RenderedPage;

/// White margin added around each crop, in pixels.
const CROP_MARGIN: u32 = 12;

pub struct Engine {
    ocr: OAROCR,
}

impl Engine {
    pub fn new(cfg: &OcrSettings) -> Result<Self> {
        let mut builder = OAROCRBuilder::new(
            cfg.det_model.clone(),
            cfg.rec_model.clone(),
            cfg.char_dict.clone(),
        )
        .text_detection_config(TextDetectionConfig {
            score_threshold: cfg.det_score_threshold,
            box_threshold: cfg.det_box_threshold,
            unclip_ratio: cfg.det_unclip_ratio,
            ..Default::default()
        })
        .text_recognition_config(TextRecognitionConfig {
            score_threshold: cfg.rec_score_threshold,
        })
        .image_batch_size(cfg.image_batch_size)
        .region_batch_size(cfg.region_batch_size);

        if let Some(m) = &cfg.doc_orientation_model {
            builder = builder.with_document_image_orientation_classification(m.clone());
        }
        if let Some(m) = &cfg.textline_orientation_model {
            builder = builder.with_text_line_orientation_classification(m.clone());
        }

        let ocr = builder
            .build()
            .context("building the OCR pipeline (check model paths / $OAR_HOME cache)")?;

        Ok(Self { ocr })
    }

    /// Extract every templated field from one rendered page.
    ///
    /// All crops for the page go through a single `predict` call, so detection
    /// and recognition are batched rather than paying per-field session
    /// overhead. Results come back in input order, which is how we map them
    /// back onto field names.
    pub fn extract_page(
        &self,
        page: &RenderedPage,
        tpl: &FormTemplate,
    ) -> Result<Vec<(String, FieldValue)>> {
        let crops: Vec<RgbImage> = tpl
            .fields
            .iter()
            .map(|f| crop_field(&page.image, f))
            .collect();

        if crops.is_empty() {
            return Ok(Vec::new());
        }

        let results = self
            .ocr
            .predict(crops)
            .with_context(|| format!("OCR failed on page {}", page.page_number))?;

        let mut out = Vec::with_capacity(tpl.fields.len());
        for (field, result) in tpl.fields.iter().zip(results) {
            let mut regions: Vec<_> = result
                .text_regions
                .into_iter()
                .filter(|r| r.text.as_ref().is_some_and(|t| !t.trim().is_empty()))
                .collect();

            regions.sort_by(|a, b| {
                let (ax, ay) = top_left(&a.bounding_box);
                let (bx, by) = top_left(&b.bounding_box);
                // Treat lines within roughlyy half a line-height as the same row.
                let row_tol = 12.0;
                if (ay - by).abs() > row_tol {
                    ay.partial_cmp(&by).unwrap_or(std::cmp::Ordering::Equal)
                } else {
                    ax.partial_cmp(&bx).unwrap_or(std::cmp::Ordering::Equal)
                }
            });

            let lines: Vec<String> = regions
                .iter()
                .filter_map(|r| r.text.as_ref().map(|t| t.trim().to_string()))
                .filter(|t| !t.is_empty())
                .collect();

            let confidences: Vec<f32> = regions.iter().filter_map(|r| r.confidence).collect();
            let confidence = if confidences.is_empty() {
                None
            } else {
                Some(confidences.iter().sum::<f32>() / confidences.len() as f32)
            };

            let joined = lines.join(field.separator());
            let text = field.transform.apply(&joined).trim().to_string();

            out.push((
                field.name.clone(),
                FieldValue {
                    empty: text.is_empty(),
                    text,
                    lines,
                    confidence,
                },
            ));
        }

        Ok(out)
    }

    /// Wholepage OCR, used by the `probe` command to help you find coordinates.
    pub fn probe_page(&self, page: &RenderedPage) -> Result<Vec<ProbeHit>> {
        let (w, h) = (page.image.width() as f32, page.image.height() as f32);
        let results = self
            .ocr
            .predict(vec![page.image.clone()])
            .with_context(|| format!("probe OCR failed on page {}", page.page_number))?;

        let mut hits = Vec::new();
        for result in results {
            for region in result.text_regions {
                let Some(text) = region.text.as_ref() else {
                    continue;
                };
                if text.trim().is_empty() {
                    continue;
                }
                let (x0, y0, x1, y1) = extents(&region.bounding_box);
                hits.push(ProbeHit {
                    text: text.trim().to_string(),
                    confidence: region.confidence,
                    rect: crate::config::Rect {
                        x: round4(x0 / w),
                        y: round4(y0 / h),
                        w: round4((x1 - x0) / w),
                        h: round4((y1 - y0) / h),
                    },
                });
            }
        }
        hits.sort_by(|a, b| {
            a.rect
                .y
                .partial_cmp(&b.rect.y)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Ok(hits)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct ProbeHit {
    pub text: String,
    pub confidence: Option<f32>,
    /// Normalized
    pub rect: crate::config::Rect,
}

/// Crop a field rect out of the page and surround it with a white margin.
pub fn crop_field(page: &RgbImage, field: &FieldSpec) -> RgbImage {
    let rect = field.rect.padded(field.pad);
    let (x, y, w, h) = rect.to_pixels(page.width(), page.height());
    let crop = imageops::crop_imm(page, x, y, w, h).to_image();
    add_margin(&crop, CROP_MARGIN)
}

fn add_margin(img: &RgbImage, margin: u32) -> RgbImage {
    let mut canvas = RgbImage::from_pixel(
        img.width() + margin * 2,
        img.height() + margin * 2,
        Rgb([255, 255, 255]),
    );
    imageops::replace(&mut canvas, img, margin as i64, margin as i64);
    canvas
}

fn top_left(bbox: &BoundingBox) -> (f32, f32) {
    let (x0, y0, _, _) = extents(bbox);
    (x0, y0)
}

/// Axis-aligned extents of a (possibly rotated) detection polygon.
fn extents(bbox: &BoundingBox) -> (f32, f32, f32, f32) {
    let mut x0 = f32::MAX;
    let mut y0 = f32::MAX;
    let mut x1 = f32::MIN;
    let mut y1 = f32::MIN;
    for p in &bbox.points {
        x0 = x0.min(p.x);
        y0 = y0.min(p.y);
        x1 = x1.max(p.x);
        y1 = y1.max(p.y);
    }
    if bbox.points.is_empty() {
        return (0.0, 0.0, 0.0, 0.0);
    }
    (x0, y0, x1, y1)
}

fn round4(v: f32) -> f32 {
    (v * 10_000.0).round() / 10_000.0
}
