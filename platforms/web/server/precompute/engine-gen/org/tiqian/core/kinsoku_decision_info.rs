use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct KinsokuDecisionInfo {
    pub measure_em: f64,
    pub level: UString,
    pub hanging: UString,
    pub reason: UString,
}

impl KinsokuDecisionInfo {
    pub fn new(measure_em: f64, level: &UStr, hanging: &UStr, reason: &UStr) -> Self {
        Self {
            measure_em,
            level: level.to_ustring(),
            hanging: hanging.to_ustring(),
            reason: reason.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("KinsokuDecisionInfo(")); __s += &(UString::from("measureEm=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.measure_em)); __s += &(UString::from(", ")); __s += &(UString::from("level=")); __s += (self.level).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("hanging=")); __s += (self.hanging).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
