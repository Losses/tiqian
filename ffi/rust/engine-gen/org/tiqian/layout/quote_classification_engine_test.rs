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
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum QuoteClassificationEngineTestSkipsNeutralDashBeforeLatinQuotePairInLayoutFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
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
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(&"keepsLatinTechnicalPunctuationInLatinRun");
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(&"well-known/path", 320 as f64, None).unwrap();
        let mut x = String::new();
        let mut all = true;
        let mut any = false;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            x += &((c.text).to_string());
            all = all && (c.font_key).to_string() == "latin-primary";
            any = any || (c.text).to_string() == "well-";
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"well-known/path", x.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(all, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(any, None).unwrap();
    });
}

#[test]
fn classifies_ascii_brackets_as_latin_regardless_of_surrounding_context() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.classifiesAsciiBracketsAsLatinRegardlessOfSurroundingContext", "org.tiqian.layout.QuoteClassificationEngineTest.classifiesAsciiBracketsAsLatinRegardlessOfSurroundingContext", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(&"classifiesAsciiBracketsAsLatinRegardlessOfSurroundingContext");
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(&"中文(English)中文", 320 as f64, None).unwrap();
        let mut c: Option<Cluster> = None;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let v = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            if v.text.to_string() == "(English)" {
                c = Some(v.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"latin-primary", (c.as_ref().unwrap().font_key).to_string().as_str(), None).unwrap();
        let mut d: Option<FontDecisionInfo> = None;
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let v = ((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if v.source_text.to_string() == "(English)" {
                d = Some(v.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"LatinText", (d.as_ref().unwrap().role).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn classifies_ascii_brackets_as_latin_inside_pure_cjk_content() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.classifiesAsciiBracketsAsLatinInsidePureCjkContent", "org.tiqian.layout.QuoteClassificationEngineTest.classifiesAsciiBracketsAsLatinInsidePureCjkContent", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(&"classifiesAsciiBracketsAsLatinInsidePureCjkContent");
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(&"中文(中文)", 320 as f64, None).unwrap();
        let mut a: Option<Cluster> = None;
        let mut b: Option<Cluster> = None;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            if c.text.to_string() == "(" {
                a = Some(c.clone());
            }
            if c.text.to_string() == ")" {
                b = Some(c.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"latin-primary", (a.as_ref().unwrap().font_key).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"latin-primary", (b.as_ref().unwrap().font_key).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn ascii_closing_bracket_with_cjk_interior_is_forbidden_at_line_start() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.asciiClosingBracketWithCjkInteriorIsForbiddenAtLineStart", "org.tiqian.layout.QuoteClassificationEngineTest.asciiClosingBracketWithCjkInteriorIsForbiddenAtLineStart", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(&"asciiClosingBracketWithCjkInteriorIsForbiddenAtLineStart");
        let text = "如今已占据超七成份额(国产品牌)，互联网大厂排队抢购？".to_string();
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)))), Some(Justifier::new(Some(0.5), Some(0.25))),
Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_input(text.as_str(), 232 as f64).unwrap()).unwrap();
        let mut debug_lines = String::new();
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let l = (r.lines[usize::try_from(i).unwrap_or(0)]).clone();
            let from = (l.range).clone().start;
            let to = (l.range).clone().end;
            let s = u_string::slice(text.as_str(), i32::from_ne_bytes((from).to_ne_bytes()), i32::from_ne_bytes((to).to_ne_bytes()));
            if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                debug_lines += &(concat!("\n",
""));
            }
            debug_lines += &(format!("{}{}{}{}{}{}{}{}",
            l.cluster_range.to_string(),
            " ",
            (l.range).clone().to_string(),
            " ",
            l.end_reason.name(),
            " \"",
            s,
            "\""
        ));
        }
        let mut ok = true;
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let l = (r.lines[usize::try_from(i).unwrap_or(0)]).clone();
            ok = ok && !QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_starts_with(text.as_str(), (l.range).clone(), &")");
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, Some((debug_lines).to_string())).unwrap();
        let mut c: Option<Cluster> = None;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let v = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            if v.text.to_string() == ")" {
                c = Some(v.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"latin-primary", (c.as_ref().unwrap().font_key).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn ascii_opening_bracket_with_cjk_interior_is_forbidden_at_line_end() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.asciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEnd", "org.tiqian.layout.QuoteClassificationEngineTest.asciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEnd", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(&"asciiOpeningBracketWithCjkInteriorIsForbiddenAtLineEnd");
        let text = "如今已占据超七成份额(国产品牌)，互联网大厂排队抢购？".to_string();
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)))), Some(Justifier::new(Some(0.5), Some(0.25))),
Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_input(text.as_str(), 168 as f64).unwrap()).unwrap();
        let mut debug_lines = String::new();
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let l = (r.lines[usize::try_from(i).unwrap_or(0)]).clone();
            let from = (l.range).clone().start;
            let to = (l.range).clone().end;
            let s = u_string::slice(text.as_str(), i32::from_ne_bytes((from).to_ne_bytes()), i32::from_ne_bytes((to).to_ne_bytes()));
            if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                debug_lines += &(concat!("\n",
""));
            }
            debug_lines += &(format!("{}{}{}{}{}{}{}{}",
            l.cluster_range.to_string(),
            " ",
            (l.range).clone().to_string(),
            " ",
            l.end_reason.name(),
            " \"",
            s,
            "\""
        ));
        }
        let mut ok = true;
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let l = (r.lines[usize::try_from(i).unwrap_or(0)]).clone();
            ok = ok && !QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_ends_with(text.as_str(), (l.range).clone(), &"(");
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, Some((debug_lines).to_string())).unwrap();
        let mut c: Option<Cluster> = None;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let v = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            if v.text.to_string() == "(" {
                c = Some(v.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"latin-primary", (c.as_ref().unwrap().font_key).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn keeps_text_start_latin_quote_pair_in_latin_run() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsTextStartLatinQuotePairInLatinRun", "org.tiqian.layout.QuoteClassificationEngineTest.keepsTextStartLatinQuotePairInLatinRun", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(&"keepsTextStartLatinQuotePairInLatinRun");
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(&"“Hello” world", 320 as f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(3, u32::try_from((r.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"“Hello”", ((r.clusters[0usize]).clone().text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"latin-primary", ((r.clusters[0usize]).clone().font_key).to_string().as_str(), None).unwrap();
        let mut ok = false;
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            ok = ok || (d.source_text).to_string() == "“Hello” world" && (d.role).to_string() == "LatinText";
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, None).unwrap();
    });
}

#[test]
fn mixed_quote_contexts_reach_the_font_and_punctuation_pipeline() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.mixedQuoteContextsReachTheFontAndPunctuationPipeline", "org.tiqian.layout.QuoteClassificationEngineTest.mixedQuoteContextsReachTheFontAndPunctuationPipeline", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(&"mixedQuoteContextsReachTheFontAndPunctuationPipeline");
        let text = "中“文”中；that’s；（如 ‘O’, ‘Q’）；他说：“She said ‘hello’.”".to_string();
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(text.as_str(), 1000 as f64, None).unwrap();
        let c = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_indices(text.as_str());
        let ck: SortedSetTable<u32> = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_set(&vec![1, 3, 29, 47]);
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..match u32::try_from((r.debug).clone().punctuation_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().punctuation_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_is_curly_quote_for_test((d.char).to_string().as_str()) {
                b.put(&((d.range).clone().start));
            }
        }
        let ps: SortedSetTable<u32> = b.clone().build();
        let mut cjk_ok = true;
        let mut latin_ok = true;
        for &i in &c {
            if ck.has(&(i)) {
                cjk_ok = cjk_ok && QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_role_at((r).clone(), i) == "CjkPunctuation";
            } else {
                latin_ok = latin_ok && QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_role_at((r).clone(), i) == "LatinText";
            }
        }
        let mut rb: SortedMapTableBuilder<u32, String> = SortedTable::sorted_table_map_builder::<u32, String>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(i).unwrap_or(0)]).clone();
            rb.put(&((d.range).clone().start), &(d.overridden_role).to_string());
        }
        let roles: SortedMapTable<u32, String> = rb.clone().build();
        let mut expected: SortedMapTableBuilder<u32, String> = SortedTable::sorted_table_map_builder::<u32, String>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for &i in &c {
            expected.put(&(i), &if ck.has(&(i)) { "CjkPunctuation".to_string() } else { "LatinText".to_string() });
        }
        let _ = TracedAssertions::traced_assertions_assert_true(cjk_ok, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(latin_ok, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set((ck).clone(), (ps).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_render_role_map(expected.clone().build()).as_str(),
QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_render_role_map((roles).clone()).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(text.as_str(), (((r.input).clone().content).clone().text).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn quote_roles_survive_style_and_source_boundaries() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.quoteRolesSurviveStyleAndSourceBoundaries", "org.tiqian.layout.QuoteClassificationEngineTest.quoteRolesSurviveStyleAndSourceBoundaries", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(&"quoteRolesSurviveStyleAndSourceBoundaries");
        let text = "中‘that’s’中".to_string();
        let input = LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![
    (TextSpan::new(TextRange::new(2u32, 7u32).unwrap(), TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(700), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
]), Some(vec![1, 2, 6, 7, 8, 9]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start),
Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        let r = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout((input).clone()).unwrap();
        let mut b: SortedMapTableBuilder<u32, String> = SortedTable::sorted_table_map_builder::<u32, String>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(i).unwrap_or(0)]).clone();
            b.put(&((d.range).clone().start), &(d.overridden_role).to_string());
        }
        let roles: SortedMapTable<u32, String> = b.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(Some("CjkPunctuation".to_string()), roles.get(&(1)).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(Some("LatinText".to_string()), roles.get(&(6)).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(Some("CjkPunctuation".to_string()), roles.get(&(8)).clone(), None).unwrap();
        let mut f: Option<String> = None;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            if c.range.clone().start == 6 {
                f = Some((c.font_key).to_string());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"latin-primary", (f).as_deref().unwrap_or(""), None).unwrap();
        let mut x = String::new();
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            x += &(((r.clusters[usize::try_from(i).unwrap_or(0)]).clone().text).to_string());
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(text.as_str(), x.as_str(), None).unwrap();
    });
}

#[test]
fn adjacent_quoted_list_items_keep_cjk_quote_geometry_across_mixed_content() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.adjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContent", "org.tiqian.layout.QuoteClassificationEngineTest.adjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContent", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(&"adjacentQuotedListItemsKeepCjkQuoteGeometryAcrossMixedContent");
        let texts = vec![
    "便延伸出了“乃子”“大波”“大灯”“大雷”“大扎”“对A”“波霸”这些词".to_string(),
    concat!("这些太直白了是吧，\n",
" “欧派”“double”“double may”呢").to_string(),
];
        for text in &texts {
            let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(text.as_str(), 1000 as f64, None).unwrap();
            let qi = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_indices(text.as_str());
            let mut a: Vec<u32> = vec![];
            for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let d = ((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone();
                if d.role.to_string() == "CjkPunctuation" && (u32::from_ne_bytes((match qi.iter().position(|e| e == &(d.range).clone().start) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }).to_ne_bytes())) <= 2147483647 {
                    a.push((d.range).clone().start);
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&qi, &a, Some((text).to_string())).unwrap();
            let mut p: Vec<u32> = vec![];
            for i in 0..match u32::try_from((r.debug).clone().punctuation_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let d = ((r.debug).clone().punctuation_decisions[usize::try_from(i).unwrap_or(0)]).clone();
                if QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_is_curly_quote_for_test((d.char).to_string().as_str()) {
                    p.push((d.range).clone().start);
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&qi, &p, Some((text).to_string())).unwrap();
            let f = vec![
    QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_last_index(text.as_str(), &"“"),
    QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_last_index(text.as_str(), &"”"),
];
            let mut o: Vec<u32> = vec![];
            for i in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let d = ((r.debug).clone().role_overrides[usize::try_from(i).unwrap_or(0)]).clone();
                if u32::from_ne_bytes((match f.iter().position(|e| e == &(d.range).clone().start) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }).to_ne_bytes()) <= 2147483647 {
                    o.push((d.range).clone().start);
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&f, &o, Some((text).to_string())).unwrap();
            let mut good = true;
            for i in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let d = ((r.debug).clone().role_overrides[usize::try_from(i).unwrap_or(0)]).clone();
                if u32::from_ne_bytes((match f.iter().position(|e| e == &(d.range).clone().start) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }).to_ne_bytes()) <= 2147483647 {
                    good = good && (d.source).to_string() == "PairedPunctuationOuterScriptContext";
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(good, Some((text).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_string(text.as_str(), (((r.input).clone().content).clone().text).to_string().as_str(), None).unwrap();
        }
    });
}

#[test]
fn mi10s_adjacent_latin_transcriptions_keep_the_final_quote_pair_in_cjk_context() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.mi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContext", "org.tiqian.layout.QuoteClassificationEngineTest.mi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContext", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(&"mi10sAdjacentLatinTranscriptionsKeepTheFinalQuotePairInCjkContext");
        let text = "所以这个和 “骑ji” “说shui”“斜xiá”不一样，港台是从众的，大陆读音大多数源自韵书。".to_string();
        let mut e = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)))), Some(Justifier::new(Some(0.5), Some(0.25))),
