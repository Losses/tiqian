use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct RoleOverrideInfo {
    pub range: TextRange,
    pub source_text: UString,
    pub original_role: UString,
    pub overridden_role: UString,
    pub source: UString,
    pub reason: UString,
}

impl RoleOverrideInfo {
    pub fn new(range: TextRange, source_text: &UStr, original_role: &UStr, overridden_role: &UStr, source: &UStr, reason: &UStr) -> Self {
        Self {
            range,
            source_text: source_text.to_ustring(),
            original_role: original_role.to_ustring(),
            overridden_role: overridden_role.to_ustring(),
            source: source.to_ustring(),
            reason: reason.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("RoleOverrideInfo(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("sourceText=")); __s += (self.source_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("originalRole=")); __s += (self.original_role).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("overriddenRole=")); __s += (self.overridden_role).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("source=")); __s += (self.source).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

pub fn compare_role_override_info(a: &RoleOverrideInfo, b: &RoleOverrideInfo) -> i32 {
    let cmp_range = compare_text_range(&a.range, &b.range);
    if cmp_range != 0 { return cmp_range; }
    let cmp_source_text = SortedTable::sorted_table_compare_strings(a.source_text.as_ustr(), b.source_text.as_ustr());
    if cmp_source_text != 0 { return cmp_source_text; }
    let cmp_original_role = SortedTable::sorted_table_compare_strings(a.original_role.as_ustr(), b.original_role.as_ustr());
    if cmp_original_role != 0 { return cmp_original_role; }
    let cmp_overridden_role = SortedTable::sorted_table_compare_strings(a.overridden_role.as_ustr(), b.overridden_role.as_ustr());
    if cmp_overridden_role != 0 { return cmp_overridden_role; }
    let cmp_source = SortedTable::sorted_table_compare_strings(a.source.as_ustr(), b.source.as_ustr());
    if cmp_source != 0 { return cmp_source; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_ustr(), b.reason.as_ustr());
    if cmp_reason != 0 { return cmp_reason; }
    0
}
