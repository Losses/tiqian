use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct LayoutConstraints {
    pub max_width: f64,
    pub max_height: f64,
    pub max_lines: u32,
}

impl LayoutConstraints {
    pub fn new(max_width: f64, max_height: Option<f64>, max_lines: Option<u32>) -> Result<Self, TextRangeError> {
        let max_height = max_height.unwrap_or_else(|| f64::INFINITY);
        let max_lines = max_lines.unwrap_or_else(|| 2147483647);
        if !((max_width) > (0.0f64)) {
            return Err(TextRangeError::Message { text: UString::from("maxWidth must be positive.") });
        }
        if !((max_height) > (0.0f64)) {
            return Err(TextRangeError::Message { text: UString::from("maxHeight must be positive.") });
        }
        if i32::from_ne_bytes(((max_lines) as i32).to_ne_bytes()) <= 0 {
            return Err(TextRangeError::Message { text: UString::from("maxLines must be positive.") });
        }
        Ok(Self {
            max_width,
            max_height: max_height,
            max_lines: max_lines,
        })
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("LayoutConstraints(")); __s += &(UString::from("maxWidth=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.max_width)); __s += &(UString::from(", ")); __s += &(UString::from("maxHeight=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.max_height)); __s += &(UString::from(", ")); __s += &(UString::from("maxLines=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.max_lines)).as_str())); __s += &(UString::from(")")); __s }).as_str());
    }
}
