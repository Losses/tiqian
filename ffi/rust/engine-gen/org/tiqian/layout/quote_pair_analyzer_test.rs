#![cfg(test)]

use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::font_role_context::FontRoleContext;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePair;
use crate::org::tiqian::layout::quote_pair_analyzer::QuoteRoleDecision;
use crate::org::tiqian::layout::quote_pair_analyzer::QuoteType;
use crate::org::tiqian::layout::quote_pair_analyzer_test_support::QuotePairAnalyzerTestSupport;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string;


#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault) -> Self {
        match value {
            QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault) -> Self {
        match value {
            QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault) -> Self {
        match value {
            QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault) -> Self {
        match value {
            QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault) -> Self {
        match value {
            QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault) -> Self {
        match value {
            QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault) -> Self {
        match value {
            QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault) -> Self {
        match value {
            QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault) -> Self {
        match value {
            QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault) -> Self {
        match value {
            QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault) -> Self {
        match value {
            QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault) -> Self {
        match value {
            QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault) -> Self {
        match value {
            QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault) -> Self {
        match value {
            QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault) -> Self {
        match value {
            QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault) -> Self {
        match value {
            QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault) -> Self {
        match value {
            QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault) -> Self {
        match value {
            QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerTestMatchesSingleQuotePairFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<QuotePairAnalyzerTestMatchesSingleQuotePairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerTestMatchesSingleQuotePairFault) -> Self {
        match value {
            QuotePairAnalyzerTestMatchesSingleQuotePairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestMatchesSingleQuotePairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerTestMatchesSingleQuotePairFault) -> Self {
        match value {
            QuotePairAnalyzerTestMatchesSingleQuotePairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestMatchesSingleQuotePairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerTestMatchesSingleQuotePairFault) -> Self {
        match value {
            QuotePairAnalyzerTestMatchesSingleQuotePairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerTestMatchesSingleQuotePairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerTestMatchesSingleQuotePairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerTestMatchesSingleQuotePairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerTestMatchesSingleQuotePairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerTestMatchesSingleQuotePairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerTestMatchesSingleQuotePairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerTestMatchesNestedQuotePairsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<QuotePairAnalyzerTestMatchesNestedQuotePairsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerTestMatchesNestedQuotePairsFault) -> Self {
        match value {
            QuotePairAnalyzerTestMatchesNestedQuotePairsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestMatchesNestedQuotePairsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerTestMatchesNestedQuotePairsFault) -> Self {
        match value {
            QuotePairAnalyzerTestMatchesNestedQuotePairsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestMatchesNestedQuotePairsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerTestMatchesNestedQuotePairsFault) -> Self {
        match value {
            QuotePairAnalyzerTestMatchesNestedQuotePairsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerTestMatchesNestedQuotePairsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerTestMatchesNestedQuotePairsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerTestMatchesNestedQuotePairsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerTestMatchesNestedQuotePairsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerTestMatchesNestedQuotePairsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerTestMatchesNestedQuotePairsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerTestMatchesDoubleQuotePairFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<QuotePairAnalyzerTestMatchesDoubleQuotePairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerTestMatchesDoubleQuotePairFault) -> Self {
        match value {
            QuotePairAnalyzerTestMatchesDoubleQuotePairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestMatchesDoubleQuotePairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerTestMatchesDoubleQuotePairFault) -> Self {
        match value {
            QuotePairAnalyzerTestMatchesDoubleQuotePairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestMatchesDoubleQuotePairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerTestMatchesDoubleQuotePairFault) -> Self {
        match value {
            QuotePairAnalyzerTestMatchesDoubleQuotePairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerTestMatchesDoubleQuotePairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerTestMatchesDoubleQuotePairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerTestMatchesDoubleQuotePairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerTestMatchesDoubleQuotePairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerTestMatchesDoubleQuotePairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerTestMatchesDoubleQuotePairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault) -> Self {
        match value {
            QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault) -> Self {
        match value {
            QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault) -> Self {
        match value {
            QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault) -> Self {
        match value {
            QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault) -> Self {
        match value {
            QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault) -> Self {
        match value {
            QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault) -> Self {
        match value {
            QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault) -> Self {
        match value {
            QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault) -> Self {
        match value {
            QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault) -> Self {
        match value {
            QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault) -> Self {
        match value {
            QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault) -> Self {
        match value {
            QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault) -> Self {
        match value {
            QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault) -> Self {
        match value {
            QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault) -> Self {
        match value {
            QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerTestAdjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContextFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
}

impl From<QuotePairAnalyzerTestAdjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerTestAdjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContextFault) -> Self {
        match value {
            QuotePairAnalyzerTestAdjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerTestAdjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerTestAdjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContextFault) -> Self {
        match value {
            QuotePairAnalyzerTestAdjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerTestAdjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerTestAdjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerTestAdjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerTestAdjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContextFault::TextRangeErrorFault(value)
    }
}

#[test]
fn matches_double_quote_pair() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.matchesDoubleQuotePair", "org.tiqian.layout.QuotePairAnalyzerTest.matchesDoubleQuotePair", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(&"matchesDoubleQuotePair");
        let p = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(&"他说“你好”").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((p.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_quote_pair(QuotePair::new(2u32, 5u32, QuoteType::Double), (p[0usize]).clone(), None).unwrap();
    });
}

#[test]
fn matches_single_quote_pair() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.matchesSingleQuotePair", "org.tiqian.layout.QuotePairAnalyzerTest.matchesSingleQuotePair", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(&"matchesSingleQuotePair");
        let p = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(&"他说‘你好’").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((p.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_quote_pair(QuotePair::new(2u32, 5u32, QuoteType::Single), (p[0usize]).clone(), None).unwrap();
    });
}

#[test]
fn matches_nested_quote_pairs() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.matchesNestedQuotePairs", "org.tiqian.layout.QuotePairAnalyzerTest.matchesNestedQuotePairs", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(&"matchesNestedQuotePairs");
        let p = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(&"他说：“她说‘你好’。”").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((p.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mut has6 = false;
        let mut has3 = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((p.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if p[usize::try_from(i).unwrap_or(0)].open_index == 6 {
                has6 = true;
            }
            if p[usize::try_from(i).unwrap_or(0)].open_index == 3 {
                has3 = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has6, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(has3, None).unwrap();
    });
}

#[test]
fn unmatched_quotes_produce_no_pairs() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.unmatchedQuotesProduceNoPairs", "org.tiqian.layout.QuotePairAnalyzerTest.unmatchedQuotesProduceNoPairs", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(&"unmatchedQuotesProduceNoPairs");
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(&"it’s").unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn contraction_apostrophe_does_not_close_outer_single_quote() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.contractionApostropheDoesNotCloseOuterSingleQuote", "org.tiqian.layout.QuotePairAnalyzerTest.contractionApostropheDoesNotCloseOuterSingleQuote", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(&"contractionApostropheDoesNotCloseOuterSingleQuote");
        let t = "‘that’s’".to_string();
        let _ = TracedAssertions::traced_assertions_assert_equals_quote_pair_array(&vec![(QuotePair::new(0u32, 7u32, QuoteType::Single)).clone()], &QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(t.as_str()).unwrap(), None).unwrap();
    });
}

#[test]
fn contraction_inside_cjk_single_quotes_keeps_apostrophe_latin() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.contractionInsideCjkSingleQuotesKeepsApostropheLatin", "org.tiqian.layout.QuotePairAnalyzerTest.contractionInsideCjkSingleQuotesKeepsApostropheLatin", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(&"contractionInsideCjkSingleQuotesKeepsApostropheLatin");
        let t = "中‘that’s’中".to_string();
        let r: SortedMapTable<u32, FontRole> = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().classify_pairs(t.as_str(), &QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(t.as_str()).unwrap(), None).unwrap();
        let c = CjkFontRoleClassifier::new();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, r.get(&(1)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, r.get(&(u32::wrapping_sub(u_string::count(t.as_str()), 2))), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, r.get(&(6)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(c.classify(t.as_str(), TextRange::new(6u32, 7u32).unwrap(), None)), None).unwrap();
    });
}

#[test]
fn in_word_apostrophe_matrix_does_not_consume_outer_quote_pairs() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.inWordApostropheMatrixDoesNotConsumeOuterQuotePairs", "org.tiqian.layout.QuotePairAnalyzerTest.inWordApostropheMatrixDoesNotConsumeOuterQuotePairs", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(&"inWordApostropheMatrixDoesNotConsumeOuterQuotePairs");
        let words = vec![
    "that’s".to_string(),
    "l’été".to_string(),
    "rock’n’roll".to_string(),
    "version2’s".to_string(),
    "α’β".to_string(),
    "а’б".to_string(),
    "é’s".to_string(),
];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((words.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let w = (words[usize::try_from(i).unwrap_or(0)]).clone();
            let d = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().classify_quote_roles(w.as_str(), &vec![], None).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(w.as_str()).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, Some((w).to_string())).unwrap();
            let mut all_latin = true;
            let mut j = 0u32;
            while (i32::from_ne_bytes((j).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                if d[usize::try_from(j).unwrap_or(0)].role != FontRole::LatinText {
                    all_latin = false;
                }
                j = u32::wrapping_add(j, 1);
            }
            let _ = TracedAssertions::traced_assertions_assert_true(all_latin, Some((format!("{}{}{}",
            w,
            ": ",
            QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d)
        )).to_string())).unwrap();
            let mut all_source = true;
            j = 0u32;
            while (i32::from_ne_bytes((j).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                if d[usize::try_from(j).unwrap_or(0)].clone().source.to_string() != "NonCjkInWordApostrophe" {
                    all_source = false;
                }
                j = u32::wrapping_add(j, 1);
            }
            let _ = TracedAssertions::traced_assertions_assert_true(all_source, Some((format!("{}{}{}",
            w,
            ": ",
            QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d)
        )).to_string())).unwrap();
            let q = format!("{}{}{}",
            "‘",
            w,
            "’"
        );
            let _ = TracedAssertions::traced_assertions_assert_equals_quote_pair_array(&vec![
    (QuotePair::new(0u32, u32::wrapping_sub(u_string::unit_count(&(q)), 1), QuoteType::Single)).clone(),
], &QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(q.as_str()).unwrap(), Some((q).to_string())).unwrap();
            let mut curly = 0u32;
            let mut k = 0u32;
            let __units = u_string::units(&q);
            let __count = u_string::unit_count(&q);
            while (i32::from_ne_bytes((k).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) {
                let cq = u_string::unit_at_from(&__units, k).unwrap_or(0);
                if cq == 8216 || cq == 8217 || cq == 8220 || cq == 8221 {
                    curly = u32::wrapping_add(curly, 1);
                }
                k = u32::wrapping_add(k, 1);
            }
            let mut exp = String::new();
            for _ in 0..curly {
                exp += &("L");
            }
            let _ = TracedAssertions::traced_assertions_assert_equals_string(exp.as_str(), QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_sig(q.as_str(), QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().classify_pairs(q.as_str(),
&QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(q.as_str()).unwrap(), None).unwrap()).as_str(), Some((q).to_string())).unwrap();
            i = u32::wrapping_add(i, 1);
        }
    });
}

#[test]
fn unmatched_curly_quotes_use_directional_context() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.unmatchedCurlyQuotesUseDirectionalContext", "org.tiqian.layout.QuotePairAnalyzerTest.unmatchedCurlyQuotesUseDirectionalContext", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(&"unmatchedCurlyQuotesUseDirectionalContext");
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(&"leading elision at text start", &"’90s", &"L").unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(&"leading elision after CJK and Western space", &"中文 ’90s", &"L").unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(&"trailing possessive", &"James’ book", &"L").unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(&"truncated Latin opening quote", &"“Hello", &"L").unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(&"truncated Latin closing quote", &"Hello”", &"L").unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(&"unspaced CJK opening quote", &"中文“Hello", &"C").unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(&"unmatched CJK closing quote", &"中文”", &"C").unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(&"context-free quote", &"”", &"C").unwrap();
    });
}

#[test]
fn mismatched_nesting_leaves_quotes_unmatched() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.mismatchedNestingLeavesQuotesUnmatched", "org.tiqian.layout.QuotePairAnalyzerTest.mismatchedNestingLeavesQuotesUnmatched", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(&"mismatchedNestingLeavesQuotesUnmatched");
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(&"“hello’").unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn classifies_pair_as_cjk_when_outer_context_is_cjk() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.classifiesPairAsCjkWhenOuterContextIsCjk", "org.tiqian.layout.QuotePairAnalyzerTest.classifiesPairAsCjkWhenOuterContextIsCjk", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(&"classifiesPairAsCjkWhenOuterContextIsCjk", &"他说“你好”", &vec![2, 5], FontRole::CjkPunctuation).unwrap();
    });
}

#[test]
fn classifies_pair_as_latin_when_outer_context_is_latin() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.classifiesPairAsLatinWhenOuterContextIsLatin", "org.tiqian.layout.QuotePairAnalyzerTest.classifiesPairAsLatinWhenOuterContextIsLatin", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(&"classifiesPairAsLatinWhenOuterContextIsLatin", &"he said “hello” world", &vec![8, 14], FontRole::LatinText).unwrap();
    });
}

#[test]
fn classifies_both_quotes_as_cjk_for_cjk_quoted_latin_content() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.classifiesBothQuotesAsCjkForCjkQuotedLatinContent", "org.tiqian.layout.QuotePairAnalyzerTest.classifiesBothQuotesAsCjkForCjkQuotedLatinContent", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(&"classifiesBothQuotesAsCjkForCjkQuotedLatinContent", &"他说“hello”", &vec![2, 8], FontRole::CjkPunctuation).unwrap();
    });
}

#[test]
fn unspaced_cjk_quotation_of_latin_text_remains_cjk() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.unspacedCjkQuotationOfLatinTextRemainsCjk", "org.tiqian.layout.QuotePairAnalyzerTest.unspacedCjkQuotationOfLatinTextRemainsCjk", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(&"unspacedCjkQuotationOfLatinTextRemainsCjk", &"他说‘hello’", &vec![2, 8], FontRole::CjkPunctuation).unwrap();
    });
}

#[test]
fn spaced_cjk_quoted_content_remains_cjk() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.spacedCjkQuotedContentRemainsCjk", "org.tiqian.layout.QuotePairAnalyzerTest.spacedCjkQuotedContentRemainsCjk", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(&"spacedCjkQuotedContentRemainsCjk", &"他说 ‘你好’", &vec![3, 6], FontRole::CjkPunctuation).unwrap();
    });
}

#[test]
fn classifies_pair_as_cjk_at_text_boundary() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.classifiesPairAsCjkAtTextBoundary", "org.tiqian.layout.QuotePairAnalyzerTest.classifiesPairAsCjkAtTextBoundary", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(&"classifiesPairAsCjkAtTextBoundary", &"“你好”", &vec![0, 3], FontRole::CjkPunctuation).unwrap();
    });
}

#[test]
fn classifies_text_start_latin_pair_from_quoted_content() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.classifiesTextStartLatinPairFromQuotedContent", "org.tiqian.layout.QuotePairAnalyzerTest.classifiesTextStartLatinPairFromQuotedContent", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(&"classifiesTextStartLatinPairFromQuotedContent", &"“Hello” world", &vec![0, 6], FontRole::LatinText).unwrap();
    });
}

#[test]
fn skips_ascii_punctuation_when_resolving_context() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.skipsAsciiPunctuationWhenResolvingContext", "org.tiqian.layout.QuotePairAnalyzerTest.skipsAsciiPunctuationWhenResolvingContext", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(&"skipsAsciiPunctuationWhenResolvingContext", &"English: “hello”", &vec![9, 15], FontRole::LatinText).unwrap();
    });
}

#[test]
fn skips_neutral_dash_when_resolving_context() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.skipsNeutralDashWhenResolvingContext", "org.tiqian.layout.QuotePairAnalyzerTest.skipsNeutralDashWhenResolvingContext", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(&"skipsNeutralDashWhenResolvingContext", &"English — “hello”", &vec![10, 16], FontRole::LatinText).unwrap();
    });
}

#[test]
fn end_of_text_quote_pair_classified_by_outer_context() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.endOfTextQuotePairClassifiedByOuterContext", "org.tiqian.layout.QuotePairAnalyzerTest.endOfTextQuotePairClassifiedByOuterContext", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(&"endOfTextQuotePairClassifiedByOuterContext", &"he said “hello”", &vec![8, 14], FontRole::LatinText).unwrap();
    });
}

#[test]
fn whitespace_delimited_latin_quote_pair_overrides_cjk_outer_context() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.whitespaceDelimitedLatinQuotePairOverridesCjkOuterContext", "org.tiqian.layout.QuotePairAnalyzerTest.whitespaceDelimitedLatinQuotePairOverridesCjkOuterContext", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(&"whitespaceDelimitedLatinQuotePairOverridesCjkOuterContext");
        let t = "（如 ‘O’, ‘Q’）".to_string();
        let d = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_decisions(t.as_str()).unwrap();
        let mut indexes: Vec<u32> = Vec::new();
        let mut j = 0u32;
        while (i32::from_ne_bytes((j).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            indexes.push(d[usize::try_from(j).unwrap_or(0)].index);
            j = u32::wrapping_add(j, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![3, 5, 8, 10], &indexes, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) == 4, Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((d[0usize]).clone().source).to_string() == "DelimitedWesternQuotationRun", Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d)).to_string())).unwrap();
    });
}

