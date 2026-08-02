//! Form template: describes *where* each field lives on the page.
//!
//! Coordinates are **normalized** (0.0..=1.0, origin at the top-left of the
//! page). That is deliberate: it makes the template independent of the DPI you
//! rasterise at, and independent of whether the PDF is A4 or Letter. Change
//! `render_dpi` and the template keeps working.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormTemplate {
    /// Rasterisation DPI. 300 is a good default for handwriting; 400–600 can
    /// help on small boxes at the cost of speed.
    #[serde(default = "default_dpi")]
    pub render_dpi: f32,

    #[serde(default)]
    pub ocr: OcrSettings,

    pub fields: Vec<FieldSpec>,
}

fn default_dpi() -> f32 {
    300.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct OcrSettings {
    /// Text detection model (path, or bare registry name with `auto-download`).
    pub det_model: String,
    /// Text recognition model. The Latin PP-OCRv5 model is the right choice for
    /// Spanish (accents, Ñ) — the default `pp-ocrv5_mobile_rec` is CN/EN.
    pub rec_model: String,
    /// Character dictionary matching `rec_model`.
    pub char_dict: String,
    /// Optional text-line orientation classifier — worth enabling if scans come
    /// in slightly rotated.
    pub textline_orientation_model: Option<String>,
    /// Optional full-page orientation classifier (90/180/270 correction).
    pub doc_orientation_model: Option<String>,

    /// Detection box threshold. Lower = more permissive. Handwriting is faint
    /// and irregular, so we default below the library's 0.6.
    pub det_box_threshold: f32,
    /// Detection score threshold.
    pub det_score_threshold: f32,
    /// How much detected boxes are expanded. Handwriting has long ascenders and
    /// descenders that a tight box clips, so we go above the 1.5 default.
    pub det_unclip_ratio: f32,
    /// Drop recognitions below this confidence. 0.0 = keep everything.
    pub rec_score_threshold: f32,

    /// How many field crops are pushed through detection at once.
    pub image_batch_size: usize,
    /// How many detected text regions are recognised at once.
    pub region_batch_size: usize,
}

impl Default for OcrSettings {
    fn default() -> Self {
        Self {
            det_model: "pp-ocrv5_mobile_det.onnx".into(),
            rec_model: "latin_pp-ocrv5_mobile_rec.onnx".into(),
            char_dict: "ppocrv5_latin_dict.txt".into(),
            textline_orientation_model: None,
            doc_orientation_model: None,
            det_box_threshold: 0.4,
            det_score_threshold: 0.3,
            det_unclip_ratio: 1.8,
            rec_score_threshold: 0.0,
            image_batch_size: 8,
            region_batch_size: 16,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSpec {
    /// Key this field gets in the JSON output.
    pub name: String,

    /// Normalized rect of the *box that holds the handwriting* — not the
    /// printed label above it.
    pub rect: Rect,

    /// Expect more than one line of text inside the box.
    #[serde(default)]
    pub multiline: bool,

    /// Separator used when several text lines are found. Defaults to a space,
    /// or a newline when `multiline` is set.
    #[serde(default)]
    pub join: Option<String>,

    /// Grow the rect outward by this fraction of its own size before cropping.
    /// A little slack (0.01–0.03) absorbs scanner drift.
    #[serde(default)]
    pub pad: f32,

    /// Post-processing applied to the recognised string.
    #[serde(default)]
    pub transform: Transform,
}

impl FieldSpec {
    pub fn separator(&self) -> &str {
        match self.join.as_deref() {
            Some(s) => s,
            None if self.multiline => "\n",
            None => " ",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    /// Apply padding and clamp back into the unit square.
    pub fn padded(&self, pad: f32) -> Rect {
        if pad <= 0.0 {
            return *self;
        }
        let dx = self.w * pad;
        let dy = self.h * pad;
        let x = (self.x - dx).max(0.0);
        let y = (self.y - dy).max(0.0);
        Rect {
            x,
            y,
            w: (self.w + 2.0 * dx).min(1.0 - x),
            h: (self.h + 2.0 * dy).min(1.0 - y),
        }
    }

    /// Convert to integer pixel coordinates for an image of the given size.
    pub fn to_pixels(&self, img_w: u32, img_h: u32) -> (u32, u32, u32, u32) {
        let x = (self.x * img_w as f32).round().clamp(0.0, img_w as f32) as u32;
        let y = (self.y * img_h as f32).round().clamp(0.0, img_h as f32) as u32;
        let w = (self.w * img_w as f32).round() as u32;
        let h = (self.h * img_h as f32).round() as u32;
        // Guarantee at least one pixel and stay inside the image.
        let w = w.clamp(1, img_w.saturating_sub(x).max(1));
        let h = h.clamp(1, img_h.saturating_sub(y).max(1));
        (x, y, w, h)
    }

    fn validate(&self, field: &str) -> Result<()> {
        let in_range = |v: f32| (0.0..=1.0).contains(&v);
        if !in_range(self.x) || !in_range(self.y) {
            bail!("field `{field}`: rect origin must be within 0.0..=1.0");
        }
        if self.w <= 0.0 || self.h <= 0.0 {
            bail!("field `{field}`: rect width and height must be > 0");
        }
        if self.x + self.w > 1.0001 || self.y + self.h > 1.0001 {
            bail!("field `{field}`: rect extends past the page edge");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Transform {
    #[default]
    None,
    /// Uppercase
    Upper,
    /// Lowercase
    Lower,
    /// Keep digits only (ID numbers, phone numbers).
    Digits,
    /// Keep digits and the usual date separators.
    Date,
    /// Strip every space (useful for DNI/NIE, which OCR often splits).
    NoSpaces,
}

impl Transform {
    pub fn apply(&self, s: &str) -> String {
        match self {
            Transform::None => s.to_string(),
            Transform::Upper => s.to_uppercase(),
            Transform::Lower => s.to_lowercase(),
            Transform::Digits => s.chars().filter(|c| c.is_ascii_digit()).collect(),
            Transform::Date => s
                .chars()
                .filter(|c| c.is_ascii_digit() || matches!(c, '/' | '-' | '.'))
                .collect(),
            Transform::NoSpaces => s.chars().filter(|c| !c.is_whitespace()).collect(),
        }
    }
}

impl FormTemplate {
    pub fn load(path: &Path) -> Result<Self> {
        let raw = std::fs::read_to_string(path)
            .with_context(|| format!("reading template {}", path.display()))?;
        let tpl: FormTemplate = toml::from_str(&raw)
            .with_context(|| format!("parsing template {}", path.display()))?;
        tpl.validate()?;
        Ok(tpl)
    }

    fn validate(&self) -> Result<()> {
        if self.fields.is_empty() {
            bail!("template defines no fields");
        }
        if self.render_dpi < 36.0 || self.render_dpi > 1200.0 {
            bail!("render_dpi {} is out of the sane 36..1200 range", self.render_dpi);
        }
        let mut seen = std::collections::HashSet::new();
        for f in &self.fields {
            if !seen.insert(&f.name) {
                bail!("duplicate field name `{}`", f.name);
            }
            f.rect.validate(&f.name)?;
        }
        Ok(())
    }

    /// PDF user-space units are 1/72 inch, so this is the hayro render scale.
    pub fn render_scale(&self) -> f32 {
        self.render_dpi / 72.0
    }
}
