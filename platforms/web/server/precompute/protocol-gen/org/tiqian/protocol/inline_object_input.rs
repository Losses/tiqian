#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InlineObjectInput {
    pub start: u32,
    pub end: u32,
    pub advance: f64,
    pub ascent: f64,
    pub descent: f64,
}
