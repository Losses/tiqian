use crate::org::tiqian::clreq::clreq_punctuation_policies::ClreqPunctuationPolicies;
use crate::org::tiqian::clreq::glue_side::GlueSide;
use crate::org::tiqian::clreq::interior_punctuation_style::InteriorPunctuationStyle;
use crate::org::tiqian::clreq::punctuation_class::PunctuationClass;
use crate::org::tiqian::clreq::punctuation_glue_placement::PunctuationGluePlacement;
use crate::org::tiqian::clreq::punctuation_glue_placements::PunctuationGluePlacements;
use crate::org::tiqian::clreq::punctuation_width_policy::PunctuationWidthPolicy;
use crate::org::tiqian::core::accurate_sum::AccurateSum;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::runtime::u_string;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct PunctuationAtom {
    pub range: TextRange,
    pub char: String,
    pub punctuation_class: PunctuationClass,
    pub advance: f64,
    pub ink_bounds: Option<Rect>,
    pub body_width: f64,
    pub halt_advance: Option<f64>,
    pub halt_validation: Option<String>,
    pub leading_glue: Glue,
    pub trailing_glue: Glue,
    pub anchor: PunctuationAnchor,
    pub geometry_source: String,
    pub policy_body_floor: f64,
    pub ink_width: Option<f64>,
    pub ink_center: Option<f64>,
    pub ink_containment_body_floor: Option<f64>,
    pub ink_containment_applied: bool,
    pub ink_bounds_fallback: Option<String>,
    pub advance_expansion: f64,
    pub glyph_inline_shift: f64,
    pub glyph_placement_reason: Option<String>,
    pub leading_glue_initially_consumed: f64,
    pub trailing_glue_initially_consumed: f64,
}

