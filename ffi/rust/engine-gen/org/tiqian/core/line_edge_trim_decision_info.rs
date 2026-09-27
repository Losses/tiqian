use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct LineEdgeTrimDecisionInfo {
    pub line_range: TextRange,
    pub cluster_range: TextRange,
    pub side: UString,
    pub trim_amount: f64,
    pub consumed_before: f64,
    pub natural_glue: f64,
    pub reason: UString,
}

impl LineEdgeTrimDecisionInfo {
    pub fn new(line_range: TextRange, cluster_range: TextRange, side: &UStr, trim_amount: f64, consumed_before: f64, natural_glue: f64, reason: &UStr) -> Self {
        Self {
            line_range,
            cluster_range,
            side: side.to_ustring(),
            trim_amount,
            consumed_before,
            natural_glue,
            reason: reason.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("LineEdgeTrimDecisionInfo(")); __s += &(UString::from("lineRange=")); __s += UString::from(format!("{}", (self.line_range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("clusterRange=")); __s += UString::from(format!("{}", (self.cluster_range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("side=")); __s += (self.side).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("trimAmount=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.trim_amount)); __s += &(UString::from(", ")); __s += &(UString::from("consumedBefore=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.consumed_before)); __s += &(UString::from(", ")); __s += &(UString::from("naturalGlue=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.natural_glue)); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
