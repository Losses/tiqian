use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::runtime::sorted_table::SortedTable;


#[derive(Debug, Clone, PartialEq)]
pub struct EmergencyTrackingEligibilityDecisionInfo {
    pub range: TextRange,
    pub source_text: String,
    pub reason: String,
}

impl EmergencyTrackingEligibilityDecisionInfo {
    pub fn new(range: TextRange, source_text: &str, reason: &str) -> Self {
        Self {
            range,
            source_text: source_text.to_string(),
            reason: reason.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "EmergencyTrackingEligibilityDecisionInfo(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "sourceText=",
            (self.source_text).to_string(),
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}

pub fn compare_emergency_tracking_eligibility_decision_info(a: &EmergencyTrackingEligibilityDecisionInfo, b: &EmergencyTrackingEligibilityDecisionInfo) -> i32 {
    let cmp_range = compare_text_range(&a.range, &b.range);
    if cmp_range != 0 { return cmp_range; }
    let cmp_source_text = SortedTable::sorted_table_compare_strings(a.source_text.as_str(), b.source_text.as_str());
    if cmp_source_text != 0 { return cmp_source_text; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_str(), b.reason.as_str());
    if cmp_reason != 0 { return cmp_reason; }
    0
}
