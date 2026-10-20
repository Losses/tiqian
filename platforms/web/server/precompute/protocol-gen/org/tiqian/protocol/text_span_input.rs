use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct TextSpanInput {
    pub start: u32,
    pub end: u32,
    pub families: Vec<UString>,
    pub font_size_px: f64,
    pub font_weight: u32,
    pub italic: bool,
    pub baseline_shift: f64,
}