#[test]
fn adjacent_quoted_list_items_do_not_use_previous_item_content_as_outer_context() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.adjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContext", "org.tiqian.layout.QuotePairAnalyzerTest.adjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContext", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(&"adjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContext");
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(&"CJK list item after mixed-script item", &"便延伸出了“乃子”“大波”“大灯”“大雷”“大扎”“对A”“波霸”这些词", &"CCCCCCCCCCCCCC").unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(&"Latin list item after Latin item in CJK prose", &concat!("这些太直白了是吧，\n",
" “欧派”“double”“double may”呢"), &"CCCCCC").unwrap();
        let texts = vec![
    "便延伸出了“乃子”“大波”“大灯”“大雷”“大扎”“对A”“波霸”这些词".to_string(),
    concat!("这些太直白了是吧，\n",
" “欧派”“double”“double may”呢").to_string(),
];
        let mut ti = 0u32;
        while (i32::from_ne_bytes((ti).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((texts.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let tt = (texts[usize::try_from(ti).unwrap_or(0)]).clone();
            let mut final_open = 4294967295u32;
            let mut final_close = 4294967295u32;
            let mut p = 0u32;
            let __units1 = u_string::units(&tt);
            let __count1 = u_string::unit_count(&tt);
            while (i32::from_ne_bytes((p).to_ne_bytes())) < (i32::from_ne_bytes((__count1).to_ne_bytes())) {
                let ch = u_string::unit_at_from(&__units1, p).unwrap_or(0);
                if ch == 8220 {
                    final_open = p;
                }
                if ch == 8221 {
                    final_close = p;
                }
                p = u32::wrapping_add(p, 1);
            }
            let fd = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_decisions(tt.as_str()).unwrap();
            let mut filtered: Vec<QuoteRoleDecision> = Vec::new();
            let mut fi = 0u32;
            while (i32::from_ne_bytes((fi).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((fd.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                if fd[usize::try_from(fi).unwrap_or(0)].index == final_open || fd[usize::try_from(fi).unwrap_or(0)].index == final_close {
                    filtered.push((fd[usize::try_from(fi).unwrap_or(0)]).clone());
                }
                fi = u32::wrapping_add(fi, 1);
            }
            let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((filtered.len()) & 0xFFFF_FFFF).unwrap_or(0), Some((tt).to_string())).unwrap();
            let mut all_src = true;
            let mut si = 0u32;
            while (i32::from_ne_bytes((si).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((filtered.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                if filtered[usize::try_from(si).unwrap_or(0)].clone().source.to_string() != "PairedPunctuationOuterScriptContext" {
                    all_src = false;
                }
                si = u32::wrapping_add(si, 1);
            }
            let _ = TracedAssertions::traced_assertions_assert_true(all_src, Some((format!("{}{}{}",
            tt,
            ": ",
            QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&filtered)
        )).to_string())).unwrap();
            ti = u32::wrapping_add(ti, 1);
        }
    });
}

#[test]
fn mixed_chinese_question_at_paragraph_start_uses_paragraph_language() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.mixedChineseQuestionAtParagraphStartUsesParagraphLanguage", "org.tiqian.layout.QuotePairAnalyzerTest.mixedChineseQuestionAtParagraphStartUsesParagraphLanguage", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(&"mixedChineseQuestionAtParagraphStartUsesParagraphLanguage");
        let d = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_decisions(&"“Json是谁？”").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![0, 8], &vec![d[0usize].index, d[1usize].index], None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(d[0usize].role == FontRole::CjkPunctuation, Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((d[0usize]).clone().source).to_string() == "ParagraphLanguageQuoteContext", Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d)).to_string())).unwrap();
    });
}

