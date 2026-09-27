#![cfg(test)]

use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_box_outer_spacing::InlineBoxOuterSpacing;
use crate::org::tiqian::core::inline_box_span::InlineBoxSpan;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
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
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphFns;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;


#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertFalseFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault) -> Self {
        match value {
            PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault) -> Self {
        match value {
            PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault) -> Self {
        match value {
            PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault) -> Self {
        match value {
            PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault) -> Self {
        match value {
            PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault {
    fn from(value: PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault) -> Self {
        match value {
            PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault::TracedAssertionsAssertFalseFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault) -> Self {
        match value {
            PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault) -> Self {
        match value {
            PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault> for PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault) -> Self {
        PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault::TracedAssertionsAssertFalseFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        PreparedParagraphInlineEdgesTestEndOnlyInlineBoxEmitsEdgeWithoutInlineStartFieldFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    TracedAssertionsAssertFalseFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault) -> Self {
        match value {
            PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault) -> Self {
        match value {
            PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault) -> Self {
        match value {
            PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault) -> Self {
        match value {
            PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault {
    fn from(value: PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault) -> Self {
        match value {
            PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault::TracedAssertionsAssertFalseFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault) -> Self {
        match value {
            PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault) -> Self {
        match value {
            PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault> for PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault) -> Self {
        PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault::TracedAssertionsAssertFalseFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        PreparedParagraphInlineEdgesTestContentWithoutInlineBoxesOmitsInlineEdgesArrayFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[test]
fn content_without_inline_boxes_omits_inline_edges_array() {
    testlib::run("org.tiqian.layout.PreparedParagraphInlineEdgesTest.contentWithoutInlineBoxesOmitsInlineEdgesArray", "org.tiqian.layout.PreparedParagraphInlineEdgesTest.contentWithoutInlineBoxesOmitsInlineEdgesArray", || {
        let mut t = TestTraceRecorder::new("PreparedParagraphInlineEdgesTest");
        t.section(&"contentWithoutInlineBoxesOmitsInlineEdgesArray");
        let result = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(LayoutInput::new(TiqianTextContent::new("中文正文", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false),
Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320 as f64 as f64,
Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let json = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json((result).clone(), true).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&json, "\"inlineEdges\":", 0)).to_ne_bytes())) <= 2147483647, Some((format!("{}{}",
            "no boxes, no edges: ",
            json
        )).to_string())).unwrap();
    });
}

#[test]
fn end_only_inline_box_emits_edge_without_inline_start_field() {
    testlib::run("org.tiqian.layout.PreparedParagraphInlineEdgesTest.endOnlyInlineBoxEmitsEdgeWithoutInlineStartField", "org.tiqian.layout.PreparedParagraphInlineEdgesTest.endOnlyInlineBoxEmitsEdgeWithoutInlineStartField", || {
        let mut t = TestTraceRecorder::new("PreparedParagraphInlineEdgesTest");
        t.section(&"endOnlyInlineBoxEmitsEdgeWithoutInlineStartField");
        let result = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(LayoutInput::new(TiqianTextContent::new("中文正文", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false),
Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320 as f64 as f64,
Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![
    (InlineBoxSpan::new(TextRange::new(0u32, 2u32).unwrap(), Some(0 as f64), Some(4 as f64), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]), Some(vec![]))).unwrap();
        let json = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json((result).clone(), true).unwrap();
        let edges_at = u_string::find_from(&json, "\"inlineEdges\":[", 0);
        let _ = TracedAssertions::traced_assertions_assert_true((edges_at) >= 0, Some((format!("{}{}",
            "inlineEdges array missing: ",
            json
        )).to_string())).unwrap();
        let entry = u_string::substring_from(&json, i32::from_ne_bytes((edges_at).to_ne_bytes()));
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&entry, "\"offset\":2", 0)).to_ne_bytes())) <= 2147483647, Some((format!("{}{}",
            "edge offset (box end) missing: ",
            entry
        )).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&entry, "\"inlineEnd\":4", 0)).to_ne_bytes())) <= 2147483647, Some((format!("{}{}",
            "inlineEnd field missing: ",
            entry
        )).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&entry, "\"inlineStart\":", 0)).to_ne_bytes())) <= 2147483647, Some((format!("{}{}",
            "inlineStart must be absent for an end-only box: ",
            entry
        )).to_string())).unwrap();
    });
}
