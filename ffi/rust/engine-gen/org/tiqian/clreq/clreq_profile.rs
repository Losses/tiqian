use crate::org::tiqian::clreq::adjustment_style_policy::AdjustmentStylePolicy;
use crate::org::tiqian::clreq::auto_space_policy::AutoSpacePolicy;
use crate::org::tiqian::clreq::cjk_punctuation_glyph_policy::CjkPunctuationGlyphPolicy;
use crate::org::tiqian::clreq::clreq_region::ClreqRegion;
use crate::org::tiqian::clreq::clreq_strictness::ClreqStrictness;
use crate::org::tiqian::clreq::interior_punctuation_style::InteriorPunctuationStyle;
use crate::org::tiqian::clreq::kinsoku_mode::KinsokuMode;
use crate::org::tiqian::clreq::kinsoku_modes::KinsokuModes;
use crate::org::tiqian::clreq::line_adjustment_strategy::LineAdjustmentStrategy;
use crate::org::tiqian::clreq::line_end_punctuation_style::LineEndPunctuationStyle;
use crate::org::tiqian::clreq::punctuation_glue_placement::PunctuationGluePlacement;
use crate::org::tiqian::clreq::punctuation_glue_placements::PunctuationGluePlacements;
use crate::org::tiqian::clreq::punctuation_width_policy::PunctuationWidthPolicy;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;
use std::sync::LazyLock;


pub static CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION: [u32; 3] = [8212, 8230, 8943];

pub static CLREQ_PROFILE_MAINLAND_HORIZONTAL: LazyLock<ClreqProfile> = LazyLock::new(|| ClreqProfile::new(&(UStr::new(&[99,108,114,101,113,45,109,97,105,110,108,97,110,100,45,104,111,114,105,122,111,110,116,97,108])), ClreqStrictness::Normal, ClreqRegion::Mainland, Some(CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints), Some(CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some((*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone()), Some(PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::Mainland)), AdjustmentStylePolicy::new(Some(LineEndPunctuationStyle::ForceHalfWidth), Some(true), Some(true), Some(LineAdjustmentStrategy::PushInFirst)), KinsokuMode::MeasureAdaptive { hang_below_em: 14.0f64, gb_above_em: 24.0f64, strict_above_em: 32.0f64 }, PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false))));

pub static CLREQ_PROFILE_TAIWAN_HORIZONTAL: LazyLock<ClreqProfile> = LazyLock::new(|| ClreqProfile::new(&(UStr::new(&[99,108,114,101,113,45,116,97,105,119,97,110,45,104,111,114,105,122,111,110,116,97,108])), ClreqStrictness::Normal, ClreqRegion::Taiwan, Some(CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints), Some(CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some((*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone()), Some(PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::Taiwan)), AdjustmentStylePolicy::new(Some(LineEndPunctuationStyle::ForceHalfWidth), Some(true), Some(true), Some(LineAdjustmentStrategy::PushInFirst)), KinsokuMode::MeasureAdaptive { hang_below_em: 14.0f64, gb_above_em: 24.0f64, strict_above_em: 32.0f64 }, PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false))));

pub static CLREQ_PROFILE_HONG_KONG_HORIZONTAL: LazyLock<ClreqProfile> = LazyLock::new(|| ClreqProfile::new(&(UStr::new(&[99,108,114,101,113,45,104,111,110,103,107,111,110,103,45,104,111,114,105,122,111,110,116,97,108])), ClreqStrictness::Normal, ClreqRegion::HongKong, Some(CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints), Some(CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some((*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone()), Some(PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::HongKong)), AdjustmentStylePolicy::new(Some(LineEndPunctuationStyle::ForceHalfWidth), Some(true), Some(true), Some(LineAdjustmentStrategy::PushInFirst)), KinsokuMode::MeasureAdaptive { hang_below_em: 14.0f64, gb_above_em: 24.0f64, strict_above_em: 32.0f64 }, PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false))));

