#![cfg(test)]

use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakDecisions;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::progressive_break_decisions_coverage_test_support::ProgressiveBreakDecisionsCoverageTestSupport;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault) -> Self {
        match value {
            ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn defaults_admit_the_clean_tier_without_geometry_inputs() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.defaultsAdmitTheCleanTierWithoutGeometryInputs", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.defaultsAdmitTheCleanTierWithoutGeometryInputs", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(&"defaultsAdmitTheCleanTierWithoutGeometryInputs", {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        bo.put(&(1), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Whitespace, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_span().unwrap(), None)));
        bo.put(&(2), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Emergency, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_span().unwrap(), None)));
        let o: SortedMapTable<u32, ProgressiveBreakOpportunity> = bo.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(0, 2, (o).clone(), None, None, None, None, None, None), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_allowed(0, 2, 3, (o).clone(), None, None, None, None, None, None), None).unwrap();
}) });
    });
}

#[test]
fn line_start_at_the_overflow_boundary_scans_an_empty_range() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.lineStartAtTheOverflowBoundaryScansAnEmptyRange", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.lineStartAtTheOverflowBoundaryScansAnEmptyRange", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(&"lineStartAtTheOverflowBoundaryScansAnEmptyRange", {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        bo.put(&(2), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Emergency, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_span().unwrap(), None)));
        let o: SortedMapTable<u32, ProgressiveBreakOpportunity> = bo.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(2, 2, (o).clone(), None, None, None, None, None, None), None).unwrap();
}) });
    });
}

#[test]
fn two_same_tier_boundaries_pick_the_rightmost() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.twoSameTierBoundariesPickTheRightmost", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.twoSameTierBoundariesPickTheRightmost", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(&"twoSameTierBoundariesPickTheRightmost", {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        bo.put(&(2), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Whitespace, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_span().unwrap(), None)));
        bo.put(&(4), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Whitespace, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_span().unwrap(), None)));
        let o: SortedMapTable<u32, ProgressiveBreakOpportunity> = bo.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(0, 4, (o).clone(), Some(vec![
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(0, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(1, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(2, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(3, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(4, None, None).unwrap()).clone(),
]), Some(64 as f64 as f64), None, Some(8 as f64 as f64), None, None), None).unwrap();
}) });
    });
}

#[test]
fn visibly_loose_clean_tiers_fall_through_to_emergency() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.visiblyLooseCleanTiersFallThroughToEmergency", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.visiblyLooseCleanTiersFallThroughToEmergency", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(&"visiblyLooseCleanTiersFallThroughToEmergency", {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        bo.put(&(2), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Whitespace, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_span().unwrap(), None)));
        bo.put(&(4), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Emergency, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_span().unwrap(), None)));
        let o: SortedMapTable<u32, ProgressiveBreakOpportunity> = bo.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(0, 4, (o).clone(), Some(vec![
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(0, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(1, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(2, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(3, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(4, None, None).unwrap()).clone(),
]), Some(200 as f64 as f64), None, Some(8 as f64 as f64), None, None), None).unwrap();
}) });
    });
}

#[test]
fn a_leftward_emergency_boundary_keeps_the_best_clean_tier() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.aLeftwardEmergencyBoundaryKeepsTheBestCleanTier", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.aLeftwardEmergencyBoundaryKeepsTheBestCleanTier", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(&"aLeftwardEmergencyBoundaryKeepsTheBestCleanTier", {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        bo.put(&(2), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Emergency, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_span().unwrap(), None)));
        bo.put(&(4), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Whitespace, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_span().unwrap(), None)));
        let o: SortedMapTable<u32, ProgressiveBreakOpportunity> = bo.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(0, 4, (o).clone(), Some(vec![
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(0, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(1, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(2, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(3, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(4, None, None).unwrap()).clone(),
]), Some(200 as f64 as f64), None, Some(8 as f64 as f64), None, None), None).unwrap();
}) });
    });
}

#[test]
fn span_edge_and_whitespace_clusters_do_not_count_as_technical_units() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.spanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnits", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.spanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnits", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(&"spanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnits", {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        let s = TextRange::new(1u32, 4u32).unwrap();
        bo.put(&(2), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Whitespace, (s).clone(), None)));
        bo.put(&(3), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Emergency, (s).clone(), None)));
        let o: SortedMapTable<u32, ProgressiveBreakOpportunity> = bo.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(0, 3, (o).clone(), Some(vec![
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(0, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(1, Some(" ".to_string()), None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(2, Some("a".to_string()), None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(3, Some("b".to_string()), None).unwrap()).clone(),
]), Some(200 as f64 as f64), None, Some(8 as f64 as f64), None, None), None).unwrap();
}) });
    });
}

