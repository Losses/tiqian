#![cfg(test)]

use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::font_role_context::FontRoleContext;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::quote_pair_analyzer::QuoteRoleDecision;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverNestedAndSurrogateTestTabBeforeAWhollyWesternPairDelimitsLikeASpaceFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestTabBeforeAWhollyWesternPairDelimitsLikeASpaceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestTabBeforeAWhollyWesternPairDelimitsLikeASpaceFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestTabBeforeAWhollyWesternPairDelimitsLikeASpaceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestTabBeforeAWhollyWesternPairDelimitsLikeASpaceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestTabBeforeAWhollyWesternPairDelimitsLikeASpaceFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestTabBeforeAWhollyWesternPairDelimitsLikeASpaceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestTabBeforeAWhollyWesternPairDelimitsLikeASpaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestTabBeforeAWhollyWesternPairDelimitsLikeASpaceFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestTabBeforeAWhollyWesternPairDelimitsLikeASpaceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestTabBeforeAWhollyWesternPairDelimitsLikeASpaceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestTabBeforeAWhollyWesternPairDelimitsLikeASpaceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverNestedAndSurrogateTestTabBeforeAWhollyWesternPairDelimitsLikeASpaceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestTabBeforeAWhollyWesternPairDelimitsLikeASpaceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestTabBeforeAWhollyWesternPairDelimitsLikeASpaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestTabBeforeAWhollyWesternPairDelimitsLikeASpaceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeUnmatchedQuoteWithCjkRightSkipsTheDelimitedRuleFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeUnmatchedQuoteWithCjkRightSkipsTheDelimitedRuleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeUnmatchedQuoteWithCjkRightSkipsTheDelimitedRuleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeUnmatchedQuoteWithCjkRightSkipsTheDelimitedRuleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeUnmatchedQuoteWithCjkRightSkipsTheDelimitedRuleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeUnmatchedQuoteWithCjkRightSkipsTheDelimitedRuleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeUnmatchedQuoteWithCjkRightSkipsTheDelimitedRuleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeUnmatchedQuoteWithCjkRightSkipsTheDelimitedRuleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeUnmatchedQuoteWithCjkRightSkipsTheDelimitedRuleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeUnmatchedQuoteWithCjkRightSkipsTheDelimitedRuleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeUnmatchedQuoteWithCjkRightSkipsTheDelimitedRuleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeUnmatchedQuoteWithCjkRightSkipsTheDelimitedRuleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeUnmatchedQuoteWithCjkRightSkipsTheDelimitedRuleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeUnmatchedQuoteWithCjkRightSkipsTheDelimitedRuleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeUnmatchedQuoteWithCjkRightSkipsTheDelimitedRuleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeUnmatchedQuoteWithCjkRightSkipsTheDelimitedRuleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAPairWithNonWesternContentSkipsTheDelimitedRuleFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAPairWithNonWesternContentSkipsTheDelimitedRuleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAPairWithNonWesternContentSkipsTheDelimitedRuleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAPairWithNonWesternContentSkipsTheDelimitedRuleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAPairWithNonWesternContentSkipsTheDelimitedRuleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAPairWithNonWesternContentSkipsTheDelimitedRuleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAPairWithNonWesternContentSkipsTheDelimitedRuleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAPairWithNonWesternContentSkipsTheDelimitedRuleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAPairWithNonWesternContentSkipsTheDelimitedRuleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAPairWithNonWesternContentSkipsTheDelimitedRuleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAPairWithNonWesternContentSkipsTheDelimitedRuleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAPairWithNonWesternContentSkipsTheDelimitedRuleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAPairWithNonWesternContentSkipsTheDelimitedRuleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAPairWithNonWesternContentSkipsTheDelimitedRuleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAPairWithNonWesternContentSkipsTheDelimitedRuleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAPairWithNonWesternContentSkipsTheDelimitedRuleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAMixedContentPairReportsMixedContentFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAMixedContentPairReportsMixedContentFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAMixedContentPairReportsMixedContentFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAMixedContentPairReportsMixedContentFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAMixedContentPairReportsMixedContentFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAMixedContentPairReportsMixedContentFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAMixedContentPairReportsMixedContentFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAMixedContentPairReportsMixedContentFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAMixedContentPairReportsMixedContentFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAMixedContentPairReportsMixedContentFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAMixedContentPairReportsMixedContentFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAMixedContentPairReportsMixedContentFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAMixedContentPairReportsMixedContentFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAMixedContentPairReportsMixedContentFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAMixedContentPairReportsMixedContentFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestSpaceBeforeAMixedContentPairReportsMixedContentFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverNestedAndSurrogateTestSiblingPairsInsideOneQuotationEachInheritTheOuterRoleFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestSiblingPairsInsideOneQuotationEachInheritTheOuterRoleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestSiblingPairsInsideOneQuotationEachInheritTheOuterRoleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestSiblingPairsInsideOneQuotationEachInheritTheOuterRoleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestSiblingPairsInsideOneQuotationEachInheritTheOuterRoleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestSiblingPairsInsideOneQuotationEachInheritTheOuterRoleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestSiblingPairsInsideOneQuotationEachInheritTheOuterRoleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestSiblingPairsInsideOneQuotationEachInheritTheOuterRoleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestSiblingPairsInsideOneQuotationEachInheritTheOuterRoleFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestSiblingPairsInsideOneQuotationEachInheritTheOuterRoleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestSiblingPairsInsideOneQuotationEachInheritTheOuterRoleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestSiblingPairsInsideOneQuotationEachInheritTheOuterRoleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverNestedAndSurrogateTestSiblingPairsInsideOneQuotationEachInheritTheOuterRoleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestSiblingPairsInsideOneQuotationEachInheritTheOuterRoleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestSiblingPairsInsideOneQuotationEachInheritTheOuterRoleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestSiblingPairsInsideOneQuotationEachInheritTheOuterRoleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseFollowerOfAHighSurrogateCountsAsOneUnitFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseFollowerOfAHighSurrogateCountsAsOneUnitFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseFollowerOfAHighSurrogateCountsAsOneUnitFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseFollowerOfAHighSurrogateCountsAsOneUnitFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseFollowerOfAHighSurrogateCountsAsOneUnitFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseFollowerOfAHighSurrogateCountsAsOneUnitFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseFollowerOfAHighSurrogateCountsAsOneUnitFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseFollowerOfAHighSurrogateCountsAsOneUnitFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseFollowerOfAHighSurrogateCountsAsOneUnitFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseFollowerOfAHighSurrogateCountsAsOneUnitFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseFollowerOfAHighSurrogateCountsAsOneUnitFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseFollowerOfAHighSurrogateCountsAsOneUnitFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseFollowerOfAHighSurrogateCountsAsOneUnitFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseFollowerOfAHighSurrogateCountsAsOneUnitFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseFollowerOfAHighSurrogateCountsAsOneUnitFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseFollowerOfAHighSurrogateCountsAsOneUnitFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseCharBeforeAQuoteFailsTheLowSurrogateRangeAboveFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseCharBeforeAQuoteFailsTheLowSurrogateRangeAboveFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseCharBeforeAQuoteFailsTheLowSurrogateRangeAboveFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseCharBeforeAQuoteFailsTheLowSurrogateRangeAboveFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseCharBeforeAQuoteFailsTheLowSurrogateRangeAboveFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseCharBeforeAQuoteFailsTheLowSurrogateRangeAboveFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseCharBeforeAQuoteFailsTheLowSurrogateRangeAboveFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseCharBeforeAQuoteFailsTheLowSurrogateRangeAboveFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseCharBeforeAQuoteFailsTheLowSurrogateRangeAboveFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseCharBeforeAQuoteFailsTheLowSurrogateRangeAboveFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseCharBeforeAQuoteFailsTheLowSurrogateRangeAboveFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseCharBeforeAQuoteFailsTheLowSurrogateRangeAboveFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseCharBeforeAQuoteFailsTheLowSurrogateRangeAboveFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseCharBeforeAQuoteFailsTheLowSurrogateRangeAboveFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseCharBeforeAQuoteFailsTheLowSurrogateRangeAboveFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestPrivateUseCharBeforeAQuoteFailsTheLowSurrogateRangeAboveFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverNestedAndSurrogateTestPlainFollowerOfAHighSurrogateCountsAsOneUnitFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestPlainFollowerOfAHighSurrogateCountsAsOneUnitFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestPlainFollowerOfAHighSurrogateCountsAsOneUnitFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestPlainFollowerOfAHighSurrogateCountsAsOneUnitFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestPlainFollowerOfAHighSurrogateCountsAsOneUnitFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestPlainFollowerOfAHighSurrogateCountsAsOneUnitFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestPlainFollowerOfAHighSurrogateCountsAsOneUnitFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestPlainFollowerOfAHighSurrogateCountsAsOneUnitFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestPlainFollowerOfAHighSurrogateCountsAsOneUnitFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestPlainFollowerOfAHighSurrogateCountsAsOneUnitFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ContextualQuoteRoleResolverNestedAndSurrogateTestPlainFollowerOfAHighSurrogateCountsAsOneUnitFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestPlainFollowerOfAHighSurrogateCountsAsOneUnitFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverNestedAndSurrogateTestPlainFollowerOfAHighSurrogateCountsAsOneUnitFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestPlainFollowerOfAHighSurrogateCountsAsOneUnitFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestPlainFollowerOfAHighSurrogateCountsAsOneUnitFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestPlainFollowerOfAHighSurrogateCountsAsOneUnitFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverNestedAndSurrogateTestNonChineseLocaleResolvesNeutralContextToLatinTextFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestNonChineseLocaleResolvesNeutralContextToLatinTextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestNonChineseLocaleResolvesNeutralContextToLatinTextFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestNonChineseLocaleResolvesNeutralContextToLatinTextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestNonChineseLocaleResolvesNeutralContextToLatinTextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestNonChineseLocaleResolvesNeutralContextToLatinTextFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestNonChineseLocaleResolvesNeutralContextToLatinTextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestNonChineseLocaleResolvesNeutralContextToLatinTextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestNonChineseLocaleResolvesNeutralContextToLatinTextFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestNonChineseLocaleResolvesNeutralContextToLatinTextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestNonChineseLocaleResolvesNeutralContextToLatinTextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestNonChineseLocaleResolvesNeutralContextToLatinTextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverNestedAndSurrogateTestNonChineseLocaleResolvesNeutralContextToLatinTextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestNonChineseLocaleResolvesNeutralContextToLatinTextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestNonChineseLocaleResolvesNeutralContextToLatinTextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestNonChineseLocaleResolvesNeutralContextToLatinTextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverNestedAndSurrogateTestNestedPairInsideNeutralEnclosingInheritsTheOuterQuotationFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestNestedPairInsideNeutralEnclosingInheritsTheOuterQuotationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestNestedPairInsideNeutralEnclosingInheritsTheOuterQuotationFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestNestedPairInsideNeutralEnclosingInheritsTheOuterQuotationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestNestedPairInsideNeutralEnclosingInheritsTheOuterQuotationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestNestedPairInsideNeutralEnclosingInheritsTheOuterQuotationFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestNestedPairInsideNeutralEnclosingInheritsTheOuterQuotationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestNestedPairInsideNeutralEnclosingInheritsTheOuterQuotationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestNestedPairInsideNeutralEnclosingInheritsTheOuterQuotationFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestNestedPairInsideNeutralEnclosingInheritsTheOuterQuotationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestNestedPairInsideNeutralEnclosingInheritsTheOuterQuotationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestNestedPairInsideNeutralEnclosingInheritsTheOuterQuotationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverNestedAndSurrogateTestNestedPairInsideNeutralEnclosingInheritsTheOuterQuotationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestNestedPairInsideNeutralEnclosingInheritsTheOuterQuotationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestNestedPairInsideNeutralEnclosingInheritsTheOuterQuotationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestNestedPairInsideNeutralEnclosingInheritsTheOuterQuotationFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverNestedAndSurrogateTestMixedEnclosingLevelFallsBackToParagraphLanguageFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestMixedEnclosingLevelFallsBackToParagraphLanguageFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestMixedEnclosingLevelFallsBackToParagraphLanguageFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestMixedEnclosingLevelFallsBackToParagraphLanguageFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestMixedEnclosingLevelFallsBackToParagraphLanguageFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestMixedEnclosingLevelFallsBackToParagraphLanguageFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestMixedEnclosingLevelFallsBackToParagraphLanguageFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestMixedEnclosingLevelFallsBackToParagraphLanguageFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestMixedEnclosingLevelFallsBackToParagraphLanguageFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestMixedEnclosingLevelFallsBackToParagraphLanguageFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestMixedEnclosingLevelFallsBackToParagraphLanguageFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestMixedEnclosingLevelFallsBackToParagraphLanguageFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverNestedAndSurrogateTestMixedEnclosingLevelFallsBackToParagraphLanguageFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestMixedEnclosingLevelFallsBackToParagraphLanguageFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestMixedEnclosingLevelFallsBackToParagraphLanguageFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestMixedEnclosingLevelFallsBackToParagraphLanguageFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverNestedAndSurrogateTestLeftwardScanFromALowSurrogateWalksEveryBacktrackArmFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestLeftwardScanFromALowSurrogateWalksEveryBacktrackArmFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestLeftwardScanFromALowSurrogateWalksEveryBacktrackArmFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestLeftwardScanFromALowSurrogateWalksEveryBacktrackArmFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestLeftwardScanFromALowSurrogateWalksEveryBacktrackArmFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestLeftwardScanFromALowSurrogateWalksEveryBacktrackArmFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestLeftwardScanFromALowSurrogateWalksEveryBacktrackArmFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestLeftwardScanFromALowSurrogateWalksEveryBacktrackArmFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestLeftwardScanFromALowSurrogateWalksEveryBacktrackArmFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestLeftwardScanFromALowSurrogateWalksEveryBacktrackArmFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ContextualQuoteRoleResolverNestedAndSurrogateTestLeftwardScanFromALowSurrogateWalksEveryBacktrackArmFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestLeftwardScanFromALowSurrogateWalksEveryBacktrackArmFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverNestedAndSurrogateTestLeftwardScanFromALowSurrogateWalksEveryBacktrackArmFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestLeftwardScanFromALowSurrogateWalksEveryBacktrackArmFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestLeftwardScanFromALowSurrogateWalksEveryBacktrackArmFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestLeftwardScanFromALowSurrogateWalksEveryBacktrackArmFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextualQuoteRoleResolverNestedAndSurrogateTestHighSurrogateAtTheContentEndHasNoRoomAndThrowsFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestHighSurrogateAtTheContentEndHasNoRoomAndThrowsFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestHighSurrogateAtTheContentEndHasNoRoomAndThrowsFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestHighSurrogateAtTheContentEndHasNoRoomAndThrowsFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestHighSurrogateAtTheContentEndHasNoRoomAndThrowsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestHighSurrogateAtTheContentEndHasNoRoomAndThrowsFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestHighSurrogateAtTheContentEndHasNoRoomAndThrowsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ContextualQuoteRoleResolverNestedAndSurrogateTestHighSurrogateAtTheContentEndHasNoRoomAndThrowsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: ContextualQuoteRoleResolverNestedAndSurrogateTestHighSurrogateAtTheContentEndHasNoRoomAndThrowsFault) -> Self {
        match value {
            ContextualQuoteRoleResolverNestedAndSurrogateTestHighSurrogateAtTheContentEndHasNoRoomAndThrowsFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ContextualQuoteRoleResolverNestedAndSurrogateTestHighSurrogateAtTheContentEndHasNoRoomAndThrowsFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestHighSurrogateAtTheContentEndHasNoRoomAndThrowsFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ContextualQuoteRoleResolverNestedAndSurrogateTestHighSurrogateAtTheContentEndHasNoRoomAndThrowsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestHighSurrogateAtTheContentEndHasNoRoomAndThrowsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for ContextualQuoteRoleResolverNestedAndSurrogateTestHighSurrogateAtTheContentEndHasNoRoomAndThrowsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        ContextualQuoteRoleResolverNestedAndSurrogateTestHighSurrogateAtTheContentEndHasNoRoomAndThrowsFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct ContextualQuoteRoleResolverNestedAndSurrogateSupport;

impl ContextualQuoteRoleResolverNestedAndSurrogateSupport {
    pub fn contextual_quote_role_resolver_nested_and_surrogate_support_start(n: &str) {
        TestTraceRecorder::new("ContextualQuoteRoleResolverNestedAndSurrogateTest").section(n);
    }

    pub fn contextual_quote_role_resolver_nested_and_surrogate_support_decisions(text: &str, locale: Option<String>) -> Result<Vec<QuoteRoleDecision>, TextRangeError> {
        let c = match &(locale) { None => FontRoleContext::new(Some("zh-Hans".to_string()), None), Some(__option) => FontRoleContext::new(Some((*__option).clone()), None) };
        let a = QuotePairAnalyzer::new();
        return Ok(a.classify_quote_roles(text, &a.analyze(text)?, Some((c).clone()))?);
    }

    pub fn contextual_quote_role_resolver_nested_and_surrogate_support_locate(ds: &Vec<QuoteRoleDecision>, i: u32) -> QuoteRoleDecision {
        let mut n = 0u32;
        while (i32::from_ne_bytes((n).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((ds.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if ds[usize::try_from(n).unwrap_or(0)].index == i {
                return (ds[usize::try_from(n).unwrap_or(0)]).clone();
            }
            n = u32::wrapping_add(n, 1);
        }
        return (ds[0usize]).clone();
    }

    pub fn contextual_quote_role_resolver_nested_and_surrogate_support_surrogate_text(codes: &Vec<u32>) -> String {
        let mut s = String::new();
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((codes.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            s += &(if codes[usize::try_from(i).unwrap_or(0)] > 0xFFFF { String::from_utf16(&[0xD800 + (((codes[usize::try_from(i).unwrap_or(0)]) - 0x10000) >> 10) as u16, 0xDC00 + (((codes[usize::try_from(i).unwrap_or(0)]) - 0x10000) & 0x3FF) as u16]).unwrap() } else {
String::from_utf16_lossy(&[(codes[usize::try_from(i).unwrap_or(0)]) as u16]) });
            i = u32::wrapping_add(i, 1);
        }
        return s;
    }

    pub fn contextual_quote_role_resolver_nested_and_surrogate_support_assert_illegal(f: Arc<dyn Fn() -> Result<(), IllegalStateException> + Send + Sync + 'static>) -> Result<(), TracedAssertionsAssertFailsWithFault> {
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), (f).clone())?;
        Ok(())
    }
}

#[test]
fn nested_pair_inside_neutral_enclosing_inherits_the_outer_quotation() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.nestedPairInsideNeutralEnclosingInheritsTheOuterQuotation", "org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.nestedPairInsideNeutralEnclosingInheritsTheOuterQuotation", || {
        ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_start(&"nestedPairInsideNeutralEnclosingInheritsTheOuterQuotation");
        let d = ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_decisions(&"“—‘文’—”", None).unwrap();
        let outer = ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_locate(&d, 0);
        let inner = ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_locate(&d, 2);
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"PairedPunctuationEnclosingQuoteContext", (inner.source).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"quote-pair-inherits-enclosing-quotation", (inner.reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(outer.role, Some(inner.role), None).unwrap();
        let mut has = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if d[usize::try_from(i).unwrap_or(0)].clone().source.to_string() == "DelimitedWesternQuotationRun" {
                has = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!has, None).unwrap();
    });
}

#[test]
fn space_before_unmatched_quote_with_cjk_right_skips_the_delimited_rule() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.spaceBeforeUnmatchedQuoteWithCjkRightSkipsTheDelimitedRule", "org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.spaceBeforeUnmatchedQuoteWithCjkRightSkipsTheDelimitedRule", ||
{
        ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_start(&"spaceBeforeUnmatchedQuoteWithCjkRightSkipsTheDelimitedRule");
        let d = ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_decisions(&" ’中", None).unwrap();
        let x = ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_locate(&d, 1);
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"UnmatchedQuoteSurroundingScriptContext", (x.source).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(x.role), None).unwrap();
    });
}

