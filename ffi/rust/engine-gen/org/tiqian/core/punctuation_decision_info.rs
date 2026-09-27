use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::text_range::TextRange;


#[derive(Debug, Clone, PartialEq)]
pub struct PunctuationDecisionInfo {
    pub range: TextRange,
    pub char: String,
    pub punctuation_class: String,
    pub advance: f64,
    pub body_width: f64,
    pub leading_glue_natural: f64,
    pub trailing_glue_natural: f64,
    pub anchor: String,
    pub ink_bounds: Option<Rect>,
    pub geometry_source: String,
    pub policy_body_floor: f64,
    pub ink_width: Option<f64>,
    pub ink_center: Option<f64>,
    pub ink_containment_body_floor: Option<f64>,
    pub ink_containment_applied: bool,
    pub ink_bounds_fallback: Option<String>,
    pub halt_advance: Option<f64>,
    pub halt_validation: Option<String>,
    pub advance_expansion: f64,
    pub glyph_inline_shift: f64,
    pub glyph_placement_reason: Option<String>,
    pub leading_glue_initially_consumed: f64,
    pub trailing_glue_initially_consumed: f64,
}

impl PunctuationDecisionInfo {
    pub fn new(range: TextRange, char: &str, punctuation_class: &str, advance: f64, body_width: f64, leading_glue_natural: f64, trailing_glue_natural: f64, anchor: &str, ink_bounds: Option<Rect>, geometry_source: Option<String>, policy_body_floor: f64, ink_width: Option<f64>,
ink_center: Option<f64>, ink_containment_body_floor: Option<f64>, ink_containment_applied: Option<bool>, ink_bounds_fallback: Option<String>, halt_advance: Option<f64>, halt_validation: Option<String>, advance_expansion: Option<f64>, glyph_inline_shift: Option<f64>,
glyph_placement_reason: Option<String>, leading_glue_initially_consumed: Option<f64>, trailing_glue_initially_consumed: Option<f64>) -> Self {
        let ink_bounds = ink_bounds.or_else(|| None);
        let geometry_source = geometry_source.unwrap_or_else(|| "PolicyDerived".to_string());
        let ink_width = ink_width.or_else(|| None);
        let ink_center = ink_center.or_else(|| None);
        let ink_containment_body_floor = ink_containment_body_floor.or_else(|| None);
        let ink_containment_applied = ink_containment_applied.unwrap_or_else(|| false);
        let ink_bounds_fallback = ink_bounds_fallback.or_else(|| None);
        let halt_advance = halt_advance.or_else(|| None);
        let halt_validation = halt_validation.or_else(|| None);
        let advance_expansion = advance_expansion.unwrap_or_else(|| 0.0);
        let glyph_inline_shift = glyph_inline_shift.unwrap_or_else(|| 0.0);
        let glyph_placement_reason = glyph_placement_reason.or_else(|| None);
        let leading_glue_initially_consumed = leading_glue_initially_consumed.unwrap_or_else(|| 0.0);
        let trailing_glue_initially_consumed = trailing_glue_initially_consumed.unwrap_or_else(|| 0.0);
        Self {
            range,
            char: char.to_string(),
            punctuation_class: punctuation_class.to_string(),
            advance,
            body_width,
            leading_glue_natural,
            trailing_glue_natural,
            anchor: anchor.to_string(),
            ink_bounds: ink_bounds,
            geometry_source: geometry_source,
            policy_body_floor,
            ink_width: ink_width,
            ink_center: ink_center,
            ink_containment_body_floor: ink_containment_body_floor,
            ink_containment_applied: ink_containment_applied,
            ink_bounds_fallback: ink_bounds_fallback,
            halt_advance: halt_advance,
            halt_validation: halt_validation,
            advance_expansion: advance_expansion,
            glyph_inline_shift: glyph_inline_shift,
            glyph_placement_reason: glyph_placement_reason,
            leading_glue_initially_consumed: leading_glue_initially_consumed,
            trailing_glue_initially_consumed: trailing_glue_initially_consumed,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "PunctuationDecisionInfo(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "char=",
            (self.char).to_string(),
            ", ",
            "punctuationClass=",
            (self.punctuation_class).to_string(),
            ", ",
            "advance=",
            self.advance,
            ", ",
            "bodyWidth=",
            self.body_width,
            ", ",
            "leadingGlueNatural=",
            self.leading_glue_natural,
            ", ",
            "trailingGlueNatural=",
            self.trailing_glue_natural,
            ", ",
            "anchor=",
            (self.anchor).to_string(),
            ", ",
            "inkBounds=",
            (match &(self.ink_bounds) { None => "null".to_string(), Some(__option) => __option.to_string() }),
            ", ",
            "geometrySource=",
            (self.geometry_source).to_string(),
            ", ",
            "policyBodyFloor=",
            self.policy_body_floor,
            ", ",
            "inkWidth=",
            match self.ink_width { Some(v) => crate::runtime::fp_helper::FPHelper::format_float(v), None => "null".to_string() },
            ", ",
            "inkCenter=",
            match self.ink_center { Some(v) => crate::runtime::fp_helper::FPHelper::format_float(v), None => "null".to_string() },
            ", ",
            "inkContainmentBodyFloor=",
            match self.ink_containment_body_floor { Some(v) => crate::runtime::fp_helper::FPHelper::format_float(v), None => "null".to_string() },
            ", ",
            "inkContainmentApplied=",
            self.ink_containment_applied,
            ", ",
            "inkBoundsFallback=",
            match (self.ink_bounds_fallback).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ", ",
            "haltAdvance=",
            match self.halt_advance { Some(v) => crate::runtime::fp_helper::FPHelper::format_float(v), None => "null".to_string() },
            ", ",
            "haltValidation=",
            match (self.halt_validation).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ", ",
            "advanceExpansion=",
            self.advance_expansion,
            ", ",
            "glyphInlineShift=",
            self.glyph_inline_shift,
            ", ",
            "glyphPlacementReason=",
            match (self.glyph_placement_reason).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ", ",
            "leadingGlueInitiallyConsumed=",
            self.leading_glue_initially_consumed,
            ", ",
            "trailingGlueInitiallyConsumed=",
            self.trailing_glue_initially_consumed,
            ")"
        );
    }
}
