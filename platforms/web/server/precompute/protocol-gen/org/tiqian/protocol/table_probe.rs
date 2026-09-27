#[derive(Clone, PartialEq)]
pub struct TableProbe {
    pub text: String,
    pub advance_px: f64,
    pub font_size_px: f64,
    pub font_weight: f64,
    pub italic: bool,
    pub script: String,
    pub language: String,
    pub features: Vec<String>,
}

impl TableProbe {
    pub fn new(text: &str, advance_px: f64, font_size_px: f64, font_weight: f64, italic: bool, script: &str, language: &str, features: Vec<String>) -> Self {
        Self {
            text: text.to_string(),
            advance_px,
            font_size_px,
            font_weight,
            italic,
            script: script.to_string(),
            language: language.to_string(),
            features,
        }
    }
}
