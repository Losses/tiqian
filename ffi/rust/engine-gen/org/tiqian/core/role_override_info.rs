use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::runtime::sorted_table::SortedTable;


#[derive(Debug, Clone, PartialEq)]
pub struct RoleOverrideInfo {
    pub range: TextRange,
    pub source_text: String,
    pub original_role: String,
    pub overridden_role: String,
    pub source: String,
    pub reason: String,
}

impl RoleOverrideInfo {
    pub fn new(range: TextRange, source_text: &str, original_role: &str, overridden_role: &str, source: &str, reason: &str) -> Self {
        Self {
            range,
            source_text: source_text.to_string(),
            original_role: original_role.to_string(),
            overridden_role: overridden_role.to_string(),
            source: source.to_string(),
            reason: reason.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "RoleOverrideInfo(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "sourceText=",
            (self.source_text).to_string(),
            ", ",
            "originalRole=",
            (self.original_role).to_string(),
            ", ",
            "overriddenRole=",
            (self.overridden_role).to_string(),
            ", ",
            "source=",
            (self.source).to_string(),
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}

pub fn compare_role_override_info(a: &RoleOverrideInfo, b: &RoleOverrideInfo) -> i32 {
    let cmp_range = compare_text_range(&a.range, &b.range);
    if cmp_range != 0 { return cmp_range; }
    let cmp_source_text = SortedTable::sorted_table_compare_strings(a.source_text.as_str(), b.source_text.as_str());
    if cmp_source_text != 0 { return cmp_source_text; }
    let cmp_original_role = SortedTable::sorted_table_compare_strings(a.original_role.as_str(), b.original_role.as_str());
    if cmp_original_role != 0 { return cmp_original_role; }
    let cmp_overridden_role = SortedTable::sorted_table_compare_strings(a.overridden_role.as_str(), b.overridden_role.as_str());
    if cmp_overridden_role != 0 { return cmp_overridden_role; }
    let cmp_source = SortedTable::sorted_table_compare_strings(a.source.as_str(), b.source.as_str());
    if cmp_source != 0 { return cmp_source; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_str(), b.reason.as_str());
    if cmp_reason != 0 { return cmp_reason; }
    0
}
