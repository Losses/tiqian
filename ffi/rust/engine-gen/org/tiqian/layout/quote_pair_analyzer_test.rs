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
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestWhitespaceDelimitedLatinQuotePairOverridesCjkOuterContextFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestUnmatchedQuotesProduceNoPairsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestRoleDecisionSourcesStayExplainableAcrossFallbackPathsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestNumberedCjkQuotePrefixUsesQuotedContentFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestMixedChineseQuestionAtParagraphStartUsesParagraphLanguageFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestMismatchedNestingLeavesQuotesUnmatchedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for QuotePairAnalyzerTestMatchesSingleQuotePairFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerTestMatchesSingleQuotePairFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestMatchesSingleQuotePairFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestMatchesSingleQuotePairFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for QuotePairAnalyzerTestMatchesNestedQuotePairsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerTestMatchesNestedQuotePairsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestMatchesNestedQuotePairsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestMatchesNestedQuotePairsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for QuotePairAnalyzerTestMatchesDoubleQuotePairFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerTestMatchesDoubleQuotePairFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestMatchesDoubleQuotePairFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestMatchesDoubleQuotePairFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestInWordApostropheMatrixDoesNotConsumeOuterQuotePairsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestExplicitEnglishParagraphLanguageWinsForMixedQuotationFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestContractionInsideCjkSingleQuotesKeepsApostropheLatinFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestContractionApostropheDoesNotCloseOuterSingleQuoteFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestCommonDigitsDoNotChooseTheQuoteRoleFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for QuotePairAnalyzerTestAdjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerTestAdjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContextFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerTestAdjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
        }
    }
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
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(UStr::new(&[109,97,116,99,104,101,115,68,111,117,98,108,101,81,117,111,116,101,80,97,105,114]));
        let p = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(UStr::new(&[20182,35828,8220,20320,22909,8221])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((p.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_quote_pair(QuotePair::new(2u32, 5u32, QuoteType::Double), (p[0usize]).clone(), None).unwrap();
    });
}

#[test]
fn matches_single_quote_pair() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.matchesSingleQuotePair", "org.tiqian.layout.QuotePairAnalyzerTest.matchesSingleQuotePair", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(UStr::new(&[109,97,116,99,104,101,115,83,105,110,103,108,101,81,117,111,116,101,80,97,105,114]));
        let p = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(UStr::new(&[20182,35828,8216,20320,22909,8217])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((p.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_quote_pair(QuotePair::new(2u32, 5u32, QuoteType::Single), (p[0usize]).clone(), None).unwrap();
    });
}

#[test]
fn matches_nested_quote_pairs() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.matchesNestedQuotePairs", "org.tiqian.layout.QuotePairAnalyzerTest.matchesNestedQuotePairs", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(UStr::new(&[109,97,116,99,104,101,115,78,101,115,116,101,100,81,117,111,116,101,80,97,105,114,115]));
        let p = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(UStr::new(&[20182,35828,65306,8220,22905,35828,8216,20320,22909,8217,12290,8221])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((p.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mut has6 = false;
        let mut has3 = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((p.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
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
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(UStr::new(&[117,110,109,97,116,99,104,101,100,81,117,111,116,101,115,80,114,111,100,117,99,101,78,111,80,97,105,114,115]));
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(UStr::new(&[105,116,8217,115])).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn contraction_apostrophe_does_not_close_outer_single_quote() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.contractionApostropheDoesNotCloseOuterSingleQuote", "org.tiqian.layout.QuotePairAnalyzerTest.contractionApostropheDoesNotCloseOuterSingleQuote", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(UStr::new(&[99,111,110,116,114,97,99,116,105,111,110,65,112,111,115,116,114,111,112,104,101,68,111,101,115,78,111,116,67,108,111,115,101,79,117,116,101,114,83,105,110,103,108,101,81,117,111,116,101]));
        let t = UString::from("‘that’s’").to_ustring();
        let _ = TracedAssertions::traced_assertions_assert_equals_quote_pair_array(&vec![(QuotePair::new(0u32, 7u32, QuoteType::Single)).clone()], &QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(t.as_ustr()).unwrap(), None).unwrap();
    });
}

#[test]
fn contraction_inside_cjk_single_quotes_keeps_apostrophe_latin() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.contractionInsideCjkSingleQuotesKeepsApostropheLatin", "org.tiqian.layout.QuotePairAnalyzerTest.contractionInsideCjkSingleQuotesKeepsApostropheLatin", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(UStr::new(&[99,111,110,116,114,97,99,116,105,111,110,73,110,115,105,100,101,67,106,107,83,105,110,103,108,101,81,117,111,116,101,115,75,101,101,112,115,65,112,111,115,116,114,111,112,104,101,76,97,116,105,110]));
        let t = UString::from("中‘that’s’中").to_ustring();
        let r: SortedMapTable<u32, FontRole> = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().classify_pairs(t.as_ustr(), &QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(t.as_ustr()).unwrap(), None).unwrap();
        let c = CjkFontRoleClassifier::new();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, r.get(&(1)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, r.get(&(u32::wrapping_sub(u_string::count(t.as_ustr()), 2))), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, r.get(&(6)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(c.classify(t.as_ustr(), TextRange::new(6u32, 7u32).unwrap(), None)), None).unwrap();
    });
}

