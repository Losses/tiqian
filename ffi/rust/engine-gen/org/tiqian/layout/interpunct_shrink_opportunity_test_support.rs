use crate::org::tiqian::clreq::adjustment_style_policy::AdjustmentStylePolicy;
use crate::org::tiqian::clreq::cjk_punctuation_glyph_policy::CjkPunctuationGlyphPolicy;
use crate::org::tiqian::clreq::clreq_profile::ClreqProfile;
use crate::org::tiqian::clreq::clreq_profile_resolver::ClreqProfileResolver;
use crate::org::tiqian::clreq::clreq_region::ClreqRegion;
use crate::org::tiqian::clreq::clreq_strictness::ClreqStrictness;
use crate::org::tiqian::clreq::interior_punctuation_style::InteriorPunctuationStyle;
use crate::org::tiqian::clreq::kinsoku_mode::KinsokuMode;
use crate::org::tiqian::clreq::line_adjustment_strategy::LineAdjustmentStrategy;
use crate::org::tiqian::clreq::line_end_punctuation_style::LineEndPunctuationStyle;
use crate::org::tiqian::clreq::punctuation_glue_placements::PunctuationGluePlacements;
use crate::org::tiqian::clreq::punctuation_width_policy::PunctuationWidthPolicy;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::glyph_run::GlyphRun;
use crate::org::tiqian::core::layout_profile_id::LayoutProfileId;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::shaping::text_shaper::ITextShaper;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::org::tiqian::shaping::text_shaper::ShapingResult;
use crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault;
use crate::runtime::u_string;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Clone, Copy)]
pub struct InterpunctShrinkOpportunityTestSupport;

impl InterpunctShrinkOpportunityTestSupport {
    pub fn interpunct_shrink_opportunity_test_support_halt_ink_shaper() -> Arc<Mutex<dyn ITextShaper>> {
        return Arc::new(Mutex::new(InterpunctHaltShaper::new()));
    }

    pub fn interpunct_shrink_opportunity_test_support_preserve_resolver() -> Box<dyn ClreqProfileResolver> {
        return Box::new(InterpunctPreserveResolver::new());
    }
}

#[derive(Clone, PartialEq)]
pub struct InterpunctPreserveResolver {
}

impl InterpunctPreserveResolver {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn resolve(&self, _profile_id: LayoutProfileId) -> ClreqProfile {
        return ClreqProfile::new(((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().id).to_ustring().as_ustr(), ClreqStrictness::Normal, ClreqRegion::Mainland, Some(CjkPunctuationGlyphPolicy::PreserveInput), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some((*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone()), Some(PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::Mainland)), AdjustmentStylePolicy::new(Some(LineEndPunctuationStyle::ForceHalfWidth), Some(true), Some(true), Some(LineAdjustmentStrategy::PushInFirst)), KinsokuMode::MeasureAdaptive { hang_below_em: 14.0f64, gb_above_em: 24.0f64, strict_above_em: 32.0f64 }, PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false)));
    }
}

impl ClreqProfileResolver for InterpunctPreserveResolver {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.InterpunctShrinkOpportunityTestSupport.InterpunctPreserveResolver"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ClreqProfileResolver> {
        Box::new(self.clone())
    }

    fn resolve(&self, _profile_id: LayoutProfileId) -> ClreqProfile {
        return ClreqProfile::new(((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().id).to_ustring().as_ustr(), ClreqStrictness::Normal, ClreqRegion::Mainland, Some(CjkPunctuationGlyphPolicy::PreserveInput), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some((*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone()), Some(PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::Mainland)), AdjustmentStylePolicy::new(Some(LineEndPunctuationStyle::ForceHalfWidth), Some(true), Some(true), Some(LineAdjustmentStrategy::PushInFirst)), KinsokuMode::MeasureAdaptive { hang_below_em: 14.0f64, gb_above_em: 24.0f64, strict_above_em: 32.0f64 }, PunctuationWidthPolicy::new(Some(InteriorPunctuationStyle::FullWidth), Some(false)));
    }
}

