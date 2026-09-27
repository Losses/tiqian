use crate::org::tiqian::core::text_range::TextRange;


#[derive(Debug, Clone, PartialEq)]
pub struct LineEdgeTrimDecisionInfo {
    pub line_range: TextRange,
    pub cluster_range: TextRange,
    pub side: String,
    pub trim_amount: f64,
    pub consumed_before: f64,
    pub natural_glue: f64,
    pub reason: String,
}

impl LineEdgeTrimDecisionInfo {
    pub fn new(line_range: TextRange, cluster_range: TextRange, side: &str, trim_amount: f64, consumed_before: f64, natural_glue: f64, reason: &str) -> Self {
        Self {
            line_range,
            cluster_range,
            side: side.to_string(),
            trim_amount,
            consumed_before,
            natural_glue,
            reason: reason.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "LineEdgeTrimDecisionInfo(",
            "lineRange=",
            (self.line_range).clone().to_string(),
            ", ",
            "clusterRange=",
            (self.cluster_range).clone().to_string(),
            ", ",
            "side=",
            (self.side).to_string(),
            ", ",
            "trimAmount=",
            self.trim_amount,
            ", ",
            "consumedBefore=",
            self.consumed_before,
            ", ",
            "naturalGlue=",
            self.natural_glue,
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}
