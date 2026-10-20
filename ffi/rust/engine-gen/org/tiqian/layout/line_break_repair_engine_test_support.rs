use crate::org::tiqian::clreq::clreq_profile::ClreqProfile;
use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::clreq::clreq_profile_resolver::ClreqProfileResolver;
use crate::org::tiqian::clreq::hanging_punctuation_style::HangingPunctuationStyle;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::clreq::kinsoku_mode::KinsokuMode;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_profile_id::LayoutProfileId;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_break_span::LineBreakSpan;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::line_breaker::LineBreaker;
use crate::org::tiqian::layout::line_breaker::LookaheadLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::linebreak::hyphenator::Hyphenator;
use crate::org::tiqian::linebreak::hyphenator::NoHyphenator;
use crate::org::tiqian::linebreak::hyphenator::TailHyphenator;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::string_tools;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestSupportBlobTestTailFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestSupportBlobTestTailFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestSupportBlobTestTailFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestSupportBlobTestTailFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestSupportBlobTestTailFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestSupportBlobTestTailFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestSupportBlobTestTailFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestSupportBlobTestTailFault) -> Self {
        match value {
            LineBreakRepairEngineTestSupportBlobTestTailFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestSupportBlobTestTailFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestSupportBlobTestTailFault) -> Self {
        match value {
            LineBreakRepairEngineTestSupportBlobTestTailFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestSupportBlobTestTailFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestSupportBlobTestTailFault) -> Self {
        match value {
            LineBreakRepairEngineTestSupportBlobTestTailFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestSupportBlobTestTailFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestSupportBlobTestTailFault) -> Self {
        match value {
            LineBreakRepairEngineTestSupportBlobTestTailFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestSupportBlobTestTailFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestSupportBlobTestTailFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestSupportBlobTestTailFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestSupportBlobTestTailFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestSupportBlobTestTailFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestSupportBlobTestTailFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestSupportBlobTestTailFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestSupportBlobTestTailFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestSupportBlobTestNonLexicalFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestSupportBlobTestNonLexicalFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestSupportBlobTestNonLexicalFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestSupportBlobTestNonLexicalFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestSupportBlobTestNonLexicalFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestSupportBlobTestNonLexicalFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestSupportBlobTestNonLexicalFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestSupportBlobTestNonLexicalFault) -> Self {
        match value {
            LineBreakRepairEngineTestSupportBlobTestNonLexicalFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestSupportBlobTestNonLexicalFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestSupportBlobTestNonLexicalFault) -> Self {
        match value {
            LineBreakRepairEngineTestSupportBlobTestNonLexicalFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestSupportBlobTestNonLexicalFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestSupportBlobTestNonLexicalFault) -> Self {
        match value {
            LineBreakRepairEngineTestSupportBlobTestNonLexicalFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestSupportBlobTestNonLexicalFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestSupportBlobTestNonLexicalFault) -> Self {
        match value {
            LineBreakRepairEngineTestSupportBlobTestNonLexicalFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestSupportBlobTestNonLexicalFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestSupportBlobTestNonLexicalFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestSupportBlobTestNonLexicalFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestSupportBlobTestNonLexicalFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestSupportBlobTestNonLexicalFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestSupportBlobTestNonLexicalFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestSupportBlobTestNonLexicalFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestSupportBlobTestNonLexicalFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestSupportBlobTestFitsAloneFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestSupportBlobTestFitsAloneFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestSupportBlobTestFitsAloneFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestSupportBlobTestFitsAloneFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestSupportBlobTestFitsAloneFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestSupportBlobTestFitsAloneFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestSupportBlobTestFitsAloneFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestSupportBlobTestFitsAloneFault) -> Self {
        match value {
            LineBreakRepairEngineTestSupportBlobTestFitsAloneFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestSupportBlobTestFitsAloneFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestSupportBlobTestFitsAloneFault) -> Self {
        match value {
            LineBreakRepairEngineTestSupportBlobTestFitsAloneFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestSupportBlobTestFitsAloneFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestSupportBlobTestFitsAloneFault) -> Self {
        match value {
            LineBreakRepairEngineTestSupportBlobTestFitsAloneFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestSupportBlobTestFitsAloneFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestSupportBlobTestFitsAloneFault) -> Self {
        match value {
            LineBreakRepairEngineTestSupportBlobTestFitsAloneFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestSupportBlobTestFitsAloneFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestSupportBlobTestFitsAloneFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestSupportBlobTestFitsAloneFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestSupportBlobTestFitsAloneFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestSupportBlobTestFitsAloneFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestSupportBlobTestFitsAloneFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestSupportBlobTestFitsAloneFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestSupportBlobTestFitsAloneFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakRepairEngineTestSupportBlobTestFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineBreakRepairEngineTestSupportBlobTestFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakRepairEngineTestSupportBlobTestFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestSupportBlobTestFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestSupportBlobTestFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakRepairEngineTestSupportBlobTestFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakRepairEngineTestSupportBlobTestFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakRepairEngineTestSupportBlobTestFault) -> Self {
        match value {
            LineBreakRepairEngineTestSupportBlobTestFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestSupportBlobTestFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakRepairEngineTestSupportBlobTestFault) -> Self {
        match value {
            LineBreakRepairEngineTestSupportBlobTestFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestSupportBlobTestFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakRepairEngineTestSupportBlobTestFault) -> Self {
        match value {
            LineBreakRepairEngineTestSupportBlobTestFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakRepairEngineTestSupportBlobTestFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakRepairEngineTestSupportBlobTestFault) -> Self {
        match value {
            LineBreakRepairEngineTestSupportBlobTestFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakRepairEngineTestSupportBlobTestFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakRepairEngineTestSupportBlobTestFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakRepairEngineTestSupportBlobTestFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakRepairEngineTestSupportBlobTestFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakRepairEngineTestSupportBlobTestFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakRepairEngineTestSupportBlobTestFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakRepairEngineTestSupportBlobTestFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakRepairEngineTestSupportBlobTestFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct LineBreakRepairEngineTestSupport;

impl LineBreakRepairEngineTestSupport {
    pub fn line_break_repair_engine_test_support_input(text: &UStr, width: f64, spans: Option<Vec<LineBreakSpan>>) -> Result<LayoutInput, TextRangeError> {
        return Ok(LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some((match &(spans) { None => vec![], Some(__option2) => (*__option2).clone() }).clone()), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(width, Some(f64::INFINITY), Some(2147483647))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])));
    }

    pub fn line_break_repair_engine_test_support_layout(text: &UStr, width: f64, breaker: Option<Box<dyn LineBreaker>>, hyphenator: Option<Box<dyn Hyphenator>>, spans: Option<Vec<LineBreakSpan>>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), breaker, Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), hyphenator, Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?.layout(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_input(text, width, (spans).clone()).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn line_break_repair_engine_test_support_layout_with_grid(text: &UStr, width: f64, grid_enabled: bool, breaker: Option<Box<dyn LineBreaker>>, hyphenator: Option<Box<dyn Hyphenator>>, spans: Option<Vec<LineBreakSpan>>) -> Result<LayoutResult,
ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), breaker, Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), hyphenator, Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?.layout(LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some((match &(spans) { None => vec![], Some(__option58) => (*__option58).clone() }).clone()), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(grid_enabled), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(width, Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn line_break_repair_engine_test_support_line_text(r: LayoutResult, n: u32) -> UString {
        let l = (r.lines[usize::try_from(n).unwrap_or(0)]).clone();
        let mut s = UString::new();
        for i in l.cluster_range.start..u32::wrapping_add(l.cluster_range.end, 1) {
            s += &(((r.clusters[usize::try_from(i).unwrap_or(0)]).clone().text).to_ustring());
        }
        return s;
    }

    pub fn line_break_repair_engine_test_support_has_text(r: LayoutResult, s: &UStr) -> bool {
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_ustring() == s {
                return true;
            }
        }
        return false;
    }

    pub fn line_break_repair_engine_test_support_no_hyphen(r: LayoutResult) -> bool {
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.lines[usize::try_from(i).unwrap_or(0)].hyphen_advance != 0 as f64 {
                return false;
            }
        }
        return true;
    }

    pub fn line_break_repair_engine_test_support_fixed(level: Option<KinsokuLevel>, hanging: Option<HangingPunctuationStyle>) -> Result<ExplainableStubParagraphLayoutEngine, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(FixedProfileResolver::new(level, hanging))), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(Box::new(NoHyphenator::new())), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?);
    }

    pub fn line_break_repair_engine_test_support_render_strings(a: &[UString]) -> UString {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push((a[usize::try_from(i).unwrap_or(0)]).clone());
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined = parts; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined[index]); index += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn line_break_repair_engine_test_support_render_list<T: Clone + std::fmt::Debug>(a: &[T]) -> UString {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push(UString::from(format!("{}", format!("{:?}", (a[usize::try_from(i).unwrap_or(0)]).clone())).as_str()));
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined1 = parts; let mut out = String::new(); let n = joined1.len(); let mut index1 = 0usize; while index1 < n { if index1 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined1[index1]); index1 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn line_break_repair_engine_test_support_blob_test_tail(_t: TestTraceRecorder) -> Result<(), LineBreakRepairEngineTestSupportBlobTestTailFault> {
        let p = UString::from("为什么历史是 ").to_ustring();
        let s = { let mut __s = UString::new(); __s += string_tools::StringTools::string_tools_lpad(UStr::new(&[]), UStr::new(&[115]), 40i32).as_ustr(); __s += &(UString::from("herstory")); __s };
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(UString::from(format!("{}", { let mut __s = UString::new(); __s += p.as_ustr(); __s += s.as_ustr(); __s }).as_str()).as_ustr(), 160 as f64, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)))), Some(Box::new(TailHyphenator::new())), None).map_err(|e| LineBreakRepairEngineTestSupportBlobTestTailFault::ParagraphLayoutEngineNewFaultFault(e))?;
        let x = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((r).clone(), 0);
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(x))) as i32).to_ne_bytes())) > (7), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("first line should carry part of the opaque letter blob: ")); __s += x.as_ustr(); __s }).as_str()))).map_err(|e| LineBreakRepairEngineTestSupportBlobTestTailFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_no_hyphen((r).clone()), None).map_err(|e| LineBreakRepairEngineTestSupportBlobTestTailFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true(!LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), s.as_ustr()), None).map_err(|e| LineBreakRepairEngineTestSupportBlobTestTailFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn line_break_repair_engine_test_support_blob_test_fits_alone(_t: TestTraceRecorder) -> Result<(), LineBreakRepairEngineTestSupportBlobTestFitsAloneFault> {
        let p = UString::from("为什么历史是 ").to_ustring();
        let s = { let mut __s = UString::new(); __s += string_tools::StringTools::string_tools_lpad(UStr::new(&[]), UStr::new(&[115]), 40i32).as_ustr(); __s += &(UString::from("herstory")); __s };
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(UString::from(format!("{}", { let mut __s = UString::new(); __s += p.as_ustr(); __s += s.as_ustr(); __s }).as_str()).as_ustr(), 800 as f64, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)))), Some(Box::new(TailHyphenator::new())), None).map_err(|e| LineBreakRepairEngineTestSupportBlobTestFitsAloneFault::ParagraphLayoutEngineNewFaultFault(e))?;
        let x = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((r).clone(), 0);
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(x))) as i32).to_ne_bytes())) > (7), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("first line should carry part of the long opaque token instead of stretching only '")); __s += p.as_ustr(); __s += &(UString::from("': ")); __s += x.as_ustr(); __s }).as_str()))).map_err(|e| LineBreakRepairEngineTestSupportBlobTestFitsAloneFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true(!LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), s.as_ustr()), None).map_err(|e| LineBreakRepairEngineTestSupportBlobTestFitsAloneFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_no_hyphen((r).clone()), None).map_err(|e| LineBreakRepairEngineTestSupportBlobTestFitsAloneFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn line_break_repair_engine_test_support_blob_test_non_lexical(_t: TestTraceRecorder) -> Result<(), LineBreakRepairEngineTestSupportBlobTestNonLexicalFault> {
        let p = UString::from("为什么历史是 ").to_ustring();
        let s = { let mut __s = UString::new(); __s += string_tools::StringTools::string_tools_lpad(UStr::new(&[]), UStr::new(&[115]), 40i32).as_ustr(); __s += &(UString::from("herstory")); __s };
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(UString::from(format!("{}", { let mut __s = UString::new(); __s += p.as_ustr(); __s += s.as_ustr(); __s }).as_str()).as_ustr(), 160 as f64, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)))), Some(Box::new(NoHyphenator::new())), None).map_err(|e| LineBreakRepairEngineTestSupportBlobTestNonLexicalFault::ParagraphLayoutEngineNewFaultFault(e))?;
        let x = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((r).clone(), 0);
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(x))) as i32).to_ne_bytes())) > (7), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("first line should carry part of the non-lexical letter run: ")); __s += x.as_ustr(); __s }).as_str()))).map_err(|e| LineBreakRepairEngineTestSupportBlobTestNonLexicalFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_no_hyphen((r).clone()), None).map_err(|e| LineBreakRepairEngineTestSupportBlobTestNonLexicalFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn line_break_repair_engine_test_support_blob_test(_t: TestTraceRecorder, w: f64, _ignored: bool) -> Result<(), LineBreakRepairEngineTestSupportBlobTestFault> {
        let p = UString::from("为什么历史是 ").to_ustring();
        let s = { let mut __s = UString::new(); __s += string_tools::StringTools::string_tools_lpad(UStr::new(&[]), UStr::new(&[115]), 40i32).as_ustr(); __s += &(UString::from("herstory")); __s };
        let r = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(UString::from(format!("{}", { let mut __s = UString::new(); __s += p.as_ustr(); __s += s.as_ustr(); __s }).as_str()).as_ustr(), w, Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)))), Some(Box::new(NoHyphenator::new())), None).map_err(|e| LineBreakRepairEngineTestSupportBlobTestFault::ParagraphLayoutEngineNewFaultFault(e))?;
        let x = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((r).clone(), 0);
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(x))) as i32).to_ne_bytes())) > (7), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("first line should carry part of the long opaque token instead of stretching only '")); __s += p.as_ustr(); __s += &(UString::from("': ")); __s += x.as_ustr(); __s }).as_str()))).map_err(|e| LineBreakRepairEngineTestSupportBlobTestFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_no_hyphen((r).clone()), None).map_err(|e| LineBreakRepairEngineTestSupportBlobTestFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true(!LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_has_text((r).clone(), s.as_ustr()), None).map_err(|e| LineBreakRepairEngineTestSupportBlobTestFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn line_break_repair_engine_test_support_kinsoku_start(n: &UStr) -> TestTraceRecorder {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[75,105,110,115,111,107,117,65,110,100,67,111,104,101,115,105,111,110,82,101,112,97,105,114,69,110,103,105,110,101,84,101,115,116])));
        t.section(n);
        return t;
    }
}

