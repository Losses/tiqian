use crate::org::tiqian::clreq::adjustment_style_policy::AdjustmentStylePolicy;
use crate::org::tiqian::clreq::clreq_profile::ClreqProfile;
use crate::org::tiqian::clreq::clreq_profile_resolver::ClreqProfileResolver;
use crate::org::tiqian::clreq::line_adjustment_strategy::LineAdjustmentStrategy;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::glyph_run::GlyphRun;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::justification_allocation_info::JustificationAllocationInfo;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_profile_id::LayoutProfileId;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
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
use crate::org::tiqian::linebreak::hyphenator::NoHyphenator;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::shaping::text_shaper::ITextShaper;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::org::tiqian::shaping::text_shaper::ShapingResult;
use crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::runtime::u_string;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub enum JustifierEngineTestSupportEngineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}

impl From<JustifierEngineTestSupportEngineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierEngineTestSupportEngineFault) -> Self {
        match value {
            JustifierEngineTestSupportEngineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierEngineTestSupportEngineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierEngineTestSupportEngineFault) -> Self {
        match value {
            JustifierEngineTestSupportEngineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierEngineTestSupportEngineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierEngineTestSupportEngineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierEngineTestSupportEngineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierEngineTestSupportEngineFault::UStringFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct JustifierEngineTestSupport;

impl JustifierEngineTestSupport {
    pub fn justifier_engine_test_support_start(n: &str) -> TestTraceRecorder {
        let mut t = TestTraceRecorder::new("JustifierEngineTest");
        t.section(n);
        return t;
    }

    pub fn justifier_engine_test_support_engine() -> Result<ExplainableStubParagraphLayoutEngine, ParagraphLayoutEngineNewFault> {
        let p = (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone();
        let r: Box<dyn ClreqProfileResolver> = Box::new(Fixed::new(ClreqProfile::new((p.id).to_string().as_str(), p.strictness, p.region, Some(p.punctuation_glyph_policy), Some((p.coalesce_repeatable_punctuation).clone()), Some((p.auto_space).clone()), Some(p.glue_placement),
AdjustmentStylePolicy::new(Some((p.adjustment).clone().line_end_punctuation), Some((p.adjustment).clone().allow_inline_stop_compression), Some((p.adjustment).clone().allow_sino_western_gap_adjustment), Some(LineAdjustmentStrategy::PushOutOnly)), (p.kinsoku_mode).clone(),
(p.punctuation_width).clone())));
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string()))?)),
Some((r).clone()), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()),
Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512))))?);
    }

    pub fn justifier_engine_test_support_positioned() -> Result<ExplainableStubParagraphLayoutEngine, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string()))?)),
