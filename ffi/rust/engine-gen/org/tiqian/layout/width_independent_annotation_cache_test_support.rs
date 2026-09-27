use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_result::LayoutResult;
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
use crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCache;
use crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheFns;
use crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationKey;
use crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentParagraphAnnotation;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::shaping::text_shaper::ITextShaper;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::org::tiqian::shaping::text_shaper::ShapingResult;
use crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault;
use crate::org::tiqian::test::test_helpers::TestHelpers;
use crate::org::tiqian::test::trace::test_trace::TestTrace;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheTestSupportLayoutWithCacheFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for WidthIndependentAnnotationCacheTestSupportLayoutWithCacheFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WidthIndependentAnnotationCacheTestSupportLayoutWithCacheFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            WidthIndependentAnnotationCacheTestSupportLayoutWithCacheFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestSupportLayoutWithCacheFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: WidthIndependentAnnotationCacheTestSupportLayoutWithCacheFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestSupportLayoutWithCacheFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestSupportLayoutWithCacheFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheTestSupportLayoutWithCacheFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestSupportLayoutWithCacheFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for WidthIndependentAnnotationCacheTestSupportLayoutWithCacheFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        WidthIndependentAnnotationCacheTestSupportLayoutWithCacheFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheTestSupportLayoutWithCacheFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheTestSupportLayoutWithCacheFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault) -> Self {
        match value {
            WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Clone)]
pub struct CountingTextShaper {
    pub shape_call_count: u32,
    pub(crate) delegate: Option<Arc<Mutex<dyn ITextShaper>>>,
}

impl CountingTextShaper {
    pub fn new(delegate: Option<Arc<Mutex<dyn ITextShaper>>>) -> Self {
        Self {
            delegate,
            shape_call_count: 0,
        }
    }

    pub fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        self.shape_call_count += 1;
        let mut d: Option<Arc<Mutex<dyn ITextShaper>>> = match &(self.delegate) { None => Some({ let __shared_handle: Arc<Mutex<dyn ITextShaper>> = Arc::new(Mutex::new(ExplainableStubTextShaper::new())); __shared_handle }), Some(__option) => (Some((*__option).clone())).clone() };
        return Ok(d.as_mut().unwrap().lock().unwrap().shape((input).clone())?);
    }
}

impl ITextShaper for CountingTextShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.WidthIndependentAnnotationCacheTestSupport.CountingTextShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        self.shape_call_count += 1;
        let mut d: Option<Arc<Mutex<dyn ITextShaper>>> = match &(self.delegate) { None => Some({ let __shared_handle: Arc<Mutex<dyn ITextShaper>> = Arc::new(Mutex::new(ExplainableStubTextShaper::new())); __shared_handle }),
Some(__option1) => (Some((*__option1).clone())).clone() };
        return Ok(d.as_mut().unwrap().lock().unwrap().shape((input).clone())?);
    }
}

#[derive(Clone, Copy)]
pub struct WidthIndependentAnnotationCacheTestSupport;

