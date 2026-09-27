use crate::runtime::sorted_table::SortedTable;


#[derive(Debug, Clone, PartialEq)]
pub struct CjkPunctuationGlyphSubstitution {
    pub source_text: String,
    pub display_text: String,
    pub reason: String,
}

impl CjkPunctuationGlyphSubstitution {
    pub fn new(source_text: &str, display_text: &str, reason: &str) -> Self {
        Self {
            source_text: source_text.to_string(),
            display_text: display_text.to_string(),
            reason: reason.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "CjkPunctuationGlyphSubstitution(",
            "sourceText=",
            (self.source_text).to_string(),
            ", ",
            "displayText=",
            (self.display_text).to_string(),
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}

pub fn compare_cjk_punctuation_glyph_substitution(a: &CjkPunctuationGlyphSubstitution, b: &CjkPunctuationGlyphSubstitution) -> i32 {
    let cmp_source_text = SortedTable::sorted_table_compare_strings(a.source_text.as_str(), b.source_text.as_str());
    if cmp_source_text != 0 { return cmp_source_text; }
    let cmp_display_text = SortedTable::sorted_table_compare_strings(a.display_text.as_str(), b.display_text.as_str());
    if cmp_display_text != 0 { return cmp_display_text; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_str(), b.reason.as_str());
    if cmp_reason != 0 { return cmp_reason; }
    0
}
