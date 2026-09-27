#![cfg(test)]

use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::contextual_kinsoku_decision_info::ContextualKinsokuDecisionInfo;
use crate::org::tiqian::core::east_asian_spacing_edges::EastAsianSpacingEdges;
use crate::org::tiqian::core::east_asian_spacing_value::EastAsianSpacingValue;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePair;
use crate::org::tiqian::layout::quote_pair_analyzer::QuoteType;
use crate::org::tiqian::layout::unicode_punctuation_boundary_resolver::UnicodePunctuationBoundaryResolver;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbiddenFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbiddenFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbiddenFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbiddenFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbiddenFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbiddenFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbiddenFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbiddenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbiddenFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbiddenFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbiddenFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbiddenFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbiddenFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbiddenFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbiddenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbiddenFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWordApostrophe2019Fault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWordApostrophe2019Fault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWordApostrophe2019Fault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWordApostrophe2019Fault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWordApostrophe2019Fault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWordApostrophe2019Fault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWordApostrophe2019Fault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWordApostrophe2019Fault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWordApostrophe2019Fault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWordApostrophe2019Fault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWordApostrophe2019Fault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWordApostrophe2019Fault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWordApostrophe2019Fault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWordApostrophe2019Fault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWordApostrophe2019Fault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWordApostrophe2019Fault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStartFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStartFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStartFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStartFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStartFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStartFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStartFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStartFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStartFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStartFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStartFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStartFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStartFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStartFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStartFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStartFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnresolvedQuoteFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnresolvedQuoteFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnresolvedQuoteFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnresolvedQuoteFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnresolvedQuoteFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnresolvedQuoteFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnresolvedQuoteFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnresolvedQuoteFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnresolvedQuoteFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnresolvedQuoteFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnresolvedQuoteFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnresolvedQuoteFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnresolvedQuoteFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnresolvedQuoteFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnresolvedQuoteFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnresolvedQuoteFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuationFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuationFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuationFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuationFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuationFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartInfixFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartInfixFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartInfixFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartInfixFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartInfixFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartInfixFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartInfixFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartInfixFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartInfixFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartInfixFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartInfixFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartInfixFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartInfixFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartInfixFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartInfixFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartInfixFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartElseFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartElseFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartElseFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartElseFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartElseFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartElseFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartElseFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartElseFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartElseFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartElseFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartElseFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartElseFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartElseFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartElseFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartElseFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithRuleForLineStartElseFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolvedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolvedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolvedFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolvedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolvedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolvedFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolvedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolvedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolvedFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolvedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolvedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolvedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolvedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolvedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolvedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolvedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionInitialFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionInitialFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionInitialFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionInitialFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionInitialFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionInitialFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionInitialFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionInitialFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionInitialFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionInitialFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionInitialFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionInitialFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionInitialFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionInitialFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionInitialFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionInitialFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionFinalFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionFinalFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionFinalFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionFinalFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionFinalFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionFinalFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionFinalFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionFinalFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionFinalFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionFinalFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionFinalFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionFinalFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionFinalFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionFinalFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionFinalFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithQuoteDirectionFinalFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPunctuationAndSpaceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPunctuationAndSpaceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPunctuationAndSpaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPunctuationAndSpaceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPunctuationAndSpaceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPunctuationAndSpaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPunctuationAndSpaceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPunctuationAndSpaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPunctuationAndSpaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPunctuationAndSpaceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPunctuationAndSpaceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPunctuationAndSpaceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPunctuationAndSpaceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPunctuationAndSpaceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPunctuationAndSpaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPunctuationAndSpaceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContentFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContentFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContentFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContentFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContentFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContentFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContentFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContentFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContentFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContentFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContentFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContentFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContentFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContentFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContentFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContentFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreakFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreakFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreakFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreakFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreakFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreakFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreakFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreakFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreakFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreakFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreakFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreakFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreakFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreakFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreakFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreakFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterEmptyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterEmptyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterEmptyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterEmptyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterEmptyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterEmptyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterEmptyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterEmptyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterEmptyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterEmptyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterEmptyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterEmptyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterEmptyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterEmptyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterEmptyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPreviousContentClusterEmptyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPairedQuotesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPairedQuotesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPairedQuotesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPairedQuotesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPairedQuotesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPairedQuotesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPairedQuotesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPairedQuotesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPairedQuotesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPairedQuotesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPairedQuotesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPairedQuotesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPairedQuotesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPairedQuotesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPairedQuotesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithPairedQuotesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEndFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEndFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEndFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEndFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEndFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEndFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEndFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEndFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEndFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEndFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEndFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEndFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEndFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEndFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEndFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEndFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithOpenPunctuationFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreakFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreakFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreakFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreakFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreakFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreakFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreakFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreakFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreakFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreakFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreakFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreakFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreakFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreakFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreakFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreakFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterEmptyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterEmptyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterEmptyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterEmptyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterEmptyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterEmptyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterEmptyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterEmptyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterEmptyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterEmptyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterEmptyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterEmptyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterEmptyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterEmptyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterEmptyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterEmptyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithNextContentClusterFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithMultipleClustersFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithMultipleClustersFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithMultipleClustersFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithMultipleClustersFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithMultipleClustersFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithMultipleClustersFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithMultipleClustersFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithMultipleClustersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithMultipleClustersFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithMultipleClustersFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithMultipleClustersFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithMultipleClustersFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithMultipleClustersFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithMultipleClustersFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithMultipleClustersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithMultipleClustersFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLatinWordCodePointFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLatinWordCodePointFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLatinWordCodePointFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLatinWordCodePointFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLatinWordCodePointFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLatinWordCodePointFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLatinWordCodePointFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLatinWordCodePointFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLatinWordCodePointFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLatinWordCodePointFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLatinWordCodePointFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLatinWordCodePointFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLatinWordCodePointFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLatinWordCodePointFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLatinWordCodePointFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLatinWordCodePointFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLastSignificantCodePointFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLastSignificantCodePointFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLastSignificantCodePointFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLastSignificantCodePointFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLastSignificantCodePointFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLastSignificantCodePointFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLastSignificantCodePointFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLastSignificantCodePointFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLastSignificantCodePointFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLastSignificantCodePointFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLastSignificantCodePointFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLastSignificantCodePointFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLastSignificantCodePointFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLastSignificantCodePointFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLastSignificantCodePointFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithLastSignificantCodePointFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmpFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmpFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmpFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmpFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmpFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmpFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmpFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmpFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmpFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmpFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmpFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmpFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmpFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmpFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmpFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmpFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEndFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEndFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEndFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEndFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEndFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEndFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEndFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEndFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEndFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEndFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEndFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEndFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEndFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEndFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEndFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEndFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRuleFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRuleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRuleFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRuleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRuleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRuleFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRuleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRuleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRuleFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRuleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRuleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRuleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRuleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRuleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRuleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRuleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMarkFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMarkFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMarkFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMarkFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMarkFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMarkFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMarkFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMarkFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMarkFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMarkFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMarkFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMarkFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMarkFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMarkFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMarkFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMarkFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithInfixNumericSeparatorFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnlyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnlyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnlyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnlyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnlyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnlyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnlyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnlyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnlyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnlyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnlyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnlyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnlyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakBothFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakBothFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakBothFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakBothFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakBothFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakBothFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakBothFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakBothFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakBothFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakBothFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakBothFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakBothFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakBothFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakBothFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakBothFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakBothFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithHasAuthoredBreakFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZwspFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZwspFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZwspFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZwspFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZwspFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZwspFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZwspFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZwspFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZwspFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZwspFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZwspFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZwspFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZwspFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZwspFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZwspFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZwspFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespaceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespaceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespaceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespaceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespaceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespaceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespaceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespaceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespaceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespaceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespaceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatoryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatoryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatoryFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatoryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatoryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatoryFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatoryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatoryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatoryFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatoryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatoryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatoryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatoryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatoryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatoryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatoryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstSignificantCodePointFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstSignificantCodePointFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstSignificantCodePointFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstSignificantCodePointFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstSignificantCodePointFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstSignificantCodePointFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstSignificantCodePointFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstSignificantCodePointFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstSignificantCodePointFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstSignificantCodePointFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstSignificantCodePointFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstSignificantCodePointFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstSignificantCodePointFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstSignificantCodePointFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstSignificantCodePointFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstSignificantCodePointFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstCodePointLengthFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstCodePointLengthFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstCodePointLengthFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstCodePointLengthFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstCodePointLengthFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstCodePointLengthFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstCodePointLengthFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstCodePointLengthFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstCodePointLengthFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstCodePointLengthFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstCodePointLengthFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstCodePointLengthFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstCodePointLengthFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstCodePointLengthFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstCodePointLengthFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithFirstCodePointLengthFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationMarkFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationMarkFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationMarkFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationMarkFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationMarkFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationMarkFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationMarkFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationMarkFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationMarkFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationMarkFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationMarkFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationMarkFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationMarkFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationMarkFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationMarkFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationMarkFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationClassFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationClassFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationClassFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationClassFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationClassFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationClassFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationClassFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationClassFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationClassFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationClassFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationClassFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationClassFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationClassFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationClassFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationClassFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithExclamationClassFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyRangeFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyRangeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyRangeFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyRangeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyRangeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyRangeFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyRangeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyRangeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyRangeFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyRangeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyRangeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyRangeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyRangeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyRangeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyRangeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyRangeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyClustersFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyClustersFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyClustersFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyClustersFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyClustersFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyClustersFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyClustersFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyClustersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyClustersFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyClustersFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyClustersFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyClustersFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyClustersFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyClustersFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyClustersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithEmptyClustersFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigitFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigitFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigitFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigitFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigitFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigitFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigitFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigitFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigitFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigitFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigitFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigitFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigitFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigitFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigitFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigitFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigitFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigitFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigitFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigitFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigitFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigitFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigitFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigitFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigitFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigitFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigitFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigitFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigitFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigitFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigitFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigitFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpaceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpaceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpaceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpaceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpaceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpaceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpaceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpaceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpaceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpaceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpaceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpaceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpaceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpaceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpaceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpaceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpaceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpaceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpaceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpaceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpaceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpaceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePairFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementaryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementaryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementaryFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementaryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementaryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementaryFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementaryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementaryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementaryFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementaryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementaryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementaryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementaryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementaryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementaryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementaryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogateFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogateFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogateFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogateFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogateFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogateFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogateFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogateFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogateFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogateFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogateFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogateFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogateFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogateFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogateFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogateFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementaryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementaryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementaryFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementaryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementaryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementaryFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementaryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementaryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementaryFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementaryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementaryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementaryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementaryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementaryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementaryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementaryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationClassFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationClassFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationClassFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationClassFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationClassFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationClassFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationClassFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationClassFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationClassFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationClassFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationClassFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationClassFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationClassFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationClassFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationClassFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationClassFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithClosePunctuationFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCloseParenthesisClassFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCloseParenthesisClassFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCloseParenthesisClassFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCloseParenthesisClassFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCloseParenthesisClassFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCloseParenthesisClassFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCloseParenthesisClassFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCloseParenthesisClassFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCloseParenthesisClassFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCloseParenthesisClassFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCloseParenthesisClassFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCloseParenthesisClassFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCloseParenthesisClassFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCloseParenthesisClassFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCloseParenthesisClassFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCloseParenthesisClassFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStartFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStartFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStartFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStartFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStartFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStartFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStartFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStartFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStartFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStartFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStartFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStartFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStartFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStartFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStartFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStartFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingAtLineStartFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingAtLineStartFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingAtLineStartFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingAtLineStartFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingAtLineStartFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingAtLineStartFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingAtLineStartFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingAtLineStartFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingAtLineStartFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingAtLineStartFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingAtLineStartFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingAtLineStartFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingAtLineStartFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingAtLineStartFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingAtLineStartFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithCjkClosingAtLineStartFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithAllCjkTextFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithAllCjkTextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithAllCjkTextFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithAllCjkTextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithAllCjkTextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithAllCjkTextFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithAllCjkTextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithAllCjkTextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithAllCjkTextFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithAllCjkTextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithAllCjkTextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithAllCjkTextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithAllCjkTextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithAllCjkTextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithAllCjkTextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesWithAllCjkTextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesSurrogateScanningVariationsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesSurrogateScanningVariationsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesSurrogateScanningVariationsFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesSurrogateScanningVariationsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesSurrogateScanningVariationsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesSurrogateScanningVariationsFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesSurrogateScanningVariationsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesSurrogateScanningVariationsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesSurrogateScanningVariationsFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesSurrogateScanningVariationsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesSurrogateScanningVariationsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesSurrogateScanningVariationsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesSurrogateScanningVariationsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesSurrogateScanningVariationsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesSurrogateScanningVariationsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesSurrogateScanningVariationsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeftFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeftFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeftFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeftFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeftFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeftFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeftFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeftFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeftFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeftFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeftFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeftFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeftFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeftFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeftFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeftFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnlyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnlyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnlyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnlyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnlyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnlyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnlyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnlyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnlyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnlyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnlyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnlyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnlyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWordFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWordFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWordFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWordFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWordFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWordFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWordFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWordFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWordFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWordFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWordFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWordFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWordFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWordFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWordFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWordFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnlyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnlyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnlyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnlyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnlyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnlyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnlyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnlyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnlyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnlyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnlyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnlyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnlyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeftFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeftFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeftFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeftFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeftFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeftFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeftFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeftFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeftFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeftFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeftFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeftFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeftFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeftFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeftFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeftFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNullFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNullFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNullFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNullFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNullFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNullFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNullFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNullFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNullFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNullFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNullFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNullFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNullFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNullFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNullFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNullFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContentFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContentFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContentFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContentFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContentFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContentFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContentFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContentFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContentFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContentFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContentFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContentFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContentFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContentFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContentFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContentFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmptyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmptyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmptyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmptyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmptyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmptyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmptyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmptyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmptyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmptyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmptyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmptyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmptyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmptyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmptyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmptyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnlyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnlyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnlyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnlyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnlyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnlyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnlyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnlyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnlyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnlyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnlyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnlyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnlyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesNextContentClusterReturnsContentFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesNextContentClusterReturnsContentFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesNextContentClusterReturnsContentFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesNextContentClusterReturnsContentFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesNextContentClusterReturnsContentFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesNextContentClusterReturnsContentFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesNextContentClusterReturnsContentFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesNextContentClusterReturnsContentFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesNextContentClusterReturnsContentFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesNextContentClusterReturnsContentFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesNextContentClusterReturnsContentFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesNextContentClusterReturnsContentFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesNextContentClusterReturnsContentFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesNextContentClusterReturnsContentFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesNextContentClusterReturnsContentFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesNextContentClusterReturnsContentFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEndingFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEndingFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEndingFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEndingFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEndingFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEndingFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEndingFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEndingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEndingFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEndingFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEndingFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEndingFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEndingFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEndingFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEndingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEndingFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrevFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrevFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrevFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrevFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrevFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrevFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrevFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrevFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrevFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrevFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrevFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrevFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrevFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrevFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrevFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrevFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZeroFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZeroFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZeroFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZeroFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZeroFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZeroFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZeroFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZeroFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZeroFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZeroFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZeroFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZeroFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZeroFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZeroFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZeroFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZeroFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutsideFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutsideFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutsideFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutsideFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutsideFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutsideFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutsideFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutsideFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutsideFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutsideFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutsideFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutsideFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutsideFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutsideFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutsideFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutsideFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInsideFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInsideFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInsideFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInsideFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInsideFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInsideFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInsideFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInsideFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInsideFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInsideFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInsideFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInsideFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInsideFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInsideFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInsideFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInsideFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrevFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrevFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrevFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrevFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrevFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrevFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrevFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrevFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrevFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrevFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrevFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrevFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrevFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrevFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrevFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrevFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpaceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpaceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpaceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpaceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpaceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpaceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpaceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpaceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpaceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpaceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpaceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePointFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePointFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePointFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePointFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePointFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePointFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePointFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePointFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePointFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePointFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePointFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePointFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePointFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePointFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePointFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePointFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePointFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePointFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePointFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePointFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePointFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePointFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePointFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePointFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePointFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePointFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePointFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePointFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePointFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePointFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePointFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePointFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyStringFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyStringFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyStringFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyStringFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyStringFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyStringFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyStringFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyStringFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyStringFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyStringFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyStringFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyStringFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyStringFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyStringFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyStringFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyStringFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZwspInMiddleFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZwspInMiddleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZwspInMiddleFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZwspInMiddleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZwspInMiddleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZwspInMiddleFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZwspInMiddleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZwspInMiddleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZwspInMiddleFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZwspInMiddleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZwspInMiddleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZwspInMiddleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZwspInMiddleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZwspInMiddleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZwspInMiddleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZwspInMiddleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespaceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespaceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespaceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespaceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespaceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespaceFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespaceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespaceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespaceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespaceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespaceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespaceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogateFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogateFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogateFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogateFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogateFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogateFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogateFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogateFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogateFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogateFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogateFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogateFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogateFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogateFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogateFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogateFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthBmpFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthBmpFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthBmpFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthBmpFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthBmpFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthBmpFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthBmpFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthBmpFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthBmpFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthBmpFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthBmpFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthBmpFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthBmpFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthBmpFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthBmpFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesFirstCodePointLengthBmpFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowingVariationsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowingVariationsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowingVariationsFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowingVariationsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowingVariationsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowingVariationsFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowingVariationsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowingVariationsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowingVariationsFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowingVariationsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowingVariationsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowingVariationsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowingVariationsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowingVariationsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowingVariationsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowingVariationsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbiddenFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbiddenFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbiddenFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbiddenFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbiddenFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbiddenFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbiddenFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbiddenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbiddenFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbiddenFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbiddenFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbiddenFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbiddenFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbiddenFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbiddenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbiddenFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbiddenFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbiddenFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbiddenFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbiddenFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbiddenFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbiddenFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbiddenFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbiddenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbiddenFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbiddenFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbiddenFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbiddenFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbiddenFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbiddenFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbiddenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbiddenFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbiddenFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbiddenFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbiddenFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbiddenFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbiddenFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbiddenFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbiddenFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbiddenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbiddenFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbiddenFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbiddenFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbiddenFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbiddenFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbiddenFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbiddenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbiddenFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbiddenFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbiddenFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbiddenFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbiddenFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbiddenFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbiddenFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbiddenFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbiddenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbiddenFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbiddenFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbiddenFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbiddenFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbiddenFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbiddenFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbiddenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbiddenFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbiddenFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbiddenFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbiddenFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbiddenFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbiddenFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbiddenFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbiddenFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbiddenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbiddenFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbiddenFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbiddenFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbiddenFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbiddenFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbiddenFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbiddenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbiddenFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePairFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingleFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingleFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingleFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingleFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePairFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementaryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementaryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementaryFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementaryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementaryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementaryFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementaryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementaryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementaryFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementaryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementaryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementaryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementaryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementaryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementaryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementaryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakableFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakableFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakableFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakableFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakableFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakableFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakableFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakableFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakableFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakableFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakableFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakableFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakableFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakableFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakableFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakableFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificantFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificantFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificantFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificantFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificantFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificantFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificantFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificantFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificantFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificantFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificantFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificantFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificantFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificantFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificantFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificantFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogateFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogateFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogateFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogateFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogateFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogateFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogateFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogateFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogateFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogateFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogateFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogateFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogateFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogateFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogateFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogateFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPairFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPairFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContextFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContextFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContextFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContextFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAndLatinWordBranchesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAndLatinWordBranchesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAndLatinWordBranchesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAndLatinWordBranchesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAndLatinWordBranchesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAndLatinWordBranchesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAndLatinWordBranchesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAndLatinWordBranchesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAndLatinWordBranchesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAndLatinWordBranchesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAndLatinWordBranchesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAndLatinWordBranchesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAndLatinWordBranchesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAndLatinWordBranchesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAndLatinWordBranchesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveUnicodePunctuationBoundariesApostropheAndLatinWordBranchesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithWesternBracketFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithWesternBracketFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithWesternBracketFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithWesternBracketFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithWesternBracketFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithWesternBracketFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithWesternBracketFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithWesternBracketFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithWesternBracketFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithWesternBracketFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithWesternBracketFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithWesternBracketFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithWesternBracketFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithWesternBracketFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithWesternBracketFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithWesternBracketFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithSinoWesternPairFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithSinoWesternPairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithSinoWesternPairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithSinoWesternPairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithSinoWesternPairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithSinoWesternPairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithSinoWesternPairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithSinoWesternPairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithSinoWesternPairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithSinoWesternPairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithSinoWesternPairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithSinoWesternPairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithSinoWesternPairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithSinoWesternPairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithSinoWesternPairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithSinoWesternPairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBothCjkFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBothCjkFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBothCjkFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBothCjkFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBothCjkFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBothCjkFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBothCjkFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBothCjkFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBothCjkFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBothCjkFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBothCjkFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBothCjkFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBothCjkFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBothCjkFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBothCjkFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBothCjkFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracketFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracketFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracketFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracketFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracketFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracketFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracketFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracketFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracketFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracketFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracketFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracketFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracketFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracketFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracketFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracketFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithBothCjkPunctuationFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithBothCjkPunctuationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithBothCjkPunctuationFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithBothCjkPunctuationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithBothCjkPunctuationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithBothCjkPunctuationFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithBothCjkPunctuationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithBothCjkPunctuationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithBothCjkPunctuationFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithBothCjkPunctuationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithBothCjkPunctuationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithBothCjkPunctuationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithBothCjkPunctuationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithBothCjkPunctuationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithBothCjkPunctuationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWithBothCjkPunctuationFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWesternBracketOnlyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWesternBracketOnlyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWesternBracketOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWesternBracketOnlyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWesternBracketOnlyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWesternBracketOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWesternBracketOnlyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWesternBracketOnlyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWesternBracketOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWesternBracketOnlyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWesternBracketOnlyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWesternBracketOnlyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWesternBracketOnlyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWesternBracketOnlyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWesternBracketOnlyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesWesternBracketOnlyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeftFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeftFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeftFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeftFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeftFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeftFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeftFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeftFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeftFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeftFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeftFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeftFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeftFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeftFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeftFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeftFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesSinoWesternOnlyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesSinoWesternOnlyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesSinoWesternOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesSinoWesternOnlyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesSinoWesternOnlyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesSinoWesternOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesSinoWesternOnlyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesSinoWesternOnlyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesSinoWesternOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesSinoWesternOnlyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesSinoWesternOnlyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesSinoWesternOnlyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesSinoWesternOnlyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesSinoWesternOnlyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesSinoWesternOnlyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesSinoWesternOnlyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSizeFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSizeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSizeFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSizeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSizeFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSizeFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSizeFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSizeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSizeFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSizeFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSizeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSizeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSizeFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSizeFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSizeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSizeFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizesFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizesFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizesFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizesFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizesFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizesFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizesFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSizeFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSizeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSizeFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSizeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSizeFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSizeFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSizeFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSizeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSizeFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSizeFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSizeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSizeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSizeFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSizeFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSizeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSizeFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrowFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrowFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrowFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrowFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrowFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrowFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrowFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrowFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrowFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrowFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrowFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrowFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrowFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrowFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrowFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrowFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnlyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnlyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnlyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnlyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnlyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnlyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnlyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnlyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnlyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnlyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnlyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnlyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnlyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunctFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunctFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunctFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunctFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunctFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunctFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunctFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunctFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunctFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunctFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunctFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunctFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunctFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunctFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunctFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunctFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailingFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailingFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailingFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailingFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailingFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailingFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailingFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailingFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailingFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailingFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailingFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailingFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailingFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailingFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrowFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrowFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrowFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrowFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrowFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrowFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrowFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrowFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrowFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrowFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrowFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrowFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrowFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrowFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrowFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrowFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnlyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnlyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnlyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnlyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnlyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnlyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnlyFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnlyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnlyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnlyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnlyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnlyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnlyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnlyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesNarrowNarrowPairFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesNarrowNarrowPairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesNarrowNarrowPairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesNarrowNarrowPairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesNarrowNarrowPairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesNarrowNarrowPairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesNarrowNarrowPairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesNarrowNarrowPairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesNarrowNarrowPairFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesNarrowNarrowPairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesNarrowNarrowPairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesNarrowNarrowPairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesNarrowNarrowPairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesNarrowNarrowPairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesNarrowNarrowPairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesNarrowNarrowPairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesAllConditionsFalseFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesAllConditionsFalseFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesAllConditionsFalseFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesAllConditionsFalseFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesAllConditionsFalseFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesAllConditionsFalseFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesAllConditionsFalseFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesAllConditionsFalseFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesAllConditionsFalseFault) -> Self {
        match value {
            UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesAllConditionsFalseFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesAllConditionsFalseFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesAllConditionsFalseFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesAllConditionsFalseFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesAllConditionsFalseFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesAllConditionsFalseFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationBoundaryResolverCoverageTestResolveAttachedInlineInterCharBoundariesAllConditionsFalseFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct UnicodePunctuationBoundaryResolverCoverageSupport;

impl UnicodePunctuationBoundaryResolverCoverageSupport {
    pub fn unicode_punctuation_boundary_resolver_coverage_support_start(n: &str) {
        TestTraceRecorder::new("UnicodePunctuationBoundaryResolverCoverageTest").section(n);
    }

    pub fn unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(t: &str) -> Result<Vec<Cluster>, TextRangeError> {
    let __units = u_string::units(&t);
    let __count = u_string::unit_count(&t);
        let mut a: Vec<Cluster> = vec![];
        let mut i = 0u32;
        let __units1 = u_string::units(&t);
        let __count1 = u_string::unit_count(&t);
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) {
            let s = u_string::char_at_from(&__units1, i);
            a.push(Cluster::new(TextRange::new(i, u32::wrapping_add(i, 1))?, s.as_str(), "cjk", 16.0f64, Some(s.to_string()), Some(0.0), Some(0.0), Some(0.0)));
            i = u32::wrapping_add(i, 1);
        }
        return Ok(a);
    }

    pub fn unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t: &str) -> Result<Vec<Cluster>, TextRangeError> {
    let __units2 = u_string::units(&t);
    let __count2 = u_string::unit_count(&t);
        let mut a: Vec<Cluster> = vec![];
        let mut i = 0u32;
        let __units3 = u_string::units(&t);
        let __count3 = u_string::unit_count(&t);
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((__count2).to_ne_bytes())) {
            let s = u_string::char_at_from(&__units3, i);
            a.push(Cluster::new(TextRange::new(i, u32::wrapping_add(i, 1))?, s.as_str(), "latin", 8.0f64, Some(s.to_string()), Some(0.0), Some(0.0), Some(0.0)));
            i = u32::wrapping_add(i, 1);
        }
        return Ok(a);
    }

    pub fn unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(t: &str) -> Vec<FontRole> {
        let mut a: Vec<FontRole> = vec![];
        for _ in 0..match u32::try_from(u_string::unit_count(&(t))) { Ok(value) => value, Err(_) => u32::MAX } {
            a.push(FontRole::CjkText);
        }
        return a;
    }

    pub fn unicode_punctuation_boundary_resolver_coverage_support_latin_roles(t: &str) -> Vec<FontRole> {
        let mut a: Vec<FontRole> = vec![];
        for _ in 0..match u32::try_from(u_string::unit_count(&(t))) { Ok(value) => value, Err(_) => u32::MAX } {
            a.push(FontRole::LatinText);
        }
        return a;
    }

    pub fn unicode_punctuation_boundary_resolver_coverage_support_surrogate(codes: &Vec<u32>) -> String {
        let mut s = String::new();
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((codes.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let unit = codes[usize::try_from(i).unwrap_or(0)];
            if ({ let v: u32 = unit; i32::from_ne_bytes(v.to_ne_bytes()) }) >= 55296 && ({ let v: u32 = unit; i32::from_ne_bytes(v.to_ne_bytes()) }) <= 56319 && (i32::from_ne_bytes((u32::wrapping_add(i, 1)).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((codes.len()) &
0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) && ({ let v: u32 = codes[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) >= 56320 && ({ let v: u32 = codes[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)];
i32::from_ne_bytes(v.to_ne_bytes()) }) <= 57343 {
                let low = codes[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)];
                s += &(if u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320)) > 0xFFFF { String::from_utf16(&[0xD800 + (((u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)),
u32::wrapping_sub(low, 56320))) - 0x10000) >> 10) as u16, 0xDC00 + (((u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320))) - 0x10000) & 0x3FF) as u16]).unwrap() } else {
String::from_utf16_lossy(&[(u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(unit, 55296)) << (10)), u32::wrapping_sub(low, 56320))) as u16]) });
                i = u32::wrapping_add(i, 2);
            } else {
                s += &(if unit > 0xFFFF { String::from_utf16(&[0xD800 + (((unit) - 0x10000) >> 10) as u16, 0xDC00 + (((unit) - 0x10000) & 0x3FF) as u16]).unwrap() } else { String::from_utf16_lossy(&[(unit) as u16]) });
                i = u32::wrapping_add(i, 1);
            }
        }
        return s;
    }

    pub fn unicode_punctuation_boundary_resolver_coverage_support_map_text(m: SortedMapTable<u32, u32>) -> String {
        let mut s = "{".to_string();
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::from_ne_bytes((m.size()).to_ne_bytes())).to_ne_bytes())) {
            if i32::from_ne_bytes((i).to_ne_bytes()) > (0) {
                s += &(", ");
            }
            s += &(format!("{}{}{}",
            crate::runtime::int_text::IntText::int_text(m.key_at(i32::from_ne_bytes((i).to_ne_bytes()))),
            "=",
            match m.get(&(m.key_at(i32::from_ne_bytes((i).to_ne_bytes())))) { Some(v) => crate::runtime::int_text::IntText::int_text(v), None => "null".to_string() }
        ));
            i = u32::wrapping_add(i, 1);
        }
        return format!("{}{}",
            s,
            "}"
        );
    }
}

