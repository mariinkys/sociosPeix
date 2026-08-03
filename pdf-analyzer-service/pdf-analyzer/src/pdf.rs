use anyhow::{Context, Result, bail};
use image::RgbImage;
use std::path::Path;
use std::sync::Arc;

use hayro::{RenderCache, hayro_syntax::Pdf};

pub struct RenderedPage {
    pub page_number: usize,
    pub image: RgbImage,
}

pub struct PdfDocument {
    pdf: Pdf,
    page_count: usize,
}

impl PdfDocument {
    pub fn open(path: &Path) -> Result<Self> {
        let data = std::fs::read(path)
            .with_context(|| format!("reading PDF {}", path.display()))?;

        if !data.starts_with(b"%PDF") {
            bail!("{} does not look like a PDF", path.display());
        }

        let pdf = Pdf::new(Arc::new(data))
            .map_err(|e| anyhow::anyhow!("failed to parse {}: {e:?}", path.display()))?;
        let page_count = pdf.pages().len();

        if page_count == 0 {
            bail!("{} contains no pages", path.display());
        }

        Ok(Self { pdf, page_count })
    }

    pub fn page_count(&self) -> usize {
        self.page_count
    }

    /// Render every page at the given scale (1.0 == 72 DPI).
    ///
    /// A single `RenderCache` is shared across pages, which matters for forms:
    /// the same background template, fonts and logos repeat on every page and
    /// get reused instead of re-decoded.
    pub fn render_all(&self, scale: f32) -> Result<Vec<RenderedPage>> {
        let cache = RenderCache::new();
        let mut out = Vec::with_capacity(self.page_count);
        for n in 1..=self.page_count {
            out.push(self.render_one(n, scale, &cache)?);
        }
        Ok(out)
    }

    fn render_one<'a>(
        &'a self,
        page_number: usize,
        scale: f32,
        cache: &RenderCache<'a>,
    ) -> Result<RenderedPage> {
        use hayro::RenderSettings;

        let page = self
            .pdf
            .pages()
            .get(page_number - 1)
            .ok_or_else(|| anyhow::anyhow!("page {page_number} not found"))?;

        let media_box = page.media_box();
        let w = (media_box.x1 - media_box.x0) as f32;
        let h = (media_box.y1 - media_box.y0) as f32;
        if w <= 0.0 || h <= 0.0 {
            bail!("page {page_number} has an invalid media box ({w}x{h})");
        }

        let settings = RenderSettings {
            x_scale: scale,
            y_scale: scale,
            // hayro defaults the background to transparent; forcing white means
            // the RGBA→RGB drop of the alpha channel yields white paper rather
            // than black, which would wreck the OCR.
            bg_color: hayro::vello_cpu::color::palette::css::WHITE,
            ..Default::default()
        };

        let interpreter = hayro::hayro_interpret::InterpreterSettings::default();
        let pixmap = hayro::render(page, cache, &interpreter, &settings);

        let rgba = pixmap.data_as_u8_slice();
        let mut rgb = Vec::with_capacity(rgba.len() / 4 * 3);
        for px in rgba.chunks_exact(4) {
            rgb.push(px[0]);
            rgb.push(px[1]);
            rgb.push(px[2]);
        }

        let image = RgbImage::from_raw(
            u32::from(pixmap.width()),
            u32::from(pixmap.height()),
            rgb,
        )
        .ok_or_else(|| anyhow::anyhow!("page {page_number}: pixmap → image conversion failed"))?;

        Ok(RenderedPage {
            page_number,
            image,
        })
    }
}
