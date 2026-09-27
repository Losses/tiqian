#[derive(Clone, PartialEq)]
pub struct StyleRow {
    pub font_size_px: f64,
    pub font_weight: f64,
    pub italic: u32,
    pub script_ref: u32,
    pub language_ref: u32,
}

impl StyleRow {
    pub fn new(font_size_px: f64, font_weight: f64, italic: u32, script_ref: u32, language_ref: u32) -> Self {
        Self {
            font_size_px,
            font_weight,
            italic,
            script_ref,
            language_ref,
        }
    }
}
