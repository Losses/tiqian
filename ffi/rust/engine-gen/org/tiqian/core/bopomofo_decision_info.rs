use crate::org::tiqian::core::bopomofo_glyph_placement::BopomofoGlyphPlacement;
use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct BopomofoDecisionInfo {
    pub base_range: TextRange,
    pub text: UString,
    pub line_index: u32,
    pub placements: Vec<BopomofoGlyphPlacement>,
    pub font_families: Vec<UString>,
    pub font_weight: u32,
    pub locale: UString,
}

impl BopomofoDecisionInfo {
    pub fn new(base_range: TextRange, text: &UStr, line_index: u32, placements: Vec<BopomofoGlyphPlacement>, font_families: Option<Vec<UString>>, font_weight: Option<u32>, locale: Option<UString>) -> Self {
        let font_families = font_families.unwrap_or_else(|| vec![]);
        let font_weight = font_weight.unwrap_or_else(|| 400);
        let locale = locale.unwrap_or_else(|| UString::from("zh-Hans"));
        Self {
            base_range,
            text: text.to_ustring(),
            line_index,
            placements,
            font_families: font_families,
            font_weight: font_weight,
            locale: locale,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("BopomofoDecisionInfo(")); __s += &(UString::from("baseRange=")); __s += UString::from(format!("{}", (self.base_range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("text=")); __s += (self.text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineIndex=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.line_index)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("placements=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.placements).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("fontFamilies=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.font_families).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("fontWeight=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.font_weight)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("locale=")); __s += (self.locale).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