#[test]
fn in_word_apostrophe_matrix_does_not_consume_outer_quote_pairs() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.inWordApostropheMatrixDoesNotConsumeOuterQuotePairs", "org.tiqian.layout.QuotePairAnalyzerTest.inWordApostropheMatrixDoesNotConsumeOuterQuotePairs", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(UStr::new(&[105,110,87,111,114,100,65,112,111,115,116,114,111,112,104,101,77,97,116,114,105,120,68,111,101,115,78,111,116,67,111,110,115,117,109,101,79,117,116,101,114,81,117,111,116,101,80,97,105,114,115]));
        let words = vec![
    UString::from("that’s").to_ustring(),
    UString::from("l’été").to_ustring(),
    UString::from("rock’n’roll").to_ustring(),
    UString::from("version2’s").to_ustring(),
    UString::from("α’β").to_ustring(),
    UString::from("а’б").to_ustring(),
    UString::from("é’s").to_ustring(),
];
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((words.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let w = (words[usize::try_from(i).unwrap_or(0)]).clone();
            let d = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().classify_quote_roles(w.as_ustr(), &vec![], None).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(w.as_ustr()).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, Some((w).to_ustring())).unwrap();
            let mut all_latin = true;
            let mut j = 0u32;
            while (i32::from_ne_bytes(((j) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                if d[usize::try_from(j).unwrap_or(0)].role != FontRole::LatinText {
                    all_latin = false;
                }
                j = u32::wrapping_add(j, 1);
            }
            let _ = TracedAssertions::traced_assertions_assert_true(all_latin, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += w.as_ustr(); __s += &(UString::from(": ")); __s += QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d).as_ustr(); __s }).as_str()))).unwrap();
            let mut all_source = true;
            j = 0u32;
            while (i32::from_ne_bytes(((j) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                if d[usize::try_from(j).unwrap_or(0)].clone().source.to_ustring() != UString::from("NonCjkInWordApostrophe") {
                    all_source = false;
                }
                j = u32::wrapping_add(j, 1);
            }
            let _ = TracedAssertions::traced_assertions_assert_true(all_source, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += w.as_ustr(); __s += &(UString::from(": ")); __s += QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d).as_ustr(); __s }).as_str()))).unwrap();
            let q = { let mut __s = UString::new(); __s += &(UString::from("‘")); __s += w.as_ustr(); __s += &(UString::from("’")); __s };
            let _ = TracedAssertions::traced_assertions_assert_equals_quote_pair_array(&vec![
    (QuotePair::new(0u32, u32::wrapping_sub(u_string::unit_count(&(q)), 1), QuoteType::Single)).clone(),
], &QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(q.as_ustr()).unwrap(), Some((q).to_ustring())).unwrap();
            let mut curly = 0u32;
            let mut k = 0u32;
            let __units = u_string::units(&q);
            let __count = u_string::unit_count(&q);
            while (i32::from_ne_bytes(((k) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((__count) as i32).to_ne_bytes())) {
                let cq = u_string::unit_at_from(&__units, k).unwrap_or(0);
                if cq == 8216 || cq == 8217 || cq == 8220 || cq == 8221 {
                    curly = u32::wrapping_add(curly, 1);
                }
                k = u32::wrapping_add(k, 1);
            }
            let mut exp = UString::new();
            for _ in 0..curly {
                exp += &(UString::from("L"));
            }
            let _ = TracedAssertions::traced_assertions_assert_equals_string(exp.as_ustr(), QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_sig(q.as_ustr(), QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().classify_pairs(q.as_ustr(), &QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(q.as_ustr()).unwrap(), None).unwrap()).as_ustr(), Some((q).to_ustring())).unwrap();
            i = u32::wrapping_add(i, 1);
        }
    });
}

