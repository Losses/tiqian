use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct Cluster {
    pub range: TextRange,
    pub text: UString,
    pub display_text: UString,
    pub font_key: UString,
    pub advance: f64,
    pub baseline_shift: f64,
    pub leading_layout_advance: f64,
    pub glyph_inline_shift: f64,
}

impl Cluster {
    pub fn new(range: TextRange, text: &UStr, font_key: &UStr, advance: f64, display_text: Option<UString>, baseline_shift: Option<f64>, leading_layout_advance: Option<f64>, glyph_inline_shift: Option<f64>) -> Self {
        let display_text = display_text.unwrap_or_else(|| text.to_ustring());
        let baseline_shift = baseline_shift.unwrap_or_else(|| 0.0);
        let leading_layout_advance = leading_layout_advance.unwrap_or_else(|| 0.0);
        let glyph_inline_shift = glyph_inline_shift.unwrap_or_else(|| 0.0);
        Self {
            range,
            text: text.to_ustring(),
            font_key: font_key.to_ustring(),
            advance,
            display_text: display_text,
            baseline_shift: baseline_shift,
            leading_layout_advance: leading_layout_advance,
            glyph_inline_shift: glyph_inline_shift,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Cluster(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("text=")); __s += (self.text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("displayText=")); __s += (self.display_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("fontKey=")); __s += (self.font_key).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("advance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.advance)); __s += &(UString::from(", ")); __s += &(UString::from("baselineShift=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.baseline_shift)); __s += &(UString::from(", ")); __s += &(UString::from("leadingLayoutAdvance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.leading_layout_advance)); __s += &(UString::from(", ")); __s += &(UString::from("glyphInlineShift=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.glyph_inline_shift)); __s += &(UString::from(")")); __s }).as_str());
    }
}
