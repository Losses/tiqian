use crate::org::tiqian::core::bopomofo_glyph_role::BopomofoGlyphRole;
use crate::org::tiqian::core::glyph::Glyph;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct BopomofoGlyphPlacement {
    pub text: String,
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
    pub fn new(text: &str, left: f64, top: f64, width: f64, height: f64, role: BopomofoGlyphRole, glyphs: Option<Vec<Glyph>>, draw_x: f64, baseline_y: f64, font_size: f64) -> Self {
        let glyphs = glyphs.unwrap_or_else(|| vec![]);
        Self {
            text: text.to_string(),
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

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "BopomofoGlyphPlacement(",
            "text=",
            (self.text).to_string(),
            ", ",
            "left=",
            self.left,
            ", ",
            "top=",
            self.top,
            ", ",
            "width=",
            self.width,
            ", ",
            "height=",
            self.height,
            ", ",
            "role=",
            self.role.name(),
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
            "drawX=",
            self.draw_x,
            ", ",
            "baselineY=",
            self.baseline_y,
            ", ",
            "fontSize=",
            self.font_size,
            ")"
        );
    }
}
