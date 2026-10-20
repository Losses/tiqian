#![cfg(test)]

use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::core::font_decision_info::FontDecisionInfo;
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
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::font::font_role::FontRole;
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
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum ContextualDashEllipsisClusterCoverageTestLayoutFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for ContextualDashEllipsisClusterCoverageTestLayoutFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualDashEllipsisClusterCoverageTestLayoutFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualDashEllipsisClusterCoverageTestLayoutFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ContextualDashEllipsisClusterCoverageTestLayoutFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualDashEllipsisClusterCoverageTestLayoutFault) -> Self {
        match value {
            ContextualDashEllipsisClusterCoverageTestLayoutFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualDashEllipsisClusterCoverageTestLayoutFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualDashEllipsisClusterCoverageTestLayoutFault) -> Self {
        match value {
            ContextualDashEllipsisClusterCoverageTestLayoutFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualDashEllipsisClusterCoverageTestLayoutFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualDashEllipsisClusterCoverageTestLayoutFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualDashEllipsisClusterCoverageTestLayoutFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualDashEllipsisClusterCoverageTestLayoutFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualDashEllipsisClusterCoverageTestStyleSpanInsideLatinDashRunSplitsTheClusterFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for ContextualDashEllipsisClusterCoverageTestStyleSpanInsideLatinDashRunSplitsTheClusterFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualDashEllipsisClusterCoverageTestStyleSpanInsideLatinDashRunSplitsTheClusterFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualDashEllipsisClusterCoverageTestStyleSpanInsideLatinDashRunSplitsTheClusterFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ContextualDashEllipsisClusterCoverageTestStyleSpanInsideLatinDashRunSplitsTheClusterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualDashEllipsisClusterCoverageTestStyleSpanInsideLatinDashRunSplitsTheClusterFault) -> Self {
        match value {
            ContextualDashEllipsisClusterCoverageTestStyleSpanInsideLatinDashRunSplitsTheClusterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualDashEllipsisClusterCoverageTestStyleSpanInsideLatinDashRunSplitsTheClusterFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ContextualDashEllipsisClusterCoverageTestStyleSpanInsideLatinDashRunSplitsTheClusterFault) -> Self {
        match value {
            ContextualDashEllipsisClusterCoverageTestStyleSpanInsideLatinDashRunSplitsTheClusterFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualDashEllipsisClusterCoverageTestStyleSpanInsideLatinDashRunSplitsTheClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualDashEllipsisClusterCoverageTestStyleSpanInsideLatinDashRunSplitsTheClusterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ContextualDashEllipsisClusterCoverageTestStyleSpanInsideLatinDashRunSplitsTheClusterFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ContextualDashEllipsisClusterCoverageTestStyleSpanInsideLatinDashRunSplitsTheClusterFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualDashEllipsisClusterCoverageTestLatinDashRunAtParagraphEndStaysOneClusterFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for ContextualDashEllipsisClusterCoverageTestLatinDashRunAtParagraphEndStaysOneClusterFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextualDashEllipsisClusterCoverageTestLatinDashRunAtParagraphEndStaysOneClusterFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ContextualDashEllipsisClusterCoverageTestLatinDashRunAtParagraphEndStaysOneClusterFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<ContextualDashEllipsisClusterCoverageTestLatinDashRunAtParagraphEndStaysOneClusterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualDashEllipsisClusterCoverageTestLatinDashRunAtParagraphEndStaysOneClusterFault) -> Self {
        match value {
            ContextualDashEllipsisClusterCoverageTestLatinDashRunAtParagraphEndStaysOneClusterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualDashEllipsisClusterCoverageTestLatinDashRunAtParagraphEndStaysOneClusterFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: ContextualDashEllipsisClusterCoverageTestLatinDashRunAtParagraphEndStaysOneClusterFault) -> Self {
        match value {
            ContextualDashEllipsisClusterCoverageTestLatinDashRunAtParagraphEndStaysOneClusterFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualDashEllipsisClusterCoverageTestLatinDashRunAtParagraphEndStaysOneClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualDashEllipsisClusterCoverageTestLatinDashRunAtParagraphEndStaysOneClusterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for ContextualDashEllipsisClusterCoverageTestLatinDashRunAtParagraphEndStaysOneClusterFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        ContextualDashEllipsisClusterCoverageTestLatinDashRunAtParagraphEndStaysOneClusterFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct ContextualDashEllipsisClusterCoverageTestSupport;

impl ContextualDashEllipsisClusterCoverageTestSupport {
    pub fn contextual_dash_ellipsis_cluster_coverage_test_support_single_mark(d: &[FontDecisionInfo]) -> Result<FontDecisionInfo, TextRangeError> {
        for i in 0..match u32::try_from(d.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if u32::from_ne_bytes(((u_string::find_from(&(((d[usize::try_from(i).unwrap_or(0)]).clone().source_text).to_ustring()), UString::from("—").as_ustr(), 0)) as u32).to_ne_bytes()) <= 2147483647 {
                return Ok((d[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        return Err(TextRangeError::Message { text: UString::from("no fontDecision with sourceText containing dash") });
    }

    pub fn contextual_dash_ellipsis_cluster_coverage_test_support_dash_singles(d: &[FontDecisionInfo]) -> Vec<FontDecisionInfo> {
        let mut a: Vec<FontDecisionInfo> = vec![];
        for i in 0..match u32::try_from(d.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if d[usize::try_from(i).unwrap_or(0)].clone().source_text.to_ustring() == UString::from("—") {
                a.push((d[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        return a;
    }

    pub fn contextual_dash_ellipsis_cluster_coverage_test_support_layout(text: &UStr, spans: Option<Vec<TextSpan>>) -> Result<LayoutResult, ParagraphLayoutEngineNewFault> {
        return Ok(ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some((DefaultHyphenator::default_hyphenator_default_hyphenator()?).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512)))))?.layout(LayoutInput::new(TiqianTextContent::new(text, Some((match &(spans) { None => vec![], Some(__option24) => (*__option24).clone() }).clone()), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(1000.0f64, Some(f64::INFINITY), Some(2147483647)).map_err(|e| ParagraphLayoutEngineNewFault::TextRangeErrorFault(e))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).map_err(|e| ParagraphLayoutEngineNewFault::LayoutWithRejectedTechnicalTiersFault(e))?);
    }
}

#[test]
fn latin_dash_run_at_paragraph_end_stays_one_cluster() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisClusterCoverageTest.latinDashRunAtParagraphEndStaysOneCluster", "org.tiqian.layout.ContextualDashEllipsisClusterCoverageTest.latinDashRunAtParagraphEndStaysOneCluster", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[67,111,110,116,101,120,116,117,97,108,68,97,115,104,69,108,108,105,112,115,105,115,67,108,117,115,116,101,114,67,111,118,101,114,97,103,101,84,101,115,116])));
        t.section(UStr::new(&[108,97,116,105,110,68,97,115,104,82,117,110,65,116,80,97,114,97,103,114,97,112,104,69,110,100,83,116,97,121,115,79,110,101,67,108,117,115,116,101,114]));
        let d = ContextualDashEllipsisClusterCoverageTestSupport::contextual_dash_ellipsis_cluster_coverage_test_support_single_mark(&(ContextualDashEllipsisClusterCoverageTestSupport::contextual_dash_ellipsis_cluster_coverage_test_support_layout(UStr::new(&[69,110,100,8212,8212]), None).unwrap().debug).clone().font_decisions).unwrap();
        if d.source_text.to_ustring() != UString::from("——") || (d.role).to_ustring() != UString::from(FontRole::LatinText.name()) {
            panic!("{}", TextRangeError::Message { text: UString::from("latin dash cluster") });
        }
    });
}

#[test]
fn style_span_inside_latin_dash_run_splits_the_cluster() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisClusterCoverageTest.styleSpanInsideLatinDashRunSplitsTheCluster", "org.tiqian.layout.ContextualDashEllipsisClusterCoverageTest.styleSpanInsideLatinDashRunSplitsTheCluster", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[67,111,110,116,101,120,116,117,97,108,68,97,115,104,69,108,108,105,112,115,105,115,67,108,117,115,116,101,114,67,111,118,101,114,97,103,101,84,101,115,116])));
        t.section(UStr::new(&[115,116,121,108,101,83,112,97,110,73,110,115,105,100,101,76,97,116,105,110,68,97,115,104,82,117,110,83,112,108,105,116,115,84,104,101,67,108,117,115,116,101,114]));
        let r = ContextualDashEllipsisClusterCoverageTestSupport::contextual_dash_ellipsis_cluster_coverage_test_support_layout(UStr::new(&[65,8212,8212,66]), Some(vec![
    (TextSpan::new(TextRange::new(2u32, 3u32).unwrap(), TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(700), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
])).unwrap();
        let ds = ContextualDashEllipsisClusterCoverageTestSupport::contextual_dash_ellipsis_cluster_coverage_test_support_dash_singles(&(r.debug).clone().font_decisions);
        if u32::try_from((ds.len()) & 0xFFFF_FFFF).unwrap_or(0) != 2 {
            panic!("{}", TextRangeError::Message { text: UString::from("dash split count") });
        }
        for i in 0..match u32::try_from(ds.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if ds[usize::try_from(i).unwrap_or(0)].clone().role.to_ustring() != UString::from(FontRole::LatinText.name()) {
                panic!("{}", TextRangeError::Message { text: UString::from("dash split role") });
            }
        }
    });
}
