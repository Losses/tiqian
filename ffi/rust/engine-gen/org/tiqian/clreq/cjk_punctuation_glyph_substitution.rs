use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct CjkPunctuationGlyphSubstitution {
    pub source_text: UString,
    pub display_text: UString,
    pub reason: UString,
}

impl CjkPunctuationGlyphSubstitution {
    pub fn new(source_text: &UStr, display_text: &UStr, reason: &UStr) -> Self {
        Self {
            source_text: source_text.to_ustring(),
            display_text: display_text.to_ustring(),
            reason: reason.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("CjkPunctuationGlyphSubstitution(")); __s += &(UString::from("sourceText=")); __s += (self.source_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("displayText=")); __s += (self.display_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

pub fn compare_cjk_punctuation_glyph_substitution(a: &CjkPunctuationGlyphSubstitution, b: &CjkPunctuationGlyphSubstitution) -> i32 {
    let cmp_source_text = SortedTable::sorted_table_compare_strings(a.source_text.as_ustr(), b.source_text.as_ustr());
    if cmp_source_text != 0 { return cmp_source_text; }
    let cmp_display_text = SortedTable::sorted_table_compare_strings(a.display_text.as_ustr(), b.display_text.as_ustr());
    if cmp_display_text != 0 { return cmp_display_text; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_ustr(), b.reason.as_ustr());
    if cmp_reason != 0 { return cmp_reason; }
    0
}
