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
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
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
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault) -> Self {
        match value {
            BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault) -> Self {
        match value {
            BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault) -> Self {
        match value {
            BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault) -> Self {
        match value {
            BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault) -> Self {
        match value {
            BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        BaselineAlignmentTestLatinInsideCjkUsesSharedRomanBaselineFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault) -> Self {
        match value {
            BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault) -> Self {
        match value {
            BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault) -> Self {
        match value {
            BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault) -> Self {
        match value {
            BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault) -> Self {
        match value {
            BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        BaselineAlignmentTestExplicitBaselineShiftAppliesToRomanClustersFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault) -> Self {
        match value {
            BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault) -> Self {
        match value {
            BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault) -> Self {
        match value {
            BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault) -> Self {
        match value {
            BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault) -> Self {
        match value {
            BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        BaselineAlignmentTestCjkPunctuationProvidesIdeographicReferenceWithoutHanBodyFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault) -> Self {
        match value {
            BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault) -> Self {
        match value {
            BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault) -> Self {
        match value {
            BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault) -> Self {
        match value {
            BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault) -> Self {
        match value {
            BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        BaselineAlignmentTestCjkMixedSizesAlignByIdeographicBoxBottomFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[test]
fn cjk_mixed_sizes_align_by_ideographic_box_bottom() {
    testlib::run("org.tiqian.layout.BaselineAlignmentTest.cjkMixedSizesAlignByIdeographicBoxBottom", "org.tiqian.layout.BaselineAlignmentTest.cjkMixedSizesAlignByIdeographicBoxBottom", || {
        let mut t = TestTraceRecorder::new("BaselineAlignmentTest");
        t.section(&"cjkMixedSizesAlignByIdeographicBoxBottom");
        let result = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(LayoutInput::new(TiqianTextContent::new("中小大", Some(vec![
    (TextSpan::new(TextRange::new(1u32, 2u32).unwrap(), TextStyle::new(Some(vec![]), Some(12.0f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
    (TextSpan::new(TextRange::new(2u32, 3u32).unwrap(), TextStyle::new(Some(vec![]), Some(20.0f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb),
None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(400.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let base = (result.clusters[0usize]).clone();
        let small = (result.clusters[1usize]).clone();
        let large = (result.clusters[2usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, base.baseline_shift, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(TracedAssertions::traced_assertions_f32_literal(0.48f64), small.baseline_shift, TracedAssertions::traced_assertions_f32_literal(0.01f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(TracedAssertions::traced_assertions_f32_literal(-0.48f64), large.baseline_shift, TracedAssertions::traced_assertions_f32_literal(0.01f64), None).unwrap();
    });
}

#[test]
fn cjk_punctuation_provides_ideographic_reference_without_han_body() {
    testlib::run("org.tiqian.layout.BaselineAlignmentTest.cjkPunctuationProvidesIdeographicReferenceWithoutHanBody", "org.tiqian.layout.BaselineAlignmentTest.cjkPunctuationProvidesIdeographicReferenceWithoutHanBody", || {
        let mut t = TestTraceRecorder::new("BaselineAlignmentTest");
        t.section(&"cjkPunctuationProvidesIdeographicReferenceWithoutHanBody");
        let result = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(LayoutInput::new(TiqianTextContent::new("MacBook。", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400),
Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(400.0f64,
Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let punctuation = (result.clusters[usize::try_from(u32::wrapping_sub(u32::try_from((result.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, punctuation.baseline_shift, Some("CJK punctuation carries an IdeographicEmBox and must not be aligned to Latin raw descent".to_string())).unwrap();
    });
}

#[test]
fn explicit_baseline_shift_applies_to_roman_clusters() {
    testlib::run("org.tiqian.layout.BaselineAlignmentTest.explicitBaselineShiftAppliesToRomanClusters", "org.tiqian.layout.BaselineAlignmentTest.explicitBaselineShiftAppliesToRomanClusters", || {
        let mut t = TestTraceRecorder::new("BaselineAlignmentTest");
        t.section(&"explicitBaselineShiftAppliesToRomanClusters");
        let result = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(LayoutInput::new(TiqianTextContent::new("中A文", Some(vec![
    (TextSpan::new(TextRange::new(1u32, 2u32).unwrap(), TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(-6.0f64), Some(InlineAttachment::None)))).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb),
None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(400.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let latin = (result.clusters[1usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(-6.0f64, latin.baseline_shift, TracedAssertions::traced_assertions_f32_literal(0.001f64), None).unwrap();
    });
}

#[test]
fn latin_inside_cjk_uses_shared_roman_baseline() {
    testlib::run("org.tiqian.layout.BaselineAlignmentTest.latinInsideCjkUsesSharedRomanBaseline", "org.tiqian.layout.BaselineAlignmentTest.latinInsideCjkUsesSharedRomanBaseline", || {
        let mut t = TestTraceRecorder::new("BaselineAlignmentTest");
        t.section(&"latinInsideCjkUsesSharedRomanBaseline");
        let result = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(LayoutInput::new(TiqianTextContent::new("中A文", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false),
Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(400.0f64,
Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let latin = (result.clusters[1usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, latin.baseline_shift, Some("Latin mixed into CJK should use the shared Roman baseline".to_string())).unwrap();
    });
}
