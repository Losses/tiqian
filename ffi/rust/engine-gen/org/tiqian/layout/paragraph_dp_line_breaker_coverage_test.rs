#![cfg(test)]

use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::line_breaker::LineBreaker;
use crate::org::tiqian::layout::line_optimization::LineSolution;
use crate::org::tiqian::layout::paragraph_dp_line_breaker::ParagraphDpLineBreaker;
use crate::org::tiqian::layout::paragraph_dp_line_breaker_coverage_test_support::ParagraphDpLineBreakerCoverageTestSupport;
use crate::org::tiqian::layout::paragraph_dp_line_breaker_test_support::ParagraphDpLineBreakerTestSupport;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkChannel;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::UnbreakableRanges;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerCoverageTestTierPromotionRoutesTheRepairReasonThroughThePromotionCodeFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerCoverageTestTierPromotionRoutesTheRepairReasonThroughThePromotionCodeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerCoverageTestTierPromotionRoutesTheRepairReasonThroughThePromotionCodeFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestTierPromotionRoutesTheRepairReasonThroughThePromotionCodeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestTierPromotionRoutesTheRepairReasonThroughThePromotionCodeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestTierPromotionRoutesTheRepairReasonThroughThePromotionCodeFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestTierPromotionRoutesTheRepairReasonThroughThePromotionCodeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestTierPromotionRoutesTheRepairReasonThroughThePromotionCodeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestTierPromotionRoutesTheRepairReasonThroughThePromotionCodeFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestTierPromotionRoutesTheRepairReasonThroughThePromotionCodeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerCoverageTestTierPromotionRoutesTheRepairReasonThroughThePromotionCodeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerCoverageTestTierPromotionRoutesTheRepairReasonThroughThePromotionCodeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerCoverageTestTierPromotionRoutesTheRepairReasonThroughThePromotionCodeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerCoverageTestTierPromotionRoutesTheRepairReasonThroughThePromotionCodeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerCoverageTestTierPromotionRoutesTheRepairReasonThroughThePromotionCodeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerCoverageTestTierPromotionRoutesTheRepairReasonThroughThePromotionCodeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerCoverageTestShrinkPrefixSkipsNonPositiveAndOutOfRangeOpportunitiesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerCoverageTestShrinkPrefixSkipsNonPositiveAndOutOfRangeOpportunitiesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestShrinkPrefixSkipsNonPositiveAndOutOfRangeOpportunitiesFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestShrinkPrefixSkipsNonPositiveAndOutOfRangeOpportunitiesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestShrinkPrefixSkipsNonPositiveAndOutOfRangeOpportunitiesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerCoverageTestShrinkPrefixSkipsNonPositiveAndOutOfRangeOpportunitiesFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestShrinkPrefixSkipsNonPositiveAndOutOfRangeOpportunitiesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestShrinkPrefixSkipsNonPositiveAndOutOfRangeOpportunitiesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestShrinkPrefixSkipsNonPositiveAndOutOfRangeOpportunitiesFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestShrinkPrefixSkipsNonPositiveAndOutOfRangeOpportunitiesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerCoverageTestShrinkPrefixSkipsNonPositiveAndOutOfRangeOpportunitiesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerCoverageTestShrinkPrefixSkipsNonPositiveAndOutOfRangeOpportunitiesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerCoverageTestShrinkPrefixSkipsNonPositiveAndOutOfRangeOpportunitiesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerCoverageTestShrinkPrefixSkipsNonPositiveAndOutOfRangeOpportunitiesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerCoverageTestShrinkPrefixSkipsNonPositiveAndOutOfRangeOpportunitiesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerCoverageTestShrinkPrefixSkipsNonPositiveAndOutOfRangeOpportunitiesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerCoverageTestPromotionCheckReturnsFalseWhenTheCandidateEndHasNoOpportunityFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerCoverageTestPromotionCheckReturnsFalseWhenTheCandidateEndHasNoOpportunityFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerCoverageTestPromotionCheckReturnsFalseWhenTheCandidateEndHasNoOpportunityFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestPromotionCheckReturnsFalseWhenTheCandidateEndHasNoOpportunityFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestPromotionCheckReturnsFalseWhenTheCandidateEndHasNoOpportunityFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestPromotionCheckReturnsFalseWhenTheCandidateEndHasNoOpportunityFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestPromotionCheckReturnsFalseWhenTheCandidateEndHasNoOpportunityFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestPromotionCheckReturnsFalseWhenTheCandidateEndHasNoOpportunityFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestPromotionCheckReturnsFalseWhenTheCandidateEndHasNoOpportunityFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestPromotionCheckReturnsFalseWhenTheCandidateEndHasNoOpportunityFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerCoverageTestPromotionCheckReturnsFalseWhenTheCandidateEndHasNoOpportunityFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerCoverageTestPromotionCheckReturnsFalseWhenTheCandidateEndHasNoOpportunityFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerCoverageTestPromotionCheckReturnsFalseWhenTheCandidateEndHasNoOpportunityFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerCoverageTestPromotionCheckReturnsFalseWhenTheCandidateEndHasNoOpportunityFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerCoverageTestPromotionCheckReturnsFalseWhenTheCandidateEndHasNoOpportunityFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerCoverageTestPromotionCheckReturnsFalseWhenTheCandidateEndHasNoOpportunityFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerCoverageTestNegativeCandidateWindowIsRejectedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerCoverageTestNarrowWindowsDropEndsAtOrBelowTheLineStartFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerCoverageTestNarrowWindowsDropEndsAtOrBelowTheLineStartFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestNarrowWindowsDropEndsAtOrBelowTheLineStartFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestNarrowWindowsDropEndsAtOrBelowTheLineStartFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestNarrowWindowsDropEndsAtOrBelowTheLineStartFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerCoverageTestNarrowWindowsDropEndsAtOrBelowTheLineStartFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestNarrowWindowsDropEndsAtOrBelowTheLineStartFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestNarrowWindowsDropEndsAtOrBelowTheLineStartFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestNarrowWindowsDropEndsAtOrBelowTheLineStartFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestNarrowWindowsDropEndsAtOrBelowTheLineStartFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerCoverageTestNarrowWindowsDropEndsAtOrBelowTheLineStartFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerCoverageTestNarrowWindowsDropEndsAtOrBelowTheLineStartFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerCoverageTestNarrowWindowsDropEndsAtOrBelowTheLineStartFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerCoverageTestNarrowWindowsDropEndsAtOrBelowTheLineStartFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerCoverageTestNarrowWindowsDropEndsAtOrBelowTheLineStartFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerCoverageTestNarrowWindowsDropEndsAtOrBelowTheLineStartFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerCoverageTestMismatchedNaturalAndAdjustedSizesAreRejectedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerCoverageTestMandatorySegmentFiltersTheControlBoundaryFromCandidatesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerCoverageTestMandatorySegmentFiltersTheControlBoundaryFromCandidatesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestMandatorySegmentFiltersTheControlBoundaryFromCandidatesFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestMandatorySegmentFiltersTheControlBoundaryFromCandidatesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestMandatorySegmentFiltersTheControlBoundaryFromCandidatesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerCoverageTestMandatorySegmentFiltersTheControlBoundaryFromCandidatesFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestMandatorySegmentFiltersTheControlBoundaryFromCandidatesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestMandatorySegmentFiltersTheControlBoundaryFromCandidatesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestMandatorySegmentFiltersTheControlBoundaryFromCandidatesFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestMandatorySegmentFiltersTheControlBoundaryFromCandidatesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerCoverageTestMandatorySegmentFiltersTheControlBoundaryFromCandidatesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerCoverageTestMandatorySegmentFiltersTheControlBoundaryFromCandidatesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerCoverageTestMandatorySegmentFiltersTheControlBoundaryFromCandidatesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerCoverageTestMandatorySegmentFiltersTheControlBoundaryFromCandidatesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerCoverageTestMandatorySegmentFiltersTheControlBoundaryFromCandidatesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerCoverageTestMandatorySegmentFiltersTheControlBoundaryFromCandidatesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerCoverageTestLineEndOnlyCapacityFeedsTheCompressedEdgeAtTheLineEndFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerCoverageTestLineEndOnlyCapacityFeedsTheCompressedEdgeAtTheLineEndFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestLineEndOnlyCapacityFeedsTheCompressedEdgeAtTheLineEndFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestLineEndOnlyCapacityFeedsTheCompressedEdgeAtTheLineEndFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestLineEndOnlyCapacityFeedsTheCompressedEdgeAtTheLineEndFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerCoverageTestLineEndOnlyCapacityFeedsTheCompressedEdgeAtTheLineEndFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestLineEndOnlyCapacityFeedsTheCompressedEdgeAtTheLineEndFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestLineEndOnlyCapacityFeedsTheCompressedEdgeAtTheLineEndFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestLineEndOnlyCapacityFeedsTheCompressedEdgeAtTheLineEndFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestLineEndOnlyCapacityFeedsTheCompressedEdgeAtTheLineEndFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerCoverageTestLineEndOnlyCapacityFeedsTheCompressedEdgeAtTheLineEndFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerCoverageTestLineEndOnlyCapacityFeedsTheCompressedEdgeAtTheLineEndFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerCoverageTestLineEndOnlyCapacityFeedsTheCompressedEdgeAtTheLineEndFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerCoverageTestLineEndOnlyCapacityFeedsTheCompressedEdgeAtTheLineEndFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerCoverageTestLineEndOnlyCapacityFeedsTheCompressedEdgeAtTheLineEndFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerCoverageTestLineEndOnlyCapacityFeedsTheCompressedEdgeAtTheLineEndFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerCoverageTestEmptyClustersReturnAnEmptySolutionFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerCoverageTestEmptyClustersReturnAnEmptySolutionFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestEmptyClustersReturnAnEmptySolutionFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestEmptyClustersReturnAnEmptySolutionFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestEmptyClustersReturnAnEmptySolutionFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerCoverageTestEmptyClustersReturnAnEmptySolutionFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestEmptyClustersReturnAnEmptySolutionFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestEmptyClustersReturnAnEmptySolutionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestEmptyClustersReturnAnEmptySolutionFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestEmptyClustersReturnAnEmptySolutionFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerCoverageTestEmptyClustersReturnAnEmptySolutionFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerCoverageTestEmptyClustersReturnAnEmptySolutionFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerCoverageTestEmptyClustersReturnAnEmptySolutionFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerCoverageTestEmptyClustersReturnAnEmptySolutionFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerCoverageTestEmptyClustersReturnAnEmptySolutionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerCoverageTestEmptyClustersReturnAnEmptySolutionFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerCoverageTestCompressedFinalMandatoryLineUsesTheCompressedCommitBranchFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerCoverageTestCompressedFinalMandatoryLineUsesTheCompressedCommitBranchFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestCompressedFinalMandatoryLineUsesTheCompressedCommitBranchFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestCompressedFinalMandatoryLineUsesTheCompressedCommitBranchFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestCompressedFinalMandatoryLineUsesTheCompressedCommitBranchFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerCoverageTestCompressedFinalMandatoryLineUsesTheCompressedCommitBranchFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestCompressedFinalMandatoryLineUsesTheCompressedCommitBranchFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestCompressedFinalMandatoryLineUsesTheCompressedCommitBranchFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestCompressedFinalMandatoryLineUsesTheCompressedCommitBranchFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestCompressedFinalMandatoryLineUsesTheCompressedCommitBranchFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerCoverageTestCompressedFinalMandatoryLineUsesTheCompressedCommitBranchFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerCoverageTestCompressedFinalMandatoryLineUsesTheCompressedCommitBranchFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerCoverageTestCompressedFinalMandatoryLineUsesTheCompressedCommitBranchFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerCoverageTestCompressedFinalMandatoryLineUsesTheCompressedCommitBranchFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerCoverageTestCompressedFinalMandatoryLineUsesTheCompressedCommitBranchFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerCoverageTestCompressedFinalMandatoryLineUsesTheCompressedCommitBranchFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerCoverageTestCompressedEndsMayReachTheSegmentEndFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerCoverageTestCompressedEndsMayReachTheSegmentEndFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestCompressedEndsMayReachTheSegmentEndFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestCompressedEndsMayReachTheSegmentEndFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestCompressedEndsMayReachTheSegmentEndFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerCoverageTestCompressedEndsMayReachTheSegmentEndFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestCompressedEndsMayReachTheSegmentEndFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverageTestCompressedEndsMayReachTheSegmentEndFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerCoverageTestCompressedEndsMayReachTheSegmentEndFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverageTestCompressedEndsMayReachTheSegmentEndFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerCoverageTestCompressedEndsMayReachTheSegmentEndFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerCoverageTestCompressedEndsMayReachTheSegmentEndFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerCoverageTestCompressedEndsMayReachTheSegmentEndFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerCoverageTestCompressedEndsMayReachTheSegmentEndFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerCoverageTestCompressedEndsMayReachTheSegmentEndFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerCoverageTestCompressedEndsMayReachTheSegmentEndFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn empty_clusters_return_an_empty_solution() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.emptyClustersReturnAnEmptySolution", "org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.emptyClustersReturnAnEmptySolution", || {
        ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_rec(&"emptyClustersReturnAnEmptySolution");
        let s = ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_solve(&vec![], 100 as f64, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_lines_string((s).clone())).to_string())).unwrap();
    });
}