#[test]
fn unmatched_curly_quotes_use_directional_context() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.unmatchedCurlyQuotesUseDirectionalContext", "org.tiqian.layout.QuotePairAnalyzerTest.unmatchedCurlyQuotesUseDirectionalContext", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(UStr::new(&[117,110,109,97,116,99,104,101,100,67,117,114,108,121,81,117,111,116,101,115,85,115,101,68,105,114,101,99,116,105,111,110,97,108,67,111,110,116,101,120,116]));
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(UStr::new(&[108,101,97,100,105,110,103,32,101,108,105,115,105,111,110,32,97,116,32,116,101,120,116,32,115,116,97,114,116]), UStr::new(&[8217,57,48,115]), UStr::new(&[76])).unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(UStr::new(&[108,101,97,100,105,110,103,32,101,108,105,115,105,111,110,32,97,102,116,101,114,32,67,74,75,32,97,110,100,32,87,101,115,116,101,114,110,32,115,112,97,99,101]), UStr::new(&[20013,25991,32,8217,57,48,115]), UStr::new(&[76])).unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(UStr::new(&[116,114,97,105,108,105,110,103,32,112,111,115,115,101,115,115,105,118,101]), UStr::new(&[74,97,109,101,115,8217,32,98,111,111,107]), UStr::new(&[76])).unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(UStr::new(&[116,114,117,110,99,97,116,101,100,32,76,97,116,105,110,32,111,112,101,110,105,110,103,32,113,117,111,116,101]), UStr::new(&[8220,72,101,108,108,111]), UStr::new(&[76])).unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(UStr::new(&[116,114,117,110,99,97,116,101,100,32,76,97,116,105,110,32,99,108,111,115,105,110,103,32,113,117,111,116,101]), UStr::new(&[72,101,108,108,111,8221]), UStr::new(&[76])).unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(UStr::new(&[117,110,115,112,97,99,101,100,32,67,74,75,32,111,112,101,110,105,110,103,32,113,117,111,116,101]), UStr::new(&[20013,25991,8220,72,101,108,108,111]), UStr::new(&[67])).unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(UStr::new(&[117,110,109,97,116,99,104,101,100,32,67,74,75,32,99,108,111,115,105,110,103,32,113,117,111,116,101]), UStr::new(&[20013,25991,8221]), UStr::new(&[67])).unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(UStr::new(&[99,111,110,116,101,120,116,45,102,114,101,101,32,113,117,111,116,101]), UStr::new(&[8221]), UStr::new(&[67])).unwrap();
    });
}

#[test]
fn mismatched_nesting_leaves_quotes_unmatched() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.mismatchedNestingLeavesQuotesUnmatched", "org.tiqian.layout.QuotePairAnalyzerTest.mismatchedNestingLeavesQuotesUnmatched", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(UStr::new(&[109,105,115,109,97,116,99,104,101,100,78,101,115,116,105,110,103,76,101,97,118,101,115,81,117,111,116,101,115,85,110,109,97,116,99,104,101,100]));
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(UStr::new(&[8220,104,101,108,108,111,8217])).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn classifies_pair_as_cjk_when_outer_context_is_cjk() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.classifiesPairAsCjkWhenOuterContextIsCjk", "org.tiqian.layout.QuotePairAnalyzerTest.classifiesPairAsCjkWhenOuterContextIsCjk", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(UStr::new(&[99,108,97,115,115,105,102,105,101,115,80,97,105,114,65,115,67,106,107,87,104,101,110,79,117,116,101,114,67,111,110,116,101,120,116,73,115,67,106,107]), UStr::new(&[20182,35828,8220,20320,22909,8221]), &vec![2, 5], FontRole::CjkPunctuation).unwrap();
    });
}

#[test]
fn classifies_pair_as_latin_when_outer_context_is_latin() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.classifiesPairAsLatinWhenOuterContextIsLatin", "org.tiqian.layout.QuotePairAnalyzerTest.classifiesPairAsLatinWhenOuterContextIsLatin", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(UStr::new(&[99,108,97,115,115,105,102,105,101,115,80,97,105,114,65,115,76,97,116,105,110,87,104,101,110,79,117,116,101,114,67,111,110,116,101,120,116,73,115,76,97,116,105,110]), UStr::new(&[104,101,32,115,97,105,100,32,8220,104,101,108,108,111,8221,32,119,111,114,108,100]), &vec![8, 14], FontRole::LatinText).unwrap();
    });
}

#[test]
fn classifies_both_quotes_as_cjk_for_cjk_quoted_latin_content() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.classifiesBothQuotesAsCjkForCjkQuotedLatinContent", "org.tiqian.layout.QuotePairAnalyzerTest.classifiesBothQuotesAsCjkForCjkQuotedLatinContent", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(UStr::new(&[99,108,97,115,115,105,102,105,101,115,66,111,116,104,81,117,111,116,101,115,65,115,67,106,107,70,111,114,67,106,107,81,117,111,116,101,100,76,97,116,105,110,67,111,110,116,101,110,116]), UStr::new(&[20182,35828,8220,104,101,108,108,111,8221]), &vec![2, 8], FontRole::CjkPunctuation).unwrap();
    });
}

