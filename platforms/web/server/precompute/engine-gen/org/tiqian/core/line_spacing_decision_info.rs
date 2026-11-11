use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct LineSpacingDecisionInfo {
    pub natural_height: f64,
    pub requested_line_height: Option<f64>,
    pub resolved_height: f64,
    pub spacing_floor: f64,
    pub floor_applied: bool,
    pub reason: UString,
}

impl LineSpacingDecisionInfo {
    pub fn new(natural_height: f64, requested_line_height: Option<f64>, resolved_height: f64, spacing_floor: f64, floor_applied: bool, reason: &UStr) -> Self {
        Self {
            natural_height,
            requested_line_height,
            resolved_height,
            spacing_floor,
            floor_applied,
            reason: reason.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("LineSpacingDecisionInfo(")); __s += &(UString::from("naturalHeight=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.natural_height)); __s += &(UString::from(", ")); __s += &(UString::from("requestedLineHeight=")); __s += &(match self.requested_line_height { Some(v) => UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(v)).as_str()), None => UString::from("null") }); __s += &(UString::from(", ")); __s += &(UString::from("resolvedHeight=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.resolved_height)); __s += &(UString::from(", ")); __s += &(UString::from("spacingFloor=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.spacing_floor)); __s += &(UString::from(", ")); __s += &(UString::from("floorApplied=")); __s += UString::from(format!("{}", (self.floor_applied).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