Some(Box::new(ExplainableStubTextShaper::new())), Some(Box::new(NoHyphenator::new())), Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap();
        let r = e.layout(QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_input(text.as_str(), 160 as f64).unwrap()).unwrap();
        let f = vec![
    QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_last_index(text.as_str(), &"“"),
    QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_last_index(text.as_str(), &"”"),
];
        let mut a: Vec<u32> = vec![];
        for i in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(i).unwrap_or(0)]).clone();
            if u32::from_ne_bytes((match f.iter().position(|e| e == &(d.range).clone().start) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }).to_ne_bytes()) <= 2147483647 {
                a.push((d.range).clone().start);
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![19, 24], &a, None).unwrap();
        let mut roles_ok = true;
        let mut sources_ok = true;
        for i in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(i).unwrap_or(0)]).clone();
            if u32::from_ne_bytes((match f.iter().position(|e| e == &(d.range).clone().start) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }).to_ne_bytes()) <= 2147483647 {
                roles_ok = roles_ok && (d.overridden_role).to_string() == "CjkPunctuation";
                sources_ok = sources_ok && (d.source).to_string() == "PairedPunctuationOuterScriptContext";
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(roles_ok, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(sources_ok, None).unwrap();
        let mut no = true;
        let mut lm = String::new();
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let l = (r.lines[usize::try_from(i).unwrap_or(0)]).clone();
            let from = (l.range).clone().start;
            let to = (l.range).clone().end;
            let s = u_string::slice(text.as_str(), i32::from_ne_bytes((from).to_ne_bytes()), i32::from_ne_bytes((to).to_ne_bytes()));
            no = no && u32::from_ne_bytes((u_string::find_from(&s, "”", 0)).to_ne_bytes()) != 0;
            if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                lm += &(", ");
            }
            lm += &(s);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(no, Some((lm).to_string())).unwrap();
    });
}