#[test]
fn unspaced_cjk_quotation_of_latin_text_remains_cjk() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.unspacedCjkQuotationOfLatinTextRemainsCjk", "org.tiqian.layout.QuotePairAnalyzerTest.unspacedCjkQuotationOfLatinTextRemainsCjk", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(UStr::new(&[117,110,115,112,97,99,101,100,67,106,107,81,117,111,116,97,116,105,111,110,79,102,76,97,116,105,110,84,101,120,116,82,101,109,97,105,110,115,67,106,107]), UStr::new(&[20182,35828,8216,104,101,108,108,111,8217]), &vec![2, 8], FontRole::CjkPunctuation).unwrap();
    });
}

#[test]
fn spaced_cjk_quoted_content_remains_cjk() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.spacedCjkQuotedContentRemainsCjk", "org.tiqian.layout.QuotePairAnalyzerTest.spacedCjkQuotedContentRemainsCjk", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(UStr::new(&[115,112,97,99,101,100,67,106,107,81,117,111,116,101,100,67,111,110,116,101,110,116,82,101,109,97,105,110,115,67,106,107]), UStr::new(&[20182,35828,32,8216,20320,22909,8217]), &vec![3, 6], FontRole::CjkPunctuation).unwrap();
    });
}

#[test]
fn classifies_pair_as_cjk_at_text_boundary() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.classifiesPairAsCjkAtTextBoundary", "org.tiqian.layout.QuotePairAnalyzerTest.classifiesPairAsCjkAtTextBoundary", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(UStr::new(&[99,108,97,115,115,105,102,105,101,115,80,97,105,114,65,115,67,106,107,65,116,84,101,120,116,66,111,117,110,100,97,114,121]), UStr::new(&[8220,20320,22909,8221]), &vec![0, 3], FontRole::CjkPunctuation).unwrap();
    });
}

#[test]
fn classifies_text_start_latin_pair_from_quoted_content() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.classifiesTextStartLatinPairFromQuotedContent", "org.tiqian.layout.QuotePairAnalyzerTest.classifiesTextStartLatinPairFromQuotedContent", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(UStr::new(&[99,108,97,115,115,105,102,105,101,115,84,101,120,116,83,116,97,114,116,76,97,116,105,110,80,97,105,114,70,114,111,109,81,117,111,116,101,100,67,111,110,116,101,110,116]), UStr::new(&[8220,72,101,108,108,111,8221,32,119,111,114,108,100]), &vec![0, 6], FontRole::LatinText).unwrap();
    });
}

#[test]
fn skips_ascii_punctuation_when_resolving_context() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.skipsAsciiPunctuationWhenResolvingContext", "org.tiqian.layout.QuotePairAnalyzerTest.skipsAsciiPunctuationWhenResolvingContext", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(UStr::new(&[115,107,105,112,115,65,115,99,105,105,80,117,110,99,116,117,97,116,105,111,110,87,104,101,110,82,101,115,111,108,118,105,110,103,67,111,110,116,101,120,116]), UStr::new(&[69,110,103,108,105,115,104,58,32,8220,104,101,108,108,111,8221]), &vec![9, 15], FontRole::LatinText).unwrap();
    });
}

#[test]
fn skips_neutral_dash_when_resolving_context() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.skipsNeutralDashWhenResolvingContext", "org.tiqian.layout.QuotePairAnalyzerTest.skipsNeutralDashWhenResolvingContext", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(UStr::new(&[115,107,105,112,115,78,101,117,116,114,97,108,68,97,115,104,87,104,101,110,82,101,115,111,108,118,105,110,103,67,111,110,116,101,120,116]), UStr::new(&[69,110,103,108,105,115,104,32,8212,32,8220,104,101,108,108,111,8221]), &vec![10, 16], FontRole::LatinText).unwrap();
    });
}

#[test]
fn end_of_text_quote_pair_classified_by_outer_context() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.endOfTextQuotePairClassifiedByOuterContext", "org.tiqian.layout.QuotePairAnalyzerTest.endOfTextQuotePairClassifiedByOuterContext", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(UStr::new(&[101,110,100,79,102,84,101,120,116,81,117,111,116,101,80,97,105,114,67,108,97,115,115,105,102,105,101,100,66,121,79,117,116,101,114,67,111,110,116,101,120,116]), UStr::new(&[104,101,32,115,97,105,100,32,8220,104,101,108,108,111,8221]), &vec![8, 14], FontRole::LatinText).unwrap();
    });
}