#[test]
fn mismatched_natural_and_adjusted_sizes_are_rejected() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.mismatchedNaturalAndAdjustedSizesAreRejected", "org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.mismatchedNaturalAndAdjustedSizesAreRejected", || {
        ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_rec(&"mismatchedNaturalAndAdjustedSizesAreRejected");
        let e = TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        ParagraphDpLineBreaker::new(Some(8), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12 as f64), Some(12 as f64), Some(3 as f64), Some(1 as f64)).map_err(|e| IllegalStateException::new(&format!("{}",
e)))?.break_lines(&ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_han(2, None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?,
&ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_han(1, None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?, 100 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&format!("{}", e), "cluster-for-cluster", 0)).to_ne_bytes())) <= 2147483647, Some((format!("{}", e)).to_string())).unwrap();
    });
}

#[test]
fn negative_candidate_window_is_rejected() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.negativeCandidateWindowIsRejected", "org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.negativeCandidateWindowIsRejected", || {
        ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_rec(&"negativeCandidateWindowIsRejected");
        let e = TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        ParagraphDpLineBreaker::new(Some(4294967295u32), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12 as f64), Some(12 as f64), Some(3 as f64), Some(1 as f64)).map_err(|e|
IllegalStateException::new(&format!("{}", e)))?.break_lines(&ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_han(2, None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?,
&ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_han(2, None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?, 100 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&format!("{}", e), "non-negative", 0)).to_ne_bytes())) <= 2147483647, Some((format!("{}", e)).to_string())).unwrap();
    });
}

