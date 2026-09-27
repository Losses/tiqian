use crate::org::tiqian::core::text_range::TextRange;


#[derive(Debug, Clone, PartialEq)]
pub struct Cluster {
    pub range: TextRange,
    pub text: String,
    pub display_text: String,
    pub font_key: String,
    pub advance: f64,
    pub baseline_shift: f64,
    pub leading_layout_advance: f64,
    pub glyph_inline_shift: f64,
}

impl Cluster {
    pub fn new(range: TextRange, text: &str, font_key: &str, advance: f64, display_text: Option<String>, baseline_shift: Option<f64>, leading_layout_advance: Option<f64>, glyph_inline_shift: Option<f64>) -> Self {
        let display_text = display_text.unwrap_or_else(|| text.to_string());
        let baseline_shift = baseline_shift.unwrap_or_else(|| 0.0);
        let leading_layout_advance = leading_layout_advance.unwrap_or_else(|| 0.0);
        let glyph_inline_shift = glyph_inline_shift.unwrap_or_else(|| 0.0);
        Self {
            range,
            text: text.to_string(),
            font_key: font_key.to_string(),
            advance,
            display_text: display_text,
            baseline_shift: baseline_shift,
            leading_layout_advance: leading_layout_advance,
            glyph_inline_shift: glyph_inline_shift,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "Cluster(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "text=",
            (self.text).to_string(),
            ", ",
            "displayText=",
            (self.display_text).to_string(),
            ", ",
            "fontKey=",
            (self.font_key).to_string(),
            ", ",
            "advance=",
            self.advance,
            ", ",
            "baselineShift=",
            self.baseline_shift,
            ", ",
            "leadingLayoutAdvance=",
            self.leading_layout_advance,
            ", ",
            "glyphInlineShift=",
            self.glyph_inline_shift,
            ")"
        );
    }
}
