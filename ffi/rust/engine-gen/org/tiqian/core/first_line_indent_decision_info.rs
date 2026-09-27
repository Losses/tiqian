use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct FirstLineIndentDecisionInfo {
    pub source: UString,
    pub measure_em: f64,
    pub threshold_em: f64,
    pub resolved_em: f64,
}

impl FirstLineIndentDecisionInfo {
    pub fn new(source: &UStr, measure_em: f64, threshold_em: f64, resolved_em: f64) -> Self {
        Self {
            source: source.to_ustring(),
            measure_em,
            threshold_em,
            resolved_em,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("FirstLineIndentDecisionInfo(")); __s += &(UString::from("source=")); __s += (self.source).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("measureEm=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.measure_em)); __s += &(UString::from(", ")); __s += &(UString::from("thresholdEm=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.threshold_em)); __s += &(UString::from(", ")); __s += &(UString::from("resolvedEm=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.resolved_em)); __s += &(UString::from(")")); __s }).as_str());
    }
}