#[test]
fn skips_neutral_dash_before_latin_quote_pair_in_layout() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.skipsNeutralDashBeforeLatinQuotePairInLayout", "org.tiqian.layout.QuoteClassificationEngineTest.skipsNeutralDashBeforeLatinQuotePairInLayout", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(&"skipsNeutralDashBeforeLatinQuotePairInLayout");
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(&"English — “hello”", 320 as f64, None).unwrap();
        let mut c: Option<Cluster> = None;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let v = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            if u32::from_ne_bytes((u_string::find_from(&(v.text).to_string(), "“hello”", 0)).to_ne_bytes()) <= 2147483647 {
                c = Some(v.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"latin-primary", (c.as_ref().unwrap().font_key).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn keeps_slash_led_latin_technical_run_out_of_cjk_punctuation_geometry() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometry", "org.tiqian.layout.QuoteClassificationEngineTest.keepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometry", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(&"keepsSlashLedLatinTechnicalRunOutOfCjkPunctuationGeometry");
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(&"恐跨/TERFism。如果", 320 as f64, None).unwrap();
        let mut d: Option<FontDecisionInfo> = None;
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = ((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if x.source_text.to_string() == "/TERFism" {
                d = Some(x.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"LatinText", (d.as_ref().unwrap().role).to_string().as_str(), None).unwrap();
        let mut none = true;
        for i in 0..match u32::try_from((r.debug).clone().punctuation_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            none = none || (((r.debug).clone().punctuation_decisions[usize::try_from(i).unwrap_or(0)]).clone().range).clone().start != (d.as_ref().unwrap().range).clone().start;
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none, None).unwrap();
        let mut c: Option<Cluster> = None;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            if x.text.to_string() == "/TERFism" {
                c = Some(x.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"latin-primary", (c.as_ref().unwrap().font_key).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((c.as_ref().unwrap().advance) > (16 as f64), None).unwrap();
    });
}

#[test]
fn records_role_overrides_for_resolved_quote_pairs() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.recordsRoleOverridesForResolvedQuotePairs", "org.tiqian.layout.QuoteClassificationEngineTest.recordsRoleOverridesForResolvedQuotePairs", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(&"recordsRoleOverridesForResolvedQuotePairs");
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(&"“Hello” world", 320 as f64, None).unwrap();
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
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(Some("LatinText".to_string()), Some((a.as_ref().unwrap().overridden_role).to_string()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(Some("CjkPunctuation".to_string()), Some((a.as_ref().unwrap().original_role).to_string()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(Some("PairedPunctuationOuterScriptContext".to_string()), Some((a.as_ref().unwrap().source).to_string()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(Some("LatinText".to_string()), Some((b.as_ref().unwrap().overridden_role).to_string()), None).unwrap();
    });
}

#[test]
fn mixed_chinese_question_at_paragraph_start_keeps_cjk_quote_geometry() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.mixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometry", "org.tiqian.layout.QuoteClassificationEngineTest.mixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometry", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(&"mixedChineseQuestionAtParagraphStartKeepsCjkQuoteGeometry");
        let text = "“Json是谁？”".to_string();
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(text.as_str(), 320 as f64, None).unwrap();
        let q: SortedSetTable<u32> = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_set(&vec![0, 8]);
        let mut o: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
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
                roles_ok = roles_ok && (d.overridden_role).to_string() == "CjkPunctuation";
                sources_ok = sources_ok && (d.source).to_string() == "ParagraphLanguageQuoteContext";
            }
        }
        let mut p: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..match u32::try_from((r.debug).clone().punctuation_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().punctuation_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if d.char.to_string() == "“" || (d.char).to_string() == "”" {
                p.put(&((d.range).clone().start));
            }
        }
        let ps: SortedSetTable<u32> = p.clone().build();
        let mut x = String::new();
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            x += &(((r.clusters[usize::try_from(i).unwrap_or(0)]).clone().text).to_string());
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set((q).clone(), (os).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(roles_ok, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(sources_ok, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set((q).clone(), (ps).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(text.as_str(), x.as_str(), None).unwrap();
    });
}

#[test]
fn keeps_numbered_cjk_quote_pair_on_cjk_face() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsNumberedCjkQuotePairOnCjkFace", "org.tiqian.layout.QuoteClassificationEngineTest.keepsNumberedCjkQuotePairOnCjkFace", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(&"keepsNumberedCjkQuotePairOnCjkFace");
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(&"1.“你知道李白是怎么死的吗？”", 320 as f64, None).unwrap();
        let mut d: Option<FontDecisionInfo> = None;
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = ((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if x.range.clone().start == 2 {
                d = Some(x.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"CjkPunctuation", (d.as_ref().unwrap().role).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"cjk-primary", (d.as_ref().unwrap().font_key).to_string().as_str(), None).unwrap();
        let mut o: Option<RoleOverrideInfo> = None;
        for i in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = ((r.debug).clone().role_overrides[usize::try_from(i).unwrap_or(0)]).clone();
            if x.range.clone().start == 2 {
                o = Some(x.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"PairedPunctuationContentScriptContext", (o.as_ref().unwrap().source).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"quoted-content-script", (o.as_ref().unwrap().reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"CjkPunctuation", (o.as_ref().unwrap().overridden_role).to_string().as_str(), None).unwrap();
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
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(&"leavesLatinContextCurlyQuotesOutsideCjkPunctuationGeometry");
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(&"“Hello” world", 320 as f64, None).unwrap();
        let mut ok = true;
        for i in 0..match u32::try_from((r.debug).clone().punctuation_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            ok = ok && !((((r.debug).clone().punctuation_decisions[usize::try_from(i).unwrap_or(0)]).clone().char).to_string() == "“" || (((r.debug).clone().punctuation_decisions[usize::try_from(i).unwrap_or(0)]).clone().char).to_string() == "”");
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
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_begin(&"keepsContractionApostropheLatinInsideCjkSingleQuotes");
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(&"中‘that’s’中", 320 as f64, None).unwrap();
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
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"CjkPunctuation", (a.as_ref().unwrap().role).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"LatinText", (c.as_ref().unwrap().role).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"that’s", (c.as_ref().unwrap().source_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"latin-primary", (c.as_ref().unwrap().font_key).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"CjkPunctuation", (b.as_ref().unwrap().role).to_string().as_str(), None).unwrap();
        let mut cl: Option<Cluster> = None;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
            if x.text.to_string() == "that’s" {
                cl = Some(x.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"latin-primary", (cl.as_ref().unwrap().font_key).to_string().as_str(), None).unwrap();
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
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_internal(&"中文 Latin: le“t”ters 中文", &"NonCjkWordInternalQuotePair", &"LatinText", None).unwrap();
    });
}

#[test]
fn supports_supplementary_letters_inside_latin_word_internal_quotes() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.supportsSupplementaryLettersInsideLatinWordInternalQuotes", "org.tiqian.layout.QuoteClassificationEngineTest.supportsSupplementaryLettersInsideLatinWordInternalQuotes", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_internal(&"中文 a“𝐀”b 中文", &"NonCjkWordInternalQuotePair", &"LatinText", None).unwrap();
    });
}

#[test]
fn keeps_letter_bounded_word_internal_quotes_latin() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsLetterBoundedWordInternalQuotesLatin", "org.tiqian.layout.QuoteClassificationEngineTest.keepsLetterBoundedWordInternalQuotesLatin", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_internal(&"中a“b”c文", &"NonCjkWordInternalQuotePair", &"LatinText", None).unwrap();
    });
}

#[test]
fn keeps_digit_content_inside_letter_bounded_quotes_latin() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsDigitContentInsideLetterBoundedQuotesLatin", "org.tiqian.layout.QuoteClassificationEngineTest.keepsDigitContentInsideLetterBoundedQuotesLatin", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_internal(&"中a“1”c文", &"NonCjkWordInternalQuotePair", &"LatinText", None).unwrap();
    });
}

#[test]
fn keeps_digit_bounded_word_internal_quotes_cjk() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsDigitBoundedWordInternalQuotesCjk", "org.tiqian.layout.QuoteClassificationEngineTest.keepsDigitBoundedWordInternalQuotesCjk", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_internal(&"中1“1”2文", &"PairedPunctuationOuterScriptContext", &"CjkPunctuation", None).unwrap();
    });
}

