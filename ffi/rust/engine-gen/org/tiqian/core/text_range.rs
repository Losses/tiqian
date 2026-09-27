use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct TextRange {
    pub start: u32,
    pub end: u32,
}

impl TextRange {
    pub fn new(start: u32, end: u32) -> Result<Self, TextRangeError> {
        if i32::from_ne_bytes(((start) as i32).to_ne_bytes()) > (i32::from_ne_bytes(((end) as i32).to_ne_bytes())) {
            return Err(TextRangeError::StartGreaterThanEnd);
        }
        if start > 2147483647 {
            return Err(TextRangeError::NegativeStart);
        }
        Ok(Self {
            start,
            end,
        })
    }

    pub fn get_length(&self) -> u32 {
        return u32::wrapping_sub(self.end, self.start);
    }

    pub fn get_is_empty(&self) -> bool {
        return self.get_length() == 0;
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("TextRange(")); __s += &(UString::from("start=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.start)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("end=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.end)).as_str())); __s += &(UString::from(")")); __s }).as_str());
    }
}

pub fn compare_text_range(a: &TextRange, b: &TextRange) -> i32 {
    let cmp_start = if a.start < b.start { -1 } else if a.start > b.start { 1 } else { 0 };
    if cmp_start != 0 { return cmp_start; }
    let cmp_end = if a.end < b.end { -1 } else if a.end > b.end { 1 } else { 0 };
    if cmp_end != 0 { return cmp_end; }
    0
}
