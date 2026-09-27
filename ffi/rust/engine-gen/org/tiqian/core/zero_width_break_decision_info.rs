use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::runtime::sorted_table::SortedTable;


#[derive(Debug, Clone, PartialEq)]
pub struct ZeroWidthBreakDecisionInfo {
    pub range: TextRange,
    pub source_text: String,
    pub cluster_index: u32,
    pub reason: String,
}

impl ZeroWidthBreakDecisionInfo {
    pub fn new(range: TextRange, source_text: &str, cluster_index: u32, reason: Option<String>) -> Self {
        let reason = reason.unwrap_or_else(|| "ZeroWidthSpaceSoftBreakNoShape".to_string());
        Self {
            range,
            source_text: source_text.to_string(),
            cluster_index,
            reason: reason,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "ZeroWidthBreakDecisionInfo(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "sourceText=",
            (self.source_text).to_string(),
            ", ",
            "clusterIndex=",
            crate::runtime::int_text::IntText::int_text(self.cluster_index),
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}

pub fn compare_zero_width_break_decision_info(a: &ZeroWidthBreakDecisionInfo, b: &ZeroWidthBreakDecisionInfo) -> i32 {
    let cmp_range = compare_text_range(&a.range, &b.range);
    if cmp_range != 0 { return cmp_range; }
    let cmp_source_text = SortedTable::sorted_table_compare_strings(a.source_text.as_str(), b.source_text.as_str());
    if cmp_source_text != 0 { return cmp_source_text; }
    let cmp_cluster_index = if a.cluster_index < b.cluster_index { -1 } else if a.cluster_index > b.cluster_index { 1 } else { 0 };
    if cmp_cluster_index != 0 { return cmp_cluster_index; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_str(), b.reason.as_str());
    if cmp_reason != 0 { return cmp_reason; }
    0
}
