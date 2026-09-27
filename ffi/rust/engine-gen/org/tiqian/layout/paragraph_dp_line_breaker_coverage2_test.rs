#![cfg(test)]

use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::layout::paragraph_dp_line_breaker_coverage2_test_support::ParagraphDpLineBreakerCoverage2TestSupport;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkChannel;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::UnbreakableRanges;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerCoverage2TestTestTierPreferredPoolEmptyFallbackFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerCoverage2TestTestTierPreferredPoolEmptyFallbackFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestTierPreferredPoolEmptyFallbackFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestTierPreferredPoolEmptyFallbackFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverage2TestTestTierPreferredPoolEmptyFallbackFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestTierPreferredPoolEmptyFallbackFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestTierPreferredPoolEmptyFallbackFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverage2TestTestTierPreferredPoolEmptyFallbackFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestTierPreferredPoolEmptyFallbackFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestTierPreferredPoolEmptyFallbackFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerCoverage2TestTestTierPreferredPoolEmptyFallbackFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestTierPreferredPoolEmptyFallbackFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerCoverage2TestTestTierPreferredPoolEmptyFallbackFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestTierPreferredPoolEmptyFallbackFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerCoverage2TestTestTierPreferredPoolEmptyFallbackFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestTierPreferredPoolEmptyFallbackFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerCoverage2TestTestShrinkOpportunitiesNegativeAndOutOfRangeFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerCoverage2TestTestShrinkOpportunitiesNegativeAndOutOfRangeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestShrinkOpportunitiesNegativeAndOutOfRangeFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestShrinkOpportunitiesNegativeAndOutOfRangeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverage2TestTestShrinkOpportunitiesNegativeAndOutOfRangeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestShrinkOpportunitiesNegativeAndOutOfRangeFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestShrinkOpportunitiesNegativeAndOutOfRangeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverage2TestTestShrinkOpportunitiesNegativeAndOutOfRangeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestShrinkOpportunitiesNegativeAndOutOfRangeFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestShrinkOpportunitiesNegativeAndOutOfRangeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerCoverage2TestTestShrinkOpportunitiesNegativeAndOutOfRangeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestShrinkOpportunitiesNegativeAndOutOfRangeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerCoverage2TestTestShrinkOpportunitiesNegativeAndOutOfRangeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestShrinkOpportunitiesNegativeAndOutOfRangeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerCoverage2TestTestShrinkOpportunitiesNegativeAndOutOfRangeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestShrinkOpportunitiesNegativeAndOutOfRangeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerCoverage2TestTestProgressiveTierPromotionBranchesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerCoverage2TestTestProgressiveTierPromotionBranchesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestProgressiveTierPromotionBranchesFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestProgressiveTierPromotionBranchesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverage2TestTestProgressiveTierPromotionBranchesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestProgressiveTierPromotionBranchesFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestProgressiveTierPromotionBranchesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverage2TestTestProgressiveTierPromotionBranchesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestProgressiveTierPromotionBranchesFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestProgressiveTierPromotionBranchesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerCoverage2TestTestProgressiveTierPromotionBranchesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestProgressiveTierPromotionBranchesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerCoverage2TestTestProgressiveTierPromotionBranchesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestProgressiveTierPromotionBranchesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerCoverage2TestTestProgressiveTierPromotionBranchesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestProgressiveTierPromotionBranchesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerCoverage2TestTestHardBreakAfterClustersInDpCommitFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerCoverage2TestTestHardBreakAfterClustersInDpCommitFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestHardBreakAfterClustersInDpCommitFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestHardBreakAfterClustersInDpCommitFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverage2TestTestHardBreakAfterClustersInDpCommitFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestHardBreakAfterClustersInDpCommitFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestHardBreakAfterClustersInDpCommitFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverage2TestTestHardBreakAfterClustersInDpCommitFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestHardBreakAfterClustersInDpCommitFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestHardBreakAfterClustersInDpCommitFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerCoverage2TestTestHardBreakAfterClustersInDpCommitFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestHardBreakAfterClustersInDpCommitFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerCoverage2TestTestHardBreakAfterClustersInDpCommitFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestHardBreakAfterClustersInDpCommitFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerCoverage2TestTestHardBreakAfterClustersInDpCommitFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestHardBreakAfterClustersInDpCommitFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerCoverage2TestTestCommitSegmentOriginalBreakNotNullResultingBreakNullFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerCoverage2TestTestCommitSegmentOriginalBreakNotNullResultingBreakNullFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestCommitSegmentOriginalBreakNotNullResultingBreakNullFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestCommitSegmentOriginalBreakNotNullResultingBreakNullFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverage2TestTestCommitSegmentOriginalBreakNotNullResultingBreakNullFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestCommitSegmentOriginalBreakNotNullResultingBreakNullFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestCommitSegmentOriginalBreakNotNullResultingBreakNullFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverage2TestTestCommitSegmentOriginalBreakNotNullResultingBreakNullFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestCommitSegmentOriginalBreakNotNullResultingBreakNullFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestCommitSegmentOriginalBreakNotNullResultingBreakNullFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerCoverage2TestTestCommitSegmentOriginalBreakNotNullResultingBreakNullFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestCommitSegmentOriginalBreakNotNullResultingBreakNullFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerCoverage2TestTestCommitSegmentOriginalBreakNotNullResultingBreakNullFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestCommitSegmentOriginalBreakNotNullResultingBreakNullFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerCoverage2TestTestCommitSegmentOriginalBreakNotNullResultingBreakNullFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestCommitSegmentOriginalBreakNotNullResultingBreakNullFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerCoverage2TestTestCandidateWindowBoundsCompressionEdgesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerCoverage2TestTestCandidateWindowBoundsCompressionEdgesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestCandidateWindowBoundsCompressionEdgesFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestCandidateWindowBoundsCompressionEdgesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverage2TestTestCandidateWindowBoundsCompressionEdgesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestCandidateWindowBoundsCompressionEdgesFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestCandidateWindowBoundsCompressionEdgesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverage2TestTestCandidateWindowBoundsCompressionEdgesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestCandidateWindowBoundsCompressionEdgesFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestCandidateWindowBoundsCompressionEdgesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerCoverage2TestTestCandidateWindowBoundsCompressionEdgesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestCandidateWindowBoundsCompressionEdgesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerCoverage2TestTestCandidateWindowBoundsCompressionEdgesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestCandidateWindowBoundsCompressionEdgesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerCoverage2TestTestCandidateWindowBoundsCompressionEdgesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestCandidateWindowBoundsCompressionEdgesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerCoverage2TestTestCandidateEndsWindowBelowLineStartFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerCoverage2TestTestCandidateEndsWindowBelowLineStartFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestCandidateEndsWindowBelowLineStartFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestCandidateEndsWindowBelowLineStartFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverage2TestTestCandidateEndsWindowBelowLineStartFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestCandidateEndsWindowBelowLineStartFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestCandidateEndsWindowBelowLineStartFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerCoverage2TestTestCandidateEndsWindowBelowLineStartFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerCoverage2TestTestCandidateEndsWindowBelowLineStartFault) -> Self {
        match value {
            ParagraphDpLineBreakerCoverage2TestTestCandidateEndsWindowBelowLineStartFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerCoverage2TestTestCandidateEndsWindowBelowLineStartFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestCandidateEndsWindowBelowLineStartFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerCoverage2TestTestCandidateEndsWindowBelowLineStartFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestCandidateEndsWindowBelowLineStartFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerCoverage2TestTestCandidateEndsWindowBelowLineStartFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerCoverage2TestTestCandidateEndsWindowBelowLineStartFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn test_shrink_opportunities_negative_and_out_of_range() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerCoverage2Test.testShrinkOpportunitiesNegativeAndOutOfRange", "org.tiqian.layout.ParagraphDpLineBreakerCoverage2Test.testShrinkOpportunitiesNegativeAndOutOfRange", || {
        ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_rec(&"testShrinkOpportunitiesNegativeAndOutOfRange");
        let c = ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_han(3, None).unwrap();
        let s = ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_solve(&c, 100 as f64, Some(vec![
    (ShrinkOpportunity::new(4294967295u32, 1u32, 10 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
    (ShrinkOpportunity::new(0u32, 1u32, i32::from_ne_bytes((4294967291u32).to_ne_bytes()) as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
    (ShrinkOpportunity::new(5u32, 1u32, 10 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
    (ShrinkOpportunity::new(1u32, 1u32, 4 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
    (ShrinkOpportunity::new(2u32, 1u32, 4 as f64 as f64, ShrinkChannel::TrailingGlue, Some(true))).clone(),
]), None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn test_candidate_window_bounds_compression_edges() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerCoverage2Test.testCandidateWindowBoundsCompressionEdges", "org.tiqian.layout.ParagraphDpLineBreakerCoverage2Test.testCandidateWindowBoundsCompressionEdges", || {
        ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_rec(&"testCandidateWindowBoundsCompressionEdges");
        let c = ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_han(4, Some(20 as f64 as f64)).unwrap();
        let s = ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_solve(&c, 25 as f64, Some(vec![
    (ShrinkOpportunity::new(0u32, 1u32, 10 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
    (ShrinkOpportunity::new(1u32, 1u32, 10 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
    (ShrinkOpportunity::new(2u32, 1u32, 10 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
]), None, Some(true), None, None, Some(1)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn test_progressive_tier_promotion_branches() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerCoverage2Test.testProgressiveTierPromotionBranches", "org.tiqian.layout.ParagraphDpLineBreakerCoverage2Test.testProgressiveTierPromotionBranches", || {
        ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_rec(&"testProgressiveTierPromotionBranches");
        let c = ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_latin().unwrap();
        let span = TextRange::new(0u32, 5u32).unwrap();
        let other = TextRange::new(1u32, 3u32).unwrap();
        let a = ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_solve(&c, 80 as f64, Some(vec![
    (ShrinkOpportunity::new(2u32, 2u32, 5 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
]), None, Some(true), None, Some(ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_opp(&vec![2, 3], &vec![(span).clone(), (span).clone()], &vec![ProgressiveBreakTier::Whitespace, ProgressiveBreakTier::Emergency])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((a.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
        let b = ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_solve(&c, 80 as f64, Some(vec![
    (ShrinkOpportunity::new(2u32, 2u32, 5 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
]), None, Some(true), None, Some(ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_opp(&vec![2, 3], &vec![(span).clone(), (other).clone()], &vec![ProgressiveBreakTier::Emergency, ProgressiveBreakTier::Whitespace])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((b.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
        let d = ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_solve(&c, 80 as f64, Some(vec![
    (ShrinkOpportunity::new(2u32, 2u32, 5 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
]), None, Some(true), None, Some(ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_opp(&vec![2, 3], &vec![(span).clone(), (span).clone()], &vec![ProgressiveBreakTier::Emergency, ProgressiveBreakTier::Whitespace])), Some(4)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((d.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn test_commit_segment_original_break_not_null_resulting_break_null() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerCoverage2Test.testCommitSegmentOriginalBreakNotNullResultingBreakNull", "org.tiqian.layout.ParagraphDpLineBreakerCoverage2Test.testCommitSegmentOriginalBreakNotNullResultingBreakNull", || {
        ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_rec(&"testCommitSegmentOriginalBreakNotNullResultingBreakNull");
        let c = ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_latin().unwrap();
        let s = ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_solve(&c, 80 as f64, Some(vec![
    (ShrinkOpportunity::new(2u32, 2u32, 5 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
]), None, Some(true), None, Some(ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_opp(&vec![2], &vec![(TextRange::new(0u32, 5u32).unwrap()).clone()], &vec![ProgressiveBreakTier::Whitespace])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn test_tier_preferred_pool_empty_fallback() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerCoverage2Test.testTierPreferredPoolEmptyFallback", "org.tiqian.layout.ParagraphDpLineBreakerCoverage2Test.testTierPreferredPoolEmptyFallback", || {
        ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_rec(&"testTierPreferredPoolEmptyFallback");
        let s = ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_solve(&ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_han(4, Some(20 as f64 as f64)).unwrap(), 30 as f64, None, None, None,
Some(UnbreakableRanges::new(vec![(IntRange::new(0u32, 3u32)).clone()].to_vec())), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn test_hard_break_after_clusters_in_dp_commit() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerCoverage2Test.testHardBreakAfterClustersInDpCommit", "org.tiqian.layout.ParagraphDpLineBreakerCoverage2Test.testHardBreakAfterClustersInDpCommit", || {
        ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_rec(&"testHardBreakAfterClustersInDpCommit");
        let s = ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_solve(&ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_han(4, Some(20 as f64 as f64)).unwrap(), 50 as f64, None, Some(vec![1]),
None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::MandatoryBreak), &(s.lines[0usize].end_reason), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::ParagraphEnd), &(s.lines[1usize].end_reason), None).unwrap();
    });
}

#[test]
fn test_candidate_ends_window_below_line_start() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerCoverage2Test.testCandidateEndsWindowBelowLineStart", "org.tiqian.layout.ParagraphDpLineBreakerCoverage2Test.testCandidateEndsWindowBelowLineStart", || {
        ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_rec(&"testCandidateEndsWindowBelowLineStart");
        let s = ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_solve(&ParagraphDpLineBreakerCoverage2TestSupport::paragraph_dp_line_breaker_coverage2_test_support_han(3, Some(20 as f64 as f64)).unwrap(), 25 as f64, None, None, None,
None, None, Some(5)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}
