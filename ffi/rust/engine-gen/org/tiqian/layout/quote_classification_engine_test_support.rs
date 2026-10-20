use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::glyph_run::GlyphRun;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_queries::LayoutQueries;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::positioned_cluster::PositionedCluster;
use crate::org::tiqian::core::punctuation_decision_info::PunctuationDecisionInfo;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::role_override_info::RoleOverrideInfo;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::shaping_decision_info::ShapingDecisionInfo;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
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
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::shaping::text_shaper::ITextShaper;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::org::tiqian::shaping::text_shaper::ShapingResult;
use crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestSupportLayoutFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestSupportLayoutFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestSupportLayoutFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestSupportLayoutFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestSupportLayoutFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestSupportLayoutFault) -> Self {
        match value {
            QuoteClassificationEngineTestSupportLayoutFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestSupportLayoutFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestSupportLayoutFault) -> Self {
        match value {
            QuoteClassificationEngineTestSupportLayoutFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestSupportLayoutFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestSupportLayoutFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestSupportLayoutFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestSupportLayoutFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestSupportInternalFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestSupportInternalFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestSupportInternalFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestSupportInternalFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestSupportInternalFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestSupportInternalFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestSupportInternalFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestSupportInternalFault) -> Self {
        match value {
            QuoteClassificationEngineTestSupportInternalFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestSupportInternalFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestSupportInternalFault) -> Self {
        match value {
            QuoteClassificationEngineTestSupportInternalFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestSupportInternalFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestSupportInternalFault) -> Self {
        match value {
            QuoteClassificationEngineTestSupportInternalFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestSupportInternalFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestSupportInternalFault) -> Self {
        match value {
            QuoteClassificationEngineTestSupportInternalFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestSupportInternalFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestSupportInternalFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestSupportInternalFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestSupportInternalFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestSupportInternalFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestSupportInternalFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestSupportInternalFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestSupportInternalFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestSupportFullWidthTestFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestSupportFullWidthTestFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestSupportFullWidthTestFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestSupportFullWidthTestFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestSupportFullWidthTestFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestSupportFullWidthTestFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestSupportFullWidthTestFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestSupportFullWidthTestFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestSupportFullWidthTestFault) -> Self {
        match value {
            QuoteClassificationEngineTestSupportFullWidthTestFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestSupportFullWidthTestFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestSupportFullWidthTestFault) -> Self {
        match value {
            QuoteClassificationEngineTestSupportFullWidthTestFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestSupportFullWidthTestFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestSupportFullWidthTestFault) -> Self {
        match value {
            QuoteClassificationEngineTestSupportFullWidthTestFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestSupportFullWidthTestFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestSupportFullWidthTestFault) -> Self {
        match value {
            QuoteClassificationEngineTestSupportFullWidthTestFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestSupportFullWidthTestFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: QuoteClassificationEngineTestSupportFullWidthTestFault) -> Self {
        match value {
            QuoteClassificationEngineTestSupportFullWidthTestFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestSupportFullWidthTestFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestSupportFullWidthTestFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestSupportFullWidthTestFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestSupportFullWidthTestFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestSupportFullWidthTestFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestSupportFullWidthTestFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestSupportFullWidthTestFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestSupportFullWidthTestFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for QuoteClassificationEngineTestSupportFullWidthTestFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        QuoteClassificationEngineTestSupportFullWidthTestFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestSupportShapeFault {
    TextShaperShapeFaultFault(crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestSupportShapeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestSupportShapeFault::TextShaperShapeFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestSupportShapeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestSupportShapeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestSupportShapeFault> for crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault {
    fn from(value: QuoteClassificationEngineTestSupportShapeFault) -> Self {
        match value {
            QuoteClassificationEngineTestSupportShapeFault::TextShaperShapeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestSupportShapeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestSupportShapeFault) -> Self {
        match value {
            QuoteClassificationEngineTestSupportShapeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestSupportShapeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestSupportShapeFault) -> Self {
        match value {
            QuoteClassificationEngineTestSupportShapeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault> for QuoteClassificationEngineTestSupportShapeFault {
    fn from(value: crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault) -> Self {
        QuoteClassificationEngineTestSupportShapeFault::TextShaperShapeFaultFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestSupportShapeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestSupportShapeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestSupportShapeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestSupportShapeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct QuoteClassificationEngineTestSupport;

impl QuoteClassificationEngineTestSupport {
    pub fn quote_classification_engine_test_support_begin(name: &UStr) -> TestTraceRecorder {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[81,117,111,116,101,67,108,97,115,115,105,102,105,99,97,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
        t.section(name);
        return t;
    }

    pub fn quote_classification_engine_test_support_arm() -> TestTraceRecorder {
        return TestTraceRecorder::new(&(UStr::new(&[81,117,111,116,101,67,108,97,115,115,105,102,105,99,97,116,105,111,110,69,110,103,105,110,101,84,101,115,116])));
    }

    pub fn quote_classification_engine_test_support_input(text: &UStr, width: f64) -> Result<LayoutInput, TextRangeError> {
        return Ok(LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(width, Some(f64::INFINITY), Some(2147483647))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])));
    }

    pub fn quote_classification_engine_test_support_layout(text: &UStr, width: f64, engine: Option<ExplainableStubParagraphLayoutEngine>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        return Ok((match &(engine) { None => ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?, Some(__option2) => (*__option2).clone() }).layout(QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_input(text, width).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn quote_classification_engine_test_support_set(values: &Vec<u32>) -> SortedSetTable<u32> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        for &v in values {
            b.put(&(v));
        }
        return b.clone().build();
    }

    pub fn quote_classification_engine_test_support_indices(text: &UStr) -> Vec<u32> {
    let __units = u_string::units(&text);
    let __count = u_string::unit_count(&text);
        let mut r: Vec<u32> = vec![];
        for i in 0..match u32::try_from(u_string::unit_count(&(text))) { Ok(value) => value, Err(_) => u32::MAX } {
            if QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_is_curly_quote_for_test(u_string::substring(&text, { let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }, i32::from_ne_bytes(((u32::wrapping_add(i, 1)) as i32).to_ne_bytes())).as_ustr()) {
                r.push(i);
            }
        }
        return r;
    }

    pub fn quote_classification_engine_test_support_role_at(result: LayoutResult, index: u32) -> UString {
        for i in 0..match u32::try_from((result.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((result.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if i32::from_ne_bytes(((index) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((d.range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes((((d.range).clone().end) as i32).to_ne_bytes())) {
                return ((d.role).to_ustring()).clone();
            }
        }
        return UString::new();
    }

    pub fn quote_classification_engine_test_support_is_curly_quote_for_test(ch: &UStr) -> bool {
        return ch == UString::from("‘") || ch == UString::from("’") || ch == UString::from("“") || ch == UString::from("”");
    }

    pub fn quote_classification_engine_test_support_last_index(text: &UStr, mark: &UStr) -> u32 {
        let mut result = 4294967295u32;
        let mut start = 0u32;
        loop {
            let next = u_string::find_from(&(text), mark, i32::from_ne_bytes(((start) as i32).to_ne_bytes()));
            if next < (0) {
                return result;
            }
            result = u32::from_ne_bytes(((next) as u32).to_ne_bytes());
            start = u32::from_ne_bytes(((i32::wrapping_add(next, 1)) as u32).to_ne_bytes());
        }
    }

    pub fn quote_classification_engine_test_support_render_role_map(map: SortedMapTable<u32, UString>) -> UString {
        let mut out = UString::new();
        for i in 0..u32::from_ne_bytes(((map.size()) as u32).to_ne_bytes()) {
            if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                out += &(UString::from(", "));
            }
            out += &({ let mut __s = UString::new(); __s += &(UString::from("")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(map.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }))).as_str())); __s += &(UString::from("='")); __s += map.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }).as_ustr(); __s += &(UString::from("'")); __s });
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("{")); __s += out.as_ustr(); __s += &(UString::from("}")); __s }).as_str());
    }

    pub fn quote_classification_engine_test_support_internal(text: &UStr, source: &UStr, role: &UStr, section: Option<UString>) -> Result<(), QuoteClassificationEngineTestSupportInternalFault> {
        let _ = match &(section) { None => QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_arm(), Some(__option4) => QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(__option4) };
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(text, 320 as f64, None).map_err(|e| QuoteClassificationEngineTestSupportInternalFault::ParagraphLayoutEngineNewFaultFault(e))?;
        let mut a: Vec<RoleOverrideInfo> = vec![];
        for i in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(i).unwrap_or(0)]).clone();
            if d.source_text.to_ustring() == UString::from("“") || (d.source_text).to_ustring() == UString::from("”") {
                a.push(d.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0), None).map_err(|e| QuoteClassificationEngineTestSupportInternalFault::TracedAssertionsFailFaultFault(e))?;
        let mut all = true;
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            all = all && ((a[usize::try_from(i).unwrap_or(0)]).clone().overridden_role).to_ustring() == role && ((a[usize::try_from(i).unwrap_or(0)]).clone().source).to_ustring() == source;
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all, None).map_err(|e| QuoteClassificationEngineTestSupportInternalFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn quote_classification_engine_test_support_full_width_test() -> Result<(), QuoteClassificationEngineTestSupportFullWidthTestFault> {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(UStr::new(&[114,101,113,117,101,115,116,115,70,117,108,108,87,105,100,116,104,67,106,107,81,117,111,116,101,115,65,110,100,83,121,110,116,104,101,115,105,122,101,115,84,104,101,67,101,108,108,87,104,101,110,84,104,101,70,111,110,116,83,116,97,121,115,80,114,111,112,111,114,116,105,111,110,97,108]));
        let mut e = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ProportionalQuoteTextShaper::new()))), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).map_err(|e| QuoteClassificationEngineTestSupportFullWidthTestFault::ParagraphLayoutEngineNewFaultFault(e))?;
        let r = e.layout(QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_input(UStr::new(&[20013,8220,25991,8221,20013]), 320 as f64).map_err(|e| QuoteClassificationEngineTestSupportFullWidthTestFault::TextRangeErrorFault(e))?).map_err(|e| QuoteClassificationEngineTestSupportFullWidthTestFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(e))?;
        let mut o: Option<Cluster> = None;
        let mut c: Option<Cluster> = None;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            if x.text.to_ustring() == UString::from("“") {
                o = Some(x.clone());
            }
            if x.text.to_ustring() == UString::from("”") {
                c = Some(x.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, o.as_ref().unwrap().advance, None).map_err(|e| QuoteClassificationEngineTestSupportFullWidthTestFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, c.as_ref().unwrap().advance, None).map_err(|e| QuoteClassificationEngineTestSupportFullWidthTestFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10 as f64, o.as_ref().unwrap().glyph_inline_shift, None).map_err(|e| QuoteClassificationEngineTestSupportFullWidthTestFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, c.as_ref().unwrap().glyph_inline_shift, None).map_err(|e| QuoteClassificationEngineTestSupportFullWidthTestFault::TracedAssertionsFailFaultFault(e))?;
        let mut a: Option<PunctuationDecisionInfo> = None;
        let mut b: Option<PunctuationDecisionInfo> = None;
        for i in 0..match u32::try_from((r.debug).clone().punctuation_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = ((r.debug).clone().punctuation_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if x.char.to_ustring() == UString::from("“") {
                a = Some(x.clone());
            }
            if x.char.to_ustring() == UString::from("”") {
                b = Some(x.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10 as f64, a.as_ref().unwrap().advance_expansion, None).map_err(|e| QuoteClassificationEngineTestSupportFullWidthTestFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[85,110,100,101,114,119,105,100,116,104,80,117,110,99,116,117,97,116,105,111,110,70,117,108,108,87,105,100,116,104,66,111,120,80,108,97,99,101,109,101,110,116]), ((a.as_ref().unwrap().glyph_placement_reason).clone()).as_deref().unwrap_or(UStr::new(&[])), None).map_err(|e| QuoteClassificationEngineTestSupportFullWidthTestFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(None.clone(), (b.as_ref().unwrap().glyph_placement_reason).clone().clone(), None).map_err(|e| QuoteClassificationEngineTestSupportFullWidthTestFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,107,66,111,117,110,100,115,70,105,116,116,101,100,66,111,100,121,67,111,109,112,114,101,115,115,105,111,110]), (a.as_ref().unwrap().geometry_source).to_ustring().as_ustr(), None).map_err(|e| QuoteClassificationEngineTestSupportFullWidthTestFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,107,66,111,117,110,100,115,70,105,116,116,101,100,66,111,100,121,67,111,109,112,114,101,115,115,105,111,110]), (b.as_ref().unwrap().geometry_source).to_ustring().as_ustr(), None).map_err(|e| QuoteClassificationEngineTestSupportFullWidthTestFault::TracedAssertionsFailFaultFault(e))?;
        let p = LayoutQueries::layout_queries_positioned_clusters((r).clone());
        let mut po: Option<PositionedCluster> = None;
        let mut pc: Option<PositionedCluster> = None;
        for x in &p {
            if x.range.clone().start == (o.as_ref().unwrap().range).clone().start && (x.range).clone().end == (o.as_ref().unwrap().range).clone().end {
                po = Some(x.clone());
            }
            if x.range.clone().start == (c.as_ref().unwrap().range).clone().start && (x.range).clone().end == (c.as_ref().unwrap().range).clone().end {
                pc = Some(x.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float(po.as_ref().unwrap().left + format!("{}", (10i32)).parse::<f64>().unwrap_or(0.0), po.as_ref().unwrap().draw_x, None).map_err(|e| QuoteClassificationEngineTestSupportFullWidthTestFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_equals_float(pc.as_ref().unwrap().left, pc.as_ref().unwrap().draw_x, None).map_err(|e| QuoteClassificationEngineTestSupportFullWidthTestFault::TracedAssertionsFailFaultFault(e))?;
        let q = e.layout(QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_input(UStr::new(&[8220,25991]), 320 as f64).map_err(|e| QuoteClassificationEngineTestSupportFullWidthTestFault::TextRangeErrorFault(e))?).map_err(|e| QuoteClassificationEngineTestSupportFullWidthTestFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, q.clusters[0usize].advance, None).map_err(|e| QuoteClassificationEngineTestSupportFullWidthTestFault::TracedAssertionsFailFaultFault(e))?;
        let qp = LayoutQueries::layout_queries_positioned_clusters((q).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, qp[0usize].draw_x, None).map_err(|e| QuoteClassificationEngineTestSupportFullWidthTestFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, qp[1usize].left, None).map_err(|e| QuoteClassificationEngineTestSupportFullWidthTestFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn quote_classification_engine_test_support_line_reason(r: LineEndReason) -> UString {
        let x = match r {
    LineEndReason::AutoWrap => UString::from("AutoWrap").to_ustring().to_ustring(),
    LineEndReason::MandatoryBreak => UString::from("MandatoryBreak").to_ustring().to_ustring(),
    LineEndReason::ParagraphEnd => UString::from("ParagraphEnd").to_ustring().to_ustring(),
};
        return x;
    }

    pub fn quote_classification_engine_test_support_starts_with(text: &UStr, range: TextRange, mark: &UStr) -> bool {
        let from = range.start;
        let to = range.end;
        let x = u_string::slice(text, i32::from_ne_bytes(((from) as i32).to_ne_bytes()), i32::from_ne_bytes(((to) as i32).to_ne_bytes()));
        let mut r = false;
        if u32::from_ne_bytes(((u_string::find_from(&(x), mark, 0)) as u32).to_ne_bytes()) == 0 {
            r = true;
        }
        return r;
    }

    pub fn quote_classification_engine_test_support_ends_with(text: &UStr, range: TextRange, mark: &UStr) -> bool {
        let from = range.start;
        let to = range.end;
        let x = u_string::slice(text, i32::from_ne_bytes(((from) as i32).to_ne_bytes()), i32::from_ne_bytes(((to) as i32).to_ne_bytes()));
        return u32::from_ne_bytes(((match (x).rfind(&mark) { Some(v) => i32::from_ne_bytes(u32::try_from((v) & 4294967295).unwrap_or(0).to_ne_bytes()), None => -1 }) as u32).to_ne_bytes()) == u32::wrapping_sub(u_string::unit_count(&(x)), 1);
    }
}

#[derive(Clone)]
pub struct ProportionalQuoteTextShaper {
    pub(crate) delegate: Arc<Mutex<ExplainableStubTextShaper>>,
}

impl ProportionalQuoteTextShaper {
    pub fn new() -> Self {
        Self {
            delegate: Arc::new(Mutex::new(ExplainableStubTextShaper::new())),
        }
    }

    pub fn shape(&self, input: ShapingInput) -> Result<ShapingResult, QuoteClassificationEngineTestSupportShapeFault> {
        let result = self.delegate.lock().unwrap().shape((input).clone()).map_err(|e| QuoteClassificationEngineTestSupportShapeFault::TextShaperShapeFaultFault(e))?;
        if input.display_text.to_ustring() != UString::from("“") && (input.display_text).to_ustring() != UString::from("”") {
            return Ok(result);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("fwid=1").to_ustring()], &input.open_type_features, None).map_err(|e| QuoteClassificationEngineTestSupportShapeFault::TracedAssertionsFailFaultFault(e))?;
        let mut clusters: Vec<Cluster> = vec![];
        for i in 0..match u32::try_from(result.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (result.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            clusters.push(Cluster::new((c.range).clone(), (c.text).to_ustring().as_ustr(), (c.font_key).to_ustring().as_ustr(), 6.0f64, Some((c.display_text).to_ustring()), Some(c.baseline_shift), Some(c.leading_layout_advance), Some(c.glyph_inline_shift)));
        }
        let mut runs: Vec<GlyphRun> = vec![];
        for ri in 0..match u32::try_from(result.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (result.glyph_runs[usize::try_from(ri).unwrap_or(0)]).clone();
            let mut gs: Vec<Glyph> = vec![];
            for gi in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let g = (run.glyphs[usize::try_from(gi).unwrap_or(0)]).clone();
                gs.push(Glyph::new(g.id, (g.cluster_range).clone(), 6.0f64, Some(g.x), Some(g.y), g.render_font_key.clone(), Some(Rect::new(1 as f64 as f64, i32::from_ne_bytes(((4294967286u32) as i32).to_ne_bytes()) as f64 as f64, 5 as f64 as f64, 0 as f64 as f64)), g.halt_advance, g.halt_placement_x));
            }
            let mut features: Vec<UString> = vec![];
            for fi in 0..match u32::try_from(run.open_type_features.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                features.push((run.open_type_features[usize::try_from(fi).unwrap_or(0)]).clone());
            }
            runs.push(GlyphRun::new((run.range).clone(), (run.font_key).to_ustring().as_ustr(), gs.to_vec(), 6.0f64, Some((features).clone())));
        }
        let mut decisions: Vec<ShapingDecisionInfo> = vec![];
        for i in 0..match u32::try_from(result.decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = (result.decisions[usize::try_from(i).unwrap_or(0)]).clone();
            decisions.push(ShapingDecisionInfo::new((d.range).clone(), (d.source_text).to_ustring().as_ustr(), (d.display_text).to_ustring().as_ustr(), (d.font_key).to_ustring().as_ustr(), d.glyph_count, 6.0f64, (d.source).to_ustring().as_ustr(), (d.reason).to_ustring().as_ustr(), Some(d.glyphs_without_ink_bounds), Some(d.missing_glyphs), d.resolved_face.clone(), d.script.clone(), d.language.clone(), d.strategy.clone(), d.feature_evidence.clone(), d.capability_issue.clone()));
        }
        return Ok(ShapingResult::new(clusters.to_vec(), runs.to_vec(), Some((decisions).clone())));
    }
}

impl ITextShaper for ProportionalQuoteTextShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.QuoteClassificationEngineTestSupport.ProportionalQuoteTextShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let result = self.delegate.lock().unwrap().shape((input).clone())?;
        if input.display_text.to_ustring() != UString::from("“") && (input.display_text).to_ustring() != UString::from("”") {
            return Ok(result);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("fwid=1").to_ustring()], &input.open_type_features, None).map_err(|e| TextShaperShapeFault::TracedAssertionsFailFaultFault(e))?;
        let mut clusters: Vec<Cluster> = vec![];
        for i in 0..match u32::try_from(result.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (result.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            clusters.push(Cluster::new((c.range).clone(), (c.text).to_ustring().as_ustr(), (c.font_key).to_ustring().as_ustr(), 6.0f64, Some((c.display_text).to_ustring()), Some(c.baseline_shift), Some(c.leading_layout_advance), Some(c.glyph_inline_shift)));
        }
        let mut runs: Vec<GlyphRun> = vec![];
        for ri in 0..match u32::try_from(result.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (result.glyph_runs[usize::try_from(ri).unwrap_or(0)]).clone();
            let mut gs: Vec<Glyph> = vec![];
            for gi in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let g = (run.glyphs[usize::try_from(gi).unwrap_or(0)]).clone();
                gs.push(Glyph::new(g.id, (g.cluster_range).clone(), 6.0f64, Some(g.x), Some(g.y), g.render_font_key.clone(), Some(Rect::new(1 as f64 as f64, i32::from_ne_bytes(((4294967286u32) as i32).to_ne_bytes()) as f64 as f64, 5 as f64 as f64, 0 as f64 as f64)), g.halt_advance, g.halt_placement_x));
            }
            let mut features: Vec<UString> = vec![];
            for fi in 0..match u32::try_from(run.open_type_features.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                features.push((run.open_type_features[usize::try_from(fi).unwrap_or(0)]).clone());
            }
            runs.push(GlyphRun::new((run.range).clone(), (run.font_key).to_ustring().as_ustr(), gs.to_vec(), 6.0f64, Some((features).clone())));
        }
        let mut decisions: Vec<ShapingDecisionInfo> = vec![];
        for i in 0..match u32::try_from(result.decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = (result.decisions[usize::try_from(i).unwrap_or(0)]).clone();
            decisions.push(ShapingDecisionInfo::new((d.range).clone(), (d.source_text).to_ustring().as_ustr(), (d.display_text).to_ustring().as_ustr(), (d.font_key).to_ustring().as_ustr(), d.glyph_count, 6.0f64, (d.source).to_ustring().as_ustr(), (d.reason).to_ustring().as_ustr(), Some(d.glyphs_without_ink_bounds), Some(d.missing_glyphs), d.resolved_face.clone(), d.script.clone(), d.language.clone(), d.strategy.clone(), d.feature_evidence.clone(), d.capability_issue.clone()));
        }
        return Ok(ShapingResult::new(clusters.to_vec(), runs.to_vec(), Some((decisions).clone())));
    }
}