#[test]
fn whitespace_delimited_latin_quote_pair_overrides_cjk_outer_context() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.whitespaceDelimitedLatinQuotePairOverridesCjkOuterContext", "org.tiqian.layout.QuotePairAnalyzerTest.whitespaceDelimitedLatinQuotePairOverridesCjkOuterContext", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(UStr::new(&[119,104,105,116,101,115,112,97,99,101,68,101,108,105,109,105,116,101,100,76,97,116,105,110,81,117,111,116,101,80,97,105,114,79,118,101,114,114,105,100,101,115,67,106,107,79,117,116,101,114,67,111,110,116,101,120,116]));
        let t = UString::from("（如 ‘O’, ‘Q’）").to_ustring();
        let d = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_decisions(t.as_ustr()).unwrap();
        let mut indexes: Vec<u32> = Vec::new();
        let mut j = 0u32;
        while (i32::from_ne_bytes(((j) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            indexes.push(d[usize::try_from(j).unwrap_or(0)].index);
            j = u32::wrapping_add(j, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![3, 5, 8, 10], &indexes, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) == 4, Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d)).to_ustring())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((d[0usize]).clone().source).to_ustring() == UString::from("DelimitedWesternQuotationRun"), Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d)).to_ustring())).unwrap();
    });
}

#[test]
fn adjacent_quoted_list_items_do_not_use_previous_item_content_as_outer_context() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.adjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContext", "org.tiqian.layout.QuotePairAnalyzerTest.adjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContext", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(UStr::new(&[97,100,106,97,99,101,110,116,81,117,111,116,101,100,76,105,115,116,73,116,101,109,115,68,111,78,111,116,85,115,101,80,114,101,118,105,111,117,115,73,116,101,109,67,111,110,116,101,110,116,65,115,79,117,116,101,114,67,111,110,116,101,120,116]));
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(UStr::new(&[67,74,75,32,108,105,115,116,32,105,116,101,109,32,97,102,116,101,114,32,109,105,120,101,100,45,115,99,114,105,112,116,32,105,116,101,109]), UStr::new(&[20415,24310,20280,20986,20102,8220,20035,23376,8221,8220,22823,27874,8221,8220,22823,28783,8221,8220,22823,38647,8221,8220,22823,25166,8221,8220,23545,65,8221,8220,27874,38712,8221,36825,20123,35789]), UStr::new(&[67,67,67,67,67,67,67,67,67,67,67,67,67,67])).unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(UStr::new(&[76,97,116,105,110,32,108,105,115,116,32,105,116,101,109,32,97,102,116,101,114,32,76,97,116,105,110,32,105,116,101,109,32,105,110,32,67,74,75,32,112,114,111,115,101]), UStr::new(&[36825,20123,22826,30452,30333,20102,26159,21543,65292,10,32,8220,27431,27966,8221,8220,100,111,117,98,108,101,8221,8220,100,111,117,98,108,101,32,109,97,121,8221,21602]), UStr::new(&[67,67,67,67,67,67])).unwrap();
        let texts = vec![
    UString::from("便延伸出了“乃子”“大波”“大灯”“大雷”“大扎”“对A”“波霸”这些词").to_ustring(),
    UString::from(concat!("这些太直白了是吧，\n",
" “欧派”“double”“double may”呢")).to_ustring(),
];
        let mut ti = 0u32;
        while (i32::from_ne_bytes(((ti) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((texts.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let tt = (texts[usize::try_from(ti).unwrap_or(0)]).clone();
            let mut final_open = 4294967295u32;
            let mut final_close = 4294967295u32;
            let mut p = 0u32;
            let __units1 = u_string::units(&tt);
            let __count1 = u_string::unit_count(&tt);
            while (i32::from_ne_bytes(((p) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((__count1) as i32).to_ne_bytes())) {
                let ch = u_string::unit_at_from(&__units1, p).unwrap_or(0);
                if ch == 8220 {
                    final_open = p;
                }
                if ch == 8221 {
                    final_close = p;
                }
                p = u32::wrapping_add(p, 1);
            }
            let fd = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_decisions(tt.as_ustr()).unwrap();
            let mut filtered: Vec<QuoteRoleDecision> = Vec::new();
            let mut fi = 0u32;
            while (i32::from_ne_bytes(((fi) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((fd.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                if fd[usize::try_from(fi).unwrap_or(0)].index == final_open || fd[usize::try_from(fi).unwrap_or(0)].index == final_close {
                    filtered.push((fd[usize::try_from(fi).unwrap_or(0)]).clone());
                }
                fi = u32::wrapping_add(fi, 1);
            }
            let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((filtered.len()) & 0xFFFF_FFFF).unwrap_or(0), Some((tt).to_ustring())).unwrap();
            let mut all_src = true;
            let mut si = 0u32;
            while (i32::from_ne_bytes(((si) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((filtered.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                if filtered[usize::try_from(si).unwrap_or(0)].clone().source.to_ustring() != UString::from("PairedPunctuationOuterScriptContext") {
                    all_src = false;
                }
                si = u32::wrapping_add(si, 1);
            }
            let _ = TracedAssertions::traced_assertions_assert_true(all_src, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += tt.as_ustr(); __s += &(UString::from(": ")); __s += QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&filtered).as_ustr(); __s }).as_str()))).unwrap();
            ti = u32::wrapping_add(ti, 1);
        }
    });
}

#[test]
fn mixed_chinese_question_at_paragraph_start_uses_paragraph_language() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.mixedChineseQuestionAtParagraphStartUsesParagraphLanguage", "org.tiqian.layout.QuotePairAnalyzerTest.mixedChineseQuestionAtParagraphStartUsesParagraphLanguage", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(UStr::new(&[109,105,120,101,100,67,104,105,110,101,115,101,81,117,101,115,116,105,111,110,65,116,80,97,114,97,103,114,97,112,104,83,116,97,114,116,85,115,101,115,80,97,114,97,103,114,97,112,104,76,97,110,103,117,97,103,101]));
        let d = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_decisions(UStr::new(&[8220,74,115,111,110,26159,35841,65311,8221])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![0, 8], &vec![d[0usize].index, d[1usize].index], None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(d[0usize].role == FontRole::CjkPunctuation, Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d)).to_ustring())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((d[0usize]).clone().source).to_ustring() == UString::from("ParagraphLanguageQuoteContext"), Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d)).to_ustring())).unwrap();
    });
}