#[test]
fn keeps_fullwidth_letter_bounded_word_internal_quotes_cjk() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsFullwidthLetterBoundedWordInternalQuotesCjk", "org.tiqian.layout.QuoteClassificationEngineTest.keepsFullwidthLetterBoundedWordInternalQuotesCjk", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_internal(&"中Ａ“Ｂ”Ｃ文", &"ParagraphLanguageQuoteContext", &"CjkPunctuation", None).unwrap();
    });
}

#[test]
fn keeps_empty_word_internal_quotes_latin() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsEmptyWordInternalQuotesLatin", "org.tiqian.layout.QuoteClassificationEngineTest.keepsEmptyWordInternalQuotesLatin", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_internal(&"中文a“”b中文", &"NonCjkWordInternalQuotePair", &"LatinText", None).unwrap();
    });
}

#[test]
fn keeps_astral_letter_bounded_word_internal_quotes_latin() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsAstralLetterBoundedWordInternalQuotesLatin", "org.tiqian.layout.QuoteClassificationEngineTest.keepsAstralLetterBoundedWordInternalQuotesLatin", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_internal(&"中𝐀“b”𝐁文", &"NonCjkWordInternalQuotePair", &"LatinText", Some("keepsAstralLetterBoundedWordInternalQuotesLatin".to_string())).unwrap();
    });
}