#[test]
fn resolve_attached_inline_virtual_boundaries_with_multiple_previous() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineVirtualBoundariesWithMultiplePrevious", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineVirtualBoundariesWithMultiplePrevious", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineVirtualBoundariesWithMultiplePrevious");
        let attachments = vec![
    InlineAttachment::None,
    InlineAttachment::Previous,
    InlineAttachment::Previous,
    InlineAttachment::None,
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_virtual_boundaries(&attachments);
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((r.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, r[0usize].previous_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(1u32, 2u32), ((r[0usize]).clone().attached_cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(3, *(r[0usize].next_cluster_index).as_ref().unwrap(), None).unwrap();
    });
}

#[test]
fn resolve_attached_inline_virtual_boundaries_with_no_previous() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineVirtualBoundariesWithNoPrevious", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineVirtualBoundariesWithNoPrevious", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineVirtualBoundariesWithNoPrevious");
        let attachments = vec![InlineAttachment::None, InlineAttachment::None];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_virtual_boundaries(&attachments);
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((r.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_open_punctuation() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithOpenPunctuation", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithOpenPunctuation", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithOpenPunctuation");
        let text = "（中文）".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let mut found = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineEnd" {
                found = true;
                break;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(found, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_paired_quotes() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithPairedQuotes", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithPairedQuotes", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithPairedQuotes");
        let text = "中文“你好”中文".to_string();
        let pairs = vec![(QuotePair::new(2u32, 5u32, QuoteType::Double)).clone()];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&pairs).unwrap();
        let mut a = false;
        let mut b = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().reason.to_string() == "Uax14WesternPunctuationBoundary:PairedOpeningQuote" {
                a = true;
            }
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().reason.to_string() == "Uax14WesternPunctuationBoundary:PairedClosingQuote" {
                b = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(a, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(b, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_unmatched_closing_punctuation() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuation",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuation", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuation");
        let text = "中。".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_cjk_closing_at_line_start() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithCjkClosingAtLineStart", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithCjkClosingAtLineStart", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithCjkClosingAtLineStart");
        let text = "。，".to_string();
        let pairs = vec![(QuotePair::new(0u32, 1u32, QuoteType::Single)).clone()];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&pairs).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineEnd" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_exclamation_mark() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithExclamationMark", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithExclamationMark", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithExclamationMark");
        let text = "中!中".to_string();
        let clusters = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "中", "cjk", 16.0f64, Some("中".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), "!", "latin", 16.0f64, Some("!".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "中", "cjk", 16.0f64, Some("中".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &clusters,
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()), &vec![]).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineStart" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_initial_quote_forbid_line_end() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEnd",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEnd", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEnd");
        let text = "中“中".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineEnd" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_unresolved_quote() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithUnresolvedQuote", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithUnresolvedQuote", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithUnresolvedQuote");
        let text = "中’中".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineStart" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_multiple_clusters() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithMultipleClusters", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithMultipleClusters", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithMultipleClusters");
        let text = "中文，中文".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineStart" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_empty_clusters() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithEmptyClusters", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithEmptyClusters", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithEmptyClusters");
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(&"", &vec![], &vec![], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_all_cjk_text() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithAllCjkText", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithAllCjkText", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithAllCjkText");
        let text = "中文文文".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_western_closing_forbid_line_start() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStart",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStart", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStart");
        let text = "中)中".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "中", "cjk", 16.0f64, Some("中".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), ")", "latin", 16.0f64, Some(")".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "中", "cjk", 16.0f64, Some("中".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_cjk_closing_forbid_line_start() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStart",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStart", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStart");
        let text = "中。中".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_open_punctuation_forbid_line_end() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEnd",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEnd", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEnd");
        let text = "（中文".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineEnd" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_punctuation_and_space() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithPunctuationAndSpace", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithPunctuationAndSpace", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithPunctuationAndSpace");
        let text = "中 。".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "中", "cjk", 16.0f64, Some("中".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), " ", "latin", 16.0f64, Some(" ".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "。", "cjk", 16.0f64, Some("。".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineStart" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_follows_authored_boundary() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundary", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundary",
|| {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundary");
        let text = concat!("\n",
"（中文").to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let mut hit: Option<ContextualKinsokuDecisionInfo> = None;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().source_text.to_string() == "（" && ((r.decisions[usize::try_from(i).unwrap_or(0)]).clone().forbidden_position).to_string() == "LineStart" {
                hit = Some((r.decisions[usize::try_from(i).unwrap_or(0)]).clone());
                break;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"-", match &(hit) { None => "-".to_string(), Some(__option2) => __option2.to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_close_punctuation_class() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithClosePunctuationClass", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithClosePunctuationClass", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithClosePunctuationClass");
        let text = "中。".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_infix_numeric_separator() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithInfixNumericSeparator", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithInfixNumericSeparator", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithInfixNumericSeparator");
        let text = "1，2".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "1", "latin", 8.0f64, Some("1".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), "，", "cjk", 16.0f64, Some("，".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "2", "latin", 8.0f64, Some("2".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let roles = vec![FontRole::LatinText, FontRole::CjkPunctuation, FontRole::LatinText];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &roles, &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_decimal_mark_after_space() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpace", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpace", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpace");
        let text = "1 ，2".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "1", "latin", 8.0f64, Some("1".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), " ", "latin", 8.0f64, Some(" ".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "，", "cjk", 16.0f64, Some("，".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(3u32, 4u32).unwrap(), "2", "latin", 8.0f64, Some("2".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let roles = vec![FontRole::LatinText, FontRole::LatinText, FontRole::CjkPunctuation, FontRole::LatinText];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &roles, &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_rule_for_line_start_infix() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithRuleForLineStartInfix", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithRuleForLineStartInfix", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithRuleForLineStartInfix");
        let text = "1,2".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let mut hit: Option<ContextualKinsokuDecisionInfo> = None;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().source_text.to_string() == "," {
                hit = Some((r.decisions[usize::try_from(i).unwrap_or(0)]).clone());
                break;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_not_null_rendered(hit != None, match &(hit) { None => "null".to_string(), Some(__option5) => __option5.to_string() }.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"Uax14WesternPunctuationBoundary:LB15d", match &(hit) { None => "".to_string(), Some(__option8) => ((__option8.reason).to_string()).clone() }.as_str(), None).unwrap();
    });
}

#[test]
fn resolve_attached_inline_inter_char_boundaries_with_cjk_both_cjk() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesWithCjkBothCjk", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesWithCjkBothCjk", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineInterCharBoundariesWithCjkBothCjk");
        let text = "中文".to_string();
        let e = vec![
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Narrow, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Wide, false)).clone(),
];
        let a = vec![InlineAttachment::None, InlineAttachment::None];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()), &e,
SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &a).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.virtual_boundary_after_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_attached_inline_inter_char_boundaries_with_western_bracket() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesWithWesternBracket", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesWithWesternBracket", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineInterCharBoundariesWithWesternBracket");
        let text = "(中".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "(", "latin", 8.0f64, Some("(".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), "中", "cjk", 16.0f64, Some("中".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let roles = vec![FontRole::LatinText, FontRole::CjkText];
        let e = vec![
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
];
        let a = vec![InlineAttachment::None, InlineAttachment::None];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(text.as_str(), &c, &roles, &e, SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b|
SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &a).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.virtual_boundary_after_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_attached_inline_inter_char_boundaries_with_cjk_body_western_bracket() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracket",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracket", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracket");
        let text = "中)".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "中", "cjk", 16.0f64, Some("中".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), ")", "latin", 8.0f64, Some(")".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let roles = vec![FontRole::CjkText, FontRole::LatinText];
        let e = vec![
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
];
        let a = vec![InlineAttachment::None, InlineAttachment::None];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(text.as_str(), &c, &roles, &e, SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b|
SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &a).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.ordinary_western_boundary_after_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_attached_inline_inter_char_boundaries_requires_matching_cluster_role_edge_sizes() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizes",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizes", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizes");
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "a", "latin", 8.0f64, Some("a".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let e = vec![
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
];
        let a = vec![InlineAttachment::None];
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let c = (c).clone(); let e = (e).clone(); let a = (a).clone(); Arc::new(move || {
        UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(&"ab", &c, &vec![FontRole::LatinText, FontRole::LatinText], &e, SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b|
SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &a).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn resolve_attached_inline_inter_char_boundaries_requires_matching_attachment_size() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSize",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSize", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSize");
        let text = "ab".to_string();
        let c = UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap();
        let e = vec![
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
];
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let text = (text).clone(); let c = (c).clone(); let e = (e).clone(); Arc::new(move || {
        UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(text.as_str(), &c, &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&e, SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &vec![InlineAttachment::None]).map_err(|e|
IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn resolve_attached_inline_inter_char_boundaries_punctuation_western_narrow_trailing() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailing",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailing", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailing");
        let text = "a,。".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "a", "latin", 8.0f64, Some("a".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), ",", "latin", 8.0f64, Some(",".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "。", "cjk", 16.0f64, Some("。".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let roles = vec![FontRole::LatinText, FontRole::CjkPunctuation, FontRole::CjkPunctuation];
        let e = vec![
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Narrow, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
];
        let a = vec![InlineAttachment::None, InlineAttachment::Previous, InlineAttachment::None];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(text.as_str(), &c, &roles, &e, SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b|
SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &a).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.virtual_boundary_after_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_previous_content_cluster_returns_null() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNull",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNull", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNull");
        let text = "  !".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), " ", "test", 8.0f64, Some(" ".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), " ", "test", 8.0f64, Some(" ".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "!", "test", 8.0f64, Some("!".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c,
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()), &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()) == 0, None).unwrap();
    });
}

#[test]
fn resolve_attached_inline_inter_char_boundaries_punctuation_western_trailing_not_narrow() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrow",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrow", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrow");
        let text = "a,中".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "a", "latin", 8.0f64, Some("a".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), ",", "latin", 8.0f64, Some(",".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "中", "cjk", 16.0f64, Some("中".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let roles = vec![FontRole::CjkPunctuation, FontRole::CjkPunctuation, FontRole::CjkText];
        let e = vec![
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
];
        let a = vec![InlineAttachment::None, InlineAttachment::Previous, InlineAttachment::None];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(text.as_str(), &c, &roles, &e, SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b|
SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &a).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.virtual_boundary_after_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_attached_inline_inter_char_boundaries_punctuation_western_trailing_narrow_not_cjk_punct() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunct",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunct", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunct");
        let text = "a,中".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "a", "latin", 8.0f64, Some("a".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), ",", "latin", 8.0f64, Some(",".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "中", "cjk", 16.0f64, Some("中".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let roles = vec![FontRole::LatinText, FontRole::CjkPunctuation, FontRole::CjkText];
        let e = vec![
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Narrow, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
];
        let a = vec![InlineAttachment::None, InlineAttachment::Previous, InlineAttachment::None];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(text.as_str(), &c, &roles, &e, SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b|
SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &a).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.virtual_boundary_after_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_infix_numeric_separator_not_decimal_mark() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMark",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMark", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMark");
        let text = "1，".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "1", "latin", 8.0f64, Some("1".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), "，", "cjk", 16.0f64, Some("，".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::LatinText, FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_decimal_mark_after_non_space() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpace", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpace",
|| {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpace");
        let text = "1,，2".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "1", "latin", 8.0f64, Some("1".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), ",", "latin", 8.0f64, Some(",".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "，", "cjk", 16.0f64, Some("，".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(3u32, 4u32).unwrap(), "2", "latin", 8.0f64, Some("2".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let roles = vec![FontRole::LatinText, FontRole::LatinText, FontRole::CjkPunctuation, FontRole::LatinText];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &roles, &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_quote_direction_final() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithQuoteDirectionFinal", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithQuoteDirectionFinal", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithQuoteDirectionFinal");
        let text = "中”中".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineStart" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_quote_direction_initial() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithQuoteDirectionInitial", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithQuoteDirectionInitial", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithQuoteDirectionInitial");
        let text = "中“中".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineEnd" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_quote_direction_unresolved() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolved", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolved",
|| {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolved");
        let text = "中«中".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineEnd" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_word_apostrophe2019() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithWordApostrophe2019", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithWordApostrophe2019", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithWordApostrophe2019");
        let text = "it’s".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_latin_word_code_point() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithLatinWordCodePoint", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithLatinWordCodePoint", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithLatinWordCodePoint");
        let text = "café".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_first_significant_code_point() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithFirstSignificantCodePoint",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithFirstSignificantCodePoint", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithFirstSignificantCodePoint");
        let text = "  “".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineEnd" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_last_significant_code_point() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithLastSignificantCodePoint", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithLastSignificantCodePoint",
|| {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithLastSignificantCodePoint");
        let text = "a”  ".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineStart" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_has_authored_break() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithHasAuthoredBreak", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithHasAuthoredBreak", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithHasAuthoredBreak");
        let text = concat!("\n",
"“").to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_next_content_cluster() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithNextContentCluster", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithNextContentCluster", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithNextContentCluster");
        let text = "a”中".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.unbreakable_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_previous_content_cluster_has_content() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContent",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContent", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContent");
        let text = "中”".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.unbreakable_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_close_punctuation() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithClosePunctuation", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithClosePunctuation", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithClosePunctuation");
        let text = "中）".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_exclamation_class() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithExclamationClass", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithExclamationClass", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithExclamationClass");
        let text = "中！".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_close_parenthesis_class() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithCloseParenthesisClass", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithCloseParenthesisClass", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithCloseParenthesisClass");
        let text = "中）".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if u32::from_ne_bytes((u_string::find_from(&((r.decisions[usize::try_from(i).unwrap_or(0)]).clone().reason).to_string(), "LB13", 0)).to_ne_bytes()) <= 2147483647 {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_infix_numeric_separator_rule() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRule",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRule", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRule");
        let text = "1,2".to_string();
        let c = UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c,
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()), &vec![]).unwrap();
        let mut hit: Option<ContextualKinsokuDecisionInfo> = None;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().source_text.to_string() == "," {
                hit = Some((r.decisions[usize::try_from(i).unwrap_or(0)]).clone());
                break;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_not_null_rendered(hit != None, match &(hit) { None => "null".to_string(), Some(__option11) => __option11.to_string() }.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(match &(hit) { Some(__option13) => u32::from_ne_bytes((u_string::find_from(&(__option13.reason).to_string(), "LB15d", 0)).to_ne_bytes()) <= 2147483647, None => false }, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_rule_for_line_start_else() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithRuleForLineStartElse", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithRuleForLineStartElse", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithRuleForLineStartElse");
        let text = "中、".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_attached_inline_inter_char_boundaries_with_sino_western_pair() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesWithSinoWesternPair", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesWithSinoWesternPair", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineInterCharBoundariesWithSinoWesternPair");
        let text = "中，中".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "中", "cjk", 16.0f64, Some("中".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), "，", "cjk", 16.0f64, Some("，".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "中", "cjk", 16.0f64, Some("中".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let roles = vec![FontRole::CjkText, FontRole::CjkPunctuation, FontRole::CjkText];
        let e = vec![
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Narrow, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Wide, false)).clone(),
];
        let a = vec![InlineAttachment::None, InlineAttachment::Previous, InlineAttachment::None];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(text.as_str(), &c, &roles, &e, SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b|
SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &a).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.virtual_sino_western_boundary_after_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_code_point_before_supplementary() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementary",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementary", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementary");
        let text = "中”".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_empty_range() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithEmptyRange", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithEmptyRange", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithEmptyRange");
        let text = "中".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 0u32).unwrap(), "", "cjk", 0.0f64, Some("".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "中", "cjk", 16.0f64, Some("中".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::CjkText, FontRole::CjkText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_first_code_point_length() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithFirstCodePointLength", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithFirstCodePointLength", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithFirstCodePointLength");
        let text = "中".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_cjk_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_is_whitespace_code_point() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithIsWhitespaceCodePoint", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithIsWhitespaceCodePoint", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithIsWhitespaceCodePoint");
        let text = " “".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineEnd" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_follows_authored_boundary_mandatory() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatory",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatory", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatory");
        let text = "\r“".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_follows_authored_boundary_zwsp() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZWSP",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZWSP", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZWSP");
        let text = "​“".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_decimal_mark_following_inside_digit() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigit",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigit", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigit");
        let text = " 1，23".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), " ", "latin", 8.0f64, Some(" ".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), "1", "latin", 8.0f64, Some("1".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "，", "cjk", 16.0f64, Some("，".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(3u32, 5u32).unwrap(), "23", "latin", 8.0f64, Some("23".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let roles = vec![FontRole::LatinText, FontRole::LatinText, FontRole::CjkPunctuation, FontRole::LatinText];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &roles, &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_decimal_mark_following_outside_digit() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigit",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigit", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigit");
        let text = " a，2".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), " ", "latin", 8.0f64, Some(" ".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), "a", "latin", 8.0f64, Some("a".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "，", "cjk", 16.0f64, Some("，".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(3u32, 4u32).unwrap(), "2", "latin", 8.0f64, Some("2".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let roles = vec![FontRole::LatinText, FontRole::LatinText, FontRole::LatinText, FontRole::LatinText];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &roles, &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0) ||
(i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_previous_content_cluster_has_authored_break() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreak",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreak", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreak");
        let text = concat!("\n",
"（").to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()) == 0, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_next_content_cluster_has_authored_break() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreak",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreak", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreak");
        let text = concat!("”\n",
"").to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((r.unbreakable_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_is_whitespace_code_point_non_bmp() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmp",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmp", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmp");
        let text = UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]);
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 2u32).unwrap(), text.as_str(), "latin", 8.0f64, Some(text.to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_has_authored_break_both() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithHasAuthoredBreakBoth", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithHasAuthoredBreakBoth", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithHasAuthoredBreakBoth");
        let text = concat!("\n",
"“\n",
"").to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_attached_inline_inter_char_boundaries_with_both_cjk_punctuation() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesWithBothCjkPunctuation", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesWithBothCjkPunctuation",
|| {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineInterCharBoundariesWithBothCjkPunctuation");
        let text = "、。中".to_string();
        let clusters = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "、", "cjk", 16.0f64, Some("、".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), "。", "cjk", 16.0f64, Some("。".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "中", "cjk", 16.0f64, Some("中".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let roles = vec![FontRole::CjkPunctuation, FontRole::CjkPunctuation, FontRole::CjkText];
        let edges = vec![
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
];
        let result = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(text.as_str(), &clusters, &roles, &edges, SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b|
SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &vec![InlineAttachment::None, InlineAttachment::Previous, InlineAttachment::None]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((result.virtual_boundary_after_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_follows_authored_boundary_whitespace() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespace",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespace", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespace");
        let text = " “".to_string();
        let result = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((result.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_follows_authored_boundary_whitespace_then_non_whitespace() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespace",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespace", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespace");
        let text = " A“".to_string();
        let result = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((result.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0) ||
(i32::from_ne_bytes((u32::from_ne_bytes((result.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_previous_content_cluster_empty() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithPreviousContentClusterEmpty",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithPreviousContentClusterEmpty", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithPreviousContentClusterEmpty");
        let text = "“".to_string();
        let clusters = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "“", "latin", 16.0f64, Some("“".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let result = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &clusters, &vec![FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((result.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0) ||
(i32::from_ne_bytes((u32::from_ne_bytes((result.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0) || (i32::from_ne_bytes((u32::try_from((result.unbreakable_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_next_content_cluster_empty() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithNextContentClusterEmpty", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithNextContentClusterEmpty",
|| {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithNextContentClusterEmpty");
        let text = "a”b".to_string();
        let result = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((result.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0) || (i32::from_ne_bytes((u32::try_from((result.unbreakable_ranges.len()) &
0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_code_point_at_or_null_surrogate() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogate",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogate", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogate");
        let text = format!("{}{}",
            "“",
            UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832])
        );
        let clusters = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "“", "latin", 16.0f64, Some("“".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 3u32).unwrap(), UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]).as_str(), "emoji", 16.0f64,
Some(UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832])), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let result = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &clusters, &vec![FontRole::LatinText, FontRole::Emoji], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((result.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0) ||
(i32::from_ne_bytes((u32::from_ne_bytes((result.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_has_authored_break_mandatory_only() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnly",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnly", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnly");
        let text = "\r“".to_string();
        let result = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((result.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_code_point_before_surrogate_pair() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePair",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePair", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePair");
        let text = format!("{}{}",
            UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]),
            "”"
        );
        let clusters = vec![
    (Cluster::new(TextRange::new(0u32, 2u32).unwrap(), UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]).as_str(), "emoji", 16.0f64,
Some(UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832])), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "”", "latin", 16.0f64, Some("”".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let result = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &clusters, &vec![FontRole::Emoji, FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((result.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0) ||
(i32::from_ne_bytes((u32::from_ne_bytes((result.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_with_code_point_at_or_null_supplementary() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementary",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementary", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementary");
        let text = format!("{}{}",
            UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]),
            "“"
        );
        let clusters = vec![
    (Cluster::new(TextRange::new(0u32, 2u32).unwrap(), UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]).as_str(), "emoji", 16.0f64,
Some(UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832])), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "“", "latin", 16.0f64, Some("“".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let result = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &clusters, &vec![FontRole::Emoji, FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((result.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0) ||
(i32::from_ne_bytes((u32::from_ne_bytes((result.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_attached_inline_virtual_boundaries_at_start() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineVirtualBoundariesAtStart", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineVirtualBoundariesAtStart", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineVirtualBoundariesAtStart");
        let result = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_virtual_boundaries(&vec![InlineAttachment::Previous, InlineAttachment::None]);
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((result.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn resolve_attached_inline_inter_char_boundaries_requires_matching_edges_size() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSize",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSize", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSize");
        let clusters = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "a", "latin", 8.0f64, Some("a".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let edges = vec![
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
];
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let clusters = (clusters).clone(); let edges = (edges).clone(); Arc::new(move || {
        UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(&"a", &clusters, &vec![FontRole::LatinText], &edges, SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b|
SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &vec![InlineAttachment::None]).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn resolve_attached_inline_inter_char_boundaries_punctuation_western_leading_not_narrow() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrow",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrow", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrow");
        let text = ",中a".to_string();
        let clusters = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), ",", "cjk", 16.0f64, Some(",".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), "中", "cjk", 16.0f64, Some("中".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "a", "latin", 8.0f64, Some("a".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let edges = vec![
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
];
        let result = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(text.as_str(), &clusters, &vec![FontRole::CjkPunctuation, FontRole::CjkText, FontRole::LatinText], &edges,
SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &vec![InlineAttachment::None, InlineAttachment::Previous,
InlineAttachment::None]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((result.virtual_boundary_after_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_attached_inline_inter_char_boundaries_all_conditions_false() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesAllConditionsFalse", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesAllConditionsFalse", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineInterCharBoundariesAllConditionsFalse");
        let text = "a*b".to_string();
        let clusters = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "a", "latin", 8.0f64, Some("a".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), "*", "latin", 8.0f64, Some("*".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "b", "latin", 8.0f64, Some("b".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let edges = vec![
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
];
        let result = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(text.as_str(), &clusters, &vec![FontRole::LatinText, FontRole::LatinText, FontRole::LatinText], &edges,
SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &vec![InlineAttachment::None, InlineAttachment::Previous,
InlineAttachment::None]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((result.virtual_boundary_after_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_attached_inline_inter_char_boundaries_narrow_narrow_pair() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesNarrowNarrowPair", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesNarrowNarrowPair", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineInterCharBoundariesNarrowNarrowPair");
        let text = "a*b".to_string();
        let clusters = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "a", "latin", 8.0f64, Some("a".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), "*", "latin", 8.0f64, Some("*".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "b", "latin", 8.0f64, Some("b".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let edges = vec![
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Narrow, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Wide, false)).clone(),
];
        let result = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(text.as_str(), &clusters, &vec![FontRole::LatinText, FontRole::LatinText, FontRole::LatinText], &edges,
SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &vec![InlineAttachment::None, InlineAttachment::Previous,
InlineAttachment::None]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((result.virtual_boundary_after_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_infix_numeric_separator_with_space_and_no_space() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpace",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpace", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpace");
        let mut t = " .5".to_string();
        let mut r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(t.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
        t = " 1.5".to_string();
        r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t.as_str()).unwrap(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(t.as_str()), &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(r.forbidden_line_start_clusters.has(&(2)), None).unwrap();
        let mut found = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().reason.to_string() == "Uax14WesternPunctuationBoundary:LB15d" {
                found = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(found, None).unwrap();
        t = ".5".to_string();
        r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t.as_str()).unwrap(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(t.as_str()), &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_decimal_mark_following_variations() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesDecimalMarkFollowingVariations",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesDecimalMarkFollowingVariations", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesDecimalMarkFollowingVariations");
        let mut t = ".".to_string();
        let mut c = vec![
    (Cluster::new(TextRange::new(0u32, 0u32).unwrap(), "", "latin", 0.0f64, Some("".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), ".", "latin", 8.0f64, Some(".".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let mut r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(), &c, &vec![FontRole::LatinText, FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
        t = "a .#".to_string();
        r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t.as_str()).unwrap(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(t.as_str()), &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(r.forbidden_line_start_clusters.has(&(2)), None).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().reason.to_string() == "Uax14WesternPunctuationBoundary:LB15d" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
        t = "a .a".to_string();
        r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t.as_str()).unwrap(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(t.as_str()), &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(r.forbidden_line_start_clusters.has(&(2)), None).unwrap();
        f = false;
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().reason.to_string() == "Uax14WesternPunctuationBoundary:LB15d" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
        t = " .5".to_string();
        c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), " ", "latin", 8.0f64, Some(" ".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 3u32).unwrap(), ".5", "latin", 16.0f64, Some(".5".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(), &c, &vec![FontRole::LatinText, FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_apostrophe_and_latin_word_branches() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesApostropheAndLatinWordBranches",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesApostropheAndLatinWordBranches", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesApostropheAndLatinWordBranches");
        let mut t = "a’ ".to_string();
        let mut r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(t.as_str()),
&vec![]).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineStart" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
        t = " ’a".to_string();
        r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t.as_str()).unwrap(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(t.as_str()), &vec![]).unwrap();
        f = false;
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineEnd" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
        t = "À’ɏ".to_string();
        r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t.as_str()).unwrap(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(t.as_str()), &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
        t = "¿’中".to_string();
        r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t.as_str()).unwrap(),
&vec![FontRole::LatinText, FontRole::LatinText, FontRole::CjkText], &vec![]).unwrap();
        f = false;
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineStart" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_surrogate_scanning_variations() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesSurrogateScanningVariations", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesSurrogateScanningVariations",
|| {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesSurrogateScanningVariations");
        let strings = vec![
    "a".to_string(),
    UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]).clone(),
    "中".to_string(),
    "hello".to_string(),
];
        let mut si = 0u32;
        while (i32::from_ne_bytes((si).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((strings.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let s = (strings[usize::try_from(si).unwrap_or(0)]).clone();
            let t = format!("{}{}{}",
            " ",
            s,
            ")"
        );
            let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), " ", "latin", 8.0f64, Some(" ".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, u32::wrapping_add(1, u_string::unit_count(&(s)))).unwrap(), s.as_str(), "latin", 16.0f64, Some(s.to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(u32::wrapping_add(1, u_string::unit_count(&(s))), u32::wrapping_add(2, u_string::unit_count(&(s)))).unwrap(), ")", "latin", 8.0f64, Some(")".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
            let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(), &c, &vec![FontRole::LatinText, FontRole::LatinText, FontRole::LatinText], &vec![]).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0) || (i32::from_ne_bytes((u32::try_from((r.decisions.len()) &
0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0) || (i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_end_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0) || u32::try_from((r.unbreakable_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 ||
(i32::from_ne_bytes((u32::try_from((r.unbreakable_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
            si = u32::wrapping_add(si, 1);
        }
        let strings2 = vec![
    "a’".to_string(),
    format!("{}{}",
            UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]),
            "’"
        ).clone(),
    "中’".to_string(),
];
        si = 0u32;
        while (i32::from_ne_bytes((si).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((strings2.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let s = (strings2[usize::try_from(si).unwrap_or(0)]).clone();
            let t = format!("{}{}{}",
            " ",
            s,
            " "
        );
            let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), " ", "latin", 8.0f64, Some(" ".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, u32::wrapping_add(1, u_string::unit_count(&(s)))).unwrap(), s.as_str(), "latin", 16.0f64, Some(s.to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(u32::wrapping_add(1, u_string::unit_count(&(s))), u32::wrapping_add(2, u_string::unit_count(&(s)))).unwrap(), " ", "latin", 8.0f64, Some(" ".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
            let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(), &c, &vec![FontRole::LatinText, FontRole::LatinText, FontRole::LatinText], &vec![]).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
            si = u32::wrapping_add(si, 1);
        }
        let mut t = format!("{}{}{}",
            " ",
            UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]),
            ".5"
        );
        let mut c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), " ", "latin", 8.0f64, Some(" ".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 3u32).unwrap(), UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]).as_str(), "emoji", 16.0f64,
Some(UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832])), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(3u32, 5u32).unwrap(), ".5", "latin", 16.0f64, Some(".5".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let mut r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(), &c, &vec![FontRole::LatinText, FontRole::Emoji, FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(r.forbidden_line_start_clusters.has(&(2)), None).unwrap();
        t = "(​a".to_string();
        c = UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t.as_str()).unwrap();
        r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(), &c, &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(t.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(r.forbidden_line_end_clusters.has(&(0)), None).unwrap();
        t = concat!("(\n",
"a").to_string();
        c = UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t.as_str()).unwrap();
        r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(), &c, &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(t.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(r.forbidden_line_end_clusters.has(&(0)), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_is_decimal_mark_after_space_index_zero() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZero",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZero", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZero");
        let t = ".5".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(t.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_is_decimal_mark_after_space_non_whitespace_prev() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrev",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrev", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrev");
        let t = "1，2".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(t.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_is_decimal_mark_after_space_empty_prev() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrev",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrev", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrev");
        let t = "a，5".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(t.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_follows_authored_boundary_non_whitespace() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespace",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespace", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespace");
        let t = "a“".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(t.as_str()),
&vec![]).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineEnd" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_previous_content_cluster_returns_content() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContent",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContent", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContent");
        let t = "a ”".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(t.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.unbreakable_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_previous_content_cluster_empty_only() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnly",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnly", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnly");
        let t = "“".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 0u32).unwrap(), "", "latin", 0.0f64, Some("".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "“", "latin", 16.0f64, Some("“".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(), &c, &vec![FontRole::LatinText, FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_end_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_next_content_cluster_returns_content() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesNextContentClusterReturnsContent",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesNextContentClusterReturnsContent", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesNextContentClusterReturnsContent");
        let t = ")”中".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), ")", "latin", 8.0f64, Some(")".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), "”", "latin", 8.0f64, Some("”".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "中", "cjk", 16.0f64, Some("中".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(), &c, &vec![FontRole::LatinText, FontRole::LatinText, FontRole::CjkText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.unbreakable_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_has_authored_break_with_code_point() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePoint",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePoint", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePoint");
        let t = concat!("a\n",
"“").to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(t.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()) == 0, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_has_authored_break_null_code_point() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePoint",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePoint", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePoint");
        let t = "“".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(t.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_end_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_first_code_point_length_bmp() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesFirstCodePointLengthBmp", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesFirstCodePointLengthBmp", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesFirstCodePointLengthBmp");
        let t = "a“".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(t.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(t.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_first_code_point_length_surrogate() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogate",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogate", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogate");
        let t = format!("{}{}",
            UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]),
            "“"
        );
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 2u32).unwrap(), UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]).as_str(), "emoji", 16.0f64,
Some(UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832])), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "“", "latin", 16.0f64, Some("“".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(), &c, &vec![FontRole::Emoji, FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_code_point_at_or_null_surrogate_pair() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePair",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePair", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePair");
        let t = format!("{}{}",
            UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]),
            "“"
        );
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 2u32).unwrap(), UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]).as_str(), "emoji", 16.0f64,
Some(UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832])), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "“", "latin", 16.0f64, Some("“".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(t.as_str(), &c, &vec![FontRole::Emoji, FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_code_point_before_surrogate_pair() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePair", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePair",
|| {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePair");
        let text = UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]);
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 2u32).unwrap(), text.as_str(), "emoji", 16.0f64, Some(text.to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::Emoji], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_code_point_before_low_surrogate() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogate", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogate",
|| {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogate");
        let text = format!("{}{}",
            "a",
            UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832])
        );
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "a", "latin", 8.0f64, Some("a".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 3u32).unwrap(), UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]).as_str(), "emoji", 16.0f64,
Some(UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832])), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::LatinText, FontRole::Emoji], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_quote_direction2019_surrogate_left() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeft",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeft", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeft");
        let text = format!("{}{}",
            UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]),
            "’"
        );
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 2u32).unwrap(), UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]).as_str(), "emoji", 16.0f64,
Some(UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832])), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "’", "latin", 8.0f64, Some("’".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::Emoji, FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_previous_content_cluster_multiple_empty() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmpty",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmpty", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmpty");
        let text = "a ”".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 0u32).unwrap(), "", "latin", 0.0f64, Some("".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), " ", "latin", 8.0f64, Some(" ".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "”", "latin", 8.0f64, Some("”".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::LatinText, FontRole::LatinText, FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_follows_authored_boundary_zwsp_in_middle() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZWSPInMiddle",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZWSPInMiddle", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZWSPInMiddle");
        let text = " ​“".to_string();
        let c = UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c,
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()), &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_last_significant_code_point_surrogate_ending() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEnding",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEnding", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEnding");
        let text = format!("{}{}",
            UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]),
            "”"
        );
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 2u32).unwrap(), UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]).as_str(), "emoji", 16.0f64,
Some(UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832])), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "”", "latin", 8.0f64, Some("”".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::Emoji, FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_end_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0) || (i32::from_ne_bytes((u32::try_from((r.unbreakable_ranges.len()) &
0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_is_decimal_mark_after_space_following_inside() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInside",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInside", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInside");
        let text = "a .5".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "a", "latin", 8.0f64, Some("a".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), " ", "latin", 8.0f64, Some(" ".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 4u32).unwrap(), ".5", "latin", 16.0f64, Some(".5".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::LatinText, FontRole::LatinText, FontRole::CjkPunctuation], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundary_full_width_comma_after_space_stays_forbidden() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbidden",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbidden", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbidden");
        let text = "a ，5".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "a", "latin", 8.0f64, Some("a".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), " ", "latin", 8.0f64, Some(" ".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 4u32).unwrap(), "，5", "latin", 16.0f64, Some("，5".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::LatinText, FontRole::LatinText, FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_is_decimal_mark_after_space_following_outside() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutside",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutside", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutside");
        let text = "a .5".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "a", "latin", 8.0f64, Some("a".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), " ", "latin", 8.0f64, Some(" ".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), ".", "latin", 8.0f64, Some(".".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(3u32, 4u32).unwrap(), "5", "latin", 8.0f64, Some("5".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::LatinText, FontRole::LatinText, FontRole::CjkPunctuation, FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_quote_direction2019_bmp_left() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeft", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeft", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeft");
        let text = "A’".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0) ||
(i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_quote_direction2019_right_word_only() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnly",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnly", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnly");
        let text = " ’a".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineEnd" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_quote_direction2019_left_word_only() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnly",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnly", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnly");
        let text = "a’ ".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineStart" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_quote_direction2019_neither_word() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWord",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWord", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWord");
        let text = "!’!".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let mut f = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if r.decisions[usize::try_from(i).unwrap_or(0)].clone().forbidden_position.to_string() == "LineStart" {
                f = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(f, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_code_point_before_low_surrogate_single() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingle",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingle", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingle");
        let text = UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]);
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 2u32).unwrap(), text.as_str(), "emoji", 16.0f64, Some(text.to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::Emoji], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_code_point_at_or_null_supplementary() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementary",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementary", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementary");
        let text = UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]);
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 2u32).unwrap(), text.as_str(), "emoji", 16.0f64, Some(text.to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::Emoji], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_has_authored_break_empty_string() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyString", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyString",
|| {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyString");
        let text = "“".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::from_ne_bytes((r.forbidden_line_end_clusters.size()).to_ne_bytes())).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_attached_inline_inter_char_boundaries_virtual_from_cjk_punctuation_left() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeft",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeft", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeft");
        let text = "，x汉".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "，", "cjk", 16.0f64, Some("，".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), "x", "latin", 8.0f64, Some("x".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "汉", "cjk", 16.0f64, Some("汉".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let e = vec![
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Wide, false)).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(text.as_str(), &c, &vec![FontRole::CjkPunctuation, FontRole::LatinText, FontRole::CjkText], &e,
SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &vec![InlineAttachment::None, InlineAttachment::Previous,
InlineAttachment::None]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"{1=0}", UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_map_text((r.virtual_boundary_after_clusters).clone()).as_str(), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_decimal_mark_at_cluster_zero_forbidden() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbidden",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbidden", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbidden");
        let text = "a.5".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(1u32, 3u32).unwrap(), ".5", "latin", 16.0f64, Some(".5".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_decimal_mark_after_letter_cluster_forbidden() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbidden",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbidden", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbidden");
        let text = "a.5".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "a", "latin", 8.0f64, Some("a".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 3u32).unwrap(), ".5", "latin", 16.0f64, Some(".5".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::LatinText, FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_decimal_mark_followed_by_letter_forbidden() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbidden",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbidden", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbidden");
        let text = "a .x".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_decimal_mark_alone_after_space_forbidden() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbidden",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbidden", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbidden");
        let text = "a .".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_astral_tail_keeps_pair_as_last_significant() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificant",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificant", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificant");
        let text = format!("{}{}",
            "a .",
            UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832])
        );
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "a", "latin", 8.0f64, Some("a".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), " ", "latin", 8.0f64, Some(" ".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 5u32).unwrap(), format!("{}{}",
            ".",
            UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832])
        ).as_str(), "latin", 16.0f64, Some(format!("{}{}",
            ".",
            UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832])
        )), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::LatinText, FontRole::LatinText, FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_authored_break_inside_previous_cluster_drops_unbreakable() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakable",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakable", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakable");
        let text = concat!("a\n",
"b，").to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 3u32).unwrap(), concat!("a\n",
"b"), "latin", 24.0f64, Some(concat!("a\n",
"b").to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(3u32, 4u32).unwrap(), "，", "latin", 16.0f64, Some("，".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::LatinText, FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((r.unbreakable_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_apostrophe_at_text_start_no_left_context() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContext",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContext", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContext");
        let text = "’s".to_string();
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(),
&UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_clusters(text.as_str()).unwrap(), &UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_latin_roles(text.as_str()),
&vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_apostrophe_right_neighbour_unpaired_high_surrogate() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogate",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogate", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogate");
        let text = UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![8217, 55296, 20013]);
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 3u32).unwrap(), text.as_str(), "latin", 24.0f64, Some(text.to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_apostrophe_left_neighbour_supplementary_pair() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPair",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPair", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPair");
        let text = format!("{}{}",
            UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]),
            "’"
        );
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 2u32).unwrap(), UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832]).as_str(), "emoji", 16.0f64,
Some(UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832])), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "’", "latin", 16.0f64, Some("’".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::Emoji, FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn resolve_attached_inline_inter_char_boundaries_punctuation_western_leading_narrow_only() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnly",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnly", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnly");
        let text = "，xa".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "，", "cjk", 16.0f64, Some("，".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), "x", "latin", 8.0f64, Some("x".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "a", "latin", 8.0f64, Some("a".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let e = vec![
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Other, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Other, false)).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(text.as_str(), &c, &vec![FontRole::CjkPunctuation, FontRole::LatinText, FontRole::LatinText], &e,
SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &vec![InlineAttachment::None, InlineAttachment::Previous,
InlineAttachment::None]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"{1=0}", UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_map_text((r.virtual_boundary_after_clusters).clone()).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes((r.virtual_sino_western_boundary_after_clusters.size()).to_ne_bytes()) == 0, None).unwrap();
    });
}

#[test]
fn resolve_attached_inline_inter_char_boundaries_punctuation_western_trailing_narrow_only() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnly",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnly", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnly");
        let text = "xa，".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "x", "latin", 8.0f64, Some("x".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), "a", "latin", 8.0f64, Some("a".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "，", "cjk", 16.0f64, Some("，".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let e = vec![
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Other, EastAsianSpacingValue::Narrow, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other, false)).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(text.as_str(), &c, &vec![FontRole::LatinText, FontRole::LatinText, FontRole::CjkPunctuation], &e,
SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &vec![InlineAttachment::None, InlineAttachment::Previous,
InlineAttachment::None]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"{1=0}", UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_map_text((r.virtual_boundary_after_clusters).clone()).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes((r.virtual_sino_western_boundary_after_clusters.size()).to_ne_bytes()) == 0, None).unwrap();
    });
}

#[test]
fn resolve_attached_inline_inter_char_boundaries_sino_western_only() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesSinoWesternOnly", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesSinoWesternOnly", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineInterCharBoundariesSinoWesternOnly");
        let text = "汉xa".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "汉", "cjk", 16.0f64, Some("汉".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), "x", "latin", 8.0f64, Some("x".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "a", "latin", 8.0f64, Some("a".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let e = vec![
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Wide, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Narrow, EastAsianSpacingValue::Other, false)).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(text.as_str(), &c, &vec![FontRole::CjkText, FontRole::LatinText, FontRole::LatinText], &e,
SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &vec![InlineAttachment::None, InlineAttachment::Previous,
InlineAttachment::None]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"{1=0}", UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_map_text((r.virtual_boundary_after_clusters).clone()).as_str(), None).unwrap();
        let mut expected: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        expected.put(&(1));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set(expected.clone().build(), (r.virtual_sino_western_boundary_after_clusters).clone(), None).unwrap();
    });
}

#[test]
fn resolve_attached_inline_inter_char_boundaries_western_bracket_only() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesWesternBracketOnly", "org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveAttachedInlineInterCharBoundariesWesternBracketOnly", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveAttachedInlineInterCharBoundariesWesternBracketOnly");
        let text = "(x汉".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "(", "latin", 8.0f64, Some("(".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), "x", "latin", 8.0f64, Some("x".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32).unwrap(), "汉", "cjk", 16.0f64, Some("汉".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let e = vec![
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other, false)).clone(),
    (EastAsianSpacingEdges::new(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other, false)).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_attached_inline_inter_char_boundaries(text.as_str(), &c, &vec![FontRole::LatinText, FontRole::LatinText, FontRole::CjkText], &e,
SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build(), &vec![InlineAttachment::None, InlineAttachment::Previous,
InlineAttachment::None]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"{1=0}", UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_map_text((r.virtual_boundary_after_clusters).clone()).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes((r.virtual_sino_western_boundary_after_clusters.size()).to_ne_bytes()) == 0, None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_decimal_mark_after_empty_cluster_forbidden() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbidden",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbidden", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbidden");
        let text = "a.5".to_string();
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), "a", "latin", 8.0f64, Some("a".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 1u32).unwrap(), "", "latin", 0.0f64, Some("".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 3u32).unwrap(), ".5", "latin", 16.0f64, Some(".5".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::LatinText, FontRole::LatinText, FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::from_ne_bytes((r.forbidden_line_start_clusters.size()).to_ne_bytes()), None).unwrap();
    });
}

#[test]
fn resolve_unicode_punctuation_boundaries_apostrophe_right_neighbour_supplementary_pair() {
    testlib::run("org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPair",
"org.tiqian.layout.UnicodePunctuationBoundaryResolverCoverageTest.resolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPair", || {
        UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_start(&"resolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPair");
        let text = format!("{}{}",
            "’",
            UnicodePunctuationBoundaryResolverCoverageSupport::unicode_punctuation_boundary_resolver_coverage_support_surrogate(&vec![55357, 56832])
        );
        let c = vec![
    (Cluster::new(TextRange::new(0u32, 3u32).unwrap(), text.as_str(), "latin", 32.0f64, Some(text.to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let r = UnicodePunctuationBoundaryResolver::unicode_punctuation_boundary_resolver_resolve_unicode_punctuation_boundaries(text.as_str(), &c, &vec![FontRole::LatinText], &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((r.decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}
