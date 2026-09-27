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
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestSyllableSplitMatchesTheHyphenatorExactlyFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestReservedHyphenSqueezesPunctuationGlueToPullItInFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestHyphenationIsSkippedWhenStretchingCjkStaysTightFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for HyphenationLayoutTestHyphenationIsOnByDefaultFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HyphenationLayoutTestHyphenationIsOnByDefaultFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestHyphenationIsOnByDefaultFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestHyphenationIsOnByDefaultFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestHyphenationIsOnByDefaultFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestHyphenationIsOnByDefaultFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestHyphenIsReservedWithinTheMeasureNotHungPastItFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            HyphenationLayoutTestFittingWordHyphenatesOnlyWhenAHyphenatorIsInjectedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        let mut t = TestTraceRecorder::new(&(UStr::new(&[72,121,112,104,101,110,97,116,105,111,110,76,97,121,111,117,116,84,101,115,116])));
        t.section(UStr::new(&[102,105,116,116,105,110,103,87,111,114,100,72,121,112,104,101,110,97,116,101,115,79,110,108,121,87,104,101,110,65,72,121,112,104,101,110,97,116,111,114,73,115,73,110,106,101,99,116,101,100]));
        let n = HyphenationLayoutTestSupport::hyphenation_layout_test_support_layout_with(Box::new(NoHyphenator::new()), UStr::new(&[20013,25991,20013,32,99,111,102,102,101,101]), 112 as f64).unwrap();
        let h = HyphenationLayoutTestSupport::hyphenation_layout_test_support_layout_with(EnglishHyphenation::english_hyphenation_en_us().unwrap(), UStr::new(&[20013,25991,20013,32,99,111,102,102,101,101]), 112 as f64).unwrap();
        let mut nc = false;
        let mut nh = true;
        let mut hc = false;
        let mut cof = false;
        let mut fee = false;
        let mut hh = false;
        for i in 0..match u32::try_from(n.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if n.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_ustring() == UString::from("coffee") {
                nc = true;
            }
        }
        for i in 0..match u32::try_from(n.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if n.lines[usize::try_from(i).unwrap_or(0)].hyphen_advance > (0 as f64) {
                nh = false;
            }
        }
        for i in 0..match u32::try_from(h.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if h.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_ustring() == UString::from("coffee") {
                hc = true;
            }
            if h.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_ustring() == UString::from("cof") {
                cof = true;
            }
            if h.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_ustring() == UString::from("fee") {
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
        let _ = TracedAssertions::traced_assertions_assert_true(hh, Some(UString::from("no line hyphenated"))).unwrap();
    });
}

#[test]
fn hyphen_is_reserved_within_the_measure_not_hung_past_it() {
    testlib::run("org.tiqian.layout.HyphenationLayoutTest.hyphenIsReservedWithinTheMeasureNotHungPastIt", "org.tiqian.layout.HyphenationLayoutTest.hyphenIsReservedWithinTheMeasureNotHungPastIt", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[72,121,112,104,101,110,97,116,105,111,110,76,97,121,111,117,116,84,101,115,116])));
        t.section(UStr::new(&[104,121,112,104,101,110,73,115,82,101,115,101,114,118,101,100,87,105,116,104,105,110,84,104,101,77,101,97,115,117,114,101,78,111,116,72,117,110,103,80,97,115,116,73,116]));
        let h = HyphenationLayoutTestSupport::hyphenation_layout_test_support_layout_with(EnglishHyphenation::english_hyphenation_en_us().unwrap(), UStr::new(&[35831,36816,34892,32,105,110,116,101,114,110,97,116,105,111,110,97,108,105,122,97,116,105,111,110,32,21629,20196]), 160 as f64).unwrap();
        let mut line = (h.lines[0usize]).clone();
        for i in 0..match u32::try_from(h.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if h.lines[usize::try_from(i).unwrap_or(0)].hyphen_advance > (0 as f64) {
                line = (h.lines[usize::try_from(i).unwrap_or(0)]).clone();
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((line.indent + line.visual_width + line.hyphen_advance) <= 160.01f64, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("hyphen hung past the measure: ")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(line.indent + line.visual_width + line.hyphen_advance)); __s }).as_str()))).unwrap();
    });
}