#[test]
fn explicit_english_paragraph_language_wins_for_mixed_quotation() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.explicitEnglishParagraphLanguageWinsForMixedQuotation", "org.tiqian.layout.QuotePairAnalyzerTest.explicitEnglishParagraphLanguageWinsForMixedQuotation", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(UStr::new(&[101,120,112,108,105,99,105,116,69,110,103,108,105,115,104,80,97,114,97,103,114,97,112,104,76,97,110,103,117,97,103,101,87,105,110,115,70,111,114,77,105,120,101,100,81,117,111,116,97,116,105,111,110]));
        let t = UString::from("“Json是谁？”").to_ustring();
        let d = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().classify_quote_roles(t.as_ustr(), &QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(t.as_ustr()).unwrap(), Some(FontRoleContext::new(Some(UString::from("en")), None))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(d[0usize].role == FontRole::LatinText, Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d)).to_ustring())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((d[0usize]).clone().source).to_ustring() == UString::from("ParagraphLanguageQuoteContext"), Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d)).to_ustring())).unwrap();
    });
}

#[test]
fn common_digits_do_not_choose_the_quote_role() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.commonDigitsDoNotChooseTheQuoteRole", "org.tiqian.layout.QuotePairAnalyzerTest.commonDigitsDoNotChooseTheQuoteRole", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(UStr::new(&[99,111,109,109,111,110,68,105,103,105,116,115,68,111,78,111,116,67,104,111,111,115,101,84,104,101,81,117,111,116,101,82,111,108,101]));
        let t = UString::from("“2024”").to_ustring();
        let d = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_decisions(t.as_ustr()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(d[0usize].role == FontRole::CjkPunctuation, Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d)).to_ustring())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((d[0usize]).clone().source).to_ustring() == UString::from("ParagraphLanguageQuoteContext"), Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d)).to_ustring())).unwrap();
        let e = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().classify_quote_roles(t.as_ustr(), &QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_a().analyze(t.as_ustr()).unwrap(), Some(FontRoleContext::new(Some(UString::from("en")), None))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(e[0usize].role == FontRole::LatinText, Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&e)).to_ustring())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((e[0usize]).clone().source).to_ustring() == UString::from("ParagraphLanguageQuoteContext"), Some((QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&e)).to_ustring())).unwrap();
    });
}

