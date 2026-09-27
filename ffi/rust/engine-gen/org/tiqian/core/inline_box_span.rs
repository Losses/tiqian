use crate::org::tiqian::core::inline_box_outer_spacing::InlineBoxOuterSpacing;
use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct InlineBoxSpan {
    pub range: TextRange,
    pub inline_start: f64,
    pub inline_end: f64,
    pub outer_spacing: InlineBoxOuterSpacing,
}

impl InlineBoxSpan {
    pub fn new(range: TextRange, inline_start: Option<f64>, inline_end: Option<f64>, outer_spacing: Option<InlineBoxOuterSpacing>) -> Self {
        let inline_start = inline_start.unwrap_or_else(|| 0.0);
        let inline_end = inline_end.unwrap_or_else(|| 0.0);
        let outer_spacing = outer_spacing.unwrap_or_else(|| InlineBoxOuterSpacing::Narrow);
        Self {
            range,
            inline_start: inline_start,
            inline_end: inline_end,
            outer_spacing: outer_spacing,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("InlineBoxSpan(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("inlineStart=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.inline_start)); __s += &(UString::from(", ")); __s += &(UString::from("inlineEnd=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.inline_end)); __s += &(UString::from(", ")); __s += &(UString::from("outerSpacing=")); __s += UString::from(self.outer_spacing.name()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
