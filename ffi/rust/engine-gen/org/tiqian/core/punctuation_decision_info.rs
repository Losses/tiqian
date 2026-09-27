use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct PunctuationDecisionInfo {
    pub range: TextRange,
    pub char: UString,
    pub punctuation_class: UString,
    pub advance: f64,
    pub body_width: f64,
    pub leading_glue_natural: f64,
    pub trailing_glue_natural: f64,
    pub anchor: UString,
    pub ink_bounds: Option<Rect>,
    pub geometry_source: UString,
    pub policy_body_floor: f64,
    pub ink_width: Option<f64>,
    pub ink_center: Option<f64>,
    pub ink_containment_body_floor: Option<f64>,
    pub ink_containment_applied: bool,
    pub ink_bounds_fallback: Option<UString>,
    pub halt_advance: Option<f64>,
    pub halt_validation: Option<UString>,
    pub advance_expansion: f64,
    pub glyph_inline_shift: f64,
    pub glyph_placement_reason: Option<UString>,
    pub leading_glue_initially_consumed: f64,
    pub trailing_glue_initially_consumed: f64,
}

impl PunctuationDecisionInfo {
    pub fn new(range: TextRange, char: &UStr, punctuation_class: &UStr, advance: f64, body_width: f64, leading_glue_natural: f64, trailing_glue_natural: f64, anchor: &UStr, ink_bounds: Option<Rect>, geometry_source: Option<UString>, policy_body_floor: f64, ink_width: Option<f64>, ink_center: Option<f64>, ink_containment_body_floor: Option<f64>, ink_containment_applied: Option<bool>, ink_bounds_fallback: Option<UString>, halt_advance: Option<f64>, halt_validation: Option<UString>, advance_expansion: Option<f64>, glyph_inline_shift: Option<f64>, glyph_placement_reason: Option<UString>, leading_glue_initially_consumed: Option<f64>, trailing_glue_initially_consumed: Option<f64>) -> Self {
        let ink_bounds = ink_bounds.or_else(|| None);
        let geometry_source = geometry_source.unwrap_or_else(|| UString::from("PolicyDerived"));
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
            char: char.to_ustring(),
            punctuation_class: punctuation_class.to_ustring(),
            advance,
            body_width,
            leading_glue_natural,
            trailing_glue_natural,
            anchor: anchor.to_ustring(),
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

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("PunctuationDecisionInfo(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("char=")); __s += (self.char).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("punctuationClass=")); __s += (self.punctuation_class).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("advance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.advance)); __s += &(UString::from(", ")); __s += &(UString::from("bodyWidth=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.body_width)); __s += &(UString::from(", ")); __s += &(UString::from("leadingGlueNatural=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.leading_glue_natural)); __s += &(UString::from(", ")); __s += &(UString::from("trailingGlueNatural=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.trailing_glue_natural)); __s += &(UString::from(", ")); __s += &(UString::from("anchor=")); __s += (self.anchor).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("inkBounds=")); __s += (match &(self.ink_bounds) { None => UString::from("null"), Some(__option) => UString::from(format!("{}", __option.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("geometrySource=")); __s += (self.geometry_source).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("policyBodyFloor=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.policy_body_floor)); __s += &(UString::from(", ")); __s += &(UString::from("inkWidth=")); __s += &(match self.ink_width { Some(v) => UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(v)).as_str()), None => UString::from("null") }); __s += &(UString::from(", ")); __s += &(UString::from("inkCenter=")); __s += &(match self.ink_center { Some(v) => UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(v)).as_str()), None => UString::from("null") }); __s += &(UString::from(", ")); __s += &(UString::from("inkContainmentBodyFloor=")); __s += &(match self.ink_containment_body_floor { Some(v) => UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(v)).as_str()), None => UString::from("null") }); __s += &(UString::from(", ")); __s += &(UString::from("inkContainmentApplied=")); __s += UString::from(format!("{}", (self.ink_containment_applied).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("inkBoundsFallback=")); __s += match &((self.ink_bounds_fallback).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("haltAdvance=")); __s += &(match self.halt_advance { Some(v) => UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(v)).as_str()), None => UString::from("null") }); __s += &(UString::from(", ")); __s += &(UString::from("haltValidation=")); __s += match &((self.halt_validation).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("advanceExpansion=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.advance_expansion)); __s += &(UString::from(", ")); __s += &(UString::from("glyphInlineShift=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.glyph_inline_shift)); __s += &(UString::from(", ")); __s += &(UString::from("glyphPlacementReason=")); __s += match &((self.glyph_placement_reason).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("leadingGlueInitiallyConsumed=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.leading_glue_initially_consumed)); __s += &(UString::from(", ")); __s += &(UString::from("trailingGlueInitiallyConsumed=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.trailing_glue_initially_consumed)); __s += &(UString::from(")")); __s }).as_str());
    }
}
