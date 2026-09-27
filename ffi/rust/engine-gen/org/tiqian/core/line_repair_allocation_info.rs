use crate::org::tiqian::core::text_range::TextRange;


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

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "LineRepairAllocationInfo(",
            "clusterRange=",
            (self.cluster_range).clone().to_string(),
            ", ",
            "shrink=",
            self.shrink,
            ", ",
            "availableCapacity=",
            self.available_capacity,
            ")"
        );
    }
}
