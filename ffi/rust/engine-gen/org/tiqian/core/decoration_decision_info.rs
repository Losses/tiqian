use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct DecorationDecisionInfo {
    pub cluster_range: TextRange,
    pub source_text: UString,
    pub kind: UString,
    pub applied: bool,
    pub reason: UString,
    pub anchor_x: f64,
    pub anchor_y: f64,
    pub dot_diameter: f64,
}

impl DecorationDecisionInfo {
    pub fn new(cluster_range: TextRange, source_text: &UStr, kind: &UStr, applied: bool, reason: &UStr, anchor_x: Option<f64>, anchor_y: Option<f64>, dot_diameter: Option<f64>) -> Self {
        let anchor_x = anchor_x.unwrap_or_else(|| 0.0);
        let anchor_y = anchor_y.unwrap_or_else(|| 0.0);
        let dot_diameter = dot_diameter.unwrap_or_else(|| 0.0);
        Self {
            cluster_range,
            source_text: source_text.to_ustring(),
            kind: kind.to_ustring(),
            applied,
            reason: reason.to_ustring(),
            anchor_x: anchor_x,
            anchor_y: anchor_y,
            dot_diameter: dot_diameter,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("DecorationDecisionInfo(")); __s += &(UString::from("clusterRange=")); __s += UString::from(format!("{}", (self.cluster_range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("sourceText=")); __s += (self.source_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("kind=")); __s += (self.kind).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("applied=")); __s += UString::from(format!("{}", (self.applied).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("anchorX=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.anchor_x)); __s += &(UString::from(", ")); __s += &(UString::from("anchorY=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.anchor_y)); __s += &(UString::from(", ")); __s += &(UString::from("dotDiameter=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.dot_diameter)); __s += &(UString::from(")")); __s }).as_str());
    }
}