#[test]
fn explicit_english_paragraph_language_wins_for_mixed_quotation() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.explicitEnglishParagraphLanguageWinsForMixedQuotation", "org.tiqian.layout.QuotePairAnalyzerTest.explicitEnglishParagraphLanguageWinsForMixedQuotation", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(&"explicitEnglishParagraphLanguageWinsForMixedQuotation");
        let t = "“Json是谁？”".to_string();
        let d = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().classify_quote_roles(t.as_str(), &QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(t.as_str()).unwrap(), Some(FontRoleContext::new(Some("en".to_string()),
None))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(d[0usize].role == FontRole::LatinText, Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((d[0usize]).clone().source).to_string() == "ParagraphLanguageQuoteContext", Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d)).to_string())).unwrap();
    });
}

#[test]
fn common_digits_do_not_choose_the_quote_role() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.commonDigitsDoNotChooseTheQuoteRole", "org.tiqian.layout.QuotePairAnalyzerTest.commonDigitsDoNotChooseTheQuoteRole", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(&"commonDigitsDoNotChooseTheQuoteRole");
        let t = "“2024”".to_string();
        let d = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_decisions(t.as_str()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(d[0usize].role == FontRole::CjkPunctuation, Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((d[0usize]).clone().source).to_string() == "ParagraphLanguageQuoteContext", Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d)).to_string())).unwrap();
        let e = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().classify_quote_roles(t.as_str(), &QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(t.as_str()).unwrap(), Some(FontRoleContext::new(Some("en".to_string()),
None))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(e[0usize].role == FontRole::LatinText, Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&e)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((e[0usize]).clone().source).to_string() == "ParagraphLanguageQuoteContext", Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&e)).to_string())).unwrap();
    });
}

