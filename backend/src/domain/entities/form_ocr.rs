/// What the OCR read from one field of the registration form.
#[derive(Debug, Clone)]
pub struct FormOcrField {
    pub text: String,
    pub lines: Vec<String>,
    pub confidence: Option<f32>,
    pub empty: bool,
}

/// One page of a scanned registration form, fields in template order.
#[derive(Debug, Clone)]
pub struct FormOcrPage {
    pub page: usize,
    pub fields: Vec<(String, FormOcrField)>,
}