Some(Box::new(Fixed::new(ClreqProfile::new(((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().id).to_string().as_str(), (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().strictness,
(*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().region, Some((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().punctuation_glyph_policy),
Some(((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().coalesce_repeatable_punctuation).clone()), Some(((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().auto_space).clone()),
Some((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().glue_placement), AdjustmentStylePolicy::new(Some(((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().adjustment).clone().line_end_punctuation),
Some(((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().adjustment).clone().allow_inline_stop_compression),
Some(((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().adjustment).clone().allow_sino_western_gap_adjustment), Some(LineAdjustmentStrategy::PushOutOnly)),
((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().kinsoku_mode).clone(), ((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().punctuation_width).clone())))), Some(Box::new(StubFontMetricsResolver::new())),
Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))),
Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(PositionedPairShaper::new())), Some(Box::new(NoHyphenator::new())), Some(Box::new(LruWidthIndependentAnnotationCache::new(512))))?);
    }

    pub fn justifier_engine_test_support_layout(text: &str, width: f64) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        return Ok(JustifierEngineTestSupport::justifier_engine_test_support_engine()?.layout(LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400),
Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(width, Some(f64::INFINITY),
Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).map_err(|e|
ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn justifier_engine_test_support_allocations_text(list: &[JustificationAllocationInfo]) -> String {
        let mut out: Vec<String> = vec![];
        for i in 0..match u32::try_from(list.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            out.push((list[usize::try_from(i).unwrap_or(0)]).clone().to_string());
        }
        return format!("{}{}{}",
            "[",
            { let joined = out; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(&(", ")); } let _ = write!(out, "{}", joined[index]); index += 1; } out },
            "]"
        );
    }
}

#[derive(Clone, PartialEq)]
pub struct Fixed {
    pub(crate) p: ClreqProfile,
}

impl Fixed {
    pub fn new(p: ClreqProfile) -> Self {
        Self {
            p,
        }
    }

    pub fn resolve(&self, _id: LayoutProfileId) -> ClreqProfile {
        return ((self.p).clone()).clone();
    }
}

impl ClreqProfileResolver for Fixed {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.JustifierEngineTestSupport.Fixed"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ClreqProfileResolver> {
        Box::new(self.clone())
    }

    fn resolve(&self, _id: LayoutProfileId) -> ClreqProfile {
        return ((self.p).clone()).clone();
    }
}

#[derive(Clone, PartialEq)]
pub struct PositionedPairShaper {
}

impl PositionedPairShaper {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn shape(&self, input: ShapingInput) -> ShapingResult {
        let text = u_string::substring(&(input.text).to_string(), i32::from_ne_bytes(((input.range).clone().start).to_ne_bytes()), i32::from_ne_bytes(((input.range).clone().end).to_ne_bytes()));
        let advance = if text == "AV" { 10 } else { u32::wrapping_mul(u_string::unit_count(&(text)), 16) };
        let mut gs: Vec<Glyph> = vec![];
        if text == "AV" {
            gs.push(Glyph::new(1u32, (input.range).clone(), 5 as f64 as f64, Some(0 as f64), Some(0 as f64), None, None, None, None));
            gs.push(Glyph::new(2u32, (input.range).clone(), 5 as f64 as f64, Some(5 as f64), Some(0 as f64), None, None, None, None));
        } else {
            gs.push(Glyph::new(3u32, (input.range).clone(), { let v: u32 = advance; i32::from_ne_bytes(v.to_ne_bytes()) } as f64 as f64, Some(0 as f64), Some(0 as f64), None, None, None, None));
        }
        return ShapingResult::new(vec![
    (Cluster::new((input.range).clone(), text.as_str(), (((input.font_decision).clone().candidate).clone().key).to_string().as_str(), { let v: u32 = advance; i32::from_ne_bytes(v.to_ne_bytes()) } as f64 as f64, Some((input.display_text).to_string()), Some(0.0), Some(0.0),
Some(0.0))).clone(),
].to_vec(), vec![
    (GlyphRun::new((input.range).clone(), (((input.font_decision).clone().candidate).clone().key).to_string().as_str(), gs.to_vec(), { let v: u32 = advance; i32::from_ne_bytes(v.to_ne_bytes()) } as f64 as f64, Some(vec![]))).clone(),
].to_vec(), Some(vec![]));
    }
}

impl ITextShaper for PositionedPairShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.JustifierEngineTestSupport.PositionedPairShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let text = u_string::substring(&(input.text).to_string(), i32::from_ne_bytes(((input.range).clone().start).to_ne_bytes()), i32::from_ne_bytes(((input.range).clone().end).to_ne_bytes()));
        let advance = if text == "AV" { 10 } else { u32::wrapping_mul(u_string::unit_count(&(text)), 16) };
        let mut gs: Vec<Glyph> = vec![];
        if text == "AV" {
            gs.push(Glyph::new(1u32, (input.range).clone(), 5 as f64 as f64, Some(0 as f64), Some(0 as f64), None, None, None, None));
            gs.push(Glyph::new(2u32, (input.range).clone(), 5 as f64 as f64, Some(5 as f64), Some(0 as f64), None, None, None, None));
        } else {
            gs.push(Glyph::new(3u32, (input.range).clone(), { let v: u32 = advance; i32::from_ne_bytes(v.to_ne_bytes()) } as f64 as f64, Some(0 as f64), Some(0 as f64), None, None, None, None));
        }
        return Ok(ShapingResult::new(vec![
    (Cluster::new((input.range).clone(), text.as_str(), (((input.font_decision).clone().candidate).clone().key).to_string().as_str(), { let v: u32 = advance; i32::from_ne_bytes(v.to_ne_bytes()) } as f64 as f64, Some((input.display_text).to_string()), Some(0.0), Some(0.0),
Some(0.0))).clone(),
].to_vec(), vec![
    (GlyphRun::new((input.range).clone(), (((input.font_decision).clone().candidate).clone().key).to_string().as_str(), gs.to_vec(), { let v: u32 = advance; i32::from_ne_bytes(v.to_ne_bytes()) } as f64 as f64, Some(vec![]))).clone(),
].to_vec(), Some(vec![])));
    }
}
