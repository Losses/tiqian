use crate::org::tiqian::core::text_range::TextRange;


#[derive(Debug, Clone, PartialEq)]
pub struct LineRepairCandidateInfo {
    pub kind: String,
    pub reason_code: String,
    pub offender_range: TextRange,
    pub penalty: u32,
    pub accepted: bool,
    pub rejection_reason: Option<String>,
    pub target_cluster_index: Option<u32>,
    pub carried_cluster_index: Option<u32>,
    pub shrink: f64,
    pub required_shrink: f64,
    pub available_capacity: f64,
}

impl LineRepairCandidateInfo {
    pub fn new(kind: &str, reason_code: &str, offender_range: TextRange, penalty: u32, accepted: bool, rejection_reason: Option<String>, target_cluster_index: Option<u32>, carried_cluster_index: Option<u32>, shrink: Option<f64>, required_shrink: Option<f64>, available_capacity:
Option<f64>) -> Self {
        let rejection_reason = rejection_reason.or_else(|| None);
        let target_cluster_index = target_cluster_index.or_else(|| None);
        let carried_cluster_index = carried_cluster_index.or_else(|| None);
        let shrink = shrink.unwrap_or_else(|| 0.0);
        let required_shrink = required_shrink.unwrap_or_else(|| 0.0);
        let available_capacity = available_capacity.unwrap_or_else(|| 0.0);
        Self {
            kind: kind.to_string(),
            reason_code: reason_code.to_string(),
            offender_range,
            penalty,
            accepted,
            rejection_reason: rejection_reason,
            target_cluster_index: target_cluster_index,
            carried_cluster_index: carried_cluster_index,
            shrink: shrink,
            required_shrink: required_shrink,
            available_capacity: available_capacity,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "LineRepairCandidateInfo(",
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
            "accepted=",
            self.accepted,
            ", ",
            "rejectionReason=",
            match (self.rejection_reason).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
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
            "requiredShrink=",
            self.required_shrink,
            ", ",
            "availableCapacity=",
            self.available_capacity,
            ")"
        );
    }
}