#[test]
fn non_latin_western_scripts_participate_as_strong_script_evidence() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.nonLatinWesternScriptsParticipateAsStrongScriptEvidence", "org.tiqian.layout.QuotePairAnalyzerTest.nonLatinWesternScriptsParticipateAsStrongScriptEvidence", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(&"nonLatinWesternScriptsParticipateAsStrongScriptEvidence");
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(&"standalone Cyrillic quotation", &"“Привет”", &"LL").unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(&"mixed Greek and Chinese quotation", &"“π谁？”", &"CC").unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(&"CJK prose quoting Cyrillic", &"他说“Привет”", &"CC").unwrap();
    });
}

#[test]
fn numbered_cjk_quote_prefix_uses_quoted_content() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.numberedCjkQuotePrefixUsesQuotedContent", "org.tiqian.layout.QuotePairAnalyzerTest.numberedCjkQuotePrefixUsesQuotedContent", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(&"numberedCjkQuotePrefixUsesQuotedContent");
        let t = "1.“你知道李白是怎么死的吗？”".to_string();
        let d = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_decisions(t.as_str()).unwrap();
        let mut role2 = FontRole::CjkPunctuation;
        let mut role_last = FontRole::CjkPunctuation;
        let mut src2 = String::new();
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if d[usize::try_from(i).unwrap_or(0)].index == 2 {
                role2 = d[usize::try_from(i).unwrap_or(0)].role;
                src2 = ((d[usize::try_from(i).unwrap_or(0)]).clone().source).to_string();
            }
            if d[usize::try_from(i).unwrap_or(0)].index == u32::wrapping_sub(u_string::count(t.as_str()), 1) {
                role_last = d[usize::try_from(i).unwrap_or(0)].role;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(role2), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(role_last), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"PairedPunctuationContentScriptContext", src2.as_str(), None).unwrap();
    });
}

