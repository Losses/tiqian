use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::core::break_opportunity_decision_info::BreakOpportunityDecisionInfo;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::justification_allocation_info::JustificationAllocationInfo;
use crate::org::tiqian::core::justification_decision_info::JustificationDecisionInfo;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_break_span::LineBreakSpan;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::text_range::TextRange;
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
use crate::org::tiqian::linebreak::hyphenator::Hyphenator;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::shaping::text_shaper::ITextShaper;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::org::tiqian::shaping::text_shaper::ShapingResult;
use crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault;
use crate::org::tiqian::test::trace::test_trace::TestTrace;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum EmergencyGraphemeTrackingTestSupportLayoutWithShaperFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for EmergencyGraphemeTrackingTestSupportLayoutWithShaperFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmergencyGraphemeTrackingTestSupportLayoutWithShaperFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestSupportLayoutWithShaperFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestSupportLayoutWithShaperFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EmergencyGraphemeTrackingTestSupportLayoutWithShaperFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestSupportLayoutWithShaperFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestSupportLayoutWithShaperFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EmergencyGraphemeTrackingTestSupportLayoutWithShaperFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestSupportLayoutWithShaperFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EmergencyGraphemeTrackingTestSupportLayoutWithShaperFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EmergencyGraphemeTrackingTestSupportLayoutWithShaperFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EmergencyGraphemeTrackingTestSupportLayoutWithShaperFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EmergencyGraphemeTrackingTestSupportLayoutWithShaperFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmergencyGraphemeTrackingTestSupportLayoutWithObjectsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for EmergencyGraphemeTrackingTestSupportLayoutWithObjectsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmergencyGraphemeTrackingTestSupportLayoutWithObjectsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestSupportLayoutWithObjectsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestSupportLayoutWithObjectsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EmergencyGraphemeTrackingTestSupportLayoutWithObjectsFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestSupportLayoutWithObjectsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestSupportLayoutWithObjectsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EmergencyGraphemeTrackingTestSupportLayoutWithObjectsFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestSupportLayoutWithObjectsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EmergencyGraphemeTrackingTestSupportLayoutWithObjectsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EmergencyGraphemeTrackingTestSupportLayoutWithObjectsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EmergencyGraphemeTrackingTestSupportLayoutWithObjectsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EmergencyGraphemeTrackingTestSupportLayoutWithObjectsFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmergencyGraphemeTrackingTestSupportLayoutFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for EmergencyGraphemeTrackingTestSupportLayoutFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmergencyGraphemeTrackingTestSupportLayoutFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestSupportLayoutFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestSupportLayoutFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EmergencyGraphemeTrackingTestSupportLayoutFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestSupportLayoutFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestSupportLayoutFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EmergencyGraphemeTrackingTestSupportLayoutFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestSupportLayoutFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EmergencyGraphemeTrackingTestSupportLayoutFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EmergencyGraphemeTrackingTestSupportLayoutFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EmergencyGraphemeTrackingTestSupportLayoutFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EmergencyGraphemeTrackingTestSupportLayoutFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault) -> Self {
        match value {
            EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault::TracedAssertionsFailFaultFault(value)
    }
}

pub static EMERGENCY_GRAPHEME_TRACKING_TEST_SUPPORT_NO_INDENT: LazyLock<ParagraphStyle> = LazyLock::new(|| ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM)));

#[derive(Clone, Copy)]
pub struct EmergencyGraphemeTrackingTestSupport;

impl EmergencyGraphemeTrackingTestSupport {

