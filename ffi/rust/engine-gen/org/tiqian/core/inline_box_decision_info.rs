use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct InlineBoxDecisionInfo {
    pub range: TextRange,
    pub inline_start: f64,
    pub inline_end: f64,
    pub outer_spacing: UString,
    pub first_cluster_index: u32,
    pub last_cluster_index: u32,
    pub reason: UString,
}

impl InlineBoxDecisionInfo {
    pub fn new(range: TextRange, inline_start: f64, inline_end: f64, outer_spacing: &UStr, first_cluster_index: u32, last_cluster_index: u32, reason: Option<UString>) -> Self {
        let reason = reason.unwrap_or_else(|| UString::from("InlineBoxBoundaryAdvance"));
        Self {
            range,
            inline_start,
            inline_end,
            outer_spacing: outer_spacing.to_ustring(),
            first_cluster_index,
            last_cluster_index,
            reason: reason,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("InlineBoxDecisionInfo(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("inlineStart=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.inline_start)); __s += &(UString::from(", ")); __s += &(UString::from("inlineEnd=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.inline_end)); __s += &(UString::from(", ")); __s += &(UString::from("outerSpacing=")); __s += (self.outer_spacing).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("firstClusterIndex=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.first_cluster_index)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("lastClusterIndex=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.last_cluster_index)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
