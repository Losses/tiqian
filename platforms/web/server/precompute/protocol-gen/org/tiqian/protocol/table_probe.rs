use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Clone, PartialEq)]
pub struct TableProbe {
    pub text: UString,
    pub advance_px: f64,
    pub font_size_px: f64,
    pub font_weight: f64,
    pub italic: bool,
    pub script: UString,
    pub language: UString,
    pub features: Vec<UString>,
}

impl TableProbe {
    pub fn new(text: &UStr, advance_px: f64, font_size_px: f64, font_weight: f64, italic: bool, script: &UStr, language: &UStr, features: Vec<UString>) -> Self {
        Self {
            text: text.to_ustring(),
            advance_px,
            font_size_px,
            font_weight,
            italic,
            script: script.to_ustring(),
            language: language.to_ustring(),
            features,
        }
    }
}