#[test]
fn shrink_prefix_skips_non_positive_and_out_of_range_opportunities() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.shrinkPrefixSkipsNonPositiveAndOutOfRangeOpportunities", "org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.shrinkPrefixSkipsNonPositiveAndOutOfRangeOpportunities", || {
        ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_rec(&"shrinkPrefixSkipsNonPositiveAndOutOfRangeOpportunities");
        let c = ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_han(4, None).unwrap();
        let s = ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_solve(&c, 100 as f64, Some(vec![
    (ShrinkOpportunity::new(1u32, 2u32, 0 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
    (ShrinkOpportunity::new(4u32, 2u32, 8 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
    (ShrinkOpportunity::new(1u32, 2u32, 8 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
]), None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_lines_string((s).clone())).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 3u32), ((s.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
    });
}

#[test]
fn line_end_only_capacity_feeds_the_compressed_edge_at_the_line_end() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.lineEndOnlyCapacityFeedsTheCompressedEdgeAtTheLineEnd", "org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.lineEndOnlyCapacityFeedsTheCompressedEdgeAtTheLineEnd", || {
        ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_rec(&"lineEndOnlyCapacityFeedsTheCompressedEdgeAtTheLineEnd");
        let c = ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_han(4, None).unwrap();
        let s = ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_solve(&c, 44 as f64, Some(vec![
    (ShrinkOpportunity::new(2u32, 1u32, 4 as f64 as f64, ShrinkChannel::TrailingGlue, Some(true))).clone(),
]), None, Some(true), None, None, None, Some(vec![1])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 2u32), ((s.lines[0usize]).clone().cluster_range).clone(),
Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_lines_string((s).clone())).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_push_in_reason_starts_with(((s.lines[0usize]).clone().repair).clone(), &"LineAdjustmentPushIn"),
Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_repairs_string((s).clone())).to_string())).unwrap();
    });
}