#[test]
fn numbered_latin_quote_prefix_still_uses_latin_content() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.numberedLatinQuotePrefixStillUsesLatinContent", "org.tiqian.layout.QuotePairAnalyzerTest.numberedLatinQuotePrefixStillUsesLatinContent", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(&"numberedLatinQuotePrefixStillUsesLatinContent", &"1.“Hello”", &vec![2, 8], FontRole::LatinText).unwrap();
    });
}

#[test]
fn classifies_nested_pairs_by_outermost_context() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.classifiesNestedPairsByOutermostContext", "org.tiqian.layout.QuotePairAnalyzerTest.classifiesNestedPairsByOutermostContext", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(&"classifiesNestedPairsByOutermostContext", &"他说：“她说‘你好’。”", &vec![3, 11, 6, 9], FontRole::CjkPunctuation).unwrap();
    });
}

#[test]
fn classifies_latin_nested_quotes_by_outer_context() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.classifiesLatinNestedQuotesByOuterContext", "org.tiqian.layout.QuotePairAnalyzerTest.classifiesLatinNestedQuotesByOuterContext", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(&"classifiesLatinNestedQuotesByOuterContext", &"She said “he said ‘hello’ today” end", &vec![9, 18, 24, 31], FontRole::LatinText).unwrap();
    });
}