impl WidthIndependentAnnotationCacheTestSupport {
    pub fn width_independent_annotation_cache_test_support_layout_with_cache(cache: Arc<Mutex<dyn WidthIndependentAnnotationCache>>, text: &UStr, max_width: f64, first_line_indent: Option<Ic>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        let indent = match &(first_line_indent) { Some(__option2) => (*__option2).clone(), None => Ic::zero() };
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some((cache).clone()))?;
        return Ok(engine.layout(LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((indent).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(max_width, Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }

    pub fn width_independent_annotation_cache_test_support_annotation_key(input: LayoutInput) -> WidthIndependentAnnotationKey {
        return WidthIndependentAnnotationCacheFns::width_independent_annotation_cache_fns_to_width_independent_annotation_key((input).clone(), None);
    }

    pub fn width_independent_annotation_cache_test_support_float_text(value: f64) -> Result<UString, UStringFault> {
        return Ok(TestTraceRender::test_trace_render_float_text(value)?);
    }

    pub fn width_independent_annotation_cache_test_support_generate_sweep_widths(start: f64, step: f64, max: f64) -> Vec<f64> {
        let mut widths: Vec<f64> = vec![];
        let mut w = TestHelpers::test_helpers_f32_literal(start);
        let f32_step = TestHelpers::test_helpers_f32_literal(step);
        while (w) <= max {
            widths.push(w);
            w = TestHelpers::test_helpers_f32_literal(w + f32_step);
        }
        return widths;
    }

    pub fn width_independent_annotation_cache_test_support_assert_equals_nullable_annotation(expected: Option<WidthIndependentParagraphAnnotation>, actual: Option<WidthIndependentParagraphAnnotation>, message: Option<UString>) -> Result<(),
WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault> {
        let recorder = TestTrace::test_trace_current_recorder();
        match &(recorder) {
            Some(__option3) => {
                let e = if expected.is_none() { UString::from("-") } else { UString::from("<annotation>") };
                let a = if actual.is_none() { UString::from("-") } else { UString::from("<annotation>") };
                let mut line = { let mut __s = UString::new(); __s += &(UString::from("eq expected=")); __s += e.as_ustr(); __s += &(UString::from(" actual=")); __s += a.as_ustr(); __s };
                match &(message) {
                    Some(__option6) => {
                        line += &({ let mut __s = UString::new(); __s += &(UString::from(" msg='")); __s += TestTraceRender::test_trace_render_escape_operand(__option6).map_err(|e| WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault::UStringFaultFault(e))?.as_ustr(); __s += &(UString::from("'")); __s });
                    }
                    None => {
                    }
                }
                let _ = __option3.record(line.as_ustr()).map_err(|e| WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault::UStringFaultFault(e))?;
            }
            None => {
            }
        }
        if !(match (&expected, &actual) { (Some(__left), Some(__right)) => (__left).text == (__right).text && (__left).font_size == (__right).font_size && (__left).ruby_font_size == (__right).ruby_font_size && (__left).ruby_stack_gap == (__right).ruby_stack_gap && (__left).ruby_font_weight == (__right).ruby_font_weight && (__left).pinyin_spans == (__right).pinyin_spans && (__left).clreq_profile == (__right).clreq_profile && (__left).punctuation_glyph_substitutor == (__right).punctuation_glyph_substitutor && (__left).quote_pairs == (__right).quote_pairs && (__left).role_override_infos == (__right).role_override_infos && (__left).font_decisions == (__right).font_decisions && (__left).cluster_ranges == (__right).cluster_ranges && (__left).font_decision_by_range == (__right).font_decision_by_range && (__left).inline_object_by_range == (__right).inline_object_by_range && (__left).segment_shaping_cache == (__right).segment_shaping_cache && (__left).substitution_rollbacks == (__right).substitution_rollbacks && (__left).ruby_font_geometry_by_span == (__right).ruby_font_geometry_by_span && (__left).base_shaping_stage == (__right).base_shaping_stage, (None, None) => true, _ => false }) {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("Annotation mismatch"), Some(__option8) => __option8.to_ustring() }).to_ustring()), None).map_err(|e| WidthIndependentAnnotationCacheTestSupportAssertEqualsNullableAnnotationFault::TracedAssertionsFailFaultFault(e))?;
        }
        Ok(())
    }

    pub fn width_independent_annotation_cache_test_support_assert_equals_text_range(expected: TextRange, actual: TextRange, message: Option<UString>) -> Result<(), WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault> {
        let e = TestTraceRender::test_trace_render_canonical_numbers(UString::from(format!("{}", expected.to_string()).as_str()).as_ustr()).map_err(|e| WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault::UStringFaultFault(e))?;
        let a = TestTraceRender::test_trace_render_canonical_numbers(UString::from(format!("{}", actual.to_string()).as_str()).as_ustr()).map_err(|e| WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault::UStringFaultFault(e))?;
        let recorder = TestTrace::test_trace_current_recorder();
        match &(recorder) {
            Some(__option9) => {
                let mut line = { let mut __s = UString::new(); __s += &(UString::from("eq expected=")); __s += e.as_ustr(); __s += &(UString::from(" actual=")); __s += a.as_ustr(); __s };
                match &(message) {
                    Some(__option10) => {
                        line += &({ let mut __s = UString::new(); __s += &(UString::from(" msg='")); __s += TestTraceRender::test_trace_render_escape_operand(__option10).map_err(|e| WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault::UStringFaultFault(e))?.as_ustr(); __s += &(UString::from("'")); __s });
                    }
                    None => {
                    }
                }
                let _ = __option9.record(line.as_ustr()).map_err(|e| WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault::UStringFaultFault(e))?;
            }
            None => {
            }
        }
        if expected.start != actual.start || expected.end != actual.end {
            let _ = TracedAssertions::traced_assertions_fail(Some((match &(message) { None => UString::from("TextRange mismatch"), Some(__option12) => __option12.to_ustring() }).to_ustring()), None).map_err(|e| WidthIndependentAnnotationCacheTestSupportAssertEqualsTextRangeFault::TracedAssertionsFailFaultFault(e))?;
        }
        Ok(())
    }
}