#[test]
fn compressed_ends_may_reach_the_segment_end() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.compressedEndsMayReachTheSegmentEnd", "org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.compressedEndsMayReachTheSegmentEnd", || {
        ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_rec(&"compressedEndsMayReachTheSegmentEnd");
        let s = ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_solve(&ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_han(3, None).unwrap(), 44 as f64, Some(vec![
    (ShrinkOpportunity::new(1u32, 2u32, 12 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
]), None, Some(true), None, None, None, Some(vec![1])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 2u32), ((s.lines[0usize]).clone().cluster_range).clone(),
Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_lines_string((s).clone())).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_push_in_reason_starts_with(((s.lines[0usize]).clone().repair).clone(), &"LineAdjustmentPushIn"),
Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_repairs_string((s).clone())).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"ParagraphEnd", s.lines[usize::try_from(u32::wrapping_sub(u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)].end_reason.name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn compressed_final_mandatory_line_uses_the_compressed_commit_branch() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.compressedFinalMandatoryLineUsesTheCompressedCommitBranch", "org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.compressedFinalMandatoryLineUsesTheCompressedCommitBranch", || {
        ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_rec(&"compressedFinalMandatoryLineUsesTheCompressedCommitBranch");
        let s = ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_solve(&ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_han(4, None).unwrap(), 44 as f64, Some(vec![
    (ShrinkOpportunity::new(2u32, 1u32, 4 as f64 as f64, ShrinkChannel::TrailingGlue, Some(true))).clone(),
]), Some(vec![2]), Some(true), None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 2u32), ((s.lines[0usize]).clone().cluster_range).clone(),
Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_lines_string((s).clone())).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"MandatoryBreak", s.lines[0usize].end_reason.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_push_in_reason_starts_with(((s.lines[0usize]).clone().repair).clone(), &"LineAdjustmentPushIn"),
Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_repairs_string((s).clone())).to_string())).unwrap();
    });
}

