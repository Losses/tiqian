use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct TextRange {
    pub start: u32,
    pub end: u32,
}

impl TextRange {
    pub fn new(start: u32, end: u32) -> Result<Self, TextRangeError> {
        if start as i32 > ((end) as i32) {
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
    let cmp_start = match ((*(&a.start)) as i32).cmp(&((*(&b.start)) as i32)) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_start != 0 { return cmp_start; }
    let cmp_end = match ((*(&a.end)) as i32).cmp(&((*(&b.end)) as i32)) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_end != 0 { return cmp_end; }
    0
}
