use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::runtime::sorted_table::SortedTable;


#[derive(Debug, Clone, PartialEq)]
pub struct FontDecisionInfo {
    pub range: TextRange,
    pub source_text: String,
    pub display_text: String,
    pub role: String,
    pub font_key: String,
    pub reason: String,
    pub substitution_reason: String,
}

impl FontDecisionInfo {
    pub fn new(range: TextRange, source_text: &str, display_text: &str, role: &str, font_key: &str, reason: &str, substitution_reason: &str) -> Self {
        Self {
            range,
            source_text: source_text.to_string(),
            display_text: display_text.to_string(),
            role: role.to_string(),
            font_key: font_key.to_string(),
            reason: reason.to_string(),
            substitution_reason: substitution_reason.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "FontDecisionInfo(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "sourceText=",
            (self.source_text).to_string(),
            ", ",
            "displayText=",
            (self.display_text).to_string(),
            ", ",
            "role=",
            (self.role).to_string(),
            ", ",
            "fontKey=",
            (self.font_key).to_string(),
            ", ",
            "reason=",
            (self.reason).to_string(),
            ", ",
            "substitutionReason=",
            (self.substitution_reason).to_string(),
            ")"
        );
    }
}

pub fn compare_font_decision_info(a: &FontDecisionInfo, b: &FontDecisionInfo) -> i32 {
    let cmp_range = compare_text_range(&a.range, &b.range);
    if cmp_range != 0 { return cmp_range; }
    let cmp_source_text = SortedTable::sorted_table_compare_strings(a.source_text.as_str(), b.source_text.as_str());
    if cmp_source_text != 0 { return cmp_source_text; }
    let cmp_display_text = SortedTable::sorted_table_compare_strings(a.display_text.as_str(), b.display_text.as_str());
    if cmp_display_text != 0 { return cmp_display_text; }
    let cmp_role = SortedTable::sorted_table_compare_strings(a.role.as_str(), b.role.as_str());
    if cmp_role != 0 { return cmp_role; }
    let cmp_font_key = SortedTable::sorted_table_compare_strings(a.font_key.as_str(), b.font_key.as_str());
    if cmp_font_key != 0 { return cmp_font_key; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_str(), b.reason.as_str());
    if cmp_reason != 0 { return cmp_reason; }
    let cmp_substitution_reason = SortedTable::sorted_table_compare_strings(a.substitution_reason.as_str(), b.substitution_reason.as_str());
    if cmp_substitution_reason != 0 { return cmp_substitution_reason; }
    0
}
