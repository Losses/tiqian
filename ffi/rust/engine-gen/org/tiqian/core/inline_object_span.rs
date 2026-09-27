use crate::org::tiqian::core::inline_object_boundary_adjustment::InlineObjectBoundaryAdjustment;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;


#[derive(Debug, Clone, PartialEq)]
pub struct InlineObjectSpan {
    pub range: TextRange,
    pub advance: f64,
    pub ascent: f64,
    pub descent: f64,
    pub leading_boundary: InlineObjectBoundaryAdjustment,
    pub trailing_boundary: InlineObjectBoundaryAdjustment,
}

impl InlineObjectSpan {
    pub const INLINE_OBJECT_SPAN_INLINE_OBJECT_REPLACEMENT_CHAR: &str = "￼";

    pub fn new(range: TextRange, advance: f64, ascent: f64, descent: f64, leading_boundary: Option<InlineObjectBoundaryAdjustment>, trailing_boundary: Option<InlineObjectBoundaryAdjustment>) -> Result<Self, TextRangeError> {
        let leading_boundary = leading_boundary.unwrap_or_else(|| InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap());
        let trailing_boundary = trailing_boundary.unwrap_or_else(|| InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap());
        Ok(Self {
            range,
            advance,
            ascent,
            descent,
            leading_boundary: leading_boundary,
            trailing_boundary: trailing_boundary,
        })
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "InlineObjectSpan(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "advance=",
            self.advance,
            ", ",
            "ascent=",
            self.ascent,
            ", ",
            "descent=",
            self.descent,
            ", ",
            "leadingBoundary=",
            (self.leading_boundary).clone().to_string(),
            ", ",
            "trailingBoundary=",
            (self.trailing_boundary).clone().to_string(),
            ")"
        );
    }
}
