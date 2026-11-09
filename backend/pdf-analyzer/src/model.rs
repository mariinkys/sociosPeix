//! Shape of the JSON that comes out the other end.

use serde::Serialize;
use serde_json::{Map, Value};

/// One PDF page
#[derive(Debug, Serialize)]
pub struct Entry {
    /// 1-based page number.
    pub page: usize,
    /// Field name - value, in template order.
    pub fields: Map<String, Value>,
}

#[derive(Debug, Serialize)]
pub struct FieldValue {
    /// Final value, after joining lines and applying the transform.
    pub text: String,
    /// Individual recognised lines before joining.
    pub lines: Vec<String>,
    /// Mean recognition confidence across the lines, if anything was read.
    pub confidence: Option<f32>,
    /// True when nothing legible was found in the box.
    pub empty: bool,
}

/// Build one entry. `flat` collapses each field to a bare string, which is what
/// you usually want once the template is dialled in and you trust the output.
pub fn build_entry(page: usize, fields: Vec<(String, FieldValue)>, flat: bool) -> Entry {
    let mut map = Map::new();
    for (name, value) in fields {
        let v = if flat {
            Value::String(value.text)
        } else {
            serde_json::to_value(value).unwrap_or(Value::Null)
        };
        map.insert(name, v);
    }
    Entry { page, fields: map }
}