#[derive(Clone)]
pub struct InterpunctHaltShaper {
    pub(crate) delegate: Arc<Mutex<ExplainableStubTextShaper>>,
}

impl InterpunctHaltShaper {
    pub fn new() -> Self {
        Self {
            delegate: Arc::new(Mutex::new(ExplainableStubTextShaper::new())),
        }
    }

    pub fn shape(&self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let res = self.delegate.lock().unwrap().shape((input).clone())?;
        let source = u_string::substring(&(input.text).to_ustring(), i32::from_ne_bytes((((input.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((input.range).clone().end) as i32).to_ne_bytes()));
        let inter = source == UString::from("·") || source == UString::from("・");
        let ellipsis = source == UString::from("…");
        let mut runs: Vec<GlyphRun> = vec![];
        for ri in 0..match u32::try_from(res.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (res.glyph_runs[usize::try_from(ri).unwrap_or(0)]).clone();
            let mut glyphs: Vec<Glyph> = vec![];
            for gi in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let g = (run.glyphs[usize::try_from(gi).unwrap_or(0)]).clone();
                let bounds = Rect::new(if ellipsis { 2.0f64 } else { 4.0f64 }, 2.0f64, if ellipsis { 10.0f64 } else { 12.0f64 }, 10.0f64);
                glyphs.push(Glyph::new(g.id, (g.cluster_range).clone(), g.advance, Some(g.x), Some(g.y), g.render_font_key.clone(), Some((bounds).clone()), if inter || ellipsis { Some(8.0f64) } else { g.halt_advance }, if inter { Some(-4.0f64) } else { if ellipsis { Some(0.0f64) } else { g.halt_placement_x } }));
            }
            runs.push(GlyphRun::new((run.range).clone(), (run.font_key).to_ustring().as_ustr(), glyphs.to_vec(), run.advance, Some(vec![])));
        }
        return Ok(ShapingResult::new(res.clusters.to_vec(), runs.to_vec(), Some((res.decisions).clone())));
    }
}

impl ITextShaper for InterpunctHaltShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.InterpunctShrinkOpportunityTestSupport.InterpunctHaltShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let res = self.delegate.lock().unwrap().shape((input).clone())?;
        let source = u_string::substring(&(input.text).to_ustring(), i32::from_ne_bytes((((input.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((input.range).clone().end) as i32).to_ne_bytes()));
        let inter = source == UString::from("·") || source == UString::from("・");
        let ellipsis = source == UString::from("…");
        let mut runs: Vec<GlyphRun> = vec![];
        for ri in 0..match u32::try_from(res.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (res.glyph_runs[usize::try_from(ri).unwrap_or(0)]).clone();
            let mut glyphs: Vec<Glyph> = vec![];
            for gi in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let g = (run.glyphs[usize::try_from(gi).unwrap_or(0)]).clone();
                let bounds = Rect::new(if ellipsis { 2.0f64 } else { 4.0f64 }, 2.0f64, if ellipsis { 10.0f64 } else { 12.0f64 }, 10.0f64);
                glyphs.push(Glyph::new(g.id, (g.cluster_range).clone(), g.advance, Some(g.x), Some(g.y), g.render_font_key.clone(), Some((bounds).clone()), if inter || ellipsis { Some(8.0f64) } else { g.halt_advance }, if inter { Some(-4.0f64) } else { if ellipsis { Some(0.0f64) } else { g.halt_placement_x } }));
            }
            runs.push(GlyphRun::new((run.range).clone(), (run.font_key).to_ustring().as_ustr(), glyphs.to_vec(), run.advance, Some(vec![])));
        }
        return Ok(ShapingResult::new(res.clusters.to_vec(), runs.to_vec(), Some((res.decisions).clone())));
    }
}
