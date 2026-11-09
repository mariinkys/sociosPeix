//! PDF loading and rasterisation (via `hayro`).

use anyhow::{Context, Result, bail};
use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::Pdf;
use hayro::{PixmapSettings, RenderCache, RenderSettings};
use image::RgbImage;
use std::path::Path;
use std::sync::Arc;

pub struct RenderedPage {
    /// 1-based page number.
    pub page_number: usize,
    pub image: RgbImage,
}

pub struct PdfDocument {
    pdf: Pdf,
    page_count: usize,
}

impl PdfDocument {
    /// Parse a PDF held in memory. No temporary files and no copy: any
    /// shareable byte buffer (`Vec<u8>`, `bytes::Bytes`, ...) is used in place.
    pub fn from_bytes(data: impl AsRef<[u8]> + Send + Sync + 'static) -> Result<Self> {
        if !data.as_ref().starts_with(b"%PDF") {
            bail!("the uploaded file does not look like a PDF");
        }

        let pdf =
            Pdf::new(Arc::new(data)).map_err(|e| anyhow::anyhow!("failed to parse PDF: {e:?}"))?;
        let page_count = pdf.pages().len();
        if page_count == 0 {
            bail!("the PDF contains no pages");
        }

        Ok(Self { pdf, page_count })
    }

    pub fn open(path: &Path) -> Result<Self> {
        let data =
            std::fs::read(path).with_context(|| format!("reading PDF {}", path.display()))?;
        Self::from_bytes(data).with_context(|| format!("loading {}", path.display()))
    }

    pub fn page_count(&self) -> usize {
        self.page_count
    }

    /// Calls `f` with each rendered page, in order, at the given scale
    /// (1.0 == 72 DPI).
    ///
    /// Pages are rendered lazily and dropped as soon as `f` returns, so only
    /// one full-resolution bitmap (~26 MB for A4 at 300 DPI) is alive at any
    /// time, regardless of how many pages the document has. A single
    /// `RenderCache` is shared across pages so fonts and images that repeat on
    /// every page of a form are decoded only once.
    pub fn for_each_page<T>(
        &self,
        scale: f32,
        only: Option<usize>,
        mut f: impl FnMut(RenderedPage) -> Result<T>,
    ) -> Result<Vec<T>> {
        let cache = RenderCache::new();
        let pages: Vec<usize> = match only {
            Some(n) => vec![n],
            None => (1..=self.page_count).collect(),
        };

        let mut out = Vec::with_capacity(pages.len());
        for n in pages {
            let page = self.render_one(n, scale, &cache)?;
            out.push(f(page)?);
        }
        Ok(out)
    }

    fn render_one<'a>(
        &'a self,
        page_number: usize,
        scale: f32,
        cache: &RenderCache<'a>,
    ) -> Result<RenderedPage> {
        let page = self
            .pdf
            .pages()
            .get(page_number.wrapping_sub(1))
            .ok_or_else(|| anyhow::anyhow!("page {page_number} not found"))?;

        let (w, h) = page.render_dimensions();
        if w <= 0.0 || h <= 0.0 {
            bail!("page {page_number} has an invalid size ({w}x{h})");
        }

        let pixmap_settings = PixmapSettings {
            x_scale: scale,
            y_scale: scale,
            // hayro defaults the background to transparent; forcing white means
            // dropping the alpha channel yields white paper rather than black,
            // which would wreck the OCR.
            bg_color: hayro::vello_cpu::color::palette::css::WHITE,
        };

        let pixmap = hayro::render(
            page,
            cache,
            &InterpreterSettings::default(),
            &RenderSettings::default(),
            &pixmap_settings,
        );

        let rgba = pixmap.data_as_u8_slice();
        let mut rgb = Vec::with_capacity(rgba.len() / 4 * 3);
        for px in rgba.as_chunks::<4>().0 {
            rgb.extend_from_slice(&px[..3]);
        }
        let (pw, ph) = (u32::from(pixmap.width()), u32::from(pixmap.height()));
        drop(pixmap);

        let image = RgbImage::from_raw(pw, ph, rgb).ok_or_else(|| {
            anyhow::anyhow!("page {page_number}: pixmap → image conversion failed")
        })?;

        Ok(RenderedPage { page_number, image })
    }
}