impl PunctuationAtom {
    pub fn new(range: TextRange, char: &str, punctuation_class: PunctuationClass, advance: f64, ink_bounds: Option<Rect>, body_width: f64, halt_advance: Option<f64>, halt_validation: Option<String>, leading_glue: Glue, trailing_glue: Glue, anchor: PunctuationAnchor,
geometry_source: &str, policy_body_floor: f64, ink_width: Option<f64>, ink_center: Option<f64>, ink_containment_body_floor: Option<f64>, ink_containment_applied: bool, ink_bounds_fallback: Option<String>, advance_expansion: f64, glyph_inline_shift: f64, glyph_placement_reason:
Option<String>, leading_glue_initially_consumed: Option<f64>, trailing_glue_initially_consumed: Option<f64>) -> Result<Self, TextRangeError> {
        let leading_glue_initially_consumed = leading_glue_initially_consumed.unwrap_or_else(|| 0 as f64);
        let trailing_glue_initially_consumed = trailing_glue_initially_consumed.unwrap_or_else(|| 0 as f64);
        Ok(Self {
            range,
            char: char.to_string(),
            punctuation_class,
            advance,
            ink_bounds,
            body_width,
            halt_advance,
            halt_validation: match halt_validation { Some(v) => Some(v.to_string()), None => None },
            leading_glue,
            trailing_glue,
            anchor,
            geometry_source: geometry_source.to_string(),
            policy_body_floor,
            ink_width,
            ink_center,
            ink_containment_body_floor,
            ink_containment_applied,
            ink_bounds_fallback: match ink_bounds_fallback { Some(v) => Some(v.to_string()), None => None },
            advance_expansion,
            glyph_inline_shift,
            glyph_placement_reason: match glyph_placement_reason { Some(v) => Some(v.to_string()), None => None },
            leading_glue_initially_consumed: leading_glue_initially_consumed,
            trailing_glue_initially_consumed: trailing_glue_initially_consumed,
        })
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "PunctuationAtom(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "char=",
            (self.char).to_string(),
            ", ",
            "punctuationClass=",
            self.punctuation_class.name(),
            ", ",
            "advance=",
            self.advance,
            ", ",
            "inkBounds=",
            (match &(self.ink_bounds) { None => "null".to_string(), Some(__option) => __option.to_string() }),
            ", ",
            "bodyWidth=",
            self.body_width,
            ", ",
            "haltAdvance=",
            match self.halt_advance { Some(v) => crate::runtime::fp_helper::FPHelper::format_float(v), None => "null".to_string() },
            ", ",
            "haltValidation=",
            match (self.halt_validation).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ", ",
            "leadingGlue=",
            (self.leading_glue).clone().to_string(),
            ", ",
            "trailingGlue=",
            (self.trailing_glue).clone().to_string(),
            ", ",
            "anchor=",
            self.anchor.name(),
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

#[derive(Debug, Clone, PartialEq)]
pub struct PunctuationInkInput {
    pub advance: f64,
    pub ink_bounds: Option<Rect>,
    pub halt_advance: Option<f64>,
    pub halt_placement_x: Option<f64>,
    pub bounds_fallback_reason: Option<String>,
}

impl PunctuationInkInput {
    pub fn new(advance: f64, ink_bounds: Option<Rect>, halt_advance: Option<f64>, halt_placement_x: Option<f64>, bounds_fallback_reason: Option<String>) -> Result<Self, TextRangeError> {
        Ok(Self {
            advance,
            ink_bounds,
            halt_advance,
            halt_placement_x,
            bounds_fallback_reason: match bounds_fallback_reason { Some(v) => Some(v.to_string()), None => None },
        })
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "PunctuationInkInput(",
            "advance=",
            self.advance,
            ", ",
            "inkBounds=",
            (match &(self.ink_bounds) { None => "null".to_string(), Some(__option1) => __option1.to_string() }),
            ", ",
            "haltAdvance=",
            match self.halt_advance { Some(v) => crate::runtime::fp_helper::FPHelper::format_float(v), None => "null".to_string() },
            ", ",
            "haltPlacementX=",
            match self.halt_placement_x { Some(v) => crate::runtime::fp_helper::FPHelper::format_float(v), None => "null".to_string() },
            ", ",
            "boundsFallbackReason=",
            match (self.bounds_fallback_reason).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ")"
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Glue {
    pub kind: GlueKind,
    pub min: f64,
    pub natural: f64,
    pub max: f64,
    pub priority: u32,
    pub penalty: u32,
}

impl Glue {
    pub fn new(kind: GlueKind, min: f64, natural: f64, max: f64, priority: u32, penalty: u32) -> Result<Self, TextRangeError> {
        if min > (natural) {
            return Err(TextRangeError::Message { text: "Glue min must not exceed natural.".to_string() });
        }
        if natural > (max) {
            return Err(TextRangeError::Message { text: "Glue natural must not exceed max.".to_string() });
        }
        Ok(Self {
            kind,
            min,
            natural,
            max,
            priority,
            penalty,
        })
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "Glue(",
            "kind=",
            self.kind.name(),
            ", ",
            "min=",
            self.min,
            ", ",
            "natural=",
            self.natural,
            ", ",
            "max=",
            self.max,
            ", ",
            "priority=",
            crate::runtime::int_text::IntText::int_text(self.priority),
            ", ",
            "penalty=",
            crate::runtime::int_text::IntText::int_text(self.penalty),
            ")"
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdjustmentOpportunity {
    pub range: TextRange,
    pub glue: Glue,
}

impl AdjustmentOpportunity {
    pub fn new(range: TextRange, glue: Glue) -> Result<Self, TextRangeError> {
        Ok(Self {
            range,
            glue,
        })
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}",
            "AdjustmentOpportunity(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "glue=",
            (self.glue).clone().to_string(),
            ")"
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PunctuationSpacingAdjustment {
    pub range: TextRange,
    pub reduction_target_range: TextRange,
    pub left_char: String,
    pub right_char: String,
    pub natural_inner_glue: f64,
    pub adjusted_inner_glue: f64,
    pub reduction: f64,
    pub reason: String,
}

impl PunctuationSpacingAdjustment {
    pub fn new(range: TextRange, reduction_target_range: TextRange, left_char: &str, right_char: &str, natural_inner_glue: f64, adjusted_inner_glue: f64, reduction: f64, reason: &str) -> Result<Self, TextRangeError> {
        Ok(Self {
            range,
            reduction_target_range,
            left_char: left_char.to_string(),
            right_char: right_char.to_string(),
            natural_inner_glue,
            adjusted_inner_glue,
            reduction,
            reason: reason.to_string(),
        })
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "PunctuationSpacingAdjustment(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "reductionTargetRange=",
            (self.reduction_target_range).clone().to_string(),
            ", ",
            "leftChar=",
            (self.left_char).to_string(),
            ", ",
            "rightChar=",
            (self.right_char).to_string(),
            ", ",
            "naturalInnerGlue=",
            self.natural_inner_glue,
            ", ",
            "adjustedInnerGlue=",
            self.adjusted_inner_glue,
            ", ",
            "reduction=",
            self.reduction,
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PunctuationSpacingCompressionResult {
    pub adjustments: Vec<PunctuationSpacingAdjustment>,
}

impl PunctuationSpacingCompressionResult {
    pub fn new(adjustments: Vec<PunctuationSpacingAdjustment>) -> Result<Self, TextRangeError> {
        Ok(Self {
            adjustments,
        })
    }

    pub fn get_total_reduction(&self) -> f64 {
        let mut reduction_terms: Vec<f64> = vec![];
        for ai in 0..match u32::try_from((self.adjustments).clone().len()) { Ok(value) => value, Err(_) => u32::MAX } {
            reduction_terms.push(self.adjustments[usize::try_from(ai).unwrap_or(0)].reduction);
        }
        return AccurateSum::accurate_sum_of(&reduction_terms);
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}",
            "PunctuationSpacingCompressionResult(",
            "adjustments=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.adjustments).clone();
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

#[derive(Clone, PartialEq)]
pub struct PunctuationSpacingCompressor {
}

impl PunctuationSpacingCompressor {
    pub fn new() -> Result<Self, TextRangeError> {
        Ok(Self {
        })
    }

    pub fn compress(&self, atoms: &Vec<PunctuationAtom>, em: f64) -> Result<PunctuationSpacingCompressionResult, TextRangeError> {
        if i32::from_ne_bytes((u32::try_from((atoms.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) < (2) {
            return Ok(PunctuationSpacingCompressionResult::new(vec![].to_vec())?);
        }
        let half = em / format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0);
        let mut out: Vec<PunctuationSpacingAdjustment> = vec![];
        let mut i = 0;
        while (i) < (i32::wrapping_sub(i32::from_ne_bytes((i32::from_ne_bytes(u32::try_from(((atoms).len()) & 4294967295).unwrap_or(0).to_ne_bytes())).to_ne_bytes()), 1)) {
            let left = (atoms[usize::try_from(i).unwrap_or(0)]).clone();
            let right = (atoms[usize::try_from(i32::wrapping_add(i, 1)).unwrap_or(0)]).clone();
            if left.range.clone().end == (right.range).clone().start {
                let lt = { let __min_a = 0.0f64 as f64; let __min_b = ((left.trailing_glue).clone().natural - left.trailing_glue_initially_consumed) as f64; if __min_a.is_nan() || __min_b.is_nan() { f64::NAN } else { if __min_a > __min_b { __min_a } else if __min_b > __min_a {
__min_b } else if __min_a == 0.0 && __min_b == 0.0 { if __min_a.is_sign_negative() { __min_b } else { __min_a } } else { __min_a } } };
                let rl = { let __min_a1 = 0.0f64 as f64; let __min_b1 = ((right.leading_glue).clone().natural - right.leading_glue_initially_consumed) as f64; if __min_a1.is_nan() || __min_b1.is_nan() { f64::NAN } else { if __min_a1 > __min_b1 { __min_a1 } else if __min_b1 >
__min_a1 { __min_b1 } else if __min_a1 == 0.0 && __min_b1 == 0.0 { if __min_a1.is_sign_negative() { __min_b1 } else { __min_a1 } } else { __min_a1 } } };
                let natural = lt + rl;
                if natural > (0 as f64) {
                    let adjusted = { let __min_a2 = 0.0f64 as f64; let __min_b2 = (natural - half) as f64; if __min_a2.is_nan() || __min_b2.is_nan() { f64::NAN } else { if __min_a2 > __min_b2 { __min_a2 } else if __min_b2 > __min_a2 { __min_b2 } else if __min_a2 == 0.0 &&
__min_b2 == 0.0 { if __min_a2.is_sign_negative() { __min_b2 } else { __min_a2 } } else { __min_a2 } } };
                    let reduction = natural - adjusted;
                    if reduction > (0 as f64) {
                        out.push(PunctuationSpacingAdjustment::new(TextRange::new((left.range).clone().start, (right.range).clone().end)?, if lt >= rl { (left.range).clone() } else { (right.range).clone() }, (left.char).to_string().as_str(), (right.char).to_string().as_str(),
natural, adjusted, reduction, "collapse-adjacent-punctuation-inner-glue")?);
                    }
                }
            }
            i = i32::wrapping_add(i, 1);
        }
        return Ok(PunctuationSpacingCompressionResult::new(out.to_vec())?);
    }

    pub fn compress_cjk_closing_before_ascii_point_mark(&self, atoms: &Vec<PunctuationAtom>, text: &str, em: f64) -> Result<PunctuationSpacingCompressionResult, TextRangeError> {
    let __units = u_string::units(&text);
    let __count = u_string::unit_count(&text);
        let mut out: Vec<PunctuationSpacingAdjustment> = vec![];
        let half = em / format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0);
        for left in atoms {
            if left.punctuation_class != PunctuationClass::Closing || (i32::from_ne_bytes(((left.range).clone().end).to_ne_bytes())) >= i32::from_ne_bytes((__count).to_ne_bytes()) {
                continue;
            }
            let r = u_string::char_at_from(&__units, (left.range).clone().end);
            if !ClreqPunctuationPolicies::clreq_punctuation_policies_is_ascii_point_mark(r.as_str()) {
                continue;
            }
            let natural = { let __min_a3 = 0.0f64 as f64; let __min_b3 = ((left.trailing_glue).clone().natural - left.trailing_glue_initially_consumed) as f64; if __min_a3.is_nan() || __min_b3.is_nan() { f64::NAN } else { if __min_a3 > __min_b3 { __min_a3 } else if __min_b3 >
__min_a3 { __min_b3 } else if __min_a3 == 0.0 && __min_b3 == 0.0 { if __min_a3.is_sign_negative() { __min_b3 } else { __min_a3 } } else { __min_a3 } } };
            if natural <= 0 as f64 {
                continue;
            }
            let adjusted = { let __min_a4 = 0.0f64 as f64; let __min_b4 = (natural - half) as f64; if __min_a4.is_nan() || __min_b4.is_nan() { f64::NAN } else { if __min_a4 > __min_b4 { __min_a4 } else if __min_b4 > __min_a4 { __min_b4 } else if __min_a4 == 0.0 && __min_b4 == 0.0
{ if __min_a4.is_sign_negative() { __min_b4 } else { __min_a4 } } else { __min_a4 } } };
            if natural - adjusted > (0 as f64) {
                out.push(PunctuationSpacingAdjustment::new(TextRange::new((left.range).clone().start, u32::wrapping_add((left.range).clone().end, 1))?, (left.range).clone(), (left.char).to_string().as_str(), r.as_str(), natural, adjusted, natural - adjusted,
"collapse-cjk-closing-before-ascii-point-mark")?);
            }
        }
        return Ok(PunctuationSpacingCompressionResult::new(out.to_vec())?);
    }
}

#[derive(Clone, PartialEq)]
pub struct PunctuationAtomBuilder {
    pub(crate) default_glue_placement: PunctuationGluePlacement,
    pub(crate) default_width_policy: PunctuationWidthPolicy,
}

impl PunctuationAtomBuilder {
    const PUNCTUATION_ATOM_BUILDER_PLACEMENT_EPSILON: f64 = 0.001f64;

    pub fn new(glue_placement: Option<PunctuationGluePlacement>, width_policy: Option<PunctuationWidthPolicy>) -> Result<Self, TextRangeError> {
        Ok(Self {
            default_glue_placement: match &(glue_placement) { None => PunctuationGluePlacement::MainlandSimplified, Some(__option2) => *__option2 },
            default_width_policy: match &(width_policy) { None => PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false)), Some(__option3) => (*__option3).clone() },
        })
    }

    pub fn build_at_index(&self, text: &str, index: u32, em: f64) -> Result<Option<PunctuationAtom>, TextRangeError> {
    let __units1 = u_string::units(&text);
    let __count1 = u_string::unit_count(&text);
        if index > 2147483647 || (i32::from_ne_bytes((index).to_ne_bytes())) >= i32::from_ne_bytes((__count1).to_ne_bytes()) {
            return Ok(None);
        }
        return Ok(self.build(u_string::char_at_from(&__units1, index).as_str(), TextRange::new(u32::from_ne_bytes((index).to_ne_bytes()), u32::wrapping_add(index, 1))?, em, None, None, None)?);
    }

    pub fn build(&self, char: &str, range: TextRange, em: f64, ink_input: Option<PunctuationInkInput>, glue_placement: Option<PunctuationGluePlacement>, width_policy: Option<PunctuationWidthPolicy>) -> Result<Option<PunctuationAtom>, TextRangeError> {
        let placement = match &(glue_placement) { None => self.default_glue_placement, Some(__option4) => *__option4 };
        let wp = match &(width_policy) { None => (self.default_width_policy).clone(), Some(__option5) => (*__option5).clone() };
        let policy = ClreqPunctuationPolicies::clreq_punctuation_policies_policy_for(char);
        if policy.punctuation_class == PunctuationClass::Other {
            return Ok(None);
        }
        let policy_advance = policy.default_advance_em * em;
        let shaped = match &(ink_input) { None => None, Some(__option6) => if __option6.advance > (0 as f64) { Some(__option6.advance) } else { None } };
        let raw = match &(shaped) { None => policy_advance, Some(__option7) => *__option7 };
        let expansion = { let __min_a5 = 0.0f64 as f64; let __min_b5 = (policy_advance - raw) as f64; if __min_a5.is_nan() || __min_b5.is_nan() { f64::NAN } else { if __min_a5 > __min_b5 { __min_a5 } else if __min_b5 > __min_a5 { __min_b5 } else if __min_a5 == 0.0 && __min_b5 ==
0.0 { if __min_a5.is_sign_negative() { __min_b5 } else { __min_a5 } } else { __min_a5 } } };
        let glue_side = PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, policy.punctuation_class);
        let shift = if match &(shaped) { Some(__option10) => expansion > (0 as f64), None => false } { match glue_side {
    GlueSide::LeadingOnly => expansion,
    GlueSide::TrailingOnly => 0 as f64,
    GlueSide::BothSides => expansion / format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0),
} } else { 0 as f64 };
        let ink = if match &(ink_input) { None => true, Some(__option12) => __option12.ink_bounds.is_none() } { None } else { Some(self.shift_rect((((ink_input.as_ref().unwrap().ink_bounds).clone()).unwrap()).clone(), shift)) };
        let ink_width = match &(ink) { None => None, Some(__option13) => Some({ let __min_a6 = 0.0f64 as f64; let __min_b6 = __option13.get_width() as f64; if __min_a6.is_nan() || __min_b6.is_nan() { f64::NAN } else { if __min_a6 > __min_b6 { __min_a6 } else if __min_b6 >
__min_a6 { __min_b6 } else if __min_a6 == 0.0 && __min_b6 == 0.0 { if __min_a6.is_sign_negative() { __min_b6 } else { __min_a6 } } else { __min_a6 } } }) };
        let center = match &(ink) { None => None, Some(__option14) => Some((__option14.left + __option14.right) / format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0)) };
        let advance = { let __min_a8 = { let __min_a9 = raw as f64; let __min_b9 = policy_advance as f64; if __min_a9.is_nan() || __min_b9.is_nan() { f64::NAN } else { if __min_a9 > __min_b9 { __min_a9 } else if __min_b9 > __min_a9 { __min_b9 } else if __min_a9 == 0.0 && __min_b9
== 0.0 { if __min_a9.is_sign_negative() { __min_b9 } else { __min_a9 } } else { __min_a9 } } } as f64; let __min_b8 = (match &(ink) { None => 0 as f64, Some(__option16) => __option16.right }) as f64; if __min_a8.is_nan() || __min_b8.is_nan() { f64::NAN } else { if __min_a8 >
__min_b8 { __min_a8 } else if __min_b8 > __min_a8 { __min_b8 } else if __min_a8 == 0.0 && __min_b8 == 0.0 { if __min_a8.is_sign_negative() { __min_b8 } else { __min_a8 } } else { __min_a8 } } };
        let policy_floor = policy.default_body_em * em;
        let halt = match &(ink_input) { None => None, Some(__option17) => if expansion <= PunctuationAtomBuilder::PUNCTUATION_ATOM_BUILDER_PLACEMENT_EPSILON && __option17.halt_advance.is_some() && (__option17.halt_advance.unwrap_or(0.0)) > (0 as f64) &&
(__option17.halt_advance.unwrap_or(0.0)) < (advance) { __option17.halt_advance } else { None } };
        let forced = ClreqPunctuationPolicies::clreq_punctuation_policies_forced_half_width(char, (wp).clone());
        let forced_floor = if forced { { let __min_a10 = policy_floor as f64; let __min_b10 = (0.5f64 * em) as f64; if __min_a10.is_nan() || __min_b10.is_nan() { f64::NAN } else { if __min_a10 < __min_b10 { __min_a10 } else if __min_b10 < __min_a10 { __min_b10 } else if __min_a10
== 0.0 && __min_b10 == 0.0 { if __min_a10.is_sign_negative() { __min_a10 } else { __min_b10 } } else { __min_a10 } } } } else { policy_floor };
        let target = match &(halt) { None => forced_floor, Some(__option18) => *__option18 };
        let geo = self.compression_geometry(advance, raw, target, (ink).clone(), halt, match &(ink_input) { None => None, Some(__option20) => __option20.halt_placement_x }, policy.punctuation_class, placement)?;
        return Ok(Some(PunctuationAtom::new((range).clone(), char, policy.punctuation_class, advance, (ink).clone(), geo.body_width, halt, geo.halt_validation.clone(), Glue::new(GlueKind::PunctuationLeading, 0 as f64 as f64, geo.leading_trim, geo.leading_trim, 0u32, 0u32)?,
Glue::new(GlueKind::PunctuationTrailing, 0 as f64 as f64, geo.trailing_trim, geo.trailing_trim, 0u32, 0u32)?, geo.anchor, if forced { format!("{}{}",
            (geo.source).to_string(),
            "FixedHalfWidth"
        ).to_string() } else { (geo.source).to_string() }.as_str(), policy_floor, ink_width, center, geo.ink_body_floor, geo.ink_containment_applied, if ink.is_none() && ink_input.is_some() { (ink_input.as_ref().unwrap().bounds_fallback_reason).clone() } else { None }.clone(), {
let __min_a11 = 0.0f64 as f64; let __min_b11 = (advance - raw) as f64; if __min_a11.is_nan() || __min_b11.is_nan() { f64::NAN } else { if __min_a11 > __min_b11 { __min_a11 } else if __min_b11 > __min_a11 { __min_b11 } else if __min_a11 == 0.0 && __min_b11 == 0.0 { if
__min_a11.is_sign_negative() { __min_b11 } else { __min_a11 } } else { __min_a11 } } }, shift, if shift != 0 as f64 { Some("UnderwidthPunctuationFullWidthBoxPlacement".to_string()) } else { None }.clone(), if forced { Some(geo.leading_trim) } else { Some(0 as f64) }, if forced {
Some(geo.trailing_trim) } else { Some(0 as f64) })?));
    }

    fn shift_rect(&self, r: Rect, a: f64) -> Rect {
        return if a == 0 as f64 { r } else { Rect::new(r.left + a, r.top, r.right + a, r.bottom) };
    }

    fn compression_geometry(&self, advance: f64, raw: f64, target: f64, ink: Option<Rect>, halt: Option<f64>, placement_x: Option<f64>, cls: PunctuationClass, placement: PunctuationGluePlacement) -> Result<CompressionGeometry, TextRangeError> {
        let requested = { let __min_a12 = 0.0f64 as f64; let __min_b12 = (advance - target) as f64; if __min_a12.is_nan() || __min_b12.is_nan() { f64::NAN } else { if __min_a12 > __min_b12 { __min_a12 } else if __min_b12 > __min_a12 { __min_b12 } else if __min_a12 == 0.0 &&
__min_b12 == 0.0 { if __min_a12.is_sign_negative() { __min_b12 } else { __min_a12 } } else { __min_a12 } } };
        if match &(halt) { Some(__option22) => placement_x.is_some() && (*((placement_x).as_ref().unwrap())).is_finite(), None => false } {
            let raw_reduction = { let __min_a13 = 0.0f64 as f64; let __min_b13 = (raw - *((halt).as_ref().unwrap())) as f64; if __min_a13.is_nan() || __min_b13.is_nan() { f64::NAN } else { if __min_a13 > __min_b13 { __min_a13 } else if __min_b13 > __min_a13 { __min_b13 } else if
__min_a13 == 0.0 && __min_b13 == 0.0 { if __min_a13.is_sign_negative() { __min_b13 } else { __min_a13 } } else { __min_a13 } } };
            let req_lead = { let __min_a15 = 0.0f64 as f64; let __min_b15 = { let __min_a16 = raw_reduction as f64; let __min_b16 = (-(placement_x.unwrap_or(0.0))) as f64; if __min_a16.is_nan() || __min_b16.is_nan() { f64::NAN } else { if __min_a16 < __min_b16 { __min_a16 } else
if __min_b16 < __min_a16 { __min_b16 } else if __min_a16 == 0.0 && __min_b16 == 0.0 { if __min_a16.is_sign_negative() { __min_a16 } else { __min_b16 } } else { __min_a16 } } } as f64; if __min_a15.is_nan() || __min_b15.is_nan() { f64::NAN } else { if __min_a15 > __min_b15 {
__min_a15 } else if __min_b15 > __min_a15 { __min_b15 } else if __min_a15 == 0.0 && __min_b15 == 0.0 { if __min_a15.is_sign_negative() { __min_b15 } else { __min_a15 } } else { __min_a15 } } };
            let req_trail = { let __min_a17 = 0.0f64 as f64; let __min_b17 = (requested - req_lead) as f64; if __min_a17.is_nan() || __min_b17.is_nan() { f64::NAN } else { if __min_a17 > __min_b17 { __min_a17 } else if __min_b17 > __min_a17 { __min_b17 } else if __min_a17 == 0.0
&& __min_b17 == 0.0 { if __min_a17.is_sign_negative() { __min_b17 } else { __min_a17 } } else { __min_a17 } } };
            let lead = match &(ink) { None => req_lead, Some(__option23) => { let __min_a19 = req_lead as f64; let __min_b19 = { let __min_a20 = 0.0f64 as f64; let __min_b20 = __option23.left as f64; if __min_a20.is_nan() || __min_b20.is_nan() { f64::NAN } else { if __min_a20 >
__min_b20 { __min_a20 } else if __min_b20 > __min_a20 { __min_b20 } else if __min_a20 == 0.0 && __min_b20 == 0.0 { if __min_a20.is_sign_negative() { __min_b20 } else { __min_a20 } } else { __min_a20 } } } as f64; if __min_a19.is_nan() || __min_b19.is_nan() { f64::NAN } else { if
__min_a19 < __min_b19 { __min_a19 } else if __min_b19 < __min_a19 { __min_b19 } else if __min_a19 == 0.0 && __min_b19 == 0.0 { if __min_a19.is_sign_negative() { __min_a19 } else { __min_b19 } } else { __min_a19 } } } };
            let trail = match &(ink) { None => req_trail, Some(__option24) => { let __min_a22 = req_trail as f64; let __min_b22 = { let __min_a23 = 0.0f64 as f64; let __min_b23 = (advance - __option24.right) as f64; if __min_a23.is_nan() || __min_b23.is_nan() { f64::NAN } else {
if __min_a23 > __min_b23 { __min_a23 } else if __min_b23 > __min_a23 { __min_b23 } else if __min_a23 == 0.0 && __min_b23 == 0.0 { if __min_a23.is_sign_negative() { __min_b23 } else { __min_a23 } } else { __min_a23 } } } as f64; if __min_a22.is_nan() || __min_b22.is_nan() {
f64::NAN } else { if __min_a22 < __min_b22 { __min_a22 } else if __min_b22 < __min_a22 { __min_b22 } else if __min_a22 == 0.0 && __min_b22 == 0.0 { if __min_a22.is_sign_negative() { __min_a22 } else { __min_b22 } } else { __min_a22 } } } };
            let limited = lead + PunctuationAtomBuilder::PUNCTUATION_ATOM_BUILDER_PLACEMENT_EPSILON < (req_lead) || (trail + PunctuationAtomBuilder::PUNCTUATION_ATOM_BUILDER_PLACEMENT_EPSILON) < (req_trail);
            return Ok(CompressionGeometry::new(lead, trail, (advance - lead) - trail, self.anchor_for(lead, trail), "FontHaltFittedBodyCompression", if ink.is_none() { None } else { Some((advance - lead) - trail) }, limited, if limited {
Some("halt-trim-limited-by-default-ink-bounds".to_string()) } else { None }.clone())?);
        }
        match &(ink) {
            Some(__option27) => {
                let frame = self.fitted_body_frame(advance, { let __min_a28 = 0.0f64 as f64; let __min_b28 = { let __min_a29 = target as f64; let __min_b29 = advance as f64; if __min_a29.is_nan() || __min_b29.is_nan() { f64::NAN } else { if __min_a29 < __min_b29 { __min_a29 }
else if __min_b29 < __min_a29 { __min_b29 } else if __min_a29 == 0.0 && __min_b29 == 0.0 { if __min_a29.is_sign_negative() { __min_a29 } else { __min_b29 } } else { __min_a29 } } } as f64; if __min_a28.is_nan() || __min_b28.is_nan() { f64::NAN } else { if __min_a28 > __min_b28 {
__min_a28 } else if __min_b28 > __min_a28 { __min_b28 } else if __min_a28 == 0.0 && __min_b28 == 0.0 { if __min_a28.is_sign_negative() { __min_b28 } else { __min_a28 } } else { __min_a28 } } }, ((*__option27).clone()).clone())?;
                return Ok(CompressionGeometry::new(frame.start, { let __min_a30 = 0.0f64 as f64; let __min_b30 = ((advance - frame.start) - frame.width) as f64; if __min_a30.is_nan() || __min_b30.is_nan() { f64::NAN } else { if __min_a30 > __min_b30 { __min_a30 } else if
__min_b30 > __min_a30 { __min_b30 } else if __min_a30 == 0.0 && __min_b30 == 0.0 { if __min_a30.is_sign_negative() { __min_b30 } else { __min_a30 } } else { __min_a30 } } }, frame.width, frame.anchor, if halt.is_some() { "FontHaltAdvanceWithInkBoundsFittedPlacement".to_string() }
else { "InkBoundsFittedBodyCompression".to_string() }.as_str(), Some(frame.width), (frame.width) > (target + PunctuationAtomBuilder::PUNCTUATION_ATOM_BUILDER_PLACEMENT_EPSILON), None.clone())?);
            }
            None => {
            }
        }
        let pair = self.class_based_glue(cls, requested, placement);
        return Ok(CompressionGeometry::new(pair.leading, pair.trailing, (advance - pair.leading) - pair.trailing, self.anchor_for(pair.leading, pair.trailing), if halt.is_some() { "FontHaltAdvanceWithProfileFallback".to_string() } else {
"ProfileGlueFallbackWithoutFontGeometry".to_string() }.as_str(), None, false, None.clone())?);
    }

    fn fitted_body_frame(&self, advance: f64, target: f64, ink: Rect) -> Result<BodyFrame, TextRangeError> {
        let lw = { let __min_a32 = target as f64; let __min_b32 = { let __min_a33 = ink.right as f64; let __min_b33 = advance as f64; if __min_a33.is_nan() || __min_b33.is_nan() { f64::NAN } else { if __min_a33 < __min_b33 { __min_a33 } else if __min_b33 < __min_a33 { __min_b33 }
else if __min_a33 == 0.0 && __min_b33 == 0.0 { if __min_a33.is_sign_negative() { __min_a33 } else { __min_b33 } } else { __min_a33 } } } as f64; if __min_a32.is_nan() || __min_b32.is_nan() { f64::NAN } else { if __min_a32 > __min_b32 { __min_a32 } else if __min_b32 > __min_a32 {
__min_b32 } else if __min_a32 == 0.0 && __min_b32 == 0.0 { if __min_a32.is_sign_negative() { __min_b32 } else { __min_a32 } } else { __min_a32 } } };
        let tw = { let __min_a35 = target as f64; let __min_b35 = { let __min_a36 = (advance - ink.left) as f64; let __min_b36 = advance as f64; if __min_a36.is_nan() || __min_b36.is_nan() { f64::NAN } else { if __min_a36 < __min_b36 { __min_a36 } else if __min_b36 < __min_a36 {
__min_b36 } else if __min_a36 == 0.0 && __min_b36 == 0.0 { if __min_a36.is_sign_negative() { __min_a36 } else { __min_b36 } } else { __min_a36 } } } as f64; if __min_a35.is_nan() || __min_b35.is_nan() { f64::NAN } else { if __min_a35 > __min_b35 { __min_a35 } else if __min_b35 >
__min_a35 { __min_b35 } else if __min_a35 == 0.0 && __min_b35 == 0.0 { if __min_a35.is_sign_negative() { __min_b35 } else { __min_a35 } } else { __min_a35 } } };
        let cw = { let __min_a40 = target as f64; let __min_b40 = { let __min_a42 = { let __min_a43 = (advance - format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0) * ink.left) as f64; let __min_b43 = (format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0) * ink.right - advance) as
f64; if __min_a43.is_nan() || __min_b43.is_nan() { f64::NAN } else { if __min_a43 > __min_b43 { __min_a43 } else if __min_b43 > __min_a43 { __min_b43 } else if __min_a43 == 0.0 && __min_b43 == 0.0 { if __min_a43.is_sign_negative() { __min_b43 } else { __min_a43 } } else {
__min_a43 } } } as f64; let __min_b42 = advance as f64; if __min_a42.is_nan() || __min_b42.is_nan() { f64::NAN } else { if __min_a42 < __min_b42 { __min_a42 } else if __min_b42 < __min_a42 { __min_b42 } else if __min_a42 == 0.0 && __min_b42 == 0.0 { if
__min_a42.is_sign_negative() { __min_a42 } else { __min_b42 } } else { __min_a42 } } } as f64; if __min_a40.is_nan() || __min_b40.is_nan() { f64::NAN } else { if __min_a40 > __min_b40 { __min_a40 } else if __min_b40 > __min_a40 { __min_b40 } else if __min_a40 == 0.0 && __min_b40
== 0.0 { if __min_a40.is_sign_negative() { __min_b40 } else { __min_a40 } } else { __min_a40 } } };
        let a = vec![
    (BodyFrame::new(PunctuationAnchor::Leading, 0 as f64 as f64, lw)?).clone(),
    (BodyFrame::new(PunctuationAnchor::Center, (advance - cw) / format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0), cw)?).clone(),
    (BodyFrame::new(PunctuationAnchor::Trailing, advance - tw, tw)?).clone(),
];
        let ic = (ink.left + ink.right) / format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0);
        let mut best = (a[0usize]).clone();
        for i in 1..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if a[usize::try_from(i).unwrap_or(0)].width < (best.width) || a[usize::try_from(i).unwrap_or(0)].width == best.width && ((a[usize::try_from(i).unwrap_or(0)].start + a[usize::try_from(i).unwrap_or(0)].width / format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0) -
ic).abs()) < ((best.start + best.width / format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0) - ic).abs()) {
                best = (a[usize::try_from(i).unwrap_or(0)]).clone();
            }
        }
        return Ok(best);
    }

    fn anchor_for(&self, l: f64, t: f64) -> PunctuationAnchor {
        return if l > (PunctuationAtomBuilder::PUNCTUATION_ATOM_BUILDER_PLACEMENT_EPSILON) && (t) > (PunctuationAtomBuilder::PUNCTUATION_ATOM_BUILDER_PLACEMENT_EPSILON) { PunctuationAnchor::Center } else { if l >
(PunctuationAtomBuilder::PUNCTUATION_ATOM_BUILDER_PLACEMENT_EPSILON) { PunctuationAnchor::Trailing } else { if t > (PunctuationAtomBuilder::PUNCTUATION_ATOM_BUILDER_PLACEMENT_EPSILON) { PunctuationAnchor::Leading } else { PunctuationAnchor::Center } } };
    }

    fn class_based_glue(&self, cls: PunctuationClass, total: f64, placement: PunctuationGluePlacement) -> ClassGluePair {
        let side = PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, cls);
        return match side {
            GlueSide::LeadingOnly => ClassGluePair { leading: total, trailing: 0 as f64 },
            GlueSide::TrailingOnly => ClassGluePair { leading: 0 as f64, trailing: total },
            GlueSide::BothSides => ClassGluePair { leading: total / format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0), trailing: total / format!("{}", (2i32)).parse::<f64>().unwrap_or(0.0) },
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BodyFrame {
    pub anchor: PunctuationAnchor,
    pub start: f64,
    pub width: f64,
}

