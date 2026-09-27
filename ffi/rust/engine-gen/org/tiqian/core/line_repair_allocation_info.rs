use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct LineRepairAllocationInfo {
    pub cluster_range: TextRange,
    pub shrink: f64,
    pub available_capacity: f64,
}

impl LineRepairAllocationInfo {
    pub fn new(cluster_range: TextRange, shrink: f64, available_capacity: f64) -> Self {
        Self {
            cluster_range,
            shrink,
            available_capacity,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("LineRepairAllocationInfo(")); __s += &(UString::from("clusterRange=")); __s += UString::from(format!("{}", (self.cluster_range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("shrink=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.shrink)); __s += &(UString::from(", ")); __s += &(UString::from("availableCapacity=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.available_capacity)); __s += &(UString::from(")")); __s }).as_str());
    }
}