#[derive(Clone, PartialEq)]
pub struct FixedProfileResolver {
    pub level: KinsokuLevel,
    pub hanging: HangingPunctuationStyle,
}

impl FixedProfileResolver {
    pub fn new(level: Option<KinsokuLevel>, hanging: Option<HangingPunctuationStyle>) -> Self {
        let level = level.unwrap_or_else(|| KinsokuLevel::Basic);
        let hanging = hanging.unwrap_or_else(|| HangingPunctuationStyle::Disabled);
        Self {
            level: level,
            hanging: hanging,
        }
    }

    pub fn resolve(&self, _id: LayoutProfileId) -> ClreqProfile {
        let b = (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone();
        return ClreqProfile::new((b.id).to_ustring().as_ustr(), b.strictness, b.region, Some(b.punctuation_glyph_policy), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some((b.auto_space).clone()), Some(b.glue_placement), (b.adjustment).clone(), KinsokuMode::Fixed { level: self.level, hanging: self.hanging }, (b.punctuation_width).clone());
    }
}

impl ClreqProfileResolver for FixedProfileResolver {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.LineBreakRepairEngineTestSupport.FixedProfileResolver"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ClreqProfileResolver> {
        Box::new(self.clone())
    }

    fn resolve(&self, _id: LayoutProfileId) -> ClreqProfile {
        let b = (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone();
        return ClreqProfile::new((b.id).to_ustring().as_ustr(), b.strictness, b.region, Some(b.punctuation_glyph_policy), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some((b.auto_space).clone()), Some(b.glue_placement), (b.adjustment).clone(), KinsokuMode::Fixed { level: self.level, hanging: self.hanging }, (b.punctuation_width).clone());
    }
}