#[test]
fn representative_quote_context_matrix_remains_stable() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.representativeQuoteContextMatrixRemainsStable", "org.tiqian.layout.QuotePairAnalyzerTest.representativeQuoteContextMatrixRemainsStable", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(&"representativeQuoteContextMatrixRemainsStable");
        let xs = vec![
    "Latin content at text start|“Hello”|LL".to_string(),
    "CJK content at text start|“你好”|CC".to_string(),
    "mixed Chinese question at text start|“Json是谁？”|CC".to_string(),
    "Cyrillic content at text start|“Привет”|LL".to_string(),
    "CJK prose quoting Latin|他说“hello”|CC".to_string(),
    "Latin prose quoting CJK|He said “你好”|LL".to_string(),
    "spaced Western initials in CJK|（如 ‘O’, ‘Q’）|LLLL".to_string(),
    "spaced CJK quotation|他说 ‘你好’|CC".to_string(),
    "empty pair before Latin|“”English|LL".to_string(),
    "empty pair before CJK|“”中文|CC".to_string(),
    "context-free empty pair|“”|CC".to_string(),
    "numbered CJK quotation|1.“中文”|CC".to_string(),
    "numbered Latin quotation|1.“Hello”|LL".to_string(),
    "mixed CJK outer Latin inner|他说：“She said ‘hello’.”|CLLC".to_string(),
    "mixed Latin outer CJK inner|English “他说‘你好’” end|LCCL".to_string(),
    "CJK outer with contraction|中文‘don’t’|CLC".to_string(),
    "spaced Latin outer with contraction|中文 ‘don’t’|LLL".to_string(),
    concat!("pair across mandatory break|他说：“第一行\n",
"第二行。”|CC").to_string(),
    "tab-delimited Western quote|（如\t‘O’）|LL".to_string(),
];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let z = u_string::split(&(xs[usize::try_from(i).unwrap_or(0)]).clone(), &"|");
            let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role((z[0usize]).clone().as_str(), (z[1usize]).clone().as_str(), (z[2usize]).clone().as_str()).unwrap();
            i = u32::wrapping_add(i, 1);
        }
    });
}