#[test]
fn keeps_space_inside_pair_out_of_word_internal_fast_path_latin() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsSpaceInsidePairOutOfWordInternalFastPathLatin", "org.tiqian.layout.QuoteClassificationEngineTest.keepsSpaceInsidePairOutOfWordInternalFastPathLatin", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_internal(&"中a“b c”d文", &"ParagraphLanguageQuoteContext", &"CjkPunctuation", Some("keepsSpaceInsidePairOutOfWordInternalFastPathLatin".to_string())).unwrap();
    });
}

#[test]
fn keeps_digit_bounded_single_quote_pair_cjk_via_enclosing_quotation() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.keepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotation", "org.tiqian.layout.QuoteClassificationEngineTest.keepsDigitBoundedSingleQuotePairCjkViaEnclosingQuotation", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_arm();
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(&"尾号是“1‘2’3”。", 320 as f64, None).unwrap();
        let mut n = 0u32;
        let mut all_ok = true;
        for di in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(di).unwrap_or(0)]).clone();
            if d.source_text.to_string() == "‘" || (d.source_text).to_string() == "’" {
                n = u32::wrapping_add(n, 1);
                all_ok = all_ok && (d.overridden_role).to_string() == "CjkPunctuation" && (d.source).to_string() == "PairedPunctuationEnclosingQuoteContext";
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals(2, n, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(all_ok, None).unwrap();
        let mut d_all = true;
        for di in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(di).unwrap_or(0)]).clone();
            if d.source_text.to_string() == "“" || (d.source_text).to_string() == "”" {
                d_all = d_all && (d.overridden_role).to_string() == "CjkPunctuation";
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(d_all, None).unwrap();
    });
}

