use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::runtime::sorted_table::SortedTable;


#[derive(Debug, Clone, PartialEq)]
pub struct ContextualKinsokuDecisionInfo {
    pub range: TextRange,
    pub source_text: String,
    pub cluster_index: u32,
    pub forbidden_position: String,
    pub reason: String,
    pub impossible_measure_fallback: Option<String>,
}

impl ContextualKinsokuDecisionInfo {
    pub fn new(range: TextRange, source_text: &str, cluster_index: u32, forbidden_position: &str, reason: &str, impossible_measure_fallback: Option<String>) -> Self {
        let impossible_measure_fallback = impossible_measure_fallback.or_else(|| None);
        Self {
            range,
            source_text: source_text.to_string(),
            cluster_index,
            forbidden_position: forbidden_position.to_string(),
            reason: reason.to_string(),
            impossible_measure_fallback: impossible_measure_fallback,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "ContextualKinsokuDecisionInfo(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "sourceText=",
            (self.source_text).to_string(),
            ", ",
            "clusterIndex=",
            crate::runtime::int_text::IntText::int_text(self.cluster_index),
            ", ",
            "forbiddenPosition=",
            (self.forbidden_position).to_string(),
            ", ",
            "reason=",
            (self.reason).to_string(),
            ", ",
            "impossibleMeasureFallback=",
            match (self.impossible_measure_fallback).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ")"
        );
    }
}

pub fn compare_contextual_kinsoku_decision_info(a: &ContextualKinsokuDecisionInfo, b: &ContextualKinsokuDecisionInfo) -> i32 {
    let cmp_range = compare_text_range(&a.range, &b.range);
    if cmp_range != 0 { return cmp_range; }
    let cmp_source_text = SortedTable::sorted_table_compare_strings(a.source_text.as_str(), b.source_text.as_str());
    if cmp_source_text != 0 { return cmp_source_text; }
    let cmp_cluster_index = if a.cluster_index < b.cluster_index { -1 } else if a.cluster_index > b.cluster_index { 1 } else { 0 };
    if cmp_cluster_index != 0 { return cmp_cluster_index; }
    let cmp_forbidden_position = SortedTable::sorted_table_compare_strings(a.forbidden_position.as_str(), b.forbidden_position.as_str());
    if cmp_forbidden_position != 0 { return cmp_forbidden_position; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_str(), b.reason.as_str());
    if cmp_reason != 0 { return cmp_reason; }
    let cmp_impossible_measure_fallback = match (&a.impossible_measure_fallback, &b.impossible_measure_fallback) { (None, None) => 0, (None, Some(_)) => -1, (Some(_), None) => 1, (Some(av), Some(bv)) => match av.cmp(bv) { core::cmp::Ordering::Less => -1,
core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 } };
    if cmp_impossible_measure_fallback != 0 { return cmp_impossible_measure_fallback; }
    0
}
