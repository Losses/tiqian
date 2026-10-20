use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct ContextualKinsokuDecisionInfo {
    pub range: TextRange,
    pub source_text: UString,
    pub cluster_index: u32,
    pub forbidden_position: UString,
    pub reason: UString,
    pub impossible_measure_fallback: Option<UString>,
}

impl ContextualKinsokuDecisionInfo {
    pub fn new(range: TextRange, source_text: &UStr, cluster_index: u32, forbidden_position: &UStr, reason: &UStr, impossible_measure_fallback: Option<UString>) -> Self {
        let impossible_measure_fallback = impossible_measure_fallback.or_else(|| None);
        Self {
            range,
            source_text: source_text.to_ustring(),
            cluster_index,
            forbidden_position: forbidden_position.to_ustring(),
            reason: reason.to_ustring(),
            impossible_measure_fallback: impossible_measure_fallback,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ContextualKinsokuDecisionInfo(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("sourceText=")); __s += (self.source_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("clusterIndex=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.cluster_index)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("forbiddenPosition=")); __s += (self.forbidden_position).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("impossibleMeasureFallback=")); __s += match &((self.impossible_measure_fallback).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(")")); __s }).as_str());
    }
}

pub fn compare_contextual_kinsoku_decision_info(a: &ContextualKinsokuDecisionInfo, b: &ContextualKinsokuDecisionInfo) -> i32 {
    let cmp_range = compare_text_range(&a.range, &b.range);
    if cmp_range != 0 { return cmp_range; }
    let cmp_source_text = SortedTable::sorted_table_compare_strings(a.source_text.as_ustr(), b.source_text.as_ustr());
    if cmp_source_text != 0 { return cmp_source_text; }
    let cmp_cluster_index = if a.cluster_index < b.cluster_index { -1 } else if a.cluster_index > b.cluster_index { 1 } else { 0 };
    if cmp_cluster_index != 0 { return cmp_cluster_index; }
    let cmp_forbidden_position = SortedTable::sorted_table_compare_strings(a.forbidden_position.as_ustr(), b.forbidden_position.as_ustr());
    if cmp_forbidden_position != 0 { return cmp_forbidden_position; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_ustr(), b.reason.as_ustr());
    if cmp_reason != 0 { return cmp_reason; }
    let cmp_impossible_measure_fallback = match (&a.impossible_measure_fallback, &b.impossible_measure_fallback) { (None, None) => 0, (None, Some(_)) => -1, (Some(_), None) => 1, (Some(av), Some(bv)) => match av.cmp(bv) { core::cmp::Ordering::Less => -1,
core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 } };
    if cmp_impossible_measure_fallback != 0 { return cmp_impossible_measure_fallback; }
    0
}