#[test]
fn non_latin_western_scripts_participate_as_strong_script_evidence() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.nonLatinWesternScriptsParticipateAsStrongScriptEvidence", "org.tiqian.layout.QuotePairAnalyzerTest.nonLatinWesternScriptsParticipateAsStrongScriptEvidence", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(UStr::new(&[110,111,110,76,97,116,105,110,87,101,115,116,101,114,110,83,99,114,105,112,116,115,80,97,114,116,105,99,105,112,97,116,101,65,115,83,116,114,111,110,103,83,99,114,105,112,116,69,118,105,100,101,110,99,101]));
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(UStr::new(&[115,116,97,110,100,97,108,111,110,101,32,67,121,114,105,108,108,105,99,32,113,117,111,116,97,116,105,111,110]), UStr::new(&[8220,1055,1088,1080,1074,1077,1090,8221]), UStr::new(&[76,76])).unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(UStr::new(&[109,105,120,101,100,32,71,114,101,101,107,32,97,110,100,32,67,104,105,110,101,115,101,32,113,117,111,116,97,116,105,111,110]), UStr::new(&[8220,960,35841,65311,8221]), UStr::new(&[67,67])).unwrap();
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role(UStr::new(&[67,74,75,32,112,114,111,115,101,32,113,117,111,116,105,110,103,32,67,121,114,105,108,108,105,99]), UStr::new(&[20182,35828,8220,1055,1088,1080,1074,1077,1090,8221]), UStr::new(&[67,67])).unwrap();
    });
}

#[test]
fn numbered_cjk_quote_prefix_uses_quoted_content() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.numberedCjkQuotePrefixUsesQuotedContent", "org.tiqian.layout.QuotePairAnalyzerTest.numberedCjkQuotePrefixUsesQuotedContent", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(UStr::new(&[110,117,109,98,101,114,101,100,67,106,107,81,117,111,116,101,80,114,101,102,105,120,85,115,101,115,81,117,111,116,101,100,67,111,110,116,101,110,116]));
        let t = UString::from("1.“你知道李白是怎么死的吗？”").to_ustring();
        let d = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_decisions(t.as_ustr()).unwrap();
        let mut role2 = FontRole::CjkPunctuation;
        let mut role_last = FontRole::CjkPunctuation;
        let mut src2 = UString::new();
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if d[usize::try_from(i).unwrap_or(0)].index == 2 {
                role2 = d[usize::try_from(i).unwrap_or(0)].role;
                src2 = ((d[usize::try_from(i).unwrap_or(0)]).clone().source).to_ustring();
            }
            if d[usize::try_from(i).unwrap_or(0)].index == u32::wrapping_sub(u_string::count(t.as_ustr()), 1) {
                role_last = d[usize::try_from(i).unwrap_or(0)].role;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(role2), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(role_last), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[80,97,105,114,101,100,80,117,110,99,116,117,97,116,105,111,110,67,111,110,116,101,110,116,83,99,114,105,112,116,67,111,110,116,101,120,116]), src2.as_ustr(), None).unwrap();
    });
}

#[test]
fn numbered_latin_quote_prefix_still_uses_latin_content() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.numberedLatinQuotePrefixStillUsesLatinContent", "org.tiqian.layout.QuotePairAnalyzerTest.numberedLatinQuotePrefixStillUsesLatinContent", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(UStr::new(&[110,117,109,98,101,114,101,100,76,97,116,105,110,81,117,111,116,101,80,114,101,102,105,120,83,116,105,108,108,85,115,101,115,76,97,116,105,110,67,111,110,116,101,110,116]), UStr::new(&[49,46,8220,72,101,108,108,111,8221]), &vec![2, 8], FontRole::LatinText).unwrap();
    });
}

#[test]
fn classifies_nested_pairs_by_outermost_context() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.classifiesNestedPairsByOutermostContext", "org.tiqian.layout.QuotePairAnalyzerTest.classifiesNestedPairsByOutermostContext", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(UStr::new(&[99,108,97,115,115,105,102,105,101,115,78,101,115,116,101,100,80,97,105,114,115,66,121,79,117,116,101,114,109,111,115,116,67,111,110,116,101,120,116]), UStr::new(&[20182,35828,65306,8220,22905,35828,8216,20320,22909,8217,12290,8221]), &vec![3, 11, 6, 9], FontRole::CjkPunctuation).unwrap();
    });
}

#[test]
fn classifies_latin_nested_quotes_by_outer_context() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.classifiesLatinNestedQuotesByOuterContext", "org.tiqian.layout.QuotePairAnalyzerTest.classifiesLatinNestedQuotesByOuterContext", || {
        let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_pair_role(UStr::new(&[99,108,97,115,115,105,102,105,101,115,76,97,116,105,110,78,101,115,116,101,100,81,117,111,116,101,115,66,121,79,117,116,101,114,67,111,110,116,101,120,116]), UStr::new(&[83,104,101,32,115,97,105,100,32,8220,104,101,32,115,97,105,100,32,8216,104,101,108,108,111,8217,32,116,111,100,97,121,8221,32,101,110,100]), &vec![9, 18, 24, 31], FontRole::LatinText).unwrap();
    });
}

