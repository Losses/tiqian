use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct FontDecisionInfo {
    pub range: TextRange,
    pub source_text: UString,
    pub display_text: UString,
    pub role: UString,
    pub font_key: UString,
    pub reason: UString,
    pub substitution_reason: UString,
}

impl FontDecisionInfo {
    pub fn new(range: TextRange, source_text: &UStr, display_text: &UStr, role: &UStr, font_key: &UStr, reason: &UStr, substitution_reason: &UStr) -> Self {
        Self {
            range,
            source_text: source_text.to_ustring(),
            display_text: display_text.to_ustring(),
            role: role.to_ustring(),
            font_key: font_key.to_ustring(),
            reason: reason.to_ustring(),
            substitution_reason: substitution_reason.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("FontDecisionInfo(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("sourceText=")); __s += (self.source_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("displayText=")); __s += (self.display_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("role=")); __s += (self.role).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("fontKey=")); __s += (self.font_key).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("substitutionReason=")); __s += (self.substitution_reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

pub fn compare_font_decision_info(a: &FontDecisionInfo, b: &FontDecisionInfo) -> i32 {
    let cmp_range = compare_text_range(&a.range, &b.range);
    if cmp_range != 0 { return cmp_range; }
    let cmp_source_text = SortedTable::sorted_table_compare_strings(a.source_text.as_ustr(), b.source_text.as_ustr());
    if cmp_source_text != 0 { return cmp_source_text; }
    let cmp_display_text = SortedTable::sorted_table_compare_strings(a.display_text.as_ustr(), b.display_text.as_ustr());
    if cmp_display_text != 0 { return cmp_display_text; }
    let cmp_role = SortedTable::sorted_table_compare_strings(a.role.as_ustr(), b.role.as_ustr());
    if cmp_role != 0 { return cmp_role; }
    let cmp_font_key = SortedTable::sorted_table_compare_strings(a.font_key.as_ustr(), b.font_key.as_ustr());
    if cmp_font_key != 0 { return cmp_font_key; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_ustr(), b.reason.as_ustr());
    if cmp_reason != 0 { return cmp_reason; }
    let cmp_substitution_reason = SortedTable::sorted_table_compare_strings(a.substitution_reason.as_ustr(), b.substitution_reason.as_ustr());
    if cmp_substitution_reason != 0 { return cmp_substitution_reason; }
    0
}