#[test]
fn tier_promotion_routes_the_repair_reason_through_the_promotion_code() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.tierPromotionRoutesTheRepairReasonThroughThePromotionCode", "org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.tierPromotionRoutesTheRepairReasonThroughThePromotionCode", || {
        ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_rec(&"tierPromotionRoutesTheRepairReasonThroughThePromotionCode");
        let c = ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_latin().unwrap();
        let span = TextRange::new(0u32, 5u32).unwrap();
        let s = ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_solve(&c, 80 as f64, Some(vec![
    (ShrinkOpportunity::new(2u32, 2u32, 5 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
]), None, Some(true), None, Some(ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_opp(&vec![2, 3], &vec![(span).clone(), (span).clone()], &vec![ProgressiveBreakTier::Emergency, ProgressiveBreakTier::Whitespace])), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 2u32), ((s.lines[0usize]).clone().cluster_range).clone(),
Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_lines_string((s).clone())).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_push_in_reason_starts_with(((s.lines[0usize]).clone().repair).clone(), &"ProgressiveTechnicalTierPromotion"),
Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_repairs_string((s).clone())).to_string())).unwrap();
    });
}

#[test]
fn promotion_check_returns_false_when_the_candidate_end_has_no_opportunity() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.promotionCheckReturnsFalseWhenTheCandidateEndHasNoOpportunity", "org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.promotionCheckReturnsFalseWhenTheCandidateEndHasNoOpportunity", || {
        ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_rec(&"promotionCheckReturnsFalseWhenTheCandidateEndHasNoOpportunity");
        let c = ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_latin().unwrap();
        let s = ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_solve(&c, 80 as f64, Some(vec![
    (ShrinkOpportunity::new(2u32, 2u32, 5 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
]), None, Some(true), None, Some(ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_opp(&vec![2], &vec![(TextRange::new(0u32, 5u32).unwrap()).clone()], &vec![ProgressiveBreakTier::Emergency])), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((s.lines[0usize]).clone().cluster_range).clone(),
Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_lines_string((s).clone())).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((s.lines[0usize]).clone().repair == None, Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_repairs_string((s).clone())).to_string())).unwrap();
    });
}

