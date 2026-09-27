use crate::org::tiqian::clreq::cjk_punctuation_glyph_policy::CjkPunctuationGlyphPolicy;
use crate::org::tiqian::clreq::clreq_profile::ClreqProfile;
use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::clreq::clreq_profile_resolver::ClreqProfileResolver;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::font_decision_info::FontDecisionInfo;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::glyph_run::GlyphRun;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::justification_decision_info::JustificationDecisionInfo;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_profile_id::LayoutProfileId;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::punctuation_decision_info::PunctuationDecisionInfo;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::shaping_decision_info::ShapingDecisionInfo;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::layout::default_hyphenator::DefaultHyphenator;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::line_breaker::LookaheadLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::shaping::text_shaper::ITextShaper;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::org::tiqian::shaping::text_shaper::ShapingResult;
use crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestSupportShaperEngineFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestSupportProfileEngineFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestSupportLookaheadShaperEngineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestSupportLookaheadShaperEngineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestSupportLookaheadShaperEngineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSupportLookaheadShaperEngineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSupportLookaheadShaperEngineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestSupportLookaheadShaperEngineFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSupportLookaheadShaperEngineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSupportLookaheadShaperEngineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestSupportLookaheadShaperEngineFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSupportLookaheadShaperEngineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestSupportLookaheadShaperEngineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestSupportLookaheadShaperEngineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestSupportLookaheadShaperEngineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestSupportLookaheadShaperEngineFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault {
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault::IllegalStateExceptionFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        DisplayGlyphSubstitutionEngineTestSupportDefaultEngineFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayGlyphSubstitutionEngineTestSupportShapeFault {
    TextShaperShapeFaultFault(crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
}
impl std::fmt::Display for DisplayGlyphSubstitutionEngineTestSupportShapeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayGlyphSubstitutionEngineTestSupportShapeFault::TextShaperShapeFaultFault(value) => write!(formatter, "{}", value),
            DisplayGlyphSubstitutionEngineTestSupportShapeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSupportShapeFault> for crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault {
    fn from(value: DisplayGlyphSubstitutionEngineTestSupportShapeFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSupportShapeFault::TextShaperShapeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<DisplayGlyphSubstitutionEngineTestSupportShapeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: DisplayGlyphSubstitutionEngineTestSupportShapeFault) -> Self {
        match value {
            DisplayGlyphSubstitutionEngineTestSupportShapeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault> for DisplayGlyphSubstitutionEngineTestSupportShapeFault {
    fn from(value: crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault) -> Self {
        DisplayGlyphSubstitutionEngineTestSupportShapeFault::TextShaperShapeFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for DisplayGlyphSubstitutionEngineTestSupportShapeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        DisplayGlyphSubstitutionEngineTestSupportShapeFault::TextRangeErrorFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct DisplayGlyphSubstitutionEngineTestSupport;

impl DisplayGlyphSubstitutionEngineTestSupport {
    pub fn display_glyph_substitution_engine_test_support_default_engine() -> Result<ExplainableStubParagraphLayoutEngine, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?);
    }

    pub fn display_glyph_substitution_engine_test_support_profile_engine(policy: CjkPunctuationGlyphPolicy, coalesce: Option<Vec<u32>>) -> Result<ExplainableStubParagraphLayoutEngine, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(GlyphPolicyResolver::new(policy, (coalesce).clone()))), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?);
    }

    pub fn display_glyph_substitution_engine_test_support_shaper_engine(shaper: Arc<Mutex<dyn ITextShaper>>) -> Result<ExplainableStubParagraphLayoutEngine, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some((shaper).clone()), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?);
    }

    pub fn display_glyph_substitution_engine_test_support_lookahead_shaper_engine(shaper: Arc<Mutex<dyn ITextShaper>>) -> Result<ExplainableStubParagraphLayoutEngine, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)))), Some(Justifier::new(Some(0.5), Some(0.25))), Some((shaper).clone()), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?);
    }

    pub fn display_glyph_substitution_engine_test_support_layout320(engine: &mut ExplainableStubParagraphLayoutEngine, text: &UStr) -> Result<LayoutResult, ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> {
        return Ok(engine.layout(LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])))?);
    }

    pub fn display_glyph_substitution_engine_test_support_layout320_with_spans(engine: &mut ExplainableStubParagraphLayoutEngine, text: &UStr, spans: &Vec<TextSpan>) -> Result<LayoutResult, ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> {
        return Ok(engine.layout(LayoutInput::new(TiqianTextContent::new(text, Some((spans).clone()), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])))?);
    }

    pub fn display_glyph_substitution_engine_test_support_layout_without_grid(engine: &mut ExplainableStubParagraphLayoutEngine, text: &UStr, max_width: f64) -> Result<LayoutResult, ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> {
        return Ok(engine.layout(LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(max_width, Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])))?);
    }

    pub fn display_glyph_substitution_engine_test_support_find_justified_dash_hit(engine: &mut ExplainableStubParagraphLayoutEngine, text: &UStr) -> Result<JustifiedDashHit, DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault> {
        let mut cells = 13u32;
        while (i32::from_ne_bytes(((cells) as i32).to_ne_bytes())) <= 30 {
            let result = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_layout_without_grid(engine, text, i32::from_ne_bytes(((u32::wrapping_add(u32::wrapping_mul(cells, 16), 7)) as i32).to_ne_bytes()) as f64).map_err(|e| DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(e))?;
            let dash = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_single_cluster_with_text((result).clone(), UStr::new(&[8212,8212])).map_err(|e| DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault::IllegalStateExceptionFault(e))?;
            let found = DisplayGlyphSubstitutionEngineTestSupport::display_glyph_substitution_engine_test_support_first_justification_decision_covering((result).clone(), (dash.range).clone());
            match &(found) {
                Some(__option) => {
                    if i32::from_ne_bytes(((u32::try_from((__option.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
                    let decision = (*__option).clone();
                    return Ok(JustifiedDashHit { dash: dash, decision: decision });
                    }
                }
                None => {
                }
            }
            cells = u32::wrapping_add(cells, 1);
        }
        return Err(DisplayGlyphSubstitutionEngineTestSupportFindJustifiedDashHitFault::IllegalStateExceptionFault(IllegalStateException::new("no width produced a justified line containing the dash")));
    }

    pub fn display_glyph_substitution_engine_test_support_first_cluster_with_text(r: LayoutResult, s: &UStr) -> Result<Cluster, IllegalStateException> {
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_ustring() == s {
                return Ok((r.clusters[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        return Err(IllegalStateException::new(&({ let mut __s = UString::new(); __s += &(UString::from("No cluster with text ")); __s += s; __s }).to_utf8_lossy()));
    }

    pub fn display_glyph_substitution_engine_test_support_single_cluster(r: LayoutResult) -> Result<Cluster, IllegalStateException> {
        if u32::try_from((r.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) != 1 {
            return Err(IllegalStateException::new(&({ let mut __s = UString::new(); __s += &(UString::from("Expected a single cluster, found ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(u32::try_from((r.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0))).as_str())); __s }).to_utf8_lossy()));
        }
        return Ok((r.clusters[0usize]).clone());
    }

    pub fn display_glyph_substitution_engine_test_support_single_cluster_with_text(r: LayoutResult, s: &UStr) -> Result<Cluster, IllegalStateException> {
        let mut found: Option<Cluster> = None;
        let mut count = 0u32;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_ustring() == s {
                found = Some((r.clusters[usize::try_from(i).unwrap_or(0)]).clone());
                count = u32::wrapping_add(count, 1);
            }
        }
        if count != 1 {
            return Err(IllegalStateException::new(&({ let mut __s = UString::new(); __s += &(UString::from("Expected a single cluster with text ")); __s += s; __s += &(UString::from(", found ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(count)).as_str())); __s }).to_utf8_lossy()));
        }
        return Ok(found.as_ref().unwrap().clone());
    }

    pub fn display_glyph_substitution_engine_test_support_single_font_decision_with_source_text(r: LayoutResult, s: &UStr) -> Result<FontDecisionInfo, IllegalStateException> {
        let mut found: Option<FontDecisionInfo> = None;
        let mut count = 0u32;
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().font_decisions[usize::try_from(i).unwrap_or(0)].clone().source_text.to_ustring() == s {
                found = Some(((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone());
                count = u32::wrapping_add(count, 1);
            }
        }
        if count != 1 {
            return Err(IllegalStateException::new(&({ let mut __s = UString::new(); __s += &(UString::from("Expected a single font decision with source text ")); __s += s; __s += &(UString::from(", found ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(count)).as_str())); __s }).to_utf8_lossy()));
        }
        return Ok(found.as_ref().unwrap().clone());
    }

    pub fn display_glyph_substitution_engine_test_support_single_punctuation_decision(r: LayoutResult) -> Result<PunctuationDecisionInfo, IllegalStateException> {
        if u32::try_from(((r.debug).clone().punctuation_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) != 1 {
            return Err(IllegalStateException::new(&({ let mut __s = UString::new(); __s += &(UString::from("Expected a single punctuation decision, found ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(u32::try_from(((r.debug).clone().punctuation_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0))).as_str())); __s }).to_utf8_lossy()));
        }
        return Ok(((r.debug).clone().punctuation_decisions[0usize]).clone());
    }

    pub fn display_glyph_substitution_engine_test_support_single_glyph_with_cluster_range(r: LayoutResult, range: TextRange) -> Result<Glyph, IllegalStateException> {
        let mut found: Option<Glyph> = None;
        let mut count = 0u32;
        for ri in 0..match u32::try_from(r.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let glyphs = ((r.glyph_runs[usize::try_from(ri).unwrap_or(0)]).clone().glyphs).clone();
            for gi in 0..match u32::try_from(glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if glyphs[usize::try_from(gi).unwrap_or(0)].clone().cluster_range.clone().start == range.start && ((glyphs[usize::try_from(gi).unwrap_or(0)]).clone().cluster_range).clone().end == range.end {
                    found = Some((glyphs[usize::try_from(gi).unwrap_or(0)]).clone());
                    count = u32::wrapping_add(count, 1);
                }
            }
        }
        if count != 1 {
            return Err(IllegalStateException::new(&({ let mut __s = UString::new(); __s += &(UString::from("Expected a single glyph for range ")); __s += UString::from(format!("{}", range.to_string()).as_str()).as_ustr(); __s += &(UString::from(", found ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(count)).as_str())); __s }).to_utf8_lossy()));
        }
        return Ok(found.as_ref().unwrap().clone());
    }

    pub fn display_glyph_substitution_engine_test_support_first_justification_decision_covering(r: LayoutResult, range: TextRange) -> Option<JustificationDecisionInfo> {
        for i in 0..match u32::try_from((r.debug).clone().justification_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().justification_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if i32::from_ne_bytes(((range.start) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((d.line_range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((range.end) as i32).to_ne_bytes())) <= i32::from_ne_bytes((((d.line_range).clone().end) as i32).to_ne_bytes()) {
                return Some(d);
            }
        }
        return None;
    }

    pub fn display_glyph_substitution_engine_test_support_render_nullable_floats(a: &Vec<Option<f64>>) -> Result<UString, UStringFault> {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push(if a[usize::try_from(i).unwrap_or(0)].is_none() { UString::from("-") } else { TestTraceRender::test_trace_render_render_float(a[usize::try_from(i).unwrap_or(0)].unwrap())?.to_ustring() });
        }
        return Ok(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined = parts; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined[index]); index += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str()));
    }

    pub fn display_glyph_substitution_engine_test_support_render_string_list_array(a: &Vec<Vec<UString>>) -> Result<UString, UStringFault> {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push(TestTraceRender::test_trace_render_render_string_array(&(a[usize::try_from(i).unwrap_or(0)]).clone())?);
        }
        return Ok(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined1 = parts; let mut out = String::new(); let n = joined1.len(); let mut index1 = 0usize; while index1 < n { if index1 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined1[index1]); index1 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str()));
    }
}

#[derive(Clone, PartialEq)]
pub struct GlyphPolicyResolver {
    pub(crate) policy: CjkPunctuationGlyphPolicy,
    pub(crate) coalesce: Option<Vec<u32>>,
}

impl GlyphPolicyResolver {
    pub fn new(policy: CjkPunctuationGlyphPolicy, coalesce: Option<Vec<u32>>) -> Self {
        Self {
            policy,
            coalesce,
        }
    }

    pub fn resolve(&self, _profile_id: LayoutProfileId) -> ClreqProfile {
        let base = (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone();
        return ClreqProfile::new((base.id).to_ustring().as_ustr(), base.strictness, base.region, Some(self.policy), (self.coalesce).clone(), Some((*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone()), Some(base.glue_placement), (base.adjustment).clone(), (base.kinsoku_mode).clone(), (base.punctuation_width).clone());
    }
}

impl ClreqProfileResolver for GlyphPolicyResolver {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.DisplayGlyphSubstitutionEngineTestSupport.GlyphPolicyResolver"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ClreqProfileResolver> {
        Box::new(self.clone())
    }

    fn resolve(&self, _profile_id: LayoutProfileId) -> ClreqProfile {
        let base = (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone();
        return ClreqProfile::new((base.id).to_ustring().as_ustr(), base.strictness, base.region, Some(self.policy), (self.coalesce).clone(), Some((*crate::org::tiqian::clreq::auto_space_policy::AUTO_SPACE_POLICY_DEFAULT).clone()), Some(base.glue_placement), (base.adjustment).clone(), (base.kinsoku_mode).clone(), (base.punctuation_width).clone());
    }
}

#[derive(Clone)]
pub struct PerGlyphQuoteRunShaper {
    pub(crate) delegate: Arc<Mutex<ExplainableStubTextShaper>>,
}

impl PerGlyphQuoteRunShaper {
    pub fn new() -> Self {
        Self {
            delegate: Arc::new(Mutex::new(ExplainableStubTextShaper::new())),
        }
    }

    pub fn shape(&self, input: ShapingInput) -> Result<ShapingResult, DisplayGlyphSubstitutionEngineTestSupportShapeFault> {
        if input.display_text.to_ustring() != UString::from("A’B") {
            return Ok(self.delegate.lock().unwrap().shape((input).clone()).map_err(|e| DisplayGlyphSubstitutionEngineTestSupportShapeFault::TextShaperShapeFaultFault(e))?);
        }
        let mut clusters: Vec<Cluster> = vec![];
        let mut index = (input.range).clone().start;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes((((input.range).clone().end) as i32).to_ne_bytes())) {
            clusters.push(Cluster::new(TextRange::new(index, u32::wrapping_add(index, 1)).map_err(|e| DisplayGlyphSubstitutionEngineTestSupportShapeFault::TextRangeErrorFault(e))?, u_string::substring(&(input.text).to_ustring(), i32::from_ne_bytes(((index) as i32).to_ne_bytes()), i32::from_ne_bytes(((u32::wrapping_add(index, 1)) as i32).to_ne_bytes())).as_ustr(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), 16 as f64 as f64, Some((u_string::substring(&(input.display_text).to_ustring(), i32::from_ne_bytes(((u32::wrapping_sub(index, (input.range).clone().start)) as i32).to_ne_bytes()), i32::from_ne_bytes(((u32::wrapping_sub(u32::wrapping_add(index, 1), (input.range).clone().start)) as i32).to_ne_bytes()))).to_ustring()), Some(0.0), Some(0.0), Some(0.0)));
            index = u32::wrapping_add(index, 1);
        }
        let mut runs: Vec<GlyphRun> = vec![];
        for glyph_id in 0..match u32::try_from(clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let cluster = (clusters[usize::try_from(glyph_id).unwrap_or(0)]).clone();
            let features = if cluster.text.to_ustring() == UString::from("’") { vec![UString::from("pwid").to_ustring(), UString::from("palt").to_ustring()] } else { vec![] };
            runs.push(GlyphRun::new((cluster.range).clone(), (cluster.font_key).to_ustring().as_ustr(), vec![
    (Glyph::new(glyph_id, (cluster.range).clone(), cluster.advance, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), cluster.advance, Some((features).clone())));
        }
        return Ok(ShapingResult::new(clusters.to_vec(), runs.to_vec(), Some(vec![])));
    }
}

impl ITextShaper for PerGlyphQuoteRunShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.DisplayGlyphSubstitutionEngineTestSupport.PerGlyphQuoteRunShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        if input.display_text.to_ustring() != UString::from("A’B") {
            return Ok(self.delegate.lock().unwrap().shape((input).clone())?);
        }
        let mut clusters: Vec<Cluster> = vec![];
        let mut index = (input.range).clone().start;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes((((input.range).clone().end) as i32).to_ne_bytes())) {
            clusters.push(Cluster::new(TextRange::new(index, u32::wrapping_add(index, 1)).map_err(|e| TextShaperShapeFault::TextRangeErrorFault(e))?, u_string::substring(&(input.text).to_ustring(), i32::from_ne_bytes(((index) as i32).to_ne_bytes()), i32::from_ne_bytes(((u32::wrapping_add(index, 1)) as i32).to_ne_bytes())).as_ustr(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), 16 as f64 as f64, Some((u_string::substring(&(input.display_text).to_ustring(), i32::from_ne_bytes(((u32::wrapping_sub(index, (input.range).clone().start)) as i32).to_ne_bytes()), i32::from_ne_bytes(((u32::wrapping_sub(u32::wrapping_add(index, 1), (input.range).clone().start)) as i32).to_ne_bytes()))).to_ustring()), Some(0.0), Some(0.0), Some(0.0)));
            index = u32::wrapping_add(index, 1);
        }
        let mut runs: Vec<GlyphRun> = vec![];
        for glyph_id in 0..match u32::try_from(clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let cluster = (clusters[usize::try_from(glyph_id).unwrap_or(0)]).clone();
            let features = if cluster.text.to_ustring() == UString::from("’") { vec![UString::from("pwid").to_ustring(), UString::from("palt").to_ustring()] } else { vec![] };
            runs.push(GlyphRun::new((cluster.range).clone(), (cluster.font_key).to_ustring().as_ustr(), vec![
    (Glyph::new(glyph_id, (cluster.range).clone(), cluster.advance, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), cluster.advance, Some((features).clone())));
        }
        return Ok(ShapingResult::new(clusters.to_vec(), runs.to_vec(), Some(vec![])));
    }
}

#[derive(Clone, PartialEq)]
pub struct SingleClusterNoBoundsShaper {
}

impl SingleClusterNoBoundsShaper {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn shape(&self, input: ShapingInput) -> Result<ShapingResult, DisplayGlyphSubstitutionEngineTestSupportShapeFault> {
        return Ok(ShapingResult::new(vec![
    (Cluster::new((input.range).clone(), u_string::substring(&(input.text).to_ustring(), i32::from_ne_bytes((((input.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((input.range).clone().end) as i32).to_ne_bytes())).as_ustr(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), 16 as f64 as f64, Some((input.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0))).clone(),
].to_vec(), vec![
    (GlyphRun::new((input.range).clone(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), vec![
    (Glyph::new(0u32, (input.range).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
].to_vec(), Some(vec![])));
    }
}

impl ITextShaper for SingleClusterNoBoundsShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.DisplayGlyphSubstitutionEngineTestSupport.SingleClusterNoBoundsShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        return Ok(ShapingResult::new(vec![
    (Cluster::new((input.range).clone(), u_string::substring(&(input.text).to_ustring(), i32::from_ne_bytes((((input.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((input.range).clone().end) as i32).to_ne_bytes())).as_ustr(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), 16 as f64 as f64, Some((input.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0))).clone(),
].to_vec(), vec![
    (GlyphRun::new((input.range).clone(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), vec![
    (Glyph::new(0u32, (input.range).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
].to_vec(), Some(vec![])));
    }
}

#[derive(Clone, PartialEq)]
pub struct SingleClusterAmbiguousShaper {
}

impl SingleClusterAmbiguousShaper {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn shape(&self, input: ShapingInput) -> Result<ShapingResult, DisplayGlyphSubstitutionEngineTestSupportShapeFault> {
        return Ok(ShapingResult::new(vec![
    (Cluster::new((input.range).clone(), u_string::substring(&(input.text).to_ustring(), i32::from_ne_bytes((((input.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((input.range).clone().end) as i32).to_ne_bytes())).as_ustr(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), 32 as f64 as f64, Some((input.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0))).clone(),
].to_vec(), vec![
    (GlyphRun::new((input.range).clone(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), vec![
    (Glyph::new(0u32, (input.range).clone(), 32 as f64 as f64, Some(0.0), Some(0.0), None, Some(Rect::new(2 as f64 as f64, i32::from_ne_bytes(((4294967286u32) as i32).to_ne_bytes()) as f64 as f64, 30 as f64 as f64, i32::from_ne_bytes(((4294967290u32) as i32).to_ne_bytes()) as f64 as f64)), None, None)).clone(),
].to_vec(), 32 as f64 as f64, Some(vec![]))).clone(),
].to_vec(), Some(vec![])));
    }
}

impl ITextShaper for SingleClusterAmbiguousShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.DisplayGlyphSubstitutionEngineTestSupport.SingleClusterAmbiguousShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        return Ok(ShapingResult::new(vec![
    (Cluster::new((input.range).clone(), u_string::substring(&(input.text).to_ustring(), i32::from_ne_bytes((((input.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((input.range).clone().end) as i32).to_ne_bytes())).as_ustr(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), 32 as f64 as f64, Some((input.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0))).clone(),
].to_vec(), vec![
    (GlyphRun::new((input.range).clone(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), vec![
    (Glyph::new(0u32, (input.range).clone(), 32 as f64 as f64, Some(0.0), Some(0.0), None, Some(Rect::new(2 as f64 as f64, i32::from_ne_bytes(((4294967286u32) as i32).to_ne_bytes()) as f64 as f64, 30 as f64 as f64, i32::from_ne_bytes(((4294967290u32) as i32).to_ne_bytes()) as f64 as f64)), None, None)).clone(),
].to_vec(), 32 as f64 as f64, Some(vec![]))).clone(),
].to_vec(), Some(vec![])));
    }
}

#[derive(Clone)]
pub struct MissingGlyphReportingShaper {
    pub(crate) delegate: Arc<Mutex<ExplainableStubTextShaper>>,
}

impl MissingGlyphReportingShaper {
    pub fn new() -> Self {
        Self {
            delegate: Arc::new(Mutex::new(ExplainableStubTextShaper::new())),
        }
    }

    pub fn shape(&self, input: ShapingInput) -> Result<ShapingResult, DisplayGlyphSubstitutionEngineTestSupportShapeFault> {
        let res = self.delegate.lock().unwrap().shape((input).clone()).map_err(|e| DisplayGlyphSubstitutionEngineTestSupportShapeFault::TextShaperShapeFaultFault(e))?;
        if u32::from_ne_bytes(((u_string::find_from(&((input.display_text).to_ustring()), UString::from("⸺").as_ustr(), 0)) as u32).to_ne_bytes()) > 2147483647 {
            return Ok(res);
        }
        let mut decisions: Vec<ShapingDecisionInfo> = vec![];
        for i in 0..match u32::try_from(res.decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = (res.decisions[usize::try_from(i).unwrap_or(0)]).clone();
            decisions.push(ShapingDecisionInfo::new((d.range).clone(), (d.source_text).to_ustring().as_ustr(), (d.display_text).to_ustring().as_ustr(), (d.font_key).to_ustring().as_ustr(), d.glyph_count, d.advance, (d.source).to_ustring().as_ustr(), (d.reason).to_ustring().as_ustr(), Some(d.glyphs_without_ink_bounds), Some(1), d.resolved_face.clone(), d.script.clone(), d.language.clone(), d.strategy.clone(), d.feature_evidence.clone(), d.capability_issue.clone()));
        }
        return Ok(ShapingResult::new(res.clusters.to_vec(), res.glyph_runs.to_vec(), Some((decisions).clone())));
    }
}

impl ITextShaper for MissingGlyphReportingShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.DisplayGlyphSubstitutionEngineTestSupport.MissingGlyphReportingShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let res = self.delegate.lock().unwrap().shape((input).clone())?;
        if u32::from_ne_bytes(((u_string::find_from(&((input.display_text).to_ustring()), UString::from("⸺").as_ustr(), 0)) as u32).to_ne_bytes()) > 2147483647 {
            return Ok(res);
        }
        let mut decisions: Vec<ShapingDecisionInfo> = vec![];
        for i in 0..match u32::try_from(res.decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = (res.decisions[usize::try_from(i).unwrap_or(0)]).clone();
            decisions.push(ShapingDecisionInfo::new((d.range).clone(), (d.source_text).to_ustring().as_ustr(), (d.display_text).to_ustring().as_ustr(), (d.font_key).to_ustring().as_ustr(), d.glyph_count, d.advance, (d.source).to_ustring().as_ustr(), (d.reason).to_ustring().as_ustr(), Some(d.glyphs_without_ink_bounds), Some(1), d.resolved_face.clone(), d.script.clone(), d.language.clone(), d.strategy.clone(), d.feature_evidence.clone(), d.capability_issue.clone()));
        }
        return Ok(ShapingResult::new(res.clusters.to_vec(), res.glyph_runs.to_vec(), Some((decisions).clone())));
    }
}

#[derive(Clone)]
pub struct UnverifiedCoverageReportingShaper {
    pub(crate) delegate: Arc<Mutex<ExplainableStubTextShaper>>,
}

impl UnverifiedCoverageReportingShaper {
    pub fn new() -> Self {
        Self {
            delegate: Arc::new(Mutex::new(ExplainableStubTextShaper::new())),
        }
    }

    pub fn shape(&self, input: ShapingInput) -> Result<ShapingResult, DisplayGlyphSubstitutionEngineTestSupportShapeFault> {
        let res = self.delegate.lock().unwrap().shape((input).clone()).map_err(|e| DisplayGlyphSubstitutionEngineTestSupportShapeFault::TextShaperShapeFaultFault(e))?;
        if u32::from_ne_bytes(((u_string::find_from(&((input.display_text).to_ustring()), UString::from("⋯").as_ustr(), 0)) as u32).to_ne_bytes()) > 2147483647 {
            return Ok(res);
        }
        let mut decisions: Vec<ShapingDecisionInfo> = vec![];
        for i in 0..match u32::try_from(res.decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = (res.decisions[usize::try_from(i).unwrap_or(0)]).clone();
            decisions.push(ShapingDecisionInfo::new((d.range).clone(), (d.source_text).to_ustring().as_ustr(), (d.display_text).to_ustring().as_ustr(), (d.font_key).to_ustring().as_ustr(), d.glyph_count, d.advance, (d.source).to_ustring().as_ustr(), (d.reason).to_ustring().as_ustr(), Some(d.glyphs_without_ink_bounds), Some(d.missing_glyphs), d.resolved_face.clone(), d.script.clone(), d.language.clone(), d.strategy.clone(), d.feature_evidence.clone(), Some(UString::from("UnverifiedDisplaySubstitutionCoverage"))));
        }
        return Ok(ShapingResult::new(res.clusters.to_vec(), res.glyph_runs.to_vec(), Some((decisions).clone())));
    }
}

impl ITextShaper for UnverifiedCoverageReportingShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.DisplayGlyphSubstitutionEngineTestSupport.UnverifiedCoverageReportingShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let res = self.delegate.lock().unwrap().shape((input).clone())?;
        if u32::from_ne_bytes(((u_string::find_from(&((input.display_text).to_ustring()), UString::from("⋯").as_ustr(), 0)) as u32).to_ne_bytes()) > 2147483647 {
            return Ok(res);
        }
        let mut decisions: Vec<ShapingDecisionInfo> = vec![];
        for i in 0..match u32::try_from(res.decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = (res.decisions[usize::try_from(i).unwrap_or(0)]).clone();
            decisions.push(ShapingDecisionInfo::new((d.range).clone(), (d.source_text).to_ustring().as_ustr(), (d.display_text).to_ustring().as_ustr(), (d.font_key).to_ustring().as_ustr(), d.glyph_count, d.advance, (d.source).to_ustring().as_ustr(), (d.reason).to_ustring().as_ustr(), Some(d.glyphs_without_ink_bounds), Some(d.missing_glyphs), d.resolved_face.clone(), d.script.clone(), d.language.clone(), d.strategy.clone(), d.feature_evidence.clone(), Some(UString::from("UnverifiedDisplaySubstitutionCoverage"))));
        }
        return Ok(ShapingResult::new(res.clusters.to_vec(), res.glyph_runs.to_vec(), Some((decisions).clone())));
    }
}

#[derive(Clone)]
pub struct DashInkOverrideShaper {
    pub(crate) delegate: Arc<Mutex<ExplainableStubTextShaper>>,
    pub(crate) override_advance: f64,
    pub(crate) ink: Rect,
    pub(crate) full_override: bool,
}

impl DashInkOverrideShaper {
    pub fn new(override_advance: f64, ink: Rect, full_override: bool) -> Self {
        Self {
            override_advance,
            ink,
            full_override,
            delegate: Arc::new(Mutex::new(ExplainableStubTextShaper::new())),
        }
    }

    pub fn shape(&self, input: ShapingInput) -> Result<ShapingResult, DisplayGlyphSubstitutionEngineTestSupportShapeFault> {
        let res = self.delegate.lock().unwrap().shape((input).clone()).map_err(|e| DisplayGlyphSubstitutionEngineTestSupportShapeFault::TextShaperShapeFaultFault(e))?;
        if u32::from_ne_bytes(((u_string::find_from(&((input.display_text).to_ustring()), UString::from("⸺").as_ustr(), 0)) as u32).to_ne_bytes()) > 2147483647 {
            return Ok(res);
        }
        if !self.full_override {
            return Ok(ShapingResult::new(res.clusters.to_vec(), self.override_runs((res).clone()).to_vec(), Some((res.decisions).clone())));
        }
        let mut clusters: Vec<Cluster> = vec![];
        for i in 0..match u32::try_from(res.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (res.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            clusters.push(Cluster::new((c.range).clone(), (c.text).to_ustring().as_ustr(), (c.font_key).to_ustring().as_ustr(), self.override_advance, Some((c.display_text).to_ustring()), Some(c.baseline_shift), Some(c.leading_layout_advance), Some(c.glyph_inline_shift)));
        }
        let mut decisions: Vec<ShapingDecisionInfo> = vec![];
        for i in 0..match u32::try_from(res.decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = (res.decisions[usize::try_from(i).unwrap_or(0)]).clone();
            decisions.push(ShapingDecisionInfo::new((d.range).clone(), (d.source_text).to_ustring().as_ustr(), (d.display_text).to_ustring().as_ustr(), (d.font_key).to_ustring().as_ustr(), d.glyph_count, self.override_advance, (d.source).to_ustring().as_ustr(), (d.reason).to_ustring().as_ustr(), Some(d.glyphs_without_ink_bounds), Some(d.missing_glyphs), d.resolved_face.clone(), d.script.clone(), d.language.clone(), d.strategy.clone(), d.feature_evidence.clone(), d.capability_issue.clone()));
        }
        return Ok(ShapingResult::new(clusters.to_vec(), self.override_runs((res).clone()).to_vec(), Some((decisions).clone())));
    }

    fn override_runs(&self, res: ShapingResult) -> Vec<GlyphRun> {
        let mut runs: Vec<GlyphRun> = vec![];
        for ri in 0..match u32::try_from(res.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (res.glyph_runs[usize::try_from(ri).unwrap_or(0)]).clone();
            let mut glyphs: Vec<Glyph> = vec![];
            for gi in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let g = (run.glyphs[usize::try_from(gi).unwrap_or(0)]).clone();
                glyphs.push(Glyph::new(g.id, (g.cluster_range).clone(), self.override_advance, Some(g.x), Some(g.y), g.render_font_key.clone(), Some((self.ink).clone()), g.halt_advance, g.halt_placement_x));
            }
            runs.push(GlyphRun::new((run.range).clone(), (run.font_key).to_ustring().as_ustr(), glyphs.to_vec(), if self.full_override { self.override_advance } else { run.advance }, Some(vec![])));
        }
        return runs;
    }
}

impl ITextShaper for DashInkOverrideShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.DisplayGlyphSubstitutionEngineTestSupport.DashInkOverrideShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let res = self.delegate.lock().unwrap().shape((input).clone())?;
        if u32::from_ne_bytes(((u_string::find_from(&((input.display_text).to_ustring()), UString::from("⸺").as_ustr(), 0)) as u32).to_ne_bytes()) > 2147483647 {
            return Ok(res);
        }
        if !self.full_override {
            return Ok(ShapingResult::new(res.clusters.to_vec(), self.override_runs((res).clone()).to_vec(), Some((res.decisions).clone())));
        }
        let mut clusters: Vec<Cluster> = vec![];
        for i in 0..match u32::try_from(res.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (res.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            clusters.push(Cluster::new((c.range).clone(), (c.text).to_ustring().as_ustr(), (c.font_key).to_ustring().as_ustr(), self.override_advance, Some((c.display_text).to_ustring()), Some(c.baseline_shift), Some(c.leading_layout_advance), Some(c.glyph_inline_shift)));
        }
        let mut decisions: Vec<ShapingDecisionInfo> = vec![];
        for i in 0..match u32::try_from(res.decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = (res.decisions[usize::try_from(i).unwrap_or(0)]).clone();
            decisions.push(ShapingDecisionInfo::new((d.range).clone(), (d.source_text).to_ustring().as_ustr(), (d.display_text).to_ustring().as_ustr(), (d.font_key).to_ustring().as_ustr(), d.glyph_count, self.override_advance, (d.source).to_ustring().as_ustr(), (d.reason).to_ustring().as_ustr(), Some(d.glyphs_without_ink_bounds), Some(d.missing_glyphs), d.resolved_face.clone(), d.script.clone(), d.language.clone(), d.strategy.clone(), d.feature_evidence.clone(), d.capability_issue.clone()));
        }
        return Ok(ShapingResult::new(clusters.to_vec(), self.override_runs((res).clone()).to_vec(), Some((decisions).clone())));
    }
}

#[derive(Clone)]
pub struct TwoGlyphEllipsisShaper {
    pub(crate) delegate: Arc<Mutex<ExplainableStubTextShaper>>,
}

impl TwoGlyphEllipsisShaper {
    pub fn new() -> Self {
        Self {
            delegate: Arc::new(Mutex::new(ExplainableStubTextShaper::new())),
        }
    }

    pub fn shape(&self, input: ShapingInput) -> Result<ShapingResult, DisplayGlyphSubstitutionEngineTestSupportShapeFault> {
        if input.display_text.to_ustring() != UString::from("⋯⋯") {
            return Ok(self.delegate.lock().unwrap().shape((input).clone()).map_err(|e| DisplayGlyphSubstitutionEngineTestSupportShapeFault::TextShaperShapeFaultFault(e))?);
        }
        return Ok(ShapingResult::new(vec![
    (Cluster::new((input.range).clone(), u_string::substring(&(input.text).to_ustring(), i32::from_ne_bytes((((input.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((input.range).clone().end) as i32).to_ne_bytes())).as_ustr(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), 32 as f64 as f64, Some((input.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0))).clone(),
].to_vec(), vec![
    (GlyphRun::new((input.range).clone(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), vec![
    (Glyph::new(1u32, (input.range).clone(), 16 as f64 as f64, Some(0 as f64), Some(0.0), None, Some(Rect::new(1.5f64, i32::from_ne_bytes(((4294967289u32) as i32).to_ne_bytes()) as f64 as f64, 14.5f64, i32::from_ne_bytes(((4294967291u32) as i32).to_ne_bytes()) as f64 as f64)), None, None)).clone(),
    (Glyph::new(2u32, (input.range).clone(), 16 as f64 as f64, Some(16 as f64), Some(0.0), None, Some(Rect::new(1.5f64, i32::from_ne_bytes(((4294967289u32) as i32).to_ne_bytes()) as f64 as f64, 14.5f64, i32::from_ne_bytes(((4294967291u32) as i32).to_ne_bytes()) as f64 as f64)), None, None)).clone(),
].to_vec(), 32 as f64 as f64, Some(vec![]))).clone(),
].to_vec(), Some(vec![])));
    }
}

impl ITextShaper for TwoGlyphEllipsisShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.DisplayGlyphSubstitutionEngineTestSupport.TwoGlyphEllipsisShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        if input.display_text.to_ustring() != UString::from("⋯⋯") {
            return Ok(self.delegate.lock().unwrap().shape((input).clone())?);
        }
        return Ok(ShapingResult::new(vec![
    (Cluster::new((input.range).clone(), u_string::substring(&(input.text).to_ustring(), i32::from_ne_bytes((((input.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((input.range).clone().end) as i32).to_ne_bytes())).as_ustr(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), 32 as f64 as f64, Some((input.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0))).clone(),
].to_vec(), vec![
    (GlyphRun::new((input.range).clone(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), vec![
    (Glyph::new(1u32, (input.range).clone(), 16 as f64 as f64, Some(0 as f64), Some(0.0), None, Some(Rect::new(1.5f64, i32::from_ne_bytes(((4294967289u32) as i32).to_ne_bytes()) as f64 as f64, 14.5f64, i32::from_ne_bytes(((4294967291u32) as i32).to_ne_bytes()) as f64 as f64)), None, None)).clone(),
    (Glyph::new(2u32, (input.range).clone(), 16 as f64 as f64, Some(16 as f64), Some(0.0), None, Some(Rect::new(1.5f64, i32::from_ne_bytes(((4294967289u32) as i32).to_ne_bytes()) as f64 as f64, 14.5f64, i32::from_ne_bytes(((4294967291u32) as i32).to_ne_bytes()) as f64 as f64)), None, None)).clone(),
].to_vec(), 32 as f64 as f64, Some(vec![]))).clone(),
].to_vec(), Some(vec![])));
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct JustifiedDashHit {
    pub dash: Cluster,
    pub decision: JustificationDecisionInfo,
}
