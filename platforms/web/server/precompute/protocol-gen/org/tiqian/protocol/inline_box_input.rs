use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct InlineBoxInput {
    pub start: u32,
    pub end: u32,
    pub inline_start: f64,
    pub inline_end: f64,
    pub outer_spacing: UString,
}
