use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct SpacingDecisionInfo {
    pub range: TextRange,
    pub left_char: UString,
    pub right_char: UString,
    pub natural_inner_glue: f64,
    pub adjusted_inner_glue: f64,
    pub reduction: f64,
    pub reduction_target_range: TextRange,
    pub reason: UString,
}

impl SpacingDecisionInfo {
    pub fn new(range: TextRange, left_char: &UStr, right_char: &UStr, natural_inner_glue: f64, adjusted_inner_glue: f64, reduction: f64, reduction_target_range: TextRange, reason: &UStr) -> Self {
        Self {
            range,
            left_char: left_char.to_ustring(),
            right_char: right_char.to_ustring(),
            natural_inner_glue,
            adjusted_inner_glue,
            reduction,
            reduction_target_range,
            reason: reason.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("SpacingDecisionInfo(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("leftChar=")); __s += (self.left_char).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("rightChar=")); __s += (self.right_char).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("naturalInnerGlue=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.natural_inner_glue)); __s += &(UString::from(", ")); __s += &(UString::from("adjustedInnerGlue=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.adjusted_inner_glue)); __s += &(UString::from(", ")); __s += &(UString::from("reduction=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.reduction)); __s += &(UString::from(", ")); __s += &(UString::from("reductionTargetRange=")); __s += UString::from(format!("{}", (self.reduction_target_range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
