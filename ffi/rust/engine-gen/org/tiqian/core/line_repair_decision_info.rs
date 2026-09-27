use crate::org::tiqian::core::line_repair_allocation_info::LineRepairAllocationInfo;
use crate::org::tiqian::core::text_range::TextRange;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct LineRepairDecisionInfo {
    pub kind: String,
    pub reason_code: String,
    pub offender_range: TextRange,
    pub penalty: u32,
    pub target_cluster_index: Option<u32>,
    pub carried_cluster_index: Option<u32>,
    pub shrink: f64,
    pub available_capacity: f64,
    pub push_in_allocations: Vec<LineRepairAllocationInfo>,
}

impl LineRepairDecisionInfo {
    pub fn new(kind: &str, reason_code: &str, offender_range: TextRange, penalty: u32, target_cluster_index: Option<u32>, carried_cluster_index: Option<u32>, shrink: Option<f64>, available_capacity: Option<f64>, push_in_allocations: Option<Vec<LineRepairAllocationInfo>>) -> Self
{
        let target_cluster_index = target_cluster_index.or_else(|| None);
        let carried_cluster_index = carried_cluster_index.or_else(|| None);
        let shrink = shrink.unwrap_or_else(|| 0.0);
        let available_capacity = available_capacity.unwrap_or_else(|| 0.0);
        let push_in_allocations = push_in_allocations.unwrap_or_else(|| vec![]);
        Self {
            kind: kind.to_string(),
            reason_code: reason_code.to_string(),
            offender_range,
            penalty,
            target_cluster_index: target_cluster_index,
            carried_cluster_index: carried_cluster_index,
            shrink: shrink,
            available_capacity: available_capacity,
            push_in_allocations: push_in_allocations,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "LineRepairDecisionInfo(",
            "kind=",
            (self.kind).to_string(),
            ", ",
            "reasonCode=",
            (self.reason_code).to_string(),
            ", ",
            "offenderRange=",
            (self.offender_range).clone().to_string(),
            ", ",
            "penalty=",
            crate::runtime::int_text::IntText::int_text(self.penalty),
            ", ",
            "targetClusterIndex=",
            match self.target_cluster_index { Some(v) => crate::runtime::int_text::IntText::int_text(v), None => "null".to_string() },
            ", ",
            "carriedClusterIndex=",
            match self.carried_cluster_index { Some(v) => crate::runtime::int_text::IntText::int_text(v), None => "null".to_string() },
            ", ",
            "shrink=",
            self.shrink,
            ", ",
            "availableCapacity=",
            self.available_capacity,
            ", ",
            "pushInAllocations=",
            {
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
    },
            ")"
        );
    }
}
