use crate::org::tiqian::core::inline_object_preferred_stretch::InlineObjectPreferredStretch;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct InlineObjectBoundaryAdjustment {
    pub participates_in_uniform_stretch: bool,
    pub preferred_stretch: Option<InlineObjectPreferredStretch>,
    pub shrink_capacity: f64,
    pub line_end_discardable_advance: f64,
    pub prevents_line_break: bool,
}

impl InlineObjectBoundaryAdjustment {
    pub fn new(participates_in_uniform_stretch: Option<bool>, preferred_stretch: Option<InlineObjectPreferredStretch>, shrink_capacity: Option<f64>, line_end_discardable_advance: Option<f64>, prevents_line_break: Option<bool>) -> Result<Self, TextRangeError> {
        let participates_in_uniform_stretch = participates_in_uniform_stretch.unwrap_or_else(|| false);
        let preferred_stretch = preferred_stretch.or_else(|| None);
        let shrink_capacity = shrink_capacity.unwrap_or_else(|| 0.0);
        let line_end_discardable_advance = line_end_discardable_advance.unwrap_or_else(|| 0.0);
        let prevents_line_break = prevents_line_break.unwrap_or_else(|| false);
        if !InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_is_finite(shrink_capacity) || (shrink_capacity) < (0.0f64) {
            return Err(TextRangeError::Message { text: UString::from("Inline-object boundary shrink capacity must be finite and non-negative") });
        }
        if !InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_is_finite(line_end_discardable_advance) || (line_end_discardable_advance) < (0.0f64) {
            return Err(TextRangeError::Message { text: UString::from("Inline-object line-end discardable advance must be finite and non-negative") });
        }
        Ok(Self {
            participates_in_uniform_stretch: participates_in_uniform_stretch,
            preferred_stretch: preferred_stretch,
            shrink_capacity: shrink_capacity,
            line_end_discardable_advance: line_end_discardable_advance,
            prevents_line_break: prevents_line_break,
        })
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("InlineObjectBoundaryAdjustment(")); __s += &(UString::from("participatesInUniformStretch=")); __s += UString::from(format!("{}", (self.participates_in_uniform_stretch).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("preferredStretch=")); __s += (match &(self.preferred_stretch) { None => UString::from("null"), Some(__option) => UString::from(format!("{}", __option.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("shrinkCapacity=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.shrink_capacity)); __s += &(UString::from(", ")); __s += &(UString::from("lineEndDiscardableAdvance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.line_end_discardable_advance)); __s += &(UString::from(", ")); __s += &(UString::from("preventsLineBreak=")); __s += UString::from(format!("{}", (self.prevents_line_break).to_string()).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }

    pub fn inline_object_boundary_adjustment_fixed() -> Result<InlineObjectBoundaryAdjustment, TextRangeError> {
        return Ok(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0), Some(0.0), Some(false))?);
    }

    pub(crate) fn inline_object_boundary_adjustment_is_finite(value: f64) -> bool {
        return value == value && value != f64::INFINITY && value != f64::NEG_INFINITY;
    }
}
