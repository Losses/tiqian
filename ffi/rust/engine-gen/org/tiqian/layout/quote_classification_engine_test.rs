#![cfg(test)]

use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::font_decision_info::FontDecisionInfo;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::role_override_info::RoleOverrideInfo;
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
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::line_breaker::LookaheadLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_classification_engine_test_support::QuoteClassificationEngineTestSupport;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::linebreak::hyphenator::NoHyphenator;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault) -> Self {
        match value {
            QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault) -> Self {
        match value {
            QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault) -> Self {
        match value {
            QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault) -> Self {
        match value {
            QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault) -> Self {
        match value {
            QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault) -> Self {
        match value {
            QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault) -> Self {
        match value {
            QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault) -> Self {
        match value {
            QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestResolvesDigitBoundUnmatchedQuotesAsPrimesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault) -> Self {
        match value {
            QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault) -> Self {
        match value {
            QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault) -> Self {
        match value {
            QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault) -> Self {
        match value {
            QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestRecordsRoleOverridesForResolvedQuotePairsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault) -> Self {
        match value {
            QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault) -> Self {
        match value {
            QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault) -> Self {
        match value {
            QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault) -> Self {
        match value {
            QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault) -> Self {
        match value {
            QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        QuoteClassificationEngineTestQuoteRolesSurviveStyleAndSourceBoundariesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault) -> Self {
        match value {
            QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault) -> Self {
        match value {
            QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault) -> Self {
        match value {
            QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault) -> Self {
        match value {
            QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestMixedQuoteContextsReachTheFontAndPunctuationPipelineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault) -> Self {
        match value {
            QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault) -> Self {
        match value {
            QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault) -> Self {
        match value {
            QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault) -> Self {
        match value {
            QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestMixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault) -> Self {
        match value {
            QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault) -> Self {
        match value {
            QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault) -> Self {
        match value {
            QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault) -> Self {
        match value {
            QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault) -> Self {
        match value {
            QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        QuoteClassificationEngineTestMi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContextFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault) -> Self {
        match value {
            QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault) -> Self {
        match value {
            QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault) -> Self {
        match value {
            QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault) -> Self {
        match value {
            QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestLeavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestKeepsTextStartLatinQuotePairInLatinRunFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestKeepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestKeepsNumberedCjkQuotePairOnCjkFaceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestKeepsLatinTechnicalPunctuationInLatinRunFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestKeepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotationFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestKeepsDecadeStyleApostropheWithLetterFlankLatinFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault) -> Self {
        match value {
            QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestKeepsContractionApostropheLatinInsideCjkSingleQuotesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault) -> Self {
        match value {
            QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault) -> Self {
        match value {
            QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault) -> Self {
        match value {
            QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault) -> Self {
        match value {
            QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinRegardlessOfSurroundingContextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault) -> Self {
        match value {
            QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault) -> Self {
        match value {
            QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault) -> Self {
        match value {
            QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault) -> Self {
        match value {
            QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestClassifiesAsciiBracketsAsLatinInsidePureCjkContentFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault) -> Self {
        match value {
            QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault) -> Self {
        match value {
            QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault) -> Self {
        match value {
            QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault) -> Self {
        match value {
            QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault) -> Self {
        match value {
            QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        QuoteClassificationEngineTestAsciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEndFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault) -> Self {
        match value {
            QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault) -> Self {
        match value {
            QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault) -> Self {
        match value {
            QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault) -> Self {
        match value {
            QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault) -> Self {
        match value {
            QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        QuoteClassificationEngineTestAsciiClosingBracketWithCjkInteriorIsForbiddenAtLineStartFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault) -> Self {
        match value {
            QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault) -> Self {
        match value {
            QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault) -> Self {
        match value {
            QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault) -> Self {
        match value {
            QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuoteClassificationEngineTestAdjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContentFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn keeps_latin_technical_punctuation_in_latin_run() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsLatinTechnicalPunctuationInLatinRun", "org.tiqian.layout.QuoteClassificationEngineTest.keepsLatinTechnicalPunctuationInLatinRun", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(UStr::new(&[107,101,101,112,115,76,97,116,105,110,84,101,99,104,110,105,99,97,108,80,117,110,99,116,117,97,116,105,111,110,73,110,76,97,116,105,110,82,117,110]));
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(UStr::new(&[119,101,108,108,45,107,110,111,119,110,47,112,97,116,104]), 320 as f64, None).unwrap();
        let mut x = UString::new();
        let mut all = true;
        let mut any = false;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            x += &((c.text).to_ustring());
            all = all && (c.font_key).to_ustring() == UString::from("latin-primary");
            any = any || (c.text).to_ustring() == UString::from("well-");
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[119,101,108,108,45,107,110,111,119,110,47,112,97,116,104]), x.as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(all, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(any, None).unwrap();
    });
}

#[test]
fn classifies_ascii_brackets_as_latin_regardless_of_surrounding_context() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.classifiesAsciiBracketsAsLatinRegardlessOfSurroundingContext", "org.tiqian.layout.QuoteClassificationEngineTest.classifiesAsciiBracketsAsLatinRegardlessOfSurroundingContext", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(UStr::new(&[99,108,97,115,115,105,102,105,101,115,65,115,99,105,105,66,114,97,99,107,101,116,115,65,115,76,97,116,105,110,82,101,103,97,114,100,108,101,115,115,79,102,83,117,114,114,111,117,110,100,105,110,103,67,111,110,116,101,120,116]));
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(UStr::new(&[20013,25991,40,69,110,103,108,105,115,104,41,20013,25991]), 320 as f64, None).unwrap();
        let mut c: Option<Cluster> = None;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let v = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            if v.text.to_ustring() == UString::from("(English)") {
                c = Some(v.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[108,97,116,105,110,45,112,114,105,109,97,114,121]), (c.as_ref().unwrap().font_key).to_ustring().as_ustr(), None).unwrap();
        let mut d: Option<FontDecisionInfo> = None;
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let v = ((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if v.source_text.to_ustring() == UString::from("(English)") {
                d = Some(v.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[76,97,116,105,110,84,101,120,116]), (d.as_ref().unwrap().role).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn classifies_ascii_brackets_as_latin_inside_pure_cjk_content() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.classifiesAsciiBracketsAsLatinInsidePureCjkContent", "org.tiqian.layout.QuoteClassificationEngineTest.classifiesAsciiBracketsAsLatinInsidePureCjkContent", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(UStr::new(&[99,108,97,115,115,105,102,105,101,115,65,115,99,105,105,66,114,97,99,107,101,116,115,65,115,76,97,116,105,110,73,110,115,105,100,101,80,117,114,101,67,106,107,67,111,110,116,101,110,116]));
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(UStr::new(&[20013,25991,40,20013,25991,41]), 320 as f64, None).unwrap();
        let mut a: Option<Cluster> = None;
        let mut b: Option<Cluster> = None;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            if c.text.to_ustring() == UString::from("(") {
                a = Some(c.clone());
            }
            if c.text.to_ustring() == UString::from(")") {
                b = Some(c.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[108,97,116,105,110,45,112,114,105,109,97,114,121]), (a.as_ref().unwrap().font_key).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[108,97,116,105,110,45,112,114,105,109,97,114,121]), (b.as_ref().unwrap().font_key).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn ascii_closing_bracket_with_cjk_interior_is_forbidden_at_line_start() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.asciiClosingBracketWithCjkInteriorIsForbiddenAtLineStart", "org.tiqian.layout.QuoteClassificationEngineTest.asciiClosingBracketWithCjkInteriorIsForbiddenAtLineStart", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(UStr::new(&[97,115,99,105,105,67,108,111,115,105,110,103,66,114,97,99,107,101,116,87,105,116,104,67,106,107,73,110,116,101,114,105,111,114,73,115,70,111,114,98,105,100,100,101,110,65,116,76,105,110,101,83,116,97,114,116]));
        let text = UString::from("如今已占据超七成份额(国产品牌)，互联网大厂排队抢购？").to_ustring();
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap().layout(QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_input(text.as_ustr(), 232 as f64).unwrap()).unwrap();
        let mut debug_lines = UString::new();
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let l = (r.lines[usize::try_from(i).unwrap_or(0)]).clone();
            let from = (l.range).clone().start;
            let to = (l.range).clone().end;
            let s = u_string::slice(text.as_ustr(), i32::from_ne_bytes(((from) as i32).to_ne_bytes()), i32::from_ne_bytes(((to) as i32).to_ne_bytes()));
            if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                debug_lines += &(UString::from(concat!("\n",
"")));
            }
            debug_lines += &({ let mut __s = UString::new(); __s += UString::from(format!("{}", l.cluster_range.to_string()).as_str()).as_ustr(); __s += &(UString::from(" ")); __s += UString::from(format!("{}", (l.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(" ")); __s += UString::from(l.end_reason.name()).as_ustr(); __s += &(UString::from(" \"")); __s += s.as_ustr(); __s += &(UString::from("\"")); __s });
        }
        let mut ok = true;
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let l = (r.lines[usize::try_from(i).unwrap_or(0)]).clone();
            ok = ok && !QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_starts_with(text.as_ustr(), (l.range).clone(), UStr::new(&[41]));
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, Some((debug_lines).to_ustring())).unwrap();
        let mut c: Option<Cluster> = None;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let v = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            if v.text.to_ustring() == UString::from(")") {
                c = Some(v.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[108,97,116,105,110,45,112,114,105,109,97,114,121]), (c.as_ref().unwrap().font_key).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn ascii_opening_bracket_with_cjk_interior_is_forbidden_at_line_end() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.asciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEnd", "org.tiqian.layout.QuoteClassificationEngineTest.asciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEnd", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(UStr::new(&[97,115,99,105,105,79,112,101,110,105,110,103,66,114,97,99,107,101,116,87,105,116,104,67,106,107,73,110,116,101,114,105,111,114,73,115,70,111,114,98,105,100,100,101,110,65,116,76,105,110,101,69,110,100]));
        let text = UString::from("如今已占据超七成份额(国产品牌)，互联网大厂排队抢购？").to_ustring();
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap().layout(QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_input(text.as_ustr(), 168 as f64).unwrap()).unwrap();
        let mut debug_lines = UString::new();
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let l = (r.lines[usize::try_from(i).unwrap_or(0)]).clone();
            let from = (l.range).clone().start;
            let to = (l.range).clone().end;
            let s = u_string::slice(text.as_ustr(), i32::from_ne_bytes(((from) as i32).to_ne_bytes()), i32::from_ne_bytes(((to) as i32).to_ne_bytes()));
            if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                debug_lines += &(UString::from(concat!("\n",
"")));
            }
            debug_lines += &({ let mut __s = UString::new(); __s += UString::from(format!("{}", l.cluster_range.to_string()).as_str()).as_ustr(); __s += &(UString::from(" ")); __s += UString::from(format!("{}", (l.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(" ")); __s += UString::from(l.end_reason.name()).as_ustr(); __s += &(UString::from(" \"")); __s += s.as_ustr(); __s += &(UString::from("\"")); __s });
        }
        let mut ok = true;
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let l = (r.lines[usize::try_from(i).unwrap_or(0)]).clone();
            ok = ok && !QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_ends_with(text.as_ustr(), (l.range).clone(), UStr::new(&[40]));
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, Some((debug_lines).to_ustring())).unwrap();
        let mut c: Option<Cluster> = None;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let v = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            if v.text.to_ustring() == UString::from("(") {
                c = Some(v.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[108,97,116,105,110,45,112,114,105,109,97,114,121]), (c.as_ref().unwrap().font_key).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn keeps_text_start_latin_quote_pair_in_latin_run() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsTextStartLatinQuotePairInLatinRun", "org.tiqian.layout.QuoteClassificationEngineTest.keepsTextStartLatinQuotePairInLatinRun", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(UStr::new(&[107,101,101,112,115,84,101,120,116,83,116,97,114,116,76,97,116,105,110,81,117,111,116,101,80,97,105,114,73,110,76,97,116,105,110,82,117,110]));
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(UStr::new(&[8220,72,101,108,108,111,8221,32,119,111,114,108,100]), 320 as f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(3, u32::try_from((r.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[8220,72,101,108,108,111,8221]), ((r.clusters[0usize]).clone().text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[108,97,116,105,110,45,112,114,105,109,97,114,121]), ((r.clusters[0usize]).clone().font_key).to_ustring().as_ustr(), None).unwrap();
        let mut ok = false;
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            ok = ok || (d.source_text).to_ustring() == UString::from("“Hello” world") && (d.role).to_ustring() == UString::from("LatinText");
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, None).unwrap();
    });
}

#[test]
fn mixed_quote_contexts_reach_the_font_and_punctuation_pipeline() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.mixedQuoteContextsReachTheFontAndPunctuationPipeline", "org.tiqian.layout.QuoteClassificationEngineTest.mixedQuoteContextsReachTheFontAndPunctuationPipeline", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(UStr::new(&[109,105,120,101,100,81,117,111,116,101,67,111,110,116,101,120,116,115,82,101,97,99,104,84,104,101,70,111,110,116,65,110,100,80,117,110,99,116,117,97,116,105,111,110,80,105,112,101,108,105,110,101]));
        let text = UString::from("中“文”中；that’s；（如 ‘O’, ‘Q’）；他说：“She said ‘hello’.”").to_ustring();
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(text.as_ustr(), 1000 as f64, None).unwrap();
        let c = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_indices(text.as_ustr());
        let ck: SortedSetTable<u32> = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_set(&vec![1, 3, 29, 47]);
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        for i in 0..match u32::try_from((r.debug).clone().punctuation_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().punctuation_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_is_curly_quote_for_test((d.char).to_ustring().as_ustr()) {
                b.put(&((d.range).clone().start));
            }
        }
        let ps: SortedSetTable<u32> = b.clone().build();
        let mut cjk_ok = true;
        let mut latin_ok = true;
        for &i in &c {
            if ck.has(&(i)) {
                cjk_ok = cjk_ok && QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_role_at((r).clone(), i) == UString::from("CjkPunctuation");
            } else {
                latin_ok = latin_ok && QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_role_at((r).clone(), i) == UString::from("LatinText");
            }
        }
        let mut rb: SortedMapTableBuilder<u32, UString> = SortedTable::sorted_table_map_builder::<u32, UString>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        for i in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(i).unwrap_or(0)]).clone();
            rb.put(&((d.range).clone().start), &(d.overridden_role).to_ustring());
        }
        let roles: SortedMapTable<u32, UString> = rb.clone().build();
        let mut expected: SortedMapTableBuilder<u32, UString> = SortedTable::sorted_table_map_builder::<u32, UString>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        for &i in &c {
            expected.put(&(i), &if ck.has(&(i)) { UString::from("CjkPunctuation") } else { UString::from("LatinText") });
        }
        let _ = TracedAssertions::traced_assertions_assert_true(cjk_ok, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(latin_ok, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set((ck).clone(), (ps).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_render_role_map(expected.clone().build()).as_ustr(), QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_render_role_map((roles).clone()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(text.as_ustr(), (((r.input).clone().content).clone().text).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn quote_roles_survive_style_and_source_boundaries() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.quoteRolesSurviveStyleAndSourceBoundaries", "org.tiqian.layout.QuoteClassificationEngineTest.quoteRolesSurviveStyleAndSourceBoundaries", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(UStr::new(&[113,117,111,116,101,82,111,108,101,115,83,117,114,118,105,118,101,83,116,121,108,101,65,110,100,83,111,117,114,99,101,66,111,117,110,100,97,114,105,101,115]));
        let text = UString::from("中‘that’s’中").to_ustring();
        let input = LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![
    (TextSpan::new(TextRange::new(2u32, 7u32).unwrap(), TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(700), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
]), Some(vec![1, 2, 6, 7, 8, 9]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap().layout((input).clone()).unwrap();
        let mut b: SortedMapTableBuilder<u32, UString> = SortedTable::sorted_table_map_builder::<u32, UString>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        for i in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(i).unwrap_or(0)]).clone();
            b.put(&((d.range).clone().start), &(d.overridden_role).to_ustring());
        }
        let roles: SortedMapTable<u32, UString> = b.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(Some(UString::from("CjkPunctuation")), roles.get(&(1)).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(Some(UString::from("LatinText")), roles.get(&(6)).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(Some(UString::from("CjkPunctuation")), roles.get(&(8)).clone(), None).unwrap();
        let mut f: Option<UString> = None;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            if c.range.clone().start == 6 {
                f = Some((c.font_key).to_ustring());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[108,97,116,105,110,45,112,114,105,109,97,114,121]), (f).as_deref().unwrap_or(UStr::new(&[])), None).unwrap();
        let mut x = UString::new();
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            x += &(((r.clusters[usize::try_from(i).unwrap_or(0)]).clone().text).to_ustring());
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(text.as_ustr(), x.as_ustr(), None).unwrap();
    });
}

#[test]
fn adjacent_quoted_list_items_keep_cjk_quote_geometry_across_mixed_content() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.adjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContent", "org.tiqian.layout.QuoteClassificationEngineTest.adjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContent", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(UStr::new(&[97,100,106,97,99,101,110,116,81,117,111,116,101,100,76,105,115,116,73,116,101,109,115,75,101,101,112,67,106,107,81,117,111,116,101,71,101,111,109,101,116,114,121,65,99,114,111,115,115,77,105,120,101,100,67,111,110,116,101,110,116]));
        let texts = vec![
    UString::from("便延伸出了“乃子”“大波”“大灯”“大雷”“大扎”“对A”“波霸”这些词").to_ustring(),
    UString::from(concat!("这些太直白了是吧，\n",
" “欧派”“double”“double may”呢")).to_ustring(),
];
        for text in &texts {
            let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(text.as_ustr(), 1000 as f64, None).unwrap();
            let qi = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_indices(text.as_ustr());
            let mut a: Vec<u32> = vec![];
            for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let d = ((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone();
                if d.role.to_ustring() == UString::from("CjkPunctuation") && (u32::from_ne_bytes(((match qi.iter().position(|e| e == &(d.range).clone().start) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }) as u32).to_ne_bytes())) <= 2147483647 {
                    a.push((d.range).clone().start);
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&qi, &a, Some((text).to_ustring())).unwrap();
            let mut p: Vec<u32> = vec![];
            for i in 0..match u32::try_from((r.debug).clone().punctuation_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let d = ((r.debug).clone().punctuation_decisions[usize::try_from(i).unwrap_or(0)]).clone();
                if QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_is_curly_quote_for_test((d.char).to_ustring().as_ustr()) {
                    p.push((d.range).clone().start);
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&qi, &p, Some((text).to_ustring())).unwrap();
            let f = vec![
    QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_last_index(text.as_ustr(), UStr::new(&[8220])),
    QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_last_index(text.as_ustr(), UStr::new(&[8221])),
];
            let mut o: Vec<u32> = vec![];
            for i in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let d = ((r.debug).clone().role_overrides[usize::try_from(i).unwrap_or(0)]).clone();
                if u32::from_ne_bytes(((match f.iter().position(|e| e == &(d.range).clone().start) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }) as u32).to_ne_bytes()) <= 2147483647 {
                    o.push((d.range).clone().start);
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&f, &o, Some((text).to_ustring())).unwrap();
            let mut good = true;
            for i in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let d = ((r.debug).clone().role_overrides[usize::try_from(i).unwrap_or(0)]).clone();
                if u32::from_ne_bytes(((match f.iter().position(|e| e == &(d.range).clone().start) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }) as u32).to_ne_bytes()) <= 2147483647 {
                    good = good && (d.source).to_ustring() == UString::from("PairedPunctuationOuterScriptContext");
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(good, Some((text).to_ustring())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string(text.as_ustr(), (((r.input).clone().content).clone().text).to_ustring().as_ustr(), None).unwrap();
        }
    });
}

#[test]
fn mi10s_adjacent_latin_transcriptions_keep_the_final_quote_pair_in_cjk_context() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.mi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContext", "org.tiqian.layout.QuoteClassificationEngineTest.mi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContext", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(UStr::new(&[109,105,49,48,115,65,100,106,97,99,101,110,116,76,97,116,105,110,84,114,97,110,115,99,114,105,112,116,105,111,110,115,75,101,101,112,84,104,101,70,105,110,97,108,81,117,111,116,101,80,97,105,114,73,110,67,106,107,67,111,110,116,101,120,116]));
        let text = UString::from("所以这个和 “骑ji” “说shui”“斜xiá”不一样，港台是从众的，大陆读音大多数源自韵书。").to_ustring();
        let mut e = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(Box::new(NoHyphenator::new())), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap();
        let r = e.layout(QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_input(text.as_ustr(), 160 as f64).unwrap()).unwrap();
        let f = vec![
    QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_last_index(text.as_ustr(), UStr::new(&[8220])),
    QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_last_index(text.as_ustr(), UStr::new(&[8221])),
];
        let mut a: Vec<u32> = vec![];
        for i in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(i).unwrap_or(0)]).clone();
            if u32::from_ne_bytes(((match f.iter().position(|e| e == &(d.range).clone().start) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }) as u32).to_ne_bytes()) <= 2147483647 {
                a.push((d.range).clone().start);
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![19, 24], &a, None).unwrap();
        let mut roles_ok = true;
        let mut sources_ok = true;
        for i in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(i).unwrap_or(0)]).clone();
            if u32::from_ne_bytes(((match f.iter().position(|e| e == &(d.range).clone().start) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }) as u32).to_ne_bytes()) <= 2147483647 {
                roles_ok = roles_ok && (d.overridden_role).to_ustring() == UString::from("CjkPunctuation");
                sources_ok = sources_ok && (d.source).to_ustring() == UString::from("PairedPunctuationOuterScriptContext");
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(roles_ok, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(sources_ok, None).unwrap();
        let mut no = true;
        let mut lm = UString::new();
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let l = (r.lines[usize::try_from(i).unwrap_or(0)]).clone();
            let from = (l.range).clone().start;
            let to = (l.range).clone().end;
            let s = u_string::slice(text.as_ustr(), i32::from_ne_bytes(((from) as i32).to_ne_bytes()), i32::from_ne_bytes(((to) as i32).to_ne_bytes()));
            no = no && u32::from_ne_bytes(((u_string::find_from(&(s), UString::from("”").as_ustr(), 0)) as u32).to_ne_bytes()) != 0;
            if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                lm += &(UString::from(", "));
            }
            lm += &(s);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(no, Some((lm).to_ustring())).unwrap();
    });
}

#[test]
fn skips_neutral_dash_before_latin_quote_pair_in_layout() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.skipsNeutralDashBeforeLatinQuotePairInLayout", "org.tiqian.layout.QuoteClassificationEngineTest.skipsNeutralDashBeforeLatinQuotePairInLayout", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(UStr::new(&[115,107,105,112,115,78,101,117,116,114,97,108,68,97,115,104,66,101,102,111,114,101,76,97,116,105,110,81,117,111,116,101,80,97,105,114,73,110,76,97,121,111,117,116]));
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(UStr::new(&[69,110,103,108,105,115,104,32,8212,32,8220,104,101,108,108,111,8221]), 320 as f64, None).unwrap();
        let mut c: Option<Cluster> = None;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let v = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            if u32::from_ne_bytes(((u_string::find_from(&((v.text).to_ustring()), UString::from("“hello”").as_ustr(), 0)) as u32).to_ne_bytes()) <= 2147483647 {
                c = Some(v.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[108,97,116,105,110,45,112,114,105,109,97,114,121]), (c.as_ref().unwrap().font_key).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn keeps_slash_led_latin_technical_run_out_of_cjk_punctuation_geometry() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometry", "org.tiqian.layout.QuoteClassificationEngineTest.keepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometry", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(UStr::new(&[107,101,101,112,115,83,108,97,115,104,76,101,100,76,97,116,105,110,84,101,99,104,110,105,99,97,108,82,117,110,79,117,116,79,102,67,106,107,80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121]));
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(UStr::new(&[24656,36328,47,84,69,82,70,105,115,109,12290,22914,26524]), 320 as f64, None).unwrap();
        let mut d: Option<FontDecisionInfo> = None;
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = ((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if x.source_text.to_ustring() == UString::from("/TERFism") {
                d = Some(x.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[76,97,116,105,110,84,101,120,116]), (d.as_ref().unwrap().role).to_ustring().as_ustr(), None).unwrap();
        let mut none = true;
        for i in 0..match u32::try_from((r.debug).clone().punctuation_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            none = none || (((r.debug).clone().punctuation_decisions[usize::try_from(i).unwrap_or(0)]).clone().range).clone().start != (d.as_ref().unwrap().range).clone().start;
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none, None).unwrap();
        let mut c: Option<Cluster> = None;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            if x.text.to_ustring() == UString::from("/TERFism") {
                c = Some(x.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[108,97,116,105,110,45,112,114,105,109,97,114,121]), (c.as_ref().unwrap().font_key).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((c.as_ref().unwrap().advance) > (16 as f64), None).unwrap();
    });
}

#[test]
fn records_role_overrides_for_resolved_quote_pairs() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.recordsRoleOverridesForResolvedQuotePairs", "org.tiqian.layout.QuoteClassificationEngineTest.recordsRoleOverridesForResolvedQuotePairs", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(UStr::new(&[114,101,99,111,114,100,115,82,111,108,101,79,118,101,114,114,105,100,101,115,70,111,114,82,101,115,111,108,118,101,100,81,117,111,116,101,80,97,105,114,115]));
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(UStr::new(&[8220,72,101,108,108,111,8221,32,119,111,114,108,100]), 320 as f64, None).unwrap();
        let mut a: Option<RoleOverrideInfo> = None;
        let mut b: Option<RoleOverrideInfo> = None;
        for i in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(i).unwrap_or(0)]).clone();
            if d.range.clone().start == 0 {
                a = Some(d.clone());
            }
            if d.range.clone().start == 6 {
                b = Some(d.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(Some(UString::from("LatinText")), Some((a.as_ref().unwrap().overridden_role).to_ustring()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(Some(UString::from("CjkPunctuation")), Some((a.as_ref().unwrap().original_role).to_ustring()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(Some(UString::from("PairedPunctuationOuterScriptContext")), Some((a.as_ref().unwrap().source).to_ustring()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(Some(UString::from("LatinText")), Some((b.as_ref().unwrap().overridden_role).to_ustring()), None).unwrap();
    });
}

#[test]
fn mixed_chinese_question_at_paragraph_start_keeps_cjk_quote_geometry() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.mixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometry", "org.tiqian.layout.QuoteClassificationEngineTest.mixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometry", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(UStr::new(&[109,105,120,101,100,67,104,105,110,101,115,101,81,117,101,115,116,105,111,110,65,116,80,97,114,97,103,114,97,112,104,83,116,97,114,116,75,101,101,112,115,67,106,107,81,117,111,116,101,71,101,111,109,101,116,114,121]));
        let text = UString::from("“Json是谁？”").to_ustring();
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(text.as_ustr(), 320 as f64, None).unwrap();
        let q: SortedSetTable<u32> = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_set(&vec![0, 8]);
        let mut o: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        for i in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(i).unwrap_or(0)]).clone();
            if q.has(&((d.range).clone().start)) {
                o.put(&((d.range).clone().start));
            }
        }
        let os: SortedSetTable<u32> = o.clone().build();
        let mut roles_ok = true;
        let mut sources_ok = true;
        for i in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(i).unwrap_or(0)]).clone();
            if q.has(&((d.range).clone().start)) {
                roles_ok = roles_ok && (d.overridden_role).to_ustring() == UString::from("CjkPunctuation");
                sources_ok = sources_ok && (d.source).to_ustring() == UString::from("ParagraphLanguageQuoteContext");
            }
        }
        let mut p: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        for i in 0..match u32::try_from((r.debug).clone().punctuation_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().punctuation_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if d.char.to_ustring() == UString::from("“") || (d.char).to_ustring() == UString::from("”") {
                p.put(&((d.range).clone().start));
            }
        }
        let ps: SortedSetTable<u32> = p.clone().build();
        let mut x = UString::new();
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            x += &(((r.clusters[usize::try_from(i).unwrap_or(0)]).clone().text).to_ustring());
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set((q).clone(), (os).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(roles_ok, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(sources_ok, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set((q).clone(), (ps).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(text.as_ustr(), x.as_ustr(), None).unwrap();
    });
}

#[test]
fn keeps_numbered_cjk_quote_pair_on_cjk_face() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsNumberedCjkQuotePairOnCjkFace", "org.tiqian.layout.QuoteClassificationEngineTest.keepsNumberedCjkQuotePairOnCjkFace", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(UStr::new(&[107,101,101,112,115,78,117,109,98,101,114,101,100,67,106,107,81,117,111,116,101,80,97,105,114,79,110,67,106,107,70,97,99,101]));
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(UStr::new(&[49,46,8220,20320,30693,36947,26446,30333,26159,24590,20040,27515,30340,21527,65311,8221]), 320 as f64, None).unwrap();
        let mut d: Option<FontDecisionInfo> = None;
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = ((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if x.range.clone().start == 2 {
                d = Some(x.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[67,106,107,80,117,110,99,116,117,97,116,105,111,110]), (d.as_ref().unwrap().role).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[99,106,107,45,112,114,105,109,97,114,121]), (d.as_ref().unwrap().font_key).to_ustring().as_ustr(), None).unwrap();
        let mut o: Option<RoleOverrideInfo> = None;
        for i in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = ((r.debug).clone().role_overrides[usize::try_from(i).unwrap_or(0)]).clone();
            if x.range.clone().start == 2 {
                o = Some(x.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[80,97,105,114,101,100,80,117,110,99,116,117,97,116,105,111,110,67,111,110,116,101,110,116,83,99,114,105,112,116,67,111,110,116,101,120,116]), (o.as_ref().unwrap().source).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[113,117,111,116,101,100,45,99,111,110,116,101,110,116,45,115,99,114,105,112,116]), (o.as_ref().unwrap().reason).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[67,106,107,80,117,110,99,116,117,97,116,105,111,110]), (o.as_ref().unwrap().overridden_role).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn requests_full_width_cjk_quotes_and_synthesizes_the_cell_when_the_font_stays_proportional() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.requestsFullWidthCjkQuotesAndSynthesizesTheCellWhenTheFontStaysProportional", "org.tiqian.layout.QuoteClassificationEngineTest.requestsFullWidthCjkQuotesAndSynthesizesTheCellWhenTheFontStaysProportional", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_full_width_test().unwrap();
    });
}

#[test]
fn leaves_latin_context_curly_quotes_outside_cjk_punctuation_geometry() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.leavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometry", "org.tiqian.layout.QuoteClassificationEngineTest.leavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometry", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(UStr::new(&[108,101,97,118,101,115,76,97,116,105,110,67,111,110,116,101,120,116,67,117,114,108,121,81,117,111,116,101,115,79,117,116,115,105,100,101,67,106,107,80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121]));
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(UStr::new(&[8220,72,101,108,108,111,8221,32,119,111,114,108,100]), 320 as f64, None).unwrap();
        let mut ok = true;
        for i in 0..match u32::try_from((r.debug).clone().punctuation_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            ok = ok && !((((r.debug).clone().punctuation_decisions[usize::try_from(i).unwrap_or(0)]).clone().char).to_ustring() == UString::from("“") || (((r.debug).clone().punctuation_decisions[usize::try_from(i).unwrap_or(0)]).clone().char).to_ustring() == UString::from("”"));
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, None).unwrap();
        ok = true;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            ok = ok && r.clusters[usize::try_from(i).unwrap_or(0)].glyph_inline_shift == 0 as f64;
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, None).unwrap();
    });
}

#[test]
fn keeps_contraction_apostrophe_latin_inside_cjk_single_quotes() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsContractionApostropheLatinInsideCjkSingleQuotes", "org.tiqian.layout.QuoteClassificationEngineTest.keepsContractionApostropheLatinInsideCjkSingleQuotes", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(UStr::new(&[107,101,101,112,115,67,111,110,116,114,97,99,116,105,111,110,65,112,111,115,116,114,111,112,104,101,76,97,116,105,110,73,110,115,105,100,101,67,106,107,83,105,110,103,108,101,81,117,111,116,101,115]));
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(UStr::new(&[20013,8216,116,104,97,116,8217,115,8217,20013]), 320 as f64, None).unwrap();
        let mut a: Option<FontDecisionInfo> = None;
        let mut c: Option<FontDecisionInfo> = None;
        let mut b: Option<FontDecisionInfo> = None;
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if d.range.clone().start == 1 {
                a = Some(d.clone());
            }
            if d.range.clone().start == 2 {
                c = Some(d.clone());
            }
            if d.range.clone().start == 8 {
                b = Some(d.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[67,106,107,80,117,110,99,116,117,97,116,105,111,110]), (a.as_ref().unwrap().role).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[76,97,116,105,110,84,101,120,116]), (c.as_ref().unwrap().role).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[116,104,97,116,8217,115]), (c.as_ref().unwrap().source_text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[108,97,116,105,110,45,112,114,105,109,97,114,121]), (c.as_ref().unwrap().font_key).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[67,106,107,80,117,110,99,116,117,97,116,105,111,110]), (b.as_ref().unwrap().role).to_ustring().as_ustr(), None).unwrap();
        let mut cl: Option<Cluster> = None;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            if x.text.to_ustring() == UString::from("that’s") {
                cl = Some(x.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[108,97,116,105,110,45,112,114,105,109,97,114,121]), (cl.as_ref().unwrap().font_key).to_ustring().as_ustr(), None).unwrap();
        let mut none = true;
        for i in 0..match u32::try_from((r.debug).clone().punctuation_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            none = none && (((r.debug).clone().punctuation_decisions[usize::try_from(i).unwrap_or(0)]).clone().range).clone().start != 6;
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none, None).unwrap();
    });
}

#[test]
fn keeps_latin_word_internal_curly_quotes_in_latin_run_inside_mixed_paragraph() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsLatinWordInternalCurlyQuotesInLatinRunInsideMixedParagraph", "org.tiqian.layout.QuoteClassificationEngineTest.keepsLatinWordInternalCurlyQuotesInLatinRunInsideMixedParagraph", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_internal(UStr::new(&[20013,25991,32,76,97,116,105,110,58,32,108,101,8220,116,8221,116,101,114,115,32,20013,25991]), UStr::new(&[78,111,110,67,106,107,87,111,114,100,73,110,116,101,114,110,97,108,81,117,111,116,101,80,97,105,114]), UStr::new(&[76,97,116,105,110,84,101,120,116]), None).unwrap();
    });
}

#[test]
fn supports_supplementary_letters_inside_latin_word_internal_quotes() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.supportsSupplementaryLettersInsideLatinWordInternalQuotes", "org.tiqian.layout.QuoteClassificationEngineTest.supportsSupplementaryLettersInsideLatinWordInternalQuotes", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_internal(UStr::new(&[20013,25991,32,97,8220,55349,56320,8221,98,32,20013,25991]), UStr::new(&[78,111,110,67,106,107,87,111,114,100,73,110,116,101,114,110,97,108,81,117,111,116,101,80,97,105,114]), UStr::new(&[76,97,116,105,110,84,101,120,116]), None).unwrap();
    });
}

#[test]
fn keeps_letter_bounded_word_internal_quotes_latin() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsLetterBoundedWordInternalQuotesLatin", "org.tiqian.layout.QuoteClassificationEngineTest.keepsLetterBoundedWordInternalQuotesLatin", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_internal(UStr::new(&[20013,97,8220,98,8221,99,25991]), UStr::new(&[78,111,110,67,106,107,87,111,114,100,73,110,116,101,114,110,97,108,81,117,111,116,101,80,97,105,114]), UStr::new(&[76,97,116,105,110,84,101,120,116]), None).unwrap();
    });
}

#[test]
fn keeps_digit_content_inside_letter_bounded_quotes_latin() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsDigitContentInsideLetterBoundedQuotesLatin", "org.tiqian.layout.QuoteClassificationEngineTest.keepsDigitContentInsideLetterBoundedQuotesLatin", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_internal(UStr::new(&[20013,97,8220,49,8221,99,25991]), UStr::new(&[78,111,110,67,106,107,87,111,114,100,73,110,116,101,114,110,97,108,81,117,111,116,101,80,97,105,114]), UStr::new(&[76,97,116,105,110,84,101,120,116]), None).unwrap();
    });
}

#[test]
fn keeps_digit_bounded_word_internal_quotes_cjk() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsDigitBoundedWordInternalQuotesCjk", "org.tiqian.layout.QuoteClassificationEngineTest.keepsDigitBoundedWordInternalQuotesCjk", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_internal(UStr::new(&[20013,49,8220,49,8221,50,25991]), UStr::new(&[80,97,105,114,101,100,80,117,110,99,116,117,97,116,105,111,110,79,117,116,101,114,83,99,114,105,112,116,67,111,110,116,101,120,116]), UStr::new(&[67,106,107,80,117,110,99,116,117,97,116,105,111,110]), None).unwrap();
    });
}

#[test]
fn keeps_fullwidth_letter_bounded_word_internal_quotes_cjk() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsFullwidthLetterBoundedWordInternalQuotesCjk", "org.tiqian.layout.QuoteClassificationEngineTest.keepsFullwidthLetterBoundedWordInternalQuotesCjk", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_internal(UStr::new(&[20013,65313,8220,65314,8221,65315,25991]), UStr::new(&[80,97,114,97,103,114,97,112,104,76,97,110,103,117,97,103,101,81,117,111,116,101,67,111,110,116,101,120,116]), UStr::new(&[67,106,107,80,117,110,99,116,117,97,116,105,111,110]), None).unwrap();
    });
}

#[test]
fn keeps_empty_word_internal_quotes_latin() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsEmptyWordInternalQuotesLatin", "org.tiqian.layout.QuoteClassificationEngineTest.keepsEmptyWordInternalQuotesLatin", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_internal(UStr::new(&[20013,25991,97,8220,8221,98,20013,25991]), UStr::new(&[78,111,110,67,106,107,87,111,114,100,73,110,116,101,114,110,97,108,81,117,111,116,101,80,97,105,114]), UStr::new(&[76,97,116,105,110,84,101,120,116]), None).unwrap();
    });
}

#[test]
fn keeps_astral_letter_bounded_word_internal_quotes_latin() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsAstralLetterBoundedWordInternalQuotesLatin", "org.tiqian.layout.QuoteClassificationEngineTest.keepsAstralLetterBoundedWordInternalQuotesLatin", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_internal(UStr::new(&[20013,55349,56320,8220,98,8221,55349,56321,25991]), UStr::new(&[78,111,110,67,106,107,87,111,114,100,73,110,116,101,114,110,97,108,81,117,111,116,101,80,97,105,114]), UStr::new(&[76,97,116,105,110,84,101,120,116]), Some(UString::from("keepsAstralLetterBoundedWordInternalQuotesLatin"))).unwrap();
    });
}

#[test]
fn keeps_space_inside_pair_out_of_word_internal_fast_path_latin() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsSpaceInsidePairOutOfWordInternalFastPathLatin", "org.tiqian.layout.QuoteClassificationEngineTest.keepsSpaceInsidePairOutOfWordInternalFastPathLatin", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_internal(UStr::new(&[20013,97,8220,98,32,99,8221,100,25991]), UStr::new(&[80,97,114,97,103,114,97,112,104,76,97,110,103,117,97,103,101,81,117,111,116,101,67,111,110,116,101,120,116]), UStr::new(&[67,106,107,80,117,110,99,116,117,97,116,105,111,110]), Some(UString::from("keepsSpaceInsidePairOutOfWordInternalFastPathLatin"))).unwrap();
    });
}

#[test]
fn keeps_digit_bounded_single_quote_pair_cjk_via_enclosing_quotation() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotation", "org.tiqian.layout.QuoteClassificationEngineTest.keepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotation", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_arm();
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(UStr::new(&[23614,21495,26159,8220,49,8216,50,8217,51,8221,12290]), 320 as f64, None).unwrap();
        let mut n = 0u32;
        let mut all_ok = true;
        for di in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(di).unwrap_or(0)]).clone();
            if d.source_text.to_ustring() == UString::from("‘") || (d.source_text).to_ustring() == UString::from("’") {
                n = u32::wrapping_add(n, 1);
                all_ok = all_ok && (d.overridden_role).to_ustring() == UString::from("CjkPunctuation") && (d.source).to_ustring() == UString::from("PairedPunctuationEnclosingQuoteContext");
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals(2, n, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(all_ok, None).unwrap();
        let mut d_all = true;
        for di in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(di).unwrap_or(0)]).clone();
            if d.source_text.to_ustring() == UString::from("“") || (d.source_text).to_ustring() == UString::from("”") {
                d_all = d_all && (d.overridden_role).to_ustring() == UString::from("CjkPunctuation");
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(d_all, None).unwrap();
    });
}

#[test]
fn resolves_digit_bound_unmatched_quotes_as_primes() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.resolvesDigitBoundUnmatchedQuotesAsPrimes", "org.tiqian.layout.QuoteClassificationEngineTest.resolvesDigitBoundUnmatchedQuotesAsPrimes", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_arm();
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(UStr::new(&[20182,29992,26102,49,8217,51,48,8221,65292,23631,24149,26159,54,46,49,8221,30340,12290]), 320 as f64, None).unwrap();
        let mut n = 0u32;
        let mut all_ok = true;
        for di in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(di).unwrap_or(0)]).clone();
            if d.source_text.to_ustring() == UString::from("’") || (d.source_text).to_ustring() == UString::from("”") {
                n = u32::wrapping_add(n, 1);
                all_ok = all_ok && (d.overridden_role).to_ustring() == UString::from("LatinText") && (d.source).to_ustring() == UString::from("NumericPrimeUnmatchedQuote");
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals(3, n, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(all_ok, None).unwrap();
    });
}

#[test]
fn keeps_decade_style_apostrophe_with_letter_flank_latin() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsDecadeStyleApostropheWithLetterFlankLatin", "org.tiqian.layout.QuoteClassificationEngineTest.keepsDecadeStyleApostropheWithLetterFlankLatin", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_arm();
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(UStr::new(&[37027,26159,57,48,8217,115,30340,38899,20048,12290]), 320 as f64, None).unwrap();
        let mut a: Option<RoleOverrideInfo> = None;
        for di in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(di).unwrap_or(0)]).clone();
            if d.source_text.to_ustring() == UString::from("’") {
                a = Some(d.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[76,97,116,105,110,84,101,120,116]), (a.as_ref().unwrap().overridden_role).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[78,111,110,67,106,107,73,110,87,111,114,100,65,112,111,115,116,114,111,112,104,101]), (a.as_ref().unwrap().source).to_ustring().as_ustr(), None).unwrap();
    });
}
