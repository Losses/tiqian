use crate::org::tiqian::core::text_range::TextRange;


#[derive(Debug, Clone, PartialEq)]
pub struct InlineBoxDecisionInfo {
    pub range: TextRange,
    pub inline_start: f64,
    pub inline_end: f64,
    pub outer_spacing: String,
    pub first_cluster_index: u32,
    pub last_cluster_index: u32,
    pub reason: String,
}

impl InlineBoxDecisionInfo {
    pub fn new(range: TextRange, inline_start: f64, inline_end: f64, outer_spacing: &str, first_cluster_index: u32, last_cluster_index: u32, reason: Option<String>) -> Self {
        let reason = reason.unwrap_or_else(|| "InlineBoxBoundaryAdvance".to_string());
        Self {
            range,
            inline_start,
            inline_end,
            outer_spacing: outer_spacing.to_string(),
            first_cluster_index,
            last_cluster_index,
            reason: reason,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "InlineBoxDecisionInfo(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "inlineStart=",
            self.inline_start,
            ", ",
            "inlineEnd=",
            self.inline_end,
            ", ",
            "outerSpacing=",
            (self.outer_spacing).to_string(),
            ", ",
            "firstClusterIndex=",
            crate::runtime::int_text::IntText::int_text(self.first_cluster_index),
            ", ",
            "lastClusterIndex=",
            crate::runtime::int_text::IntText::int_text(self.last_cluster_index),
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}