#[test]
fn single_technical_unit_falls_back_to_the_cjk_gap_density() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.singleTechnicalUnitFallsBackToTheCjkGapDensity", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.singleTechnicalUnitFallsBackToTheCjkGapDensity", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(&"singleTechnicalUnitFallsBackToTheCjkGapDensity", {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        let s = TextRange::new(0u32, 1u32).unwrap();
        bo.put(&(1), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Whitespace, (s).clone(), None)));
        bo.put(&(2), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Emergency, (s).clone(), None)));
        let o: SortedMapTable<u32, ProgressiveBreakOpportunity> = bo.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(0, 2, (o).clone(), Some(vec![
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(0, Some("a".to_string()), None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(1, Some("b".to_string()), None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(2, Some("c".to_string()), None).unwrap()).clone(),
]), Some(200 as f64 as f64), Some(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![1])), Some(8 as f64 as f64), None, None), None).unwrap();
}) });
    });
}

#[test]
fn candidate_outside_the_cluster_list_is_allowed() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.candidateOutsideTheClusterListIsAllowed", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.candidateOutsideTheClusterListIsAllowed", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(&"candidateOutsideTheClusterListIsAllowed", {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        bo.put(&(1), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Emergency, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_span().unwrap(), None)));
        let o: SortedMapTable<u32, ProgressiveBreakOpportunity> = bo.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_true(ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_allowed(0, 1, 5, (o).clone(), Some(vec![
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(0, None, None).unwrap()).clone(),
]), None, None, None, None, None), None).unwrap();
}) });
    });
}

#[test]
fn candidates_outside_the_active_span_are_allowed() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.candidatesOutsideTheActiveSpanAreAllowed", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.candidatesOutsideTheActiveSpanAreAllowed", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(&"candidatesOutsideTheActiveSpanAreAllowed", {  Arc::new(move || {
        let cs = vec![
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(0, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(1, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(2, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(3, None, None).unwrap()).clone(),
];
        let a = TextRange::new(5u32, 10u32).unwrap();
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        bo.put(&(1), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Emergency, (a).clone(), None)));
        let o: SortedMapTable<u32, ProgressiveBreakOpportunity> = bo.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_true(ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_allowed(0, 1, 2, (o).clone(), Some((cs).clone()), None, None, None, None, None), None).unwrap();
        let mut bt: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        bt.put(&(1), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Emergency, TextRange::new(0u32, 2u32).unwrap(), None)));
        let t: SortedMapTable<u32, ProgressiveBreakOpportunity> = bt.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_true(ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_allowed(0, 1, 2, (t).clone(), Some((cs).clone()), None, None, None, None, None), None).unwrap();
        let mut bz: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        bz.put(&(1), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Emergency, TextRange::new(0u32, 4u32).unwrap(), None)));
        let z: SortedMapTable<u32, ProgressiveBreakOpportunity> = bz.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_false(ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_allowed(0, 1, 2, (z).clone(), Some((cs).clone()), None, None, None, None, None), None).unwrap();
}) });
    });
}

#[test]
fn candidates_of_a_different_span_are_allowed() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.candidatesOfADifferentSpanAreAllowed", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.candidatesOfADifferentSpanAreAllowed", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(&"candidatesOfADifferentSpanAreAllowed", {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        bo.put(&(1), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Emergency, TextRange::new(0u32, 2u32).unwrap(), None)));
        bo.put(&(3), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Whitespace, TextRange::new(2u32, 6u32).unwrap(), None)));
        let o: SortedMapTable<u32, ProgressiveBreakOpportunity> = bo.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_true(ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_allowed(0, 1, 3, (o).clone(), None, None, None, None, None, None), None).unwrap();
}) });
    });
}

