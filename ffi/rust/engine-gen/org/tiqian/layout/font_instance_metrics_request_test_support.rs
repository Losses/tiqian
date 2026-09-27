use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::FontMetricsRequest;
use crate::org::tiqian::font::font_metrics::FontMetricsResolver;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::font::raw_font_metrics::RawFontMetrics;
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
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum FontInstanceMetricsRequestTestSupportRecordingEngineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for FontInstanceMetricsRequestTestSupportRecordingEngineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontInstanceMetricsRequestTestSupportRecordingEngineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            FontInstanceMetricsRequestTestSupportRecordingEngineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<FontInstanceMetricsRequestTestSupportRecordingEngineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: FontInstanceMetricsRequestTestSupportRecordingEngineFault) -> Self {
        match value {
            FontInstanceMetricsRequestTestSupportRecordingEngineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontInstanceMetricsRequestTestSupportRecordingEngineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: FontInstanceMetricsRequestTestSupportRecordingEngineFault) -> Self {
        match value {
            FontInstanceMetricsRequestTestSupportRecordingEngineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for FontInstanceMetricsRequestTestSupportRecordingEngineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        FontInstanceMetricsRequestTestSupportRecordingEngineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for FontInstanceMetricsRequestTestSupportRecordingEngineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        FontInstanceMetricsRequestTestSupportRecordingEngineFault::UStringFaultFault(value)
    }
}

pub static FONT_INSTANCE_METRICS_REQUEST_TEST_SUPPORT_RECORDED: Mutex<Vec<FontMetricsRequest>> = Mutex::new(vec![]);

#[derive(Clone, Copy)]
pub struct FontInstanceMetricsRequestTestSupport;

impl FontInstanceMetricsRequestTestSupport {

    pub fn font_instance_metrics_request_test_support_recording_engine() -> Result<ExplainableStubParagraphLayoutEngine, ParagraphLayoutEngineNewFault> {
        *FONT_INSTANCE_METRICS_REQUEST_TEST_SUPPORT_RECORDED.lock().unwrap_or_else(|e| e.into_inner()) = vec![];
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(RecordingResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?);
    }

    pub fn font_instance_metrics_request_test_support_base_style() -> TextStyle {
        return TextStyle::new(Some(vec![UString::from("Fixture Sans").to_ustring()]), Some(18.0f64), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None));
    }
}

#[derive(Clone, PartialEq)]
pub struct RecordingResolver {
    pub(crate) stub: StubFontMetricsResolver,
}

impl RecordingResolver {
    pub fn new() -> Self {
        Self {
            stub: StubFontMetricsResolver::new(),
        }
    }

    pub fn resolve(&self, request: FontMetricsRequest) -> RawFontMetrics {
        let result = self.stub.resolve((request).clone());
        FONT_INSTANCE_METRICS_REQUEST_TEST_SUPPORT_RECORDED.lock().unwrap_or_else(|e| e.into_inner()).push(request);
        return result;
    }
}

impl FontMetricsResolver for RecordingResolver {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.FontInstanceMetricsRequestTestSupport.RecordingResolver"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn FontMetricsResolver> {
        Box::new(self.clone())
    }

    fn resolve(&self, request: FontMetricsRequest) -> Result<RawFontMetrics, TextRangeError> {
        let result = self.stub.resolve((request).clone());
        FONT_INSTANCE_METRICS_REQUEST_TEST_SUPPORT_RECORDED.lock().unwrap_or_else(|e| e.into_inner()).push(request);
        return Ok(result);
    }
}