impl BodyFrame {
    pub fn new(anchor: PunctuationAnchor, start: f64, width: f64) -> Result<Self, TextRangeError> {
        Ok(Self {
            anchor,
            start,
            width,
        })
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "BodyFrame(",
            "anchor=",
            self.anchor.name(),
            ", ",
            "start=",
            self.start,
            ", ",
            "width=",
            self.width,
            ")"
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompressionGeometry {
    pub leading_trim: f64,
    pub trailing_trim: f64,
    pub body_width: f64,
    pub anchor: PunctuationAnchor,
    pub source: String,
    pub ink_body_floor: Option<f64>,
    pub ink_containment_applied: bool,
    pub halt_validation: Option<String>,
}

impl CompressionGeometry {
    pub fn new(leading_trim: f64, trailing_trim: f64, body_width: f64, anchor: PunctuationAnchor, source: &str, ink_body_floor: Option<f64>, ink_containment_applied: bool, halt_validation: Option<String>) -> Result<Self, TextRangeError> {
        Ok(Self {
            leading_trim,
            trailing_trim,
            body_width,
            anchor,
            source: source.to_string(),
            ink_body_floor,
            ink_containment_applied,
            halt_validation: match halt_validation { Some(v) => Some(v.to_string()), None => None },
        })
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "CompressionGeometry(",
            "leadingTrim=",
            self.leading_trim,
            ", ",
            "trailingTrim=",
            self.trailing_trim,
            ", ",
            "bodyWidth=",
            self.body_width,
            ", ",
            "anchor=",
            self.anchor.name(),
            ", ",
            "source=",
            (self.source).to_string(),
            ", ",
            "inkBodyFloor=",
            match self.ink_body_floor { Some(v) => crate::runtime::fp_helper::FPHelper::format_float(v), None => "null".to_string() },
            ", ",
            "inkContainmentApplied=",
            self.ink_containment_applied,
            ", ",
            "haltValidation=",
            match (self.halt_validation).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ")"
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PunctuationAnchor {
    Leading,
    Center,
    Trailing,
}

pub fn compare_punctuation_anchor(a: &PunctuationAnchor, b: &PunctuationAnchor) -> i32 {
    if a == b { return 0; }
    fn rank(v: &PunctuationAnchor) -> i32 {
        match v {
            PunctuationAnchor::Leading => 0,
            PunctuationAnchor::Center => 1,
            PunctuationAnchor::Trailing => 2,
        }
    }
    rank(a) - rank(b)
}

impl PunctuationAnchor {
    pub fn to_string(&self) -> String {
        match self {
            PunctuationAnchor::Leading => "Leading".to_string(),
            PunctuationAnchor::Center => "Center".to_string(),
            PunctuationAnchor::Trailing => "Trailing".to_string(),
        }
    }
}

impl PunctuationAnchor {
    pub fn name(&self) -> &'static str {
        match self {
            PunctuationAnchor::Leading => "Leading",
            PunctuationAnchor::Center => "Center",
            PunctuationAnchor::Trailing => "Trailing",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GlueKind {
    PunctuationLeading,
    PunctuationTrailing,
    CjkLatinSpace,
    WordSpace,
    CjkInterChar,
    ProgressiveTechnical,
    EmergencyGraphemeTracking,
    InlineObjectPunctuationTrailing,
    InlineObjectRelation,
    InlineObjectBinaryOperator,
    InlineObjectBoundary,
}

pub fn compare_glue_kind(a: &GlueKind, b: &GlueKind) -> i32 {
    if a == b { return 0; }
    fn rank(v: &GlueKind) -> i32 {
        match v {
            GlueKind::PunctuationLeading => 0,
            GlueKind::PunctuationTrailing => 1,
            GlueKind::CjkLatinSpace => 2,
            GlueKind::WordSpace => 3,
            GlueKind::CjkInterChar => 4,
            GlueKind::ProgressiveTechnical => 5,
            GlueKind::EmergencyGraphemeTracking => 6,
            GlueKind::InlineObjectPunctuationTrailing => 7,
            GlueKind::InlineObjectRelation => 8,
            GlueKind::InlineObjectBinaryOperator => 9,
            GlueKind::InlineObjectBoundary => 10,
        }
    }
    rank(a) - rank(b)
}

impl GlueKind {
    pub fn to_string(&self) -> String {
        match self {
            GlueKind::PunctuationLeading => "PunctuationLeading".to_string(),
            GlueKind::PunctuationTrailing => "PunctuationTrailing".to_string(),
            GlueKind::CjkLatinSpace => "CjkLatinSpace".to_string(),
            GlueKind::WordSpace => "WordSpace".to_string(),
            GlueKind::CjkInterChar => "CjkInterChar".to_string(),
            GlueKind::ProgressiveTechnical => "ProgressiveTechnical".to_string(),
            GlueKind::EmergencyGraphemeTracking => "EmergencyGraphemeTracking".to_string(),
            GlueKind::InlineObjectPunctuationTrailing => "InlineObjectPunctuationTrailing".to_string(),
            GlueKind::InlineObjectRelation => "InlineObjectRelation".to_string(),
            GlueKind::InlineObjectBinaryOperator => "InlineObjectBinaryOperator".to_string(),
            GlueKind::InlineObjectBoundary => "InlineObjectBoundary".to_string(),
        }
    }
}

impl GlueKind {
    pub fn name(&self) -> &'static str {
        match self {
            GlueKind::PunctuationLeading => "PunctuationLeading",
            GlueKind::PunctuationTrailing => "PunctuationTrailing",
            GlueKind::CjkLatinSpace => "CjkLatinSpace",
            GlueKind::WordSpace => "WordSpace",
            GlueKind::CjkInterChar => "CjkInterChar",
            GlueKind::ProgressiveTechnical => "ProgressiveTechnical",
            GlueKind::EmergencyGraphemeTracking => "EmergencyGraphemeTracking",
            GlueKind::InlineObjectPunctuationTrailing => "InlineObjectPunctuationTrailing",
            GlueKind::InlineObjectRelation => "InlineObjectRelation",
            GlueKind::InlineObjectBinaryOperator => "InlineObjectBinaryOperator",
            GlueKind::InlineObjectBoundary => "InlineObjectBoundary",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClassGluePair {
    pub leading: f64,
    pub trailing: f64,
}
