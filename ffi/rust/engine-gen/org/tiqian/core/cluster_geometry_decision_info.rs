use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct ClusterGeometryDecisionInfo {
    pub range: TextRange,
    pub source_text: UString,
    pub display_text: UString,
    pub base_advance: f64,
    pub body_width: f64,
    pub leading_glue_natural: f64,
    pub leading_glue_consumed: f64,
    pub trailing_glue_natural: f64,
    pub trailing_glue_consumed: f64,
    pub justification_delta: f64,
    pub ruby_spread: f64,
    pub glyph_inline_shift: f64,
    pub glyph_placement_reason: Option<UString>,
    pub resolved_advance: f64,
    pub source: UString,
    pub reason: UString,
}

impl ClusterGeometryDecisionInfo {
    pub fn new(range: TextRange, source_text: &UStr, display_text: &UStr, base_advance: f64, body_width: f64, leading_glue_natural: f64, leading_glue_consumed: f64, trailing_glue_natural: f64, trailing_glue_consumed: f64, justification_delta: f64, resolved_advance: f64, source: &UStr, reason: &UStr, ruby_spread: Option<f64>, glyph_inline_shift: Option<f64>, glyph_placement_reason: Option<UString>) -> Self {
        let ruby_spread = ruby_spread.unwrap_or_else(|| 0.0);
        let glyph_inline_shift = glyph_inline_shift.unwrap_or_else(|| 0.0);
        let glyph_placement_reason = glyph_placement_reason.or_else(|| None);
        Self {
            range,
            source_text: source_text.to_ustring(),
            display_text: display_text.to_ustring(),
            base_advance,
            body_width,
            leading_glue_natural,
            leading_glue_consumed,
            trailing_glue_natural,
            trailing_glue_consumed,
            justification_delta,
            resolved_advance,
            source: source.to_ustring(),
            reason: reason.to_ustring(),
            ruby_spread: ruby_spread,
            glyph_inline_shift: glyph_inline_shift,
            glyph_placement_reason: glyph_placement_reason,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ClusterGeometryDecisionInfo(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("sourceText=")); __s += (self.source_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("displayText=")); __s += (self.display_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("baseAdvance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.base_advance)); __s += &(UString::from(", ")); __s += &(UString::from("bodyWidth=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.body_width)); __s += &(UString::from(", ")); __s += &(UString::from("leadingGlueNatural=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.leading_glue_natural)); __s += &(UString::from(", ")); __s += &(UString::from("leadingGlueConsumed=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.leading_glue_consumed)); __s += &(UString::from(", ")); __s += &(UString::from("trailingGlueNatural=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.trailing_glue_natural)); __s += &(UString::from(", ")); __s += &(UString::from("trailingGlueConsumed=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.trailing_glue_consumed)); __s += &(UString::from(", ")); __s += &(UString::from("justificationDelta=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.justification_delta)); __s += &(UString::from(", ")); __s += &(UString::from("rubySpread=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.ruby_spread)); __s += &(UString::from(", ")); __s += &(UString::from("glyphInlineShift=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.glyph_inline_shift)); __s += &(UString::from(", ")); __s += &(UString::from("glyphPlacementReason=")); __s += match &((self.glyph_placement_reason).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("resolvedAdvance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.resolved_advance)); __s += &(UString::from(", ")); __s += &(UString::from("source=")); __s += (self.source).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
