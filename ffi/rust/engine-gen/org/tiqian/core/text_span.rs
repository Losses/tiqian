use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;


#[derive(Debug, Clone, PartialEq)]
pub struct TextSpan {
    pub range: TextRange,
    pub style: TextStyle,
}

impl TextSpan {
    pub fn new(range: TextRange, style: TextStyle) -> Self {
        Self {
            range,
            style,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}",
            "TextSpan(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "style=",
            (self.style).clone().to_string(),
            ")"
        );
    }
}