#[test]
fn leftward_scan_from_a_low_surrogate_walks_every_backtrack_arm() {
    testlib::record_not_applicable("org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.leftwardScanFromALowSurrogateWalksEveryBacktrackArm",
"org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.leftwardScanFromALowSurrogateWalksEveryBacktrackArm");
}

#[test]
fn tab_before_a_wholly_western_pair_delimits_like_a_space() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.tabBeforeAWhollyWesternPairDelimitsLikeASpace", "org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.tabBeforeAWhollyWesternPairDelimitsLikeASpace", || {
        ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_start(&"tabBeforeAWhollyWesternPairDelimitsLikeASpace");
        let x = ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_locate(&ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_decisions(&"\t“a”",
None).unwrap(), 1);
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"DelimitedWesternQuotationRun", (x.source).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn space_before_a_pair_with_non_western_content_skips_the_delimited_rule() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.spaceBeforeAPairWithNonWesternContentSkipsTheDelimitedRule", "org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.spaceBeforeAPairWithNonWesternContentSkipsTheDelimitedRule", ||
{
        ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_start(&"spaceBeforeAPairWithNonWesternContentSkipsTheDelimitedRule");
        let x = ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_locate(&ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_decisions(&" “中”",
None).unwrap(), 1);
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"PairedPunctuationContentScriptContext", (x.source).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(x.role), None).unwrap();
    });
}

