use crate::org::tiqian::core::text_range::TextRange;


#[derive(Debug, Clone, PartialEq)]
pub struct AutoSpaceDecisionInfo {
    pub cluster_range: TextRange,
    pub side: String,
    pub boundary_role: String,
    pub mode: String,
    pub characters_affected: u32,
    pub reduction_per_char: f64,
    pub total_reduction: f64,
    pub reason: String,
}

impl AutoSpaceDecisionInfo {
    pub fn new(cluster_range: TextRange, side: &str, boundary_role: &str, mode: &str, characters_affected: u32, reduction_per_char: f64, total_reduction: f64, reason: &str) -> Self {
        Self {
            cluster_range,
            side: side.to_string(),
            boundary_role: boundary_role.to_string(),
            mode: mode.to_string(),
            characters_affected,
            reduction_per_char,
            total_reduction,
            reason: reason.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "AutoSpaceDecisionInfo(",
            "clusterRange=",
            (self.cluster_range).clone().to_string(),
            ", ",
            "side=",
            (self.side).to_string(),
            ", ",
            "boundaryRole=",
            (self.boundary_role).to_string(),
            ", ",
            "mode=",
            (self.mode).to_string(),
            ", ",
            "charactersAffected=",
            crate::runtime::int_text::IntText::int_text(self.characters_affected),
            ", ",
            "reductionPerChar=",
            self.reduction_per_char,
            ", ",
            "totalReduction=",
            self.total_reduction,
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}
