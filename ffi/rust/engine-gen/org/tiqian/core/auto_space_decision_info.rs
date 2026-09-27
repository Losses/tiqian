use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct AutoSpaceDecisionInfo {
    pub cluster_range: TextRange,
    pub side: UString,
    pub boundary_role: UString,
    pub mode: UString,
    pub characters_affected: u32,
    pub reduction_per_char: f64,
    pub total_reduction: f64,
    pub reason: UString,
}

impl AutoSpaceDecisionInfo {
    pub fn new(cluster_range: TextRange, side: &UStr, boundary_role: &UStr, mode: &UStr, characters_affected: u32, reduction_per_char: f64, total_reduction: f64, reason: &UStr) -> Self {
        Self {
            cluster_range,
            side: side.to_ustring(),
            boundary_role: boundary_role.to_ustring(),
            mode: mode.to_ustring(),
            characters_affected,
            reduction_per_char,
            total_reduction,
            reason: reason.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("AutoSpaceDecisionInfo(")); __s += &(UString::from("clusterRange=")); __s += UString::from(format!("{}", (self.cluster_range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("side=")); __s += (self.side).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("boundaryRole=")); __s += (self.boundary_role).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("mode=")); __s += (self.mode).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("charactersAffected=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.characters_affected)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("reductionPerChar=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.reduction_per_char)); __s += &(UString::from(", ")); __s += &(UString::from("totalReduction=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.total_reduction)); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
