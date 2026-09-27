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
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::layout::default_hyphenator::DefaultHyphenator;
use crate::org::tiqian::layout::hyphenation_layout_test_support::HyphenationLayoutTestSupport;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::linebreak::english_hyphenation::EnglishHyphenation;
use crate::org::tiqian::linebreak::hyphenator::NoHyphenator;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub enum HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault) -> Self {
        match value {
            HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault) -> Self {
        match value {
            HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault) -> Self {
        match value {
            HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault) -> Self {
        match value {
            HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault) -> Self {
        match value {
            HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault) -> Self {
        match value {
            HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault) -> Self {
        match value {
            HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault) -> Self {
        match value {
            HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault) -> Self {
        match value {
            HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault) -> Self {
        match value {
            HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault) -> Self {
        match value {
            HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault) -> Self {
        match value {
            HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault) -> Self {
        match value {
            HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HyphenationLayoutTestHyphenationIsOnByDefaultFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<HyphenationLayoutTestHyphenationIsOnByDefaultFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: HyphenationLayoutTestHyphenationIsOnByDefaultFault) -> Self {
        match value {
            HyphenationLayoutTestHyphenationIsOnByDefaultFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestHyphenationIsOnByDefaultFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: HyphenationLayoutTestHyphenationIsOnByDefaultFault) -> Self {
        match value {
            HyphenationLayoutTestHyphenationIsOnByDefaultFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestHyphenationIsOnByDefaultFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: HyphenationLayoutTestHyphenationIsOnByDefaultFault) -> Self {
        match value {
            HyphenationLayoutTestHyphenationIsOnByDefaultFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestHyphenationIsOnByDefaultFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: HyphenationLayoutTestHyphenationIsOnByDefaultFault) -> Self {
        match value {
            HyphenationLayoutTestHyphenationIsOnByDefaultFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestHyphenationIsOnByDefaultFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: HyphenationLayoutTestHyphenationIsOnByDefaultFault) -> Self {
        match value {
            HyphenationLayoutTestHyphenationIsOnByDefaultFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for HyphenationLayoutTestHyphenationIsOnByDefaultFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        HyphenationLayoutTestHyphenationIsOnByDefaultFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for HyphenationLayoutTestHyphenationIsOnByDefaultFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        HyphenationLayoutTestHyphenationIsOnByDefaultFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for HyphenationLayoutTestHyphenationIsOnByDefaultFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        HyphenationLayoutTestHyphenationIsOnByDefaultFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for HyphenationLayoutTestHyphenationIsOnByDefaultFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        HyphenationLayoutTestHyphenationIsOnByDefaultFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for HyphenationLayoutTestHyphenationIsOnByDefaultFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        HyphenationLayoutTestHyphenationIsOnByDefaultFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault) -> Self {
        match value {
            HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault) -> Self {
        match value {
            HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault) -> Self {
        match value {
            HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault) -> Self {
        match value {
            HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault) -> Self {
        match value {
            HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault) -> Self {
        match value {
            HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault) -> Self {
        match value {
            HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault) -> Self {
        match value {
            HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn fitting_word_hyphenates_only_when_a_hyphenator_is_injected() {
    testlib::run("org.tiqian.layout.HyphenationLayoutTest.fittingWordHyphenatesOnlyWhenAHyphenatorIsInjected", "org.tiqian.layout.HyphenationLayoutTest.fittingWordHyphenatesOnlyWhenAHyphenatorIsInjected", || {
        let mut t = TestTraceRecorder::new("HyphenationLayoutTest");
        t.section(&"fittingWordHyphenatesOnlyWhenAHyphenatorIsInjected");
        let n = HyphenationLayoutTestSupport::hyphenation_layout_test_support_layout_with(Box::new(NoHyphenator::new()), &"中文中 coffee", 112 as f64).unwrap();
        let h = HyphenationLayoutTestSupport::hyphenation_layout_test_support_layout_with(EnglishHyphenation::english_hyphenation_en_us().unwrap(), &"中文中 coffee", 112 as f64).unwrap();
        let mut nc = false;
        let mut nh = true;
        let mut hc = false;
        let mut cof = false;
        let mut fee = false;
        let mut hh = false;
        for i in 0..match u32::try_from(n.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if n.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_string() == "coffee" {
                nc = true;
            }
        }
        for i in 0..match u32::try_from(n.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if n.lines[usize::try_from(i).unwrap_or(0)].hyphen_advance > (0 as f64) {
                nh = false;
            }
        }
        for i in 0..match u32::try_from(h.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if h.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_string() == "coffee" {
                hc = true;
            }
            if h.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_string() == "cof" {
                cof = true;
            }
            if h.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_string() == "fee" {
                fee = true;
            }
        }
        for i in 0..match u32::try_from(h.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if h.lines[usize::try_from(i).unwrap_or(0)].hyphen_advance > (0 as f64) {
                hh = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(nc, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(nh, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!hc, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(cof, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(fee, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(hh, Some("no line hyphenated".to_string())).unwrap();
    });
}

#[test]
fn hyphen_is_reserved_within_the_measure_not_hung_past_it() {
    testlib::run("org.tiqian.layout.HyphenationLayoutTest.hyphenIsReservedWithinTheMeasureNotHungPastIt", "org.tiqian.layout.HyphenationLayoutTest.hyphenIsReservedWithinTheMeasureNotHungPastIt", || {
        let mut t = TestTraceRecorder::new("HyphenationLayoutTest");
        t.section(&"hyphenIsReservedWithinTheMeasureNotHungPastIt");
        let h = HyphenationLayoutTestSupport::hyphenation_layout_test_support_layout_with(EnglishHyphenation::english_hyphenation_en_us().unwrap(), &"请运行 internationalization 命令", 160 as f64).unwrap();
        let mut line = (h.lines[0usize]).clone();
        for i in 0..match u32::try_from(h.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if h.lines[usize::try_from(i).unwrap_or(0)].hyphen_advance > (0 as f64) {
                line = (h.lines[usize::try_from(i).unwrap_or(0)]).clone();
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((line.indent + line.visual_width + line.hyphen_advance) <= 160.01f64, Some((format!("{}{}",
            "hyphen hung past the measure: ",
            (line.indent + line.visual_width + line.hyphen_advance)
        )).to_string())).unwrap();
    });
}

#[test]
fn hyphenation_is_on_by_default() {
    testlib::run("org.tiqian.layout.HyphenationLayoutTest.hyphenationIsOnByDefault", "org.tiqian.layout.HyphenationLayoutTest.hyphenationIsOnByDefault", || {
        let mut t = TestTraceRecorder::new("HyphenationLayoutTest");
        t.section(&"hyphenationIsOnByDefault");
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(LayoutInput::new(TiqianTextContent::new("中文中 coffee", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400),
Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(112 as f64 as f64,
Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let mut cof = false;
        let mut fee = false;
        let mut hy = false;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_string() == "cof" {
                cof = true;
            }
            if r.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_string() == "fee" {
                fee = true;
            }
        }
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.lines[usize::try_from(i).unwrap_or(0)].hyphen_advance > (0 as f64) {
                hy = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(cof, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(fee, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(hy, None).unwrap();
    });
}

#[test]
fn hyphenation_is_skipped_when_stretching_cjk_stays_tight() {
    testlib::run("org.tiqian.layout.HyphenationLayoutTest.hyphenationIsSkippedWhenStretchingCjkStaysTight", "org.tiqian.layout.HyphenationLayoutTest.hyphenationIsSkippedWhenStretchingCjkStaysTight", || {
        let mut t = TestTraceRecorder::new("HyphenationLayoutTest");
        t.section(&"hyphenationIsSkippedWhenStretchingCjkStaysTight");
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(HyphenationLayoutTestSupport::hyphenation_layout_test_support_push_out_resolver()), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()),
Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())),
Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(LayoutInput::new(TiqianTextContent::new("中文中文中文中文 coffee", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])),
Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()),
Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(180 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let mut hy = false;
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.lines[usize::try_from(i).unwrap_or(0)].hyphen_advance > (0 as f64) {
                hy = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!hy, Some("should not hyphenate when tight".to_string())).unwrap();
    });
}

#[test]
fn reserved_hyphen_squeezes_punctuation_glue_to_pull_it_in() {
    testlib::run("org.tiqian.layout.HyphenationLayoutTest.reservedHyphenSqueezesPunctuationGlueToPullItIn", "org.tiqian.layout.HyphenationLayoutTest.reservedHyphenSqueezesPunctuationGlueToPullItIn", || {
        let mut t = TestTraceRecorder::new("HyphenationLayoutTest");
        t.section(&"reservedHyphenSqueezesPunctuationGlueToPullItIn");
        let r = HyphenationLayoutTestSupport::hyphenation_layout_test_support_layout_with(EnglishHyphenation::english_hyphenation_en_us().unwrap(), &"中文，internationalization", 128 as f64).unwrap();
        let mut c = (r.clusters[0usize]).clone();
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_string() == "，" {
                c = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((c.advance) < (16 as f64), Some((format!("{}{}",
            "comma glue not compressed for the hyphen: ",
            c.advance
        )).to_string())).unwrap();
    });
}

#[test]
fn syllable_split_matches_the_hyphenator_exactly() {
    testlib::run("org.tiqian.layout.HyphenationLayoutTest.syllableSplitMatchesTheHyphenatorExactly", "org.tiqian.layout.HyphenationLayoutTest.syllableSplitMatchesTheHyphenatorExactly", || {
        let mut t = TestTraceRecorder::new("HyphenationLayoutTest");
        t.section(&"syllableSplitMatchesTheHyphenatorExactly");
        let word = "internationalization".to_string();
        let r = HyphenationLayoutTestSupport::hyphenation_layout_test_support_layout_with(EnglishHyphenation::english_hyphenation_en_us().unwrap(), HyphenationLayoutTestSupport::HYPHENATION_LAYOUT_TEST_SUPPORT_TEXT.to_string().as_str(), 160 as f64).unwrap();
        let mut parts: Vec<String> = vec![];
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_string() != "" && HyphenationLayoutTestSupport::hyphenation_layout_test_support_is_latin(((r.clusters[usize::try_from(i).unwrap_or(0)]).clone().text).to_string().as_str()) {
                parts.push(((r.clusters[usize::try_from(i).unwrap_or(0)]).clone().text).to_string());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(HyphenationLayoutTestSupport::hyphenation_layout_test_support_rebuild(word.as_str(), &EnglishHyphenation::english_hyphenation_en_us().unwrap().hyphenate(word.as_str())).as_str(), { let joined2 = parts; let
mut out = String::new(); let n = joined2.len(); let mut index2 = 0usize; while index2 < n { if index2 > 0 { out.push_str(&("-")); } let _ = write!(out, "{}", joined2[index2]); index2 += 1; } out }.as_str(), None).unwrap();
    });
}
