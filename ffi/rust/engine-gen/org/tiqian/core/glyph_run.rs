use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::text_range::TextRange;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct GlyphRun {
    pub range: TextRange,
    pub font_key: String,
    pub glyphs: Vec<Glyph>,
    pub advance: f64,
    pub open_type_features: Vec<String>,
}

impl GlyphRun {
    pub fn new(range: TextRange, font_key: &str, glyphs: Vec<Glyph>, advance: f64, open_type_features: Option<Vec<String>>) -> Self {
        let open_type_features = open_type_features.unwrap_or_else(|| vec![]);
        Self {
            range,
            font_key: font_key.to_string(),
            glyphs,
            advance,
            open_type_features: open_type_features,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "GlyphRun(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "fontKey=",
            (self.font_key).to_string(),
            ", ",
            "glyphs=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.glyphs).clone();
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
            "advance=",
            self.advance,
            ", ",
            "openTypeFeatures=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.open_type_features).clone();
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
            ")"
        );
    }
}
