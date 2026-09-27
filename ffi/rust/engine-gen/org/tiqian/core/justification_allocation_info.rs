use crate::org::tiqian::core::text_range::TextRange;


#[derive(Debug, Clone, PartialEq)]
pub struct JustificationAllocationInfo {
    pub cluster_range: TextRange,
    pub kind: String,
    pub priority: u32,
    pub delta: f64,
    pub reason: String,
}

impl JustificationAllocationInfo {
    pub fn new(cluster_range: TextRange, kind: &str, priority: u32, delta: f64, reason: &str) -> Self {
        Self {
            cluster_range,
            kind: kind.to_string(),
            priority,
            delta,
            reason: reason.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "JustificationAllocationInfo(",
            "clusterRange=",
            (self.cluster_range).clone().to_string(),
            ", ",
            "kind=",
            (self.kind).to_string(),
            ", ",
            "priority=",
            crate::runtime::int_text::IntText::int_text(self.priority),
            ", ",
            "delta=",
            self.delta,
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}