#[test]
fn mandatory_segment_filters_the_control_boundary_from_candidates() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.mandatorySegmentFiltersTheControlBoundaryFromCandidates", "org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.mandatorySegmentFiltersTheControlBoundaryFromCandidates", || {
        ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_rec(&"mandatorySegmentFiltersTheControlBoundaryFromCandidates");
        let s = ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_solve(&ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_han(6, None).unwrap(), 32 as f64, None, Some(vec![2]), None, None, None,
None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 2u32), ((s.lines[0usize]).clone().cluster_range).clone(),
Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_lines_string((s).clone())).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"MandatoryBreak", s.lines[0usize].end_reason.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(5, (s.lines[usize::try_from(u32::wrapping_sub(u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone().cluster_range.end,
Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_lines_string((s).clone())).to_string())).unwrap();
    });
}

#[test]
fn narrow_windows_drop_ends_at_or_below_the_line_start() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.narrowWindowsDropEndsAtOrBelowTheLineStart", "org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.narrowWindowsDropEndsAtOrBelowTheLineStart", || {
        ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_rec(&"narrowWindowsDropEndsAtOrBelowTheLineStart");
        let s = ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_solve(&ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_han(4, None).unwrap(), 20 as f64, None, None, None, None, None, None,
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_lines_string((s).clone())).to_string())).unwrap();
        let mut all = true;
        {
            let _g1 = s.lines.clone();
            for l in &_g1 {
                if l.cluster_range.clone().start != (l.cluster_range).clone().end {
                    all = false;
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all, Some((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_ranges_string((s).clone())).to_string())).unwrap();
    });
}

#[test]
fn interface_default_strategy_name_is_custom() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.interfaceDefaultStrategyNameIsCustom", "org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.interfaceDefaultStrategyNameIsCustom", || {
        ParagraphDpLineBreakerCoverageTestSupport::paragraph_dp_line_breaker_coverage_test_support_rec(&"interfaceDefaultStrategyNameIsCustom");
        let b: Box<dyn LineBreaker> = Box::new(CustomBreaker::new());
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"custom", b.get_strategy_name().as_str(), None).unwrap();
    });
}

#[derive(Clone, PartialEq)]
pub struct CustomBreaker {
}

impl CustomBreaker {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn get_strategy_name(&self) -> String {
        return "custom".to_string();
    }

    pub fn break_lines(&self, _n: &Vec<Cluster>, _a: &Vec<Cluster>, _w: f64, _s: Option<Vec<ShrinkOpportunity>>, _u: Option<UnbreakableRanges>, _i: Option<f64>, _h: Option<SortedSetTable<u32>>, _e: Option<Vec<IntRange>>, _fs: Option<SortedSetTable<u32>>, _fe:
Option<SortedSetTable<u32>>, _hy: Option<SortedSetTable<u32>>, _cj: Option<SortedSetTable<u32>>, _mc: Option<f64>, _sw: Option<SortedSetTable<u32>>, _sc: Option<f64>, _p: Option<bool>, _bias: Option<f64>, _hb: Option<SortedSetTable<u32>>, _nc: Option<SortedSetTable<u32>>, _pr:
Option<SortedMapTable<u32, ProgressiveBreakOpportunity>>) -> Result<LineSolution, TextRangeError> {
        return Ok(LineSolution::new(Some(vec![]), Some(0 as f64))?);
    }
}

impl LineBreaker for CustomBreaker {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ParagraphDpLineBreakerCoverageTest.CustomBreaker"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn LineBreaker> {
        Box::new(self.clone())
    }

    fn get_strategy_name(&self) -> String {
        return "custom".to_string();
    }

    fn break_lines(&self, _n: &Vec<Cluster>, _a: &Vec<Cluster>, _w: f64, _s: Option<Vec<ShrinkOpportunity>>, _u: Option<UnbreakableRanges>, _i: Option<f64>, _h: Option<SortedSetTable<u32>>, _e: Option<Vec<IntRange>>, _fs: Option<SortedSetTable<u32>>, _fe:
Option<SortedSetTable<u32>>, _hy: Option<SortedSetTable<u32>>, _cj: Option<SortedSetTable<u32>>, _mc: Option<f64>, _sw: Option<SortedSetTable<u32>>, _sc: Option<f64>, _p: Option<bool>, _bias: Option<f64>, _hb: Option<SortedSetTable<u32>>, _nc: Option<SortedSetTable<u32>>, _pr:
Option<SortedMapTable<u32, ProgressiveBreakOpportunity>>) -> Result<LineSolution, TextRangeError> {
        return Ok(LineSolution::new(Some(vec![]), Some(0 as f64))?);
    }
}
