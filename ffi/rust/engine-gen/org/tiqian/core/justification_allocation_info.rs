use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct JustificationAllocationInfo {
    pub cluster_range: TextRange,
    pub kind: UString,
    pub priority: u32,
    pub delta: f64,
    pub reason: UString,
}

impl JustificationAllocationInfo {
    pub fn new(cluster_range: TextRange, kind: &UStr, priority: u32, delta: f64, reason: &UStr) -> Self {
        Self {
            cluster_range,
            kind: kind.to_ustring(),
            priority,
            delta,
            reason: reason.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("JustificationAllocationInfo(")); __s += &(UString::from("clusterRange=")); __s += UString::from(format!("{}", (self.cluster_range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("kind=")); __s += (self.kind).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("priority=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.priority)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("delta=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.delta)); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
