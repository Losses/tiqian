use crate::org::tiqian::core::bopomofo_glyph_role::BopomofoGlyphRole;
use crate::org::tiqian::core::glyph::Glyph;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct BopomofoGlyphPlacement {
    pub text: UString,
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
    pub role: BopomofoGlyphRole,
    pub glyphs: Vec<Glyph>,
    pub draw_x: f64,
    pub baseline_y: f64,
    pub font_size: f64,
}

impl BopomofoGlyphPlacement {
    pub fn new(text: &UStr, left: f64, top: f64, width: f64, height: f64, role: BopomofoGlyphRole, glyphs: Option<Vec<Glyph>>, draw_x: f64, baseline_y: f64, font_size: f64) -> Self {
        let glyphs = glyphs.unwrap_or_else(|| vec![]);
        Self {
            text: text.to_ustring(),
            left,
            top,
            width,
            height,
            role,
            glyphs: glyphs,
            draw_x,
            baseline_y,
            font_size,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("BopomofoGlyphPlacement(")); __s += &(UString::from("text=")); __s += (self.text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("left=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.left)); __s += &(UString::from(", ")); __s += &(UString::from("top=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.top)); __s += &(UString::from(", ")); __s += &(UString::from("width=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.width)); __s += &(UString::from(", ")); __s += &(UString::from("height=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.height)); __s += &(UString::from(", ")); __s += &(UString::from("role=")); __s += UString::from(self.role.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("glyphs=")); __s += UString::from(format!("{}", {
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
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("drawX=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.draw_x)); __s += &(UString::from(", ")); __s += &(UString::from("baselineY=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.baseline_y)); __s += &(UString::from(", ")); __s += &(UString::from("fontSize=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.font_size)); __s += &(UString::from(")")); __s }).as_str());
    }
}