#[test]
fn same_tier_past_the_raw_greedy_is_allowed_and_worse_tiers_are_not() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.sameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNot", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.sameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNot", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(&"sameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNot", {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        bo.put(&(2), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Whitespace, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_span().unwrap(), None)));
        bo.put(&(3), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Whitespace, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_span().unwrap(), None)));
        bo.put(&(4), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Emergency, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_span().unwrap(), None)));
        let o: SortedMapTable<u32, ProgressiveBreakOpportunity> = bo.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_true(ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_allowed(0, 2, 3, (o).clone(), None, None, None, None, None, None), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_allowed(0, 2, 4, (o).clone(), None, None, None, None, None, None), None).unwrap();
}) });
    });
}

#[test]
fn candidates_before_the_raw_greedy_must_match_the_selected_boundary() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.candidatesBeforeTheRawGreedyMustMatchTheSelectedBoundary", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.candidatesBeforeTheRawGreedyMustMatchTheSelectedBoundary", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(&"candidatesBeforeTheRawGreedyMustMatchTheSelectedBoundary", {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        bo.put(&(1), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Whitespace, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_span().unwrap(), None)));
        bo.put(&(2), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Whitespace, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_span().unwrap(), None)));
        bo.put(&(3), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Emergency, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_span().unwrap(), None)));
        let o: SortedMapTable<u32, ProgressiveBreakOpportunity> = bo.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_true(ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_allowed(0, 3, 2, (o).clone(), None, None, None, None, None, None), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_allowed(0, 3, 1, (o).clone(), None, None, None, None, None, None), None).unwrap();
}) });
    });
}

#[test]
fn hyphen_break_returns_overflow_at_plain_word_boundaries() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.hyphenBreakReturnsOverflowAtPlainWordBoundaries", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.hyphenBreakReturnsOverflowAtPlainWordBoundaries", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(&"hyphenBreakReturnsOverflowAtPlainWordBoundaries", {  Arc::new(move || {
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, ProgressiveBreakDecisions::progressive_break_decisions_decide_hyphen_break(0, 1, &vec![
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(0, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(1, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(2, None, None).unwrap()).clone(),
], 16 as f64, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![]), ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![]), 8 as f64, None, None), None).unwrap();
}) });
    });
}

#[test]
fn over_long_words_must_hyphenate_from_the_line_start() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.overLongWordsMustHyphenateFromTheLineStart", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.overLongWordsMustHyphenateFromTheLineStart", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(&"overLongWordsMustHyphenateFromTheLineStart", {  Arc::new(move || {
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, ProgressiveBreakDecisions::progressive_break_decisions_decide_hyphen_break(0, 2, &vec![
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(0, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(1, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(2, None, None).unwrap()).clone(),
], 48 as f64, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![0, 1, 2]), ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![]), 8 as f64, None, None), None).unwrap();
}) });
    });
}

#[test]
fn a_fitting_whole_word_breaks_there() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.aFittingWholeWordBreaksThere", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.aFittingWholeWordBreaksThere", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(&"aFittingWholeWordBreaksThere", {  Arc::new(move || {
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, ProgressiveBreakDecisions::progressive_break_decisions_decide_hyphen_break(0, 2, &vec![
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(0, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(1, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(2, None, None).unwrap()).clone(),
], 16 as f64, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![2]), ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![]), 8 as f64, None, None), None).unwrap();
}) });
    });
}

#[test]
fn sino_western_gaps_absorbing_the_deficit_keep_the_whole_word() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.sinoWesternGapsAbsorbingTheDeficitKeepTheWholeWord", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.sinoWesternGapsAbsorbingTheDeficitKeepTheWholeWord", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(&"sinoWesternGapsAbsorbingTheDeficitKeepTheWholeWord", {  Arc::new(move || {
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_hy(&"x", 40 as f64,
ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![1]), Some(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![1])), Some(8 as f64 as f64)).unwrap(), None).unwrap();
}) });
    });
}

#[test]
fn gapless_or_too_loose_lines_hyphenate_instead() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.gaplessOrTooLooseLinesHyphenateInstead", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.gaplessOrTooLooseLinesHyphenateInstead", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(&"gaplessOrTooLooseLinesHyphenateInstead", {  Arc::new(move || {
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_hy(&"x", 60 as f64,
ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![2]), None, None).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_hy(&"x", 100 as f64,
ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![1]), None, None).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, ProgressiveBreakDecisions::progressive_break_decisions_decide_hyphen_break(0, 3, &vec![
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(0, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(1, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(2, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(3, None, None).unwrap()).clone(),
], 36 as f64, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![3]), ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![1]), 8 as f64, None, None), None).unwrap();
}) });
    });
}
