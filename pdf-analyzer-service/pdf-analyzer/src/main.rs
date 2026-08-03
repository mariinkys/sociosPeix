//! CLI front-end.
//!
//! Three subcommands, and you will use them in this order the first time:
//!
//!   probe      — OCR the whole page and print every text run with its
//!                normalized rect, so you can read off where things are.
//!   calibrate  — draw your template's rects onto the page and dump the crops,
//!                so you can *see* whether they land on the right boxes.
//!   extract    — the real thing: emit JSON.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use image::Rgb;
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect as IpRect;
use std::path::{Path, PathBuf};

use pdf_analyzer::config::FormTemplate;
use pdf_analyzer::{extract_with, ocr, pdf};

#[derive(Parser)]
#[command(
    name = "pdf-analyzer",
    about = "Read handwritten fields from fixed-layout PDF forms into JSON",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Log level: error, warn, info, debug, trace.
    #[arg(long, global = true, default_value = "warn")]
    log: String,
}

#[derive(Subcommand)]
enum Command {
    /// Extract templated fields and print a JSON array — one entry per page.
    Extract {
        /// Input PDF.
        pdf: PathBuf,
        /// Form template describing the field rects.
        #[arg(short, long, default_value = "template.toml")]
        template: PathBuf,
        /// Write JSON here instead of stdout.
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Emit each field as a plain string rather than an object with
        /// confidence and per-line detail.
        #[arg(long)]
        flat: bool,
        /// Pretty-print the JSON.
        #[arg(long)]
        pretty: bool,
    },

    /// Overlay the template rects on each page and dump per-field crops.
    Calibrate {
        pdf: PathBuf,
        #[arg(short, long, default_value = "template.toml")]
        template: PathBuf,
        /// Directory for the overlay images and crops.
        #[arg(short, long, default_value = "calibration")]
        out_dir: PathBuf,
        /// Only render this page (1-based).
        #[arg(long)]
        page: Option<usize>,
    },

    /// OCR whole pages and report every text run with its normalized rect.
    Probe {
        pdf: PathBuf,
        #[arg(short, long, default_value = "template.toml")]
        template: PathBuf,
        /// Only probe this page (1-based).
        #[arg(long, default_value_t = 1)]
        page: usize,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| cli.log.clone().into()),
        )
        .with_writer(std::io::stderr) // keep stdout clean for JSON
        .init();

    match cli.command {
        Command::Extract {
            pdf,
            template,
            output,
            flat,
            pretty,
        } => cmd_extract(&pdf, &template, output.as_deref(), flat, pretty),

        Command::Calibrate {
            pdf,
            template,
            out_dir,
            page,
        } => cmd_calibrate(&pdf, &template, &out_dir, page),

        Command::Probe {
            pdf,
            template,
            page,
        } => cmd_probe(&pdf, &template, page),
    }
}

fn cmd_extract(
    pdf_path: &Path,
    template_path: &Path,
    output: Option<&Path>,
    flat: bool,
    pretty: bool,
) -> Result<()> {
    let template = FormTemplate::load(template_path)?;
    let engine = ocr::Engine::new(&template.ocr)?;
    let entries = extract_with(&engine, pdf_path, &template, flat)?;

    let json = if pretty {
        serde_json::to_string_pretty(&entries)?
    } else {
        serde_json::to_string(&entries)?
    };

    match output {
        Some(path) => {
            std::fs::write(path, json)
                .with_context(|| format!("writing {}", path.display()))?;
            eprintln!("wrote {} entries to {}", entries.len(), path.display());
        }
        None => println!("{json}"),
    }
    Ok(())
}

fn cmd_calibrate(
    pdf_path: &Path,
    template_path: &Path,
    out_dir: &Path,
    only_page: Option<usize>,
) -> Result<()> {
    let template = FormTemplate::load(template_path)?;
    std::fs::create_dir_all(out_dir)
        .with_context(|| format!("creating {}", out_dir.display()))?;

    let doc = pdf::PdfDocument::open(pdf_path)?;
    let pages = doc.render_all(template.render_scale())?;

    for page in &pages {
        if only_page.is_some_and(|p| p != page.page_number) {
            continue;
        }

        let mut overlay = page.image.clone();
        let (pw, ph) = (overlay.width(), overlay.height());

        for field in &template.fields {
            let rect = field.rect.padded(field.pad);
            let (x, y, w, h) = rect.to_pixels(pw, ph);

            // 3px border so it survives downscaling when you view it.
            for inset in 0..3 {
                let ix = x.saturating_add(inset);
                let iy = y.saturating_add(inset);
                let iw = w.saturating_sub(inset * 2).max(1);
                let ih = h.saturating_sub(inset * 2).max(1);
                draw_hollow_rect_mut(
                    &mut overlay,
                    IpRect::at(ix as i32, iy as i32).of_size(iw, ih),
                    Rgb([220, 30, 30]),
                );
            }

            let crop = ocr::crop_field(&page.image, field);
            let crop_path = out_dir.join(format!(
                "page{:02}_{}.png",
                page.page_number,
                sanitize(&field.name)
            ));
            crop.save(&crop_path)
                .with_context(|| format!("saving {}", crop_path.display()))?;
        }

        let overlay_path = out_dir.join(format!("page{:02}_overlay.png", page.page_number));
        overlay
            .save(&overlay_path)
            .with_context(|| format!("saving {}", overlay_path.display()))?;

        eprintln!(
            "page {} rendered at {}x{} px -> {}",
            page.page_number,
            pw,
            ph,
            overlay_path.display()
        );
    }

    eprintln!(
        "\nOpen the overlays. Every red rect must sit on the answer box, not the printed label.\nAdjust `rect` values in {} and re-run.",
        template_path.display()
    );
    Ok(())
}

fn cmd_probe(pdf_path: &Path, template_path: &Path, page_no: usize) -> Result<()> {
    let template = FormTemplate::load(template_path)?;
    let engine = ocr::Engine::new(&template.ocr)?;

    let doc = pdf::PdfDocument::open(pdf_path)?;
    let pages = doc.render_all(template.render_scale())?;
    let page = pages
        .iter()
        .find(|p| p.page_number == page_no)
        .ok_or_else(|| anyhow::anyhow!("page {page_no} not found ({} pages)", doc.page_count()))?;

    let hits = engine.probe_page(page)?;

    eprintln!("{} text runs on page {page_no}:\n", hits.len());
    for hit in &hits {
        eprintln!(
            "  {:<40} x={:.4} y={:.4} w={:.4} h={:.4}  conf={:.2}",
            truncate(&hit.text, 38),
            hit.rect.x,
            hit.rect.y,
            hit.rect.w,
            hit.rect.h,
            hit.confidence.unwrap_or(0.0)
        );
    }
    println!("{}", serde_json::to_string_pretty(&hits)?);
    Ok(())
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        s.chars().take(max - 1).collect::<String>() + "…"
    }
}
