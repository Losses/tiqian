#![cfg(test)]

use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::punctuation_decision_info::PunctuationDecisionInfo;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::layout::default_hyphenator::DefaultHyphenator;
use crate::org::tiqian::layout::interpunct_shrink_opportunity_test_support::InterpunctShrinkOpportunityTestSupport;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault) -> Self {
        match value {
            InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault) -> Self {
        match value {
            InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault) -> Self {
        match value {
            InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault) -> Self {
        match value {
            InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault) -> Self {
        match value {
            InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        InterpunctShrinkOpportunityTestPreservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrinkFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault) -> Self {
        match value {
            InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault) -> Self {
        match value {
            InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault) -> Self {
        match value {
            InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault) -> Self {
        match value {
            InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault) -> Self {
        match value {
            InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        InterpunctShrinkOpportunityTestInterpunctInkEvidenceFreesPairedGlueForTierThreeShrinkFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[test]
fn interpunct_ink_evidence_frees_paired_glue_for_tier_three_shrink() {
    testlib::run("org.tiqian.layout.InterpunctShrinkOpportunityTest.interpunctInkEvidenceFreesPairedGlueForTierThreeShrink", "org.tiqian.layout.InterpunctShrinkOpportunityTest.interpunctInkEvidenceFreesPairedGlueForTierThreeShrink", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[73,110,116,101,114,112,117,110,99,116,83,104,114,105,110,107,79,112,112,111,114,116,117,110,105,116,121,84,101,115,116])));
        t.section(UStr::new(&[105,110,116,101,114,112,117,110,99,116,73,110,107,69,118,105,100,101,110,99,101,70,114,101,101,115,80,97,105,114,101,100,71,108,117,101,70,111,114,84,105,101,114,84,104,114,101,101,83,104,114,105,110,107]));
        let text = UString::from("正文·间隔号·后文…结尾").to_ustring();
        let result = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(InterpunctShrinkOpportunityTestSupport::interpunct_shrink_opportunity_test_support_halt_ink_shaper()), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap().layout(LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let mut dots: Vec<PunctuationDecisionInfo> = vec![];
        for i in 0..match u32::try_from((result.debug).clone().punctuation_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.debug.clone().punctuation_decisions[usize::try_from(i).unwrap_or(0)].clone().char.to_ustring() == UString::from("·") {
                dots.push(((result.debug).clone().punctuation_decisions[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((dots.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        for dot in &dots {
            let _ = TracedAssertions::traced_assertions_assert_true((dot.leading_glue_natural) > (0.0f64), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("leading glue: ")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(dot.leading_glue_natural)); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true((dot.trailing_glue_natural) > (0.0f64), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("trailing glue: ")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(dot.trailing_glue_natural)); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[67,101,110,116,101,114]), (dot.anchor).to_ustring().as_ustr(), None).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[70,111,110,116,72,97,108,116,70,105,116,116,101,100,66,111,100,121,67,111,109,112,114,101,115,115,105,111,110]), (dot.geometry_source).to_ustring().as_ustr(), None).unwrap();
        }
        let ellipsis = ((result.debug).clone().punctuation_decisions[usize::try_from(u32::wrapping_sub(u32::try_from(((result.debug).clone().punctuation_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, ellipsis.leading_glue_natural, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((ellipsis.trailing_glue_natural) > (0.0f64), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("trailing glue: ")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(ellipsis.trailing_glue_natural)); __s }).as_str()))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn preserved_interpunct_codepoint_keeps_interpunct_class_for_tier_three_shrink() {
    testlib::run("org.tiqian.layout.InterpunctShrinkOpportunityTest.preservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrink", "org.tiqian.layout.InterpunctShrinkOpportunityTest.preservedInterpunctCodepointKeepsInterpunctClassForTierThreeShrink", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[73,110,116,101,114,112,117,110,99,116,83,104,114,105,110,107,79,112,112,111,114,116,117,110,105,116,121,84,101,115,116])));
        t.section(UStr::new(&[112,114,101,115,101,114,118,101,100,73,110,116,101,114,112,117,110,99,116,67,111,100,101,112,111,105,110,116,75,101,101,112,115,73,110,116,101,114,112,117,110,99,116,67,108,97,115,115,70,111,114,84,105,101,114,84,104,114,101,101,83,104,114,105,110,107]));
        let text = UString::from("正文・间隔・后文").to_ustring();
        let result = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(InterpunctShrinkOpportunityTestSupport::interpunct_shrink_opportunity_test_support_preserve_resolver()), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(InterpunctShrinkOpportunityTestSupport::interpunct_shrink_opportunity_test_support_halt_ink_shaper()), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap().layout(LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let mut interpuncts: Vec<PunctuationDecisionInfo> = vec![];
        for i in 0..match u32::try_from((result.debug).clone().punctuation_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.debug.clone().punctuation_decisions[usize::try_from(i).unwrap_or(0)].clone().punctuation_class.to_ustring() == UString::from("Interpunct") {
                interpuncts.push(((result.debug).clone().punctuation_decisions[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        let mut chars: Vec<UString> = vec![];
        for dot in &interpuncts {
            chars.push((dot.char).to_ustring());
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("・").to_ustring(), UString::from("・").to_ustring()], &chars, None).unwrap();
        for dot in &interpuncts {
            let _ = TracedAssertions::traced_assertions_assert_true((dot.leading_glue_natural) > (0.0f64), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("leading glue: ")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(dot.leading_glue_natural)); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true((dot.trailing_glue_natural) > (0.0f64), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("trailing glue: ")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(dot.trailing_glue_natural)); __s }).as_str()))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[67,101,110,116,101,114]), (dot.anchor).to_ustring().as_ustr(), None).unwrap();
        }
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), None).unwrap();
    });
}
