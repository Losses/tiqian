use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct RubyDecisionInfo {
    pub base_range: TextRange,
    pub text: UString,
    pub line_index: u32,
    pub center_x: f64,
    pub baseline_y: f64,
    pub font_size: f64,
    pub ascent: f64,
    pub descent: f64,
    pub width: f64,
    pub overhang: f64,
    pub font_families: Vec<UString>,
    pub font_weight: u32,
    pub locale: UString,
    pub glyphs: Vec<Glyph>,
}

impl RubyDecisionInfo {
    pub fn new(base_range: TextRange, text: &UStr, line_index: u32, center_x: f64, baseline_y: f64, font_size: f64, overhang: f64, ascent: Option<f64>, descent: Option<f64>, width: Option<f64>, font_families: Option<Vec<UString>>, font_weight: Option<u32>, locale: Option<UString>, glyphs: Option<Vec<Glyph>>) -> Self {
        let ascent = ascent.unwrap_or_else(|| 0.0);
        let descent = descent.unwrap_or_else(|| 0.0);
        let width = width.unwrap_or_else(|| 0.0);
        let font_families = font_families.unwrap_or_else(|| vec![]);
        let font_weight = font_weight.unwrap_or_else(|| 400);
        let locale = locale.unwrap_or_else(|| UString::from("zh-Hans"));
        let glyphs = glyphs.unwrap_or_else(|| vec![]);
        Self {
            base_range,
            text: text.to_ustring(),
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

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("RubyDecisionInfo(")); __s += &(UString::from("baseRange=")); __s += UString::from(format!("{}", (self.base_range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("text=")); __s += (self.text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineIndex=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.line_index)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("centerX=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.center_x)); __s += &(UString::from(", ")); __s += &(UString::from("baselineY=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.baseline_y)); __s += &(UString::from(", ")); __s += &(UString::from("fontSize=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.font_size)); __s += &(UString::from(", ")); __s += &(UString::from("ascent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.ascent)); __s += &(UString::from(", ")); __s += &(UString::from("descent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.descent)); __s += &(UString::from(", ")); __s += &(UString::from("width=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.width)); __s += &(UString::from(", ")); __s += &(UString::from("overhang=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.overhang)); __s += &(UString::from(", ")); __s += &(UString::from("fontFamilies=")); __s += UString::from(format!("{}", {
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
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("fontWeight=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.font_weight)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("locale=")); __s += (self.locale).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("glyphs=")); __s += UString::from(format!("{}", {
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
    }).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
