use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct MandatoryBreakDecisionInfo {
    pub range: TextRange,
    pub source_text: UString,
    pub break_after_cluster_index: u32,
    pub reason: UString,
}

impl MandatoryBreakDecisionInfo {
    pub fn new(range: TextRange, source_text: &UStr, break_after_cluster_index: u32, reason: &UStr) -> Self {
        Self {
            range,
            source_text: source_text.to_ustring(),
            break_after_cluster_index,
            reason: reason.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("MandatoryBreakDecisionInfo(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("sourceText=")); __s += (self.source_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("breakAfterClusterIndex=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.break_after_cluster_index)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

pub fn compare_mandatory_break_decision_info(a: &MandatoryBreakDecisionInfo, b: &MandatoryBreakDecisionInfo) -> i32 {
    let cmp_range = compare_text_range(&a.range, &b.range);
    if cmp_range != 0 { return cmp_range; }
    let cmp_source_text = SortedTable::sorted_table_compare_strings(a.source_text.as_ustr(), b.source_text.as_ustr());
    if cmp_source_text != 0 { return cmp_source_text; }
    let cmp_break_after_cluster_index = if a.break_after_cluster_index < b.break_after_cluster_index { -1 } else if a.break_after_cluster_index > b.break_after_cluster_index { 1 } else { 0 };
    if cmp_break_after_cluster_index != 0 { return cmp_break_after_cluster_index; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_ustr(), b.reason.as_ustr());
    if cmp_reason != 0 { return cmp_reason; }
    0
}
