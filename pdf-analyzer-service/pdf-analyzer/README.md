# pdf-form-ocr

Reads fixed-layout PDF forms, the kind with a printed label like `NOMBRE` and a
box of handwriting underneath and returns one JSON entry per page.

Built on [`oar-ocr`](https://github.com/GreatV/oar-ocr) for OCR and
[`hayro`](https://crates.io/crates/hayro) for PDF rasterisation.
ONNX Runtime binaries are downloaded at build time by `oar-ocr`'s default `download-binaries` feature.

## Requirements

- Network access on first run, to fetch the OCR models into `~/.oar`

## How it works

1. `hayro` renders each PDF page to an `RgbImage` at `render_dpi` (default 300).
2. For each field in `template.toml`, the corresponding rect is cropped out.
3. All crops for a page go through **one** `OAROCR::predict` call — batched
   detection and recognition rather than per-field session overhead.
4. Recognised lines are sorted into reading order, joined, and post-processed.

## Setting up a template

You cannot guess the coordinates, so there are two commands that find them for
you.

```bash
# 1. See what the OCR finds on page 1, with normalized coordinates.
#    The printed labels ("NOMBRE", "APELLIDOS") show up here — your answer box
#    is usually directly beneath its label.
cargo run --release -- probe form.pdf

# 2. Draw your current rects on the page and dump each crop as a PNG.
cargo run --release -- calibrate form.pdf
open calibration/page01_overlay.png
```

Iterate on `template.toml` until every red rect sits squarely on an answer box,
and every `page01_<FIELD>.png` crop contains that field's handwriting and
nothing else. Then run the extraction.

## Extracting

```bash
cargo run --release -- extract form.pdf --pretty
cargo run --release -- extract form.pdf -o out.json
cargo run --release -- extract form.pdf --flat --pretty   # values as bare strings
```

Output is one entry per page, in document order:

```json
[
  {
      "page": 1,
      "fields": {
        "NOMBRE": {
          "text": "PRUEBA",
          "lines": [
            "Prueba"
          ],
          "confidence": 0.9976950883865356,
          "empty": false
        },
        "APELLIDO": {
          "text": "PRUEBA",
          "lines": [
            "Prueba"
          ],
          "confidence": 0.9992490410804749,
          "empty": false
        },
        "SEGUNDO_APELLIDO": {
          "text": "PRUEBA",
          "lines": [
            "Prueba"
          ],
          "confidence": 0.9478728771209717,
          "empty": false
        },
        "FECHA_NACIMIENTO": {
          "text": "09/09/2026",
          "lines": [
            "09/09/2026"
          ],
          "confidence": 0.9553796052932739,
          "empty": false
        },
        "GENERO": {
          "text": "PRUEBA",
          "lines": [
            "Prueba"
          ],
          "confidence": 0.968754768371582,
          "empty": false
        },
        "TELEFONO": {
          "text": "9976950883865356",
          "lines": [
            "9976950883865356"
          ],
          "confidence": 0.9327453374862671,
          "empty": false
        },
        "CORREO_ELECTRONICO": {
          "text": "email@example.com",
          "lines": [
            "email@example.com"
          ],
          "confidence": 0.925784707069397,
          "empty": false
        },
        "PAIS": {
          "text": "ESPAÑA",
          "lines": [
            "España"
          ],
          "confidence": 0.9829570651054382,
          "empty": false
        },
        "FECHA_FIRMA": {
          "text": "02/08/2026",
          "lines": [
            "02/08/2026"
          ],
          "confidence": 0.9992727041244507,
          "empty": false
        }
      }
    },
  { "page": 2, "fields": { "…": "…" } }
]
```

With `--flat`, each field collapses to `"NOMBRE": "MARIA"`.

## As a library

```rust
use pdf_form_ocr::{config::FormTemplate, extract};
use std::path::Path;

let template = FormTemplate::load(Path::new("template.toml"))?;
let entries = extract(Path::new("form.pdf"), &template, false)?;
```

## GPU

```bash
cargo build --release --features cuda      # or coreml / directml
```
