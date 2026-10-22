use crate::org::tiqian::clreq::clreq_profile::ClreqProfile;
use crate::org::tiqian::clreq::clreq_profile_resolver::ClreqProfileResolver;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::glyph_run::GlyphRun;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_profile_id::LayoutProfileId;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::layout::default_hyphenator::DefaultHyphenator;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::linebreak::english_hyphenation::EnglishHyphenation;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::shaping::text_shaper::ITextShaper;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::org::tiqian::shaping::text_shaper::ShapingResult;
use crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageJfTestSupportEngineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for LineAdjustmentStageJfTestSupportEngineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineAdjustmentStageJfTestSupportEngineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineAdjustmentStageJfTestSupportEngineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineAdjustmentStageJfTestSupportEngineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageJfTestSupportEngineFault) -> Self {
        match value {
            LineAdjustmentStageJfTestSupportEngineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageJfTestSupportEngineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageJfTestSupportEngineFault) -> Self {
        match value {
            LineAdjustmentStageJfTestSupportEngineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageJfTestSupportEngineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageJfTestSupportEngineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageJfTestSupportEngineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageJfTestSupportEngineFault::UStringFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct LineAdjustmentStageJfTestSupport;

impl LineAdjustmentStageJfTestSupport {
    pub fn line_adjustment_stage_jf_test_support_resolver(p: ClreqProfile) -> Box<dyn ClreqProfileResolver> {
        return Box::new(FixedClreqResolver::new((p).clone()));
    }

    pub fn line_adjustment_stage_jf_test_support_engine(shaper: Option<Arc<Mutex<dyn ITextShaper>>>, hyphenate: bool, p: Option<ClreqProfile>) -> Result<ExplainableStubParagraphLayoutEngine, ParagraphLayoutEngineNewFault> {
        let r: Option<Box<dyn ClreqProfileResolver>> = match &(p) { None => None, Some(__option) => Some(LineAdjustmentStageJfTestSupport::line_adjustment_stage_jf_test_support_resolver(((*__option).clone()).clone())) };
        return Ok(match &(shaper) { Some(__option1) => ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), r, Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some((*__option1).clone()), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?, None => if hyphenate { ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), r, Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(EnglishHyphenation::english_hyphenation_en_us().map_err(|e| ParagraphLayoutEngineNewFault::UStringFaultFault(e))?), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))? } else { ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), r, Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))? } });
    }

    pub fn line_adjustment_stage_jf_test_support_layout(text: &UStr, width: f64, objects: Option<Vec<InlineObjectSpan>>, hyphenate: Option<bool>, shaper: Option<Arc<Mutex<dyn ITextShaper>>>, profile: Option<ClreqProfile>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        let mut e = LineAdjustmentStageJfTestSupport::line_adjustment_stage_jf_test_support_engine((shaper).clone(), hyphenate.unwrap_or(false), (profile).clone())?;
        return Ok(e.layout(LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(width, Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), (objects).clone())).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }
}

#[derive(Clone, PartialEq)]
pub struct FixedClreqResolver {
    pub(crate) profile: ClreqProfile,
}

impl FixedClreqResolver {
    pub fn new(p: ClreqProfile) -> Self {
        Self {
            profile: p,
        }
    }

    pub fn resolve(&self, _id: LayoutProfileId) -> ClreqProfile {
        return ((self.profile).clone()).clone();
    }
}

impl ClreqProfileResolver for FixedClreqResolver {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.LineAdjustmentStageJfTestSupport.FixedClreqResolver"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ClreqProfileResolver> {
        Box::new(self.clone())
    }

    fn resolve(&self, _id: LayoutProfileId) -> ClreqProfile {
        return ((self.profile).clone()).clone();
    }
}

#[derive(Clone)]
pub struct DashBoundsShaper {
    pub(crate) delegate: Arc<Mutex<ExplainableStubTextShaper>>,
    pub(crate) wide: bool,
}

impl DashBoundsShaper {
    pub fn new(wide: bool) -> Self {
        Self {
            wide,
            delegate: Arc::new(Mutex::new(ExplainableStubTextShaper::new())),
        }
    }

    pub fn shape(&self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let r = self.delegate.lock().unwrap().shape((input).clone())?;
        let mut runs: Vec<GlyphRun> = vec![];
        for ri in 0..match u32::try_from(r.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (r.glyph_runs[usize::try_from(ri).unwrap_or(0)]).clone();
            let mut gs: Vec<Glyph> = vec![];
            for gi in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let g = (run.glyphs[usize::try_from(gi).unwrap_or(0)]).clone();
                gs.push(if u32::from_ne_bytes(((u_string::find_from(&((input.display_text).to_ustring()), UString::from("⸺").as_ustr(), 0)) as u32).to_ne_bytes()) <= 2147483647 { Glyph::new(g.id, (g.cluster_range).clone(), g.advance, Some(g.x), Some(g.y), g.render_font_key.clone(), Some(Rect::new(if self.wide { 0.0f64 } else { 1.0f64 }, 0 as f64 as f64, if self.wide { 31.5f64 } else { 29 as f64 }, 16 as f64 as f64)), g.halt_advance, g.halt_placement_x) } else { g });
            }
            runs.push(GlyphRun::new((run.range).clone(), (run.font_key).to_ustring().as_ustr(), gs.to_vec(), run.advance, Some(vec![])));
        }
        return Ok(ShapingResult::new(r.clusters.to_vec(), runs.to_vec(), Some((r.decisions).clone())));
    }
}

impl ITextShaper for DashBoundsShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.LineAdjustmentStageJfTestSupport.DashBoundsShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let r = self.delegate.lock().unwrap().shape((input).clone())?;
        let mut runs: Vec<GlyphRun> = vec![];
        for ri in 0..match u32::try_from(r.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (r.glyph_runs[usize::try_from(ri).unwrap_or(0)]).clone();
            let mut gs: Vec<Glyph> = vec![];
            for gi in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let g = (run.glyphs[usize::try_from(gi).unwrap_or(0)]).clone();
                gs.push(if u32::from_ne_bytes(((u_string::find_from(&((input.display_text).to_ustring()), UString::from("⸺").as_ustr(), 0)) as u32).to_ne_bytes()) <= 2147483647 { Glyph::new(g.id, (g.cluster_range).clone(), g.advance, Some(g.x), Some(g.y), g.render_font_key.clone(), Some(Rect::new(if self.wide { 0.0f64 } else { 1.0f64 }, 0 as f64 as f64, if self.wide { 31.5f64 } else { 29 as f64 }, 16 as f64 as f64)), g.halt_advance, g.halt_placement_x) } else { g });
            }
            runs.push(GlyphRun::new((run.range).clone(), (run.font_key).to_ustring().as_ustr(), gs.to_vec(), run.advance, Some(vec![])));
        }
        return Ok(ShapingResult::new(r.clusters.to_vec(), runs.to_vec(), Some((r.decisions).clone())));
    }
}
