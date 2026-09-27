use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::text_range::TextRange;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct RubyDecisionInfo {
    pub base_range: TextRange,
    pub text: String,
    pub line_index: u32,
    pub center_x: f64,
    pub baseline_y: f64,
    pub font_size: f64,
    pub ascent: f64,
    pub descent: f64,
    pub width: f64,
    pub overhang: f64,
    pub font_families: Vec<String>,
    pub font_weight: u32,
    pub locale: String,
    pub glyphs: Vec<Glyph>,
}

impl RubyDecisionInfo {
    pub fn new(base_range: TextRange, text: &str, line_index: u32, center_x: f64, baseline_y: f64, font_size: f64, overhang: f64, ascent: Option<f64>, descent: Option<f64>, width: Option<f64>, font_families: Option<Vec<String>>, font_weight: Option<u32>, locale: Option<String>,
glyphs: Option<Vec<Glyph>>) -> Self {
        let ascent = ascent.unwrap_or_else(|| 0.0);
        let descent = descent.unwrap_or_else(|| 0.0);
        let width = width.unwrap_or_else(|| 0.0);
        let font_families = font_families.unwrap_or_else(|| vec![]);
        let font_weight = font_weight.unwrap_or_else(|| 400);
        let locale = locale.unwrap_or_else(|| "zh-Hans".to_string());
        let glyphs = glyphs.unwrap_or_else(|| vec![]);
        Self {
            base_range,
            text: text.to_string(),
            line_index,
            center_x,
            baseline_y,
            font_size,
            overhang,
            ascent: ascent,
            descent: descent,
            width: width,
            font_families: font_families,
            font_weight: font_weight,
            locale: locale,
            glyphs: glyphs,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "RubyDecisionInfo(",
            "baseRange=",
            (self.base_range).clone().to_string(),
            ", ",
            "text=",
            (self.text).to_string(),
            ", ",
            "lineIndex=",
            crate::runtime::int_text::IntText::int_text(self.line_index),
            ", ",
            "centerX=",
            self.center_x,
            ", ",
            "baselineY=",
            self.baseline_y,
            ", ",
            "fontSize=",
            self.font_size,
            ", ",
            "ascent=",
            self.ascent,
            ", ",
            "descent=",
            self.descent,
            ", ",
            "width=",
            self.width,
            ", ",
            "overhang=",
            self.overhang,
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
            ")"
        );
    }
}