#[test]
fn resolves_digit_bound_unmatched_quotes_as_primes() {
    testlib::run("org.tiqian.layout.QuoteClassificationEngineTest.resolvesDigitBoundUnmatchedQuotesAsPrimes", "org.tiqian.layout.QuoteClassificationEngineTest.resolvesDigitBoundUnmatchedQuotesAsPrimes", || {
        let _ = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_arm();
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(&"他用时1’30”，屏幕是6.1”的。", 320 as f64, None).unwrap();
        let mut n = 0u32;
        let mut all_ok = true;
        for di in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(di).unwrap_or(0)]).clone();
            if d.source_text.to_string() == "’" || (d.source_text).to_string() == "”" {
                n = u32::wrapping_add(n, 1);
                all_ok = all_ok && (d.overridden_role).to_string() == "LatinText" && (d.source).to_string() == "NumericPrimeUnmatchedQuote";
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
        let r = QuoteClassificationEngineTestSupport::quote_classification_engine_test_support_layout(&"那是90’s的音乐。", 320 as f64, None).unwrap();
        let mut a: Option<RoleOverrideInfo> = None;
        for di in 0..match u32::try_from((r.debug).clone().role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().role_overrides[usize::try_from(di).unwrap_or(0)]).clone();
            if d.source_text.to_string() == "’" {
                a = Some(d.clone());
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"LatinText", (a.as_ref().unwrap().overridden_role).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"NonCjkInWordApostrophe", (a.as_ref().unwrap().source).to_string().as_str(), None).unwrap();
    });
}
