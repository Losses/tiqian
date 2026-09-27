use crate::org::tiqian::core::text_range::TextRange;


#[derive(Debug, Clone, PartialEq)]
pub struct ClusterGeometryDecisionInfo {
    pub range: TextRange,
    pub source_text: String,
    pub display_text: String,
    pub base_advance: f64,
    pub body_width: f64,
    pub leading_glue_natural: f64,
    pub leading_glue_consumed: f64,
    pub trailing_glue_natural: f64,
    pub trailing_glue_consumed: f64,
    pub justification_delta: f64,
    pub ruby_spread: f64,
    pub glyph_inline_shift: f64,
    pub glyph_placement_reason: Option<String>,
    pub resolved_advance: f64,
    pub source: String,
    pub reason: String,
}

impl ClusterGeometryDecisionInfo {
    pub fn new(range: TextRange, source_text: &str, display_text: &str, base_advance: f64, body_width: f64, leading_glue_natural: f64, leading_glue_consumed: f64, trailing_glue_natural: f64, trailing_glue_consumed: f64, justification_delta: f64, resolved_advance: f64, source:
&str, reason: &str, ruby_spread: Option<f64>, glyph_inline_shift: Option<f64>, glyph_placement_reason: Option<String>) -> Self {
        let ruby_spread = ruby_spread.unwrap_or_else(|| 0.0);
        let glyph_inline_shift = glyph_inline_shift.unwrap_or_else(|| 0.0);
        let glyph_placement_reason = glyph_placement_reason.or_else(|| None);
        Self {
            range,
            source_text: source_text.to_string(),
            display_text: display_text.to_string(),
            base_advance,
            body_width,
            leading_glue_natural,
            leading_glue_consumed,
            trailing_glue_natural,
            trailing_glue_consumed,
            justification_delta,
            resolved_advance,
            source: source.to_string(),
            reason: reason.to_string(),
            ruby_spread: ruby_spread,
            glyph_inline_shift: glyph_inline_shift,
            glyph_placement_reason: glyph_placement_reason,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "ClusterGeometryDecisionInfo(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "sourceText=",
            (self.source_text).to_string(),
            ", ",
            "displayText=",
            (self.display_text).to_string(),
            ", ",
            "baseAdvance=",
            self.base_advance,
            ", ",
            "bodyWidth=",
            self.body_width,
            ", ",
            "leadingGlueNatural=",
            self.leading_glue_natural,
            ", ",
            "leadingGlueConsumed=",
            self.leading_glue_consumed,
            ", ",
            "trailingGlueNatural=",
            self.trailing_glue_natural,
            ", ",
            "trailingGlueConsumed=",
            self.trailing_glue_consumed,
            ", ",
            "justificationDelta=",
            self.justification_delta,
            ", ",
            "rubySpread=",
            self.ruby_spread,
            ", ",
            "glyphInlineShift=",
            self.glyph_inline_shift,
            ", ",
            "glyphPlacementReason=",
            match (self.glyph_placement_reason).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ", ",
            "resolvedAdvance=",
            self.resolved_advance,
            ", ",
            "source=",
            (self.source).to_string(),
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}