#[derive(Debug, Clone, PartialEq)]
pub struct ClreqProfile {
    pub id: UString,
    pub strictness: ClreqStrictness,
    pub region: ClreqRegion,
    pub punctuation_glyph_policy: CjkPunctuationGlyphPolicy,
    pub coalesce_repeatable_punctuation: Vec<u32>,
    pub auto_space: AutoSpacePolicy,
    pub glue_placement: PunctuationGluePlacement,
    pub adjustment: AdjustmentStylePolicy,
    pub kinsoku_mode: KinsokuMode,
    pub punctuation_width: PunctuationWidthPolicy,
}

impl ClreqProfile {
    pub fn new(id: &UStr, strictness: ClreqStrictness, region: ClreqRegion, punctuation_glyph_policy: Option<CjkPunctuationGlyphPolicy>, coalesce_repeatable_punctuation: Option<Vec<u32>>, auto_space: Option<AutoSpacePolicy>, glue_placement: Option<PunctuationGluePlacement>, adjustment: AdjustmentStylePolicy, kinsoku_mode: KinsokuMode, punctuation_width: PunctuationWidthPolicy) -> Self {
        let punctuation_glyph_policy = punctuation_glyph_policy.unwrap_or_else(|| CjkPunctuationGlyphPolicy::PreferClreqRecommendedCodepoints);
        let coalesce_repeatable_punctuation = coalesce_repeatable_punctuation.unwrap_or_else(|| CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec());
        let auto_space = auto_space.unwrap_or_else(|| (*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone());
        let glue_placement = glue_placement.unwrap_or_else(|| PunctuationGluePlacements::punctuation_glue_placements_for_region(region));
        Self {
            id: id.to_ustring(),
            strictness,
            region,
            punctuation_glyph_policy: punctuation_glyph_policy,
            coalesce_repeatable_punctuation: coalesce_repeatable_punctuation,
            auto_space: auto_space,
            glue_placement: glue_placement,
            adjustment,
            kinsoku_mode,
            punctuation_width,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ClreqProfile(")); __s += &(UString::from("id=")); __s += (self.id).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("strictness=")); __s += UString::from(self.strictness.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("region=")); __s += UString::from(self.region.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("punctuationGlyphPolicy=")); __s += UString::from(self.punctuation_glyph_policy.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("coalesceRepeatablePunctuation=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.coalesce_repeatable_punctuation).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", i32::from_ne_bytes(((arr[i]) as i32).to_ne_bytes()));
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("autoSpace=")); __s += UString::from(format!("{}", (self.auto_space).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("gluePlacement=")); __s += UString::from(self.glue_placement.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("adjustment=")); __s += UString::from(format!("{}", (self.adjustment).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("kinsokuMode=")); __s += UString::from(format!("{}", (self.kinsoku_mode).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("punctuationWidth=")); __s += UString::from(format!("{}", (self.punctuation_width).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }

    pub fn clreq_profile_same_profile(a: ClreqProfile, b: ClreqProfile) -> bool {
        if a.id.to_ustring() != (b.id).to_ustring() || a.strictness != b.strictness || a.region != b.region || a.punctuation_glyph_policy != b.punctuation_glyph_policy || u32::try_from((a.coalesce_repeatable_punctuation.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((b.coalesce_repeatable_punctuation.len()) & 0xFFFF_FFFF).unwrap_or(0) || !AutoSpacePolicy::auto_space_policy_same_policy((a.auto_space).clone(), (b.auto_space).clone()) || a.glue_placement != b.glue_placement || !AdjustmentStylePolicy::adjustment_style_policy_same_policy((a.adjustment).clone(), (b.adjustment).clone()) || !KinsokuModes::kinsoku_modes_same_mode((a.kinsoku_mode).clone(), (b.kinsoku_mode).clone()) || !PunctuationWidthPolicy::punctuation_width_policy_same_policy((a.punctuation_width).clone(), (b.punctuation_width).clone()) {
            return false;
        }
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((a.coalesce_repeatable_punctuation.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if a.coalesce_repeatable_punctuation[usize::try_from(index).unwrap_or(0)] != b.coalesce_repeatable_punctuation[usize::try_from(index).unwrap_or(0)] {
                return false;
            }
            index = u32::wrapping_add(index, 1);
        }
        return true;
    }
}