#[test]
fn space_before_a_mixed_content_pair_reports_mixed_content() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.spaceBeforeAMixedContentPairReportsMixedContent", "org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.spaceBeforeAMixedContentPairReportsMixedContent", || {
        ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_start(&"spaceBeforeAMixedContentPairReportsMixedContent");
        let x = ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_locate(&ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_decisions(&" “a中”",
None).unwrap(), 1);
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"ParagraphLanguageQuoteContext", (x.source).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&(x.reason).to_string(), "mixed-quoted-content", 0)).to_ne_bytes())) <= 2147483647, Some((x.reason).to_string())).unwrap();
    });
}

#[test]
fn mixed_enclosing_level_falls_back_to_paragraph_language() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.mixedEnclosingLevelFallsBackToParagraphLanguage", "org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.mixedEnclosingLevelFallsBackToParagraphLanguage", || {
        ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_start(&"mixedEnclosingLevelFallsBackToParagraphLanguage");
        let x = ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_locate(&ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_decisions(&"a“中”文",
None).unwrap(), 1);
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"ParagraphLanguageQuoteContext", (x.source).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&(x.reason).to_string(), "mixed-enclosing-level-script", 0)).to_ne_bytes())) <= 2147483647, Some((x.reason).to_string())).unwrap();
    });
}

#[test]
fn non_chinese_locale_resolves_neutral_context_to_latin_text() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.nonChineseLocaleResolvesNeutralContextToLatinText", "org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.nonChineseLocaleResolvesNeutralContextToLatinText", || {
        ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_start(&"nonChineseLocaleResolvesNeutralContextToLatinText");
        let d = ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_decisions(&"’", Some("en-US".to_string())).unwrap();
        let x = (d[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(x.role), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&(x.reason).to_string(), "paragraph-language=en-US", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[test]
fn private_use_char_before_a_quote_fails_the_low_surrogate_range_above() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.privateUseCharBeforeAQuoteFailsTheLowSurrogateRangeAbove", "org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.privateUseCharBeforeAQuoteFailsTheLowSurrogateRangeAbove", || {
        ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_start(&"privateUseCharBeforeAQuoteFailsTheLowSurrogateRangeAbove");
        let x = ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_locate(&ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_decisions(&"“中",
None).unwrap(), 1);
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"UnmatchedQuoteSurroundingScriptContext", (x.source).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(x.role), None).unwrap();
    });
}

