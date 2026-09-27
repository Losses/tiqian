use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::runtime::u_string::UString;


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

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("TextSpan(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("style=")); __s += UString::from(format!("{}", (self.style).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
