use crate::org::tiqian::core::line_repair_allocation_info::LineRepairAllocationInfo;
use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct LineRepairDecisionInfo {
    pub kind: UString,
    pub reason_code: UString,
    pub offender_range: TextRange,
    pub penalty: u32,
    pub target_cluster_index: Option<u32>,
    pub carried_cluster_index: Option<u32>,
    pub shrink: f64,
    pub available_capacity: f64,
    pub push_in_allocations: Vec<LineRepairAllocationInfo>,
}

impl LineRepairDecisionInfo {
    pub fn new(kind: &UStr, reason_code: &UStr, offender_range: TextRange, penalty: u32, target_cluster_index: Option<u32>, carried_cluster_index: Option<u32>, shrink: Option<f64>, available_capacity: Option<f64>, push_in_allocations: Option<Vec<LineRepairAllocationInfo>>) -> Self {
        let target_cluster_index = target_cluster_index.or_else(|| None);
        let carried_cluster_index = carried_cluster_index.or_else(|| None);
        let shrink = shrink.unwrap_or_else(|| 0.0);
        let available_capacity = available_capacity.unwrap_or_else(|| 0.0);
        let push_in_allocations = push_in_allocations.unwrap_or_else(|| vec![]);
        Self {
            kind: kind.to_ustring(),
            reason_code: reason_code.to_ustring(),
            offender_range,
            penalty,
            target_cluster_index: target_cluster_index,
            carried_cluster_index: carried_cluster_index,
            shrink: shrink,
            available_capacity: available_capacity,
            push_in_allocations: push_in_allocations,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("LineRepairDecisionInfo(")); __s += &(UString::from("kind=")); __s += (self.kind).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reasonCode=")); __s += (self.reason_code).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("offenderRange=")); __s += UString::from(format!("{}", (self.offender_range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("penalty=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.penalty)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("targetClusterIndex=")); __s += &(match self.target_cluster_index { Some(v) => UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(v)).as_str()), None => UString::from("null") }); __s += &(UString::from(", ")); __s += &(UString::from("carriedClusterIndex=")); __s += &(match self.carried_cluster_index { Some(v) => UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(v)).as_str()), None => UString::from("null") }); __s += &(UString::from(", ")); __s += &(UString::from("shrink=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.shrink)); __s += &(UString::from(", ")); __s += &(UString::from("availableCapacity=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.available_capacity)); __s += &(UString::from(", ")); __s += &(UString::from("pushInAllocations=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.push_in_allocations).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