#[test]
fn role_decision_sources_stay_explainable_across_fallback_paths() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.roleDecisionSourcesStayExplainableAcrossFallbackPaths", "org.tiqian.layout.QuotePairAnalyzerTest.roleDecisionSourcesStayExplainableAcrossFallbackPaths", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(&"roleDecisionSourcesStayExplainableAcrossFallbackPaths");
        let xs = vec![
    "“Hello”".to_string(),
    "“Json是谁？”".to_string(),
    "English—“Hello”".to_string(),
    "（如 ‘O’）".to_string(),
    "1.“中文”".to_string(),
    "“”English".to_string(),
    "“”".to_string(),
    "that’s".to_string(),
    "中文 ’90s".to_string(),
    "James’".to_string(),
    "’90s".to_string(),
    "”".to_string(),
];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let d = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_decisions((xs[usize::try_from(i).unwrap_or(0)]).clone().as_str()).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), Some(((xs[usize::try_from(i).unwrap_or(0)]).clone()).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u_string::unit_count(&(((d[0usize]).clone().source).to_string()))).to_ne_bytes())) > (0), Some((format!("{}{}{}",
            (xs[usize::try_from(i).unwrap_or(0)]).clone(),
            ": ",
            QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d)
        )).to_string())).unwrap();
            i = u32::wrapping_add(i, 1);
        }
    });
}