#[test]
fn high_surrogate_at_the_content_end_has_no_room_and_throws() {
    testlib::record_not_applicable("org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.highSurrogateAtTheContentEndHasNoRoomAndThrows", "org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.highSurrogateAtTheContentEndHasNoRoomAndThrows");
}

#[test]
fn sibling_pairs_inside_one_quotation_each_inherit_the_outer_role() {
    testlib::run("org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.siblingPairsInsideOneQuotationEachInheritTheOuterRole", "org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.siblingPairsInsideOneQuotationEachInheritTheOuterRole", || {
        ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_start(&"siblingPairsInsideOneQuotationEachInheritTheOuterRole");
        let d = ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_decisions(&"“‘a’‘b’”", None).unwrap();
        let a = ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_locate(&d, 1);
        let b = ContextualQuoteRoleResolverNestedAndSurrogateSupport::contextual_quote_role_resolver_nested_and_surrogate_support_locate(&d, 4);
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"PairedPunctuationEnclosingQuoteContext", (a.source).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"PairedPunctuationEnclosingQuoteContext", (b.source).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(a.role, Some(b.role), None).unwrap();
    });
}

#[test]
fn plain_follower_of_a_high_surrogate_counts_as_one_unit() {
    testlib::record_not_applicable("org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.plainFollowerOfAHighSurrogateCountsAsOneUnit", "org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.plainFollowerOfAHighSurrogateCountsAsOneUnit");
}

#[test]
fn private_use_follower_of_a_high_surrogate_counts_as_one_unit() {
    testlib::record_not_applicable("org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.privateUseFollowerOfAHighSurrogateCountsAsOneUnit", "org.tiqian.layout.ContextualQuoteRoleResolverNestedAndSurrogateTest.privateUseFollowerOfAHighSurrogateCountsAsOneUnit");
}
