use crate::org::tiqian::core::inline_object_boundary_adjustment::InlineObjectBoundaryAdjustment;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


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
    pub const INLINE_OBJECT_SPAN_INLINE_OBJECT_REPLACEMENT_CHAR: &UStr = unsafe { &*(&[0xFFFCu16] as *const [u16] as *const crate::runtime::u_string::UStr) };

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

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("InlineObjectSpan(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("advance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.advance)); __s += &(UString::from(", ")); __s += &(UString::from("ascent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.ascent)); __s += &(UString::from(", ")); __s += &(UString::from("descent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.descent)); __s += &(UString::from(", ")); __s += &(UString::from("leadingBoundary=")); __s += UString::from(format!("{}", (self.leading_boundary).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("trailingBoundary=")); __s += UString::from(format!("{}", (self.trailing_boundary).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