#[test]
fn hyphenation_is_on_by_default() {
    testlib::run("org.tiqian.layout.HyphenationLayoutTest.hyphenationIsOnByDefault", "org.tiqian.layout.HyphenationLayoutTest.hyphenationIsOnByDefault", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[72,121,112,104,101,110,97,116,105,111,110,76,97,121,111,117,116,84,101,115,116])));
        t.section(UStr::new(&[104,121,112,104,101,110,97,116,105,111,110,73,115,79,110,66,121,68,101,102,97,117,108,116]));
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap().layout(LayoutInput::new(TiqianTextContent::new(&(UStr::new(&[20013,25991,20013,32,99,111,102,102,101,101])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(112 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let mut cof = false;
        let mut fee = false;
        let mut hy = false;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_ustring() == UString::from("cof") {
                cof = true;
            }
            if r.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_ustring() == UString::from("fee") {
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
        let mut t = TestTraceRecorder::new(&(UStr::new(&[72,121,112,104,101,110,97,116,105,111,110,76,97,121,111,117,116,84,101,115,116])));
        t.section(UStr::new(&[104,121,112,104,101,110,97,116,105,111,110,73,115,83,107,105,112,112,101,100,87,104,101,110,83,116,114,101,116,99,104,105,110,103,67,106,107,83,116,97,121,115,84,105,103,104,116]));
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(HyphenationLayoutTestSupport::hyphenation_layout_test_support_push_out_resolver()), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap().layout(LayoutInput::new(TiqianTextContent::new(&(UStr::new(&[20013,25991,20013,25991,20013,25991,20013,25991,32,99,111,102,102,101,101])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(180 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let mut hy = false;
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.lines[usize::try_from(i).unwrap_or(0)].hyphen_advance > (0 as f64) {
                hy = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!hy, Some(UString::from("should not hyphenate when tight"))).unwrap();
    });
}

#[test]
fn reserved_hyphen_squeezes_punctuation_glue_to_pull_it_in() {
    testlib::run("org.tiqian.layout.HyphenationLayoutTest.reservedHyphenSqueezesPunctuationGlueToPullItIn", "org.tiqian.layout.HyphenationLayoutTest.reservedHyphenSqueezesPunctuationGlueToPullItIn", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[72,121,112,104,101,110,97,116,105,111,110,76,97,121,111,117,116,84,101,115,116])));
        t.section(UStr::new(&[114,101,115,101,114,118,101,100,72,121,112,104,101,110,83,113,117,101,101,122,101,115,80,117,110,99,116,117,97,116,105,111,110,71,108,117,101,84,111,80,117,108,108,73,116,73,110]));
        let r = HyphenationLayoutTestSupport::hyphenation_layout_test_support_layout_with(EnglishHyphenation::english_hyphenation_en_us().unwrap(), UStr::new(&[20013,25991,65292,105,110,116,101,114,110,97,116,105,111,110,97,108,105,122,97,116,105,111,110]), 128 as f64).unwrap();
        let mut c = (r.clusters[0usize]).clone();
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_ustring() == UString::from("，") {
                c = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((c.advance) < (16 as f64), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("comma glue not compressed for the hyphen: ")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(c.advance)); __s }).as_str()))).unwrap();
    });
}

#[test]
fn syllable_split_matches_the_hyphenator_exactly() {
    testlib::run("org.tiqian.layout.HyphenationLayoutTest.syllableSplitMatchesTheHyphenatorExactly", "org.tiqian.layout.HyphenationLayoutTest.syllableSplitMatchesTheHyphenatorExactly", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[72,121,112,104,101,110,97,116,105,111,110,76,97,121,111,117,116,84,101,115,116])));
        t.section(UStr::new(&[115,121,108,108,97,98,108,101,83,112,108,105,116,77,97,116,99,104,101,115,84,104,101,72,121,112,104,101,110,97,116,111,114,69,120,97,99,116,108,121]));
        let word = UString::from("internationalization").to_ustring();
        let r = HyphenationLayoutTestSupport::hyphenation_layout_test_support_layout_with(EnglishHyphenation::english_hyphenation_en_us().unwrap(), HyphenationLayoutTestSupport::HYPHENATION_LAYOUT_TEST_SUPPORT_TEXT.to_ustring().as_ustr(), 160 as f64).unwrap();
        let mut parts: Vec<UString> = vec![];
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_ustring() != UString::from("") && HyphenationLayoutTestSupport::hyphenation_layout_test_support_is_latin(((r.clusters[usize::try_from(i).unwrap_or(0)]).clone().text).to_ustring().as_ustr()) {
                parts.push(((r.clusters[usize::try_from(i).unwrap_or(0)]).clone().text).to_ustring());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(HyphenationLayoutTestSupport::hyphenation_layout_test_support_rebuild(word.as_ustr(), &EnglishHyphenation::english_hyphenation_en_us().unwrap().hyphenate(word.as_ustr())).as_ustr(), UString::from(format!("{}", { let joined2 = parts; let mut out = String::new(); let n = joined2.len(); let mut index2 = 0usize; while index2 < n { if index2 > 0 { out.push_str("-"); } let _ = write!(out, "{}", joined2[index2]); index2 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(), None).unwrap();
    });
}
