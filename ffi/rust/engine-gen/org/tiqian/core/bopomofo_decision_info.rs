use crate::org::tiqian::core::bopomofo_glyph_placement::BopomofoGlyphPlacement;
use crate::org::tiqian::core::text_range::TextRange;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct BopomofoDecisionInfo {
    pub base_range: TextRange,
    pub text: String,
    pub line_index: u32,
    pub placements: Vec<BopomofoGlyphPlacement>,
    pub font_families: Vec<String>,
    pub font_weight: u32,
    pub locale: String,
}

impl BopomofoDecisionInfo {
    pub fn new(base_range: TextRange, text: &str, line_index: u32, placements: Vec<BopomofoGlyphPlacement>, font_families: Option<Vec<String>>, font_weight: Option<u32>, locale: Option<String>) -> Self {
        let font_families = font_families.unwrap_or_else(|| vec![]);
        let font_weight = font_weight.unwrap_or_else(|| 400);
        let locale = locale.unwrap_or_else(|| "zh-Hans".to_string());
        Self {
            base_range,
            text: text.to_string(),
            line_index,
            placements,
            font_families: font_families,
            font_weight: font_weight,
            locale: locale,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "BopomofoDecisionInfo(",
            "baseRange=",
            (self.base_range).clone().to_string(),
            ", ",
            "text=",
            (self.text).to_string(),
            ", ",
            "lineIndex=",
            crate::runtime::int_text::IntText::int_text(self.line_index),
            ", ",
            "placements=",
            {
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
    },
            ", ",
            "fontFamilies=",
            {
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
    },
            ", ",
            "fontWeight=",
            crate::runtime::int_text::IntText::int_text(self.font_weight),
            ", ",
            "locale=",
            (self.locale).to_string(),
            ")"
        );
    }
}