    pub fn emergency_grapheme_tracking_test_support_engine(text_shaper: Option<Arc<Mutex<dyn ITextShaper>>>, hyphenator: Option<Box<dyn Hyphenator>>) -> Result<ExplainableStubParagraphLayoutEngine, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), text_shaper, hyphenator, Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?);
    }

    pub fn emergency_grapheme_tracking_test_support_render_ints(a: &Vec<u32>) -> UString {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(a[usize::try_from(i).unwrap_or(0)])).as_str()));
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined = parts; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined[index]); index += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn emergency_grapheme_tracking_test_support_render_clusters(a: &[Cluster]) -> UString {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push(UString::from(format!("{}", (a[usize::try_from(i).unwrap_or(0)]).clone().to_string()).as_str()));
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined1 = parts; let mut out = String::new(); let n = joined1.len(); let mut index1 = 0usize; while index1 < n { if index1 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined1[index1]); index1 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn emergency_grapheme_tracking_test_support_join_text_ranges(a: &Vec<TextRange>) -> UString {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push(UString::from(format!("{}", (a[usize::try_from(i).unwrap_or(0)]).clone().to_string()).as_str()));
        }
        return UString::from(format!("{}", { let joined2 = parts; let mut out = String::new(); let n = joined2.len(); let mut index2 = 0usize; while index2 < n { if index2 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined2[index2]); index2 += 1; } UString::from(out.as_str()) }).as_str());
    }

    pub fn emergency_grapheme_tracking_test_support_render_allocations(a: &Vec<JustificationAllocationInfo>) -> UString {
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            parts.push(UString::from(format!("{}", (a[usize::try_from(i).unwrap_or(0)]).clone().to_string()).as_str()));
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined3 = parts; let mut out = String::new(); let n = joined3.len(); let mut index3 = 0usize; while index3 < n { if index3 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined3[index3]); index3 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn emergency_grapheme_tracking_test_support_layout(text: &UStr, max_width: f64, line_break_spans: Option<Vec<LineBreakSpan>>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        return Ok(EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_engine(None, Some(EnglishHyphenation::english_hyphenation_en_us().map_err(|e| ParagraphLayoutEngineNewFault::UStringFaultFault(e))?))?.layout(LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), (line_break_spans).clone(), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some((*EMERGENCY_GRAPHEME_TRACKING_TEST_SUPPORT_NO_INDENT).clone()), LayoutConstraints::new(max_width, Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn emergency_grapheme_tracking_test_support_layout_with_shaper(text: &UStr, max_width: f64, text_shaper: Arc<Mutex<dyn ITextShaper>>, line_break_spans: Option<Vec<LineBreakSpan>>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        return Ok(EmergencyGraphemeTrackingTestSupport::emergency_grapheme_tracking_test_support_engine(Some(text_shaper), Some(EnglishHyphenation::english_hyphenation_en_us().map_err(|e| ParagraphLayoutEngineNewFault::UStringFaultFault(e))?))?.layout(LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), (line_break_spans).clone(), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some((*EMERGENCY_GRAPHEME_TRACKING_TEST_SUPPORT_NO_INDENT).clone()), LayoutConstraints::new(max_width, Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn emergency_grapheme_tracking_test_support_layout_with_objects(text: &UStr, max_width: f64, objects: &Vec<InlineObjectSpan>, line_break_spans: Option<Vec<LineBreakSpan>>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?.layout(LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), (line_break_spans).clone(), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some((*EMERGENCY_GRAPHEME_TRACKING_TEST_SUPPORT_NO_INDENT).clone()), LayoutConstraints::new(max_width, Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some((objects).clone()))).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn emergency_grapheme_tracking_test_support_break_offsets_for_tier(decisions: &[BreakOpportunityDecisionInfo], tier: &UStr) -> Vec<u32> {
        let mut result: Vec<u32> = vec![];
        for i in 0..match u32::try_from(decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if decisions[usize::try_from(i).unwrap_or(0)].clone().tier.clone().as_ref().map_or(false, |v| v == &(tier.to_ustring())) {
                for j in 0..match u32::try_from(((decisions[usize::try_from(i).unwrap_or(0)]).clone().break_offsets).clone().len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    result.push((decisions[usize::try_from(i).unwrap_or(0)]).clone().break_offsets[usize::try_from(j).unwrap_or(0)]);
                }
            }
        }
        return result;
    }

    pub fn emergency_grapheme_tracking_test_support_allocations_for_kind(decisions: &[JustificationDecisionInfo], kind: &UStr) -> Vec<JustificationAllocationInfo> {
        let mut result: Vec<JustificationAllocationInfo> = vec![];
        for i in 0..match u32::try_from(decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            for j in 0..match u32::try_from(((decisions[usize::try_from(i).unwrap_or(0)]).clone().allocations).clone().len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if decisions[usize::try_from(i).unwrap_or(0)].clone().allocations[usize::try_from(j).unwrap_or(0)].clone().kind.to_ustring() == kind {
                    result.push(((decisions[usize::try_from(i).unwrap_or(0)]).clone().allocations[usize::try_from(j).unwrap_or(0)]).clone());
                }
            }
        }
        return result;
    }

    pub fn emergency_grapheme_tracking_test_support_assert_equals_text_range(expected: TextRange, actual: TextRange) -> Result<(), EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault> {
        let e = TestTraceRender::test_trace_render_canonical_numbers(UString::from(format!("{}", expected.to_string()).as_str()).as_ustr()).map_err(|e| EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault::UStringFaultFault(e))?;
        let a = TestTraceRender::test_trace_render_canonical_numbers(UString::from(format!("{}", actual.to_string()).as_str()).as_ustr()).map_err(|e| EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault::UStringFaultFault(e))?;
        let recorder = TestTrace::test_trace_current_recorder();
        match &(recorder) {
            Some(__option) => {
                let _ = __option.record(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("eq expected=")); __s += e.as_ustr(); __s += &(UString::from(" actual=")); __s += a.as_ustr(); __s }).as_str()).as_ustr()).map_err(|e| EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault::UStringFaultFault(e))?;
            }
            None => {
            }
        }
        if expected.start != actual.start || expected.end != actual.end {
            let _ = TracedAssertions::traced_assertions_fail(Some(UString::from("TextRange mismatch")), None).map_err(|e| EmergencyGraphemeTrackingTestSupportAssertEqualsTextRangeFault::TracedAssertionsFailFaultFault(e))?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
pub struct UniformAdvanceShaper {
}

impl UniformAdvanceShaper {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn shape(&self, input: ShapingInput) -> ShapingResult {
        let source = u_string::substring(&(input.text).to_ustring(), i32::from_ne_bytes((((input.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((input.range).clone().end) as i32).to_ne_bytes()));
        return ShapingResult::new(vec![
    (Cluster::new((input.range).clone(), source.as_ustr(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), format!("{}", (i32::from_ne_bytes(((u_string::unit_count(&(source))) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) * 10.0f64, Some((input.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0))).clone(),
].to_vec(), vec![].to_vec(), Some(vec![]));
    }
}

impl ITextShaper for UniformAdvanceShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.EmergencyGraphemeTrackingTestSupport.UniformAdvanceShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let source = u_string::substring(&(input.text).to_ustring(), i32::from_ne_bytes((((input.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((input.range).clone().end) as i32).to_ne_bytes()));
        return Ok(ShapingResult::new(vec![
    (Cluster::new((input.range).clone(), source.as_ustr(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), format!("{}", (i32::from_ne_bytes(((u_string::unit_count(&(source))) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) * 10.0f64, Some((input.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0))).clone(),
].to_vec(), vec![].to_vec(), Some(vec![])));
    }
}
