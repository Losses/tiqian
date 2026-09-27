use crate::org::tiqian::core::inline_box_outer_spacing::InlineBoxOuterSpacing;
use crate::org::tiqian::core::text_range::TextRange;


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

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "InlineBoxSpan(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "inlineStart=",
            self.inline_start,
            ", ",
            "inlineEnd=",
            self.inline_end,
            ", ",
            "outerSpacing=",
            self.outer_spacing.name(),
            ")"
        );
    }
}
