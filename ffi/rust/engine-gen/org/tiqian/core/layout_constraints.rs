use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;


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
            return Err(TextRangeError::Message { text: "maxWidth must be positive.".to_string() });
        }
        if !((max_height) > (0.0f64)) {
            return Err(TextRangeError::Message { text: "maxHeight must be positive.".to_string() });
        }
        if i32::from_ne_bytes((max_lines).to_ne_bytes()) <= 0 {
            return Err(TextRangeError::Message { text: "maxLines must be positive.".to_string() });
        }
        Ok(Self {
            max_width,
            max_height: max_height,
            max_lines: max_lines,
        })
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "LayoutConstraints(",
            "maxWidth=",
            self.max_width,
            ", ",
            "maxHeight=",
            self.max_height,
            ", ",
            "maxLines=",
            crate::runtime::int_text::IntText::int_text(self.max_lines),
            ")"
        );
    }
}