#[test]
fn representative_quote_context_matrix_remains_stable() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.representativeQuoteContextMatrixRemainsStable", "org.tiqian.layout.QuotePairAnalyzerTest.representativeQuoteContextMatrixRemainsStable", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(UStr::new(&[114,101,112,114,101,115,101,110,116,97,116,105,118,101,81,117,111,116,101,67,111,110,116,101,120,116,77,97,116,114,105,120,82,101,109,97,105,110,115,83,116,97,98,108,101]));
        let xs = vec![
    UString::from("Latin content at text start|“Hello”|LL").to_ustring(),
    UString::from("CJK content at text start|“你好”|CC").to_ustring(),
    UString::from("mixed Chinese question at text start|“Json是谁？”|CC").to_ustring(),
    UString::from("Cyrillic content at text start|“Привет”|LL").to_ustring(),
    UString::from("CJK prose quoting Latin|他说“hello”|CC").to_ustring(),
    UString::from("Latin prose quoting CJK|He said “你好”|LL").to_ustring(),
    UString::from("spaced Western initials in CJK|（如 ‘O’, ‘Q’）|LLLL").to_ustring(),
    UString::from("spaced CJK quotation|他说 ‘你好’|CC").to_ustring(),
    UString::from("empty pair before Latin|“”English|LL").to_ustring(),
    UString::from("empty pair before CJK|“”中文|CC").to_ustring(),
    UString::from("context-free empty pair|“”|CC").to_ustring(),
    UString::from("numbered CJK quotation|1.“中文”|CC").to_ustring(),
    UString::from("numbered Latin quotation|1.“Hello”|LL").to_ustring(),
    UString::from("mixed CJK outer Latin inner|他说：“She said ‘hello’.”|CLLC").to_ustring(),
    UString::from("mixed Latin outer CJK inner|English “他说‘你好’” end|LCCL").to_ustring(),
    UString::from("CJK outer with contraction|中文‘don’t’|CLC").to_ustring(),
    UString::from("spaced Latin outer with contraction|中文 ‘don’t’|LLL").to_ustring(),
    UString::from(concat!("pair across mandatory break|他说：“第一行\n",
"第二行。”|CC")).to_ustring(),
    UString::from("tab-delimited Western quote|（如\t‘O’）|LL").to_ustring(),
];
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let z = u_string::split(&(xs[usize::try_from(i).unwrap_or(0)]).clone(), &UString::from("|"));
            let _ = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_role((z[0usize]).clone().as_ustr(), (z[1usize]).clone().as_ustr(), (z[2usize]).clone().as_ustr()).unwrap();
            i = u32::wrapping_add(i, 1);
        }
    });
}

#[test]
fn role_decision_sources_stay_explainable_across_fallback_paths() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerTest.roleDecisionSourcesStayExplainableAcrossFallbackPaths", "org.tiqian.layout.QuotePairAnalyzerTest.roleDecisionSourcesStayExplainableAcrossFallbackPaths", || {
        QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_rec(UStr::new(&[114,111,108,101,68,101,99,105,115,105,111,110,83,111,117,114,99,101,115,83,116,97,121,69,120,112,108,97,105,110,97,98,108,101,65,99,114,111,115,115,70,97,108,108,98,97,99,107,80,97,116,104,115]));
        let xs = vec![
    UString::from("“Hello”").to_ustring(),
    UString::from("“Json是谁？”").to_ustring(),
    UString::from("English—“Hello”").to_ustring(),
    UString::from("（如 ‘O’）").to_ustring(),
    UString::from("1.“中文”").to_ustring(),
    UString::from("“”English").to_ustring(),
    UString::from("“”").to_ustring(),
    UString::from("that’s").to_ustring(),
    UString::from("中文 ’90s").to_ustring(),
    UString::from("James’").to_ustring(),
    UString::from("’90s").to_ustring(),
    UString::from("”").to_ustring(),
];
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let d = QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_decisions((xs[usize::try_from(i).unwrap_or(0)]).clone().as_ustr()).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), Some(((xs[usize::try_from(i).unwrap_or(0)]).clone()).to_ustring())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(((d[0usize]).clone().source).to_ustring()))) as i32).to_ne_bytes())) > (0), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (xs[usize::try_from(i).unwrap_or(0)]).clone().as_ustr(); __s += &(UString::from(": ")); __s += QuotePairAnalyzerTestSupport::quote_pair_analyzer_test_support_render_decisions(&d).as_ustr(); __s }).as_str()))).unwrap();
            i = u32::wrapping_add(i, 1);
        }
    });
}
