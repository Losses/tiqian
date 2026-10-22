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
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestVisiblyLooseCleanTiersFallThroughToEmergencyFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestTwoSameTierBoundariesPickTheRightmostFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestSpanEdgeAndWhitespaceClustersDoNotCountAsTechnicalUnitsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestSinoWesternGapsAbsorbingTheDeficitKeepTheWholeWordFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestSingleTechnicalUnitFallsBackToTheCjkGapDensityFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestSameTierPastTheRawGreedyIsAllowedAndWorseTiersAreNotFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestOverLongWordsMustHyphenateFromTheLineStartFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestLineStartAtTheOverflowBoundaryScansAnEmptyRangeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestHyphenBreakReturnsOverflowAtPlainWordBoundariesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestGaplessOrTooLooseLinesHyphenateInsteadFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestDefaultsAdmitTheCleanTierWithoutGeometryInputsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestCandidatesOutsideTheActiveSpanAreAllowedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestCandidatesOfADifferentSpanAreAllowedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestCandidatesBeforeTheRawGreedyMustMatchTheSelectedBoundaryFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestCandidateOutsideTheClusterListIsAllowedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestALeftwardEmergencyBoundaryKeepsTheBestCleanTierFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            ProgressiveBreakDecisionsCoverageTestAFittingWholeWordBreaksThereFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(UStr::new(&[100,101,102,97,117,108,116,115,65,100,109,105,116,84,104,101,67,108,101,97,110,84,105,101,114,87,105,116,104,111,117,116,71,101,111,109,101,116,114,121,73,110,112,117,116,115]), {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
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
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(UStr::new(&[108,105,110,101,83,116,97,114,116,65,116,84,104,101,79,118,101,114,102,108,111,119,66,111,117,110,100,97,114,121,83,99,97,110,115,65,110,69,109,112,116,121,82,97,110,103,101]), {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        bo.put(&(2), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Emergency, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_span().unwrap(), None)));
        let o: SortedMapTable<u32, ProgressiveBreakOpportunity> = bo.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(2, 2, (o).clone(), None, None, None, None, None, None), None).unwrap();
}) });
    });
}

#[test]
fn two_same_tier_boundaries_pick_the_rightmost() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.twoSameTierBoundariesPickTheRightmost", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.twoSameTierBoundariesPickTheRightmost", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(UStr::new(&[116,119,111,83,97,109,101,84,105,101,114,66,111,117,110,100,97,114,105,101,115,80,105,99,107,84,104,101,82,105,103,104,116,109,111,115,116]), {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
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
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(UStr::new(&[118,105,115,105,98,108,121,76,111,111,115,101,67,108,101,97,110,84,105,101,114,115,70,97,108,108,84,104,114,111,117,103,104,84,111,69,109,101,114,103,101,110,99,121]), {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
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
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(UStr::new(&[97,76,101,102,116,119,97,114,100,69,109,101,114,103,101,110,99,121,66,111,117,110,100,97,114,121,75,101,101,112,115,84,104,101,66,101,115,116,67,108,101,97,110,84,105,101,114]), {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
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
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(UStr::new(&[115,112,97,110,69,100,103,101,65,110,100,87,104,105,116,101,115,112,97,99,101,67,108,117,115,116,101,114,115,68,111,78,111,116,67,111,117,110,116,65,115,84,101,99,104,110,105,99,97,108,85,110,105,116,115]), {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        let s = TextRange::new(1u32, 4u32).unwrap();
        bo.put(&(2), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Whitespace, (s).clone(), None)));
        bo.put(&(3), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Emergency, (s).clone(), None)));
        let o: SortedMapTable<u32, ProgressiveBreakOpportunity> = bo.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(0, 3, (o).clone(), Some(vec![
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(0, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(1, Some(UString::from(" ")), None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(2, Some(UString::from("a")), None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(3, Some(UString::from("b")), None).unwrap()).clone(),
]), Some(200 as f64 as f64), None, Some(8 as f64 as f64), None, None), None).unwrap();
}) });
    });
}

#[test]
fn single_technical_unit_falls_back_to_the_cjk_gap_density() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.singleTechnicalUnitFallsBackToTheCjkGapDensity", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.singleTechnicalUnitFallsBackToTheCjkGapDensity", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(UStr::new(&[115,105,110,103,108,101,84,101,99,104,110,105,99,97,108,85,110,105,116,70,97,108,108,115,66,97,99,107,84,111,84,104,101,67,106,107,71,97,112,68,101,110,115,105,116,121]), {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        let s = TextRange::new(0u32, 1u32).unwrap();
        bo.put(&(1), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Whitespace, (s).clone(), None)));
        bo.put(&(2), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Emergency, (s).clone(), None)));
        let o: SortedMapTable<u32, ProgressiveBreakOpportunity> = bo.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(0, 2, (o).clone(), Some(vec![
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(0, Some(UString::from("a")), None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(1, Some(UString::from("b")), None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(2, Some(UString::from("c")), None).unwrap()).clone(),
]), Some(200 as f64 as f64), Some(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![1])), Some(8 as f64 as f64), None, None), None).unwrap();
}) });
    });
}

#[test]
fn candidate_outside_the_cluster_list_is_allowed() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.candidateOutsideTheClusterListIsAllowed", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.candidateOutsideTheClusterListIsAllowed", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(UStr::new(&[99,97,110,100,105,100,97,116,101,79,117,116,115,105,100,101,84,104,101,67,108,117,115,116,101,114,76,105,115,116,73,115,65,108,108,111,119,101,100]), {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
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
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(UStr::new(&[99,97,110,100,105,100,97,116,101,115,79,117,116,115,105,100,101,84,104,101,65,99,116,105,118,101,83,112,97,110,65,114,101,65,108,108,111,119,101,100]), {  Arc::new(move || {
        let cs = vec![
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(0, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(1, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(2, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(3, None, None).unwrap()).clone(),
];
        let a = TextRange::new(5u32, 10u32).unwrap();
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        bo.put(&(1), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Emergency, (a).clone(), None)));
        let o: SortedMapTable<u32, ProgressiveBreakOpportunity> = bo.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_true(ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_allowed(0, 1, 2, (o).clone(), Some((cs).clone()), None, None, None, None, None), None).unwrap();
        let mut bt: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        bt.put(&(1), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Emergency, TextRange::new(0u32, 2u32).unwrap(), None)));
        let t: SortedMapTable<u32, ProgressiveBreakOpportunity> = bt.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_true(ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_allowed(0, 1, 2, (t).clone(), Some((cs).clone()), None, None, None, None, None), None).unwrap();
        let mut bz: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        bz.put(&(1), &(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_op(ProgressiveBreakTier::Emergency, TextRange::new(0u32, 4u32).unwrap(), None)));
        let z: SortedMapTable<u32, ProgressiveBreakOpportunity> = bz.clone().build();
        let _ = TracedAssertions::traced_assertions_assert_false(ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_allowed(0, 1, 2, (z).clone(), Some((cs).clone()), None, None, None, None, None), None).unwrap();
}) });
    });
}

#[test]
fn candidates_of_a_different_span_are_allowed() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.candidatesOfADifferentSpanAreAllowed", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.candidatesOfADifferentSpanAreAllowed", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(UStr::new(&[99,97,110,100,105,100,97,116,101,115,79,102,65,68,105,102,102,101,114,101,110,116,83,112,97,110,65,114,101,65,108,108,111,119,101,100]), {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
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
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(UStr::new(&[115,97,109,101,84,105,101,114,80,97,115,116,84,104,101,82,97,119,71,114,101,101,100,121,73,115,65,108,108,111,119,101,100,65,110,100,87,111,114,115,101,84,105,101,114,115,65,114,101,78,111,116]), {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
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
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(UStr::new(&[99,97,110,100,105,100,97,116,101,115,66,101,102,111,114,101,84,104,101,82,97,119,71,114,101,101,100,121,77,117,115,116,77,97,116,99,104,84,104,101,83,101,108,101,99,116,101,100,66,111,117,110,100,97,114,121]), {  Arc::new(move || {
        let mut bo: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
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
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(UStr::new(&[104,121,112,104,101,110,66,114,101,97,107,82,101,116,117,114,110,115,79,118,101,114,102,108,111,119,65,116,80,108,97,105,110,87,111,114,100,66,111,117,110,100,97,114,105,101,115]), {  Arc::new(move || {
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
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(UStr::new(&[111,118,101,114,76,111,110,103,87,111,114,100,115,77,117,115,116,72,121,112,104,101,110,97,116,101,70,114,111,109,84,104,101,76,105,110,101,83,116,97,114,116]), {  Arc::new(move || {
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
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(UStr::new(&[97,70,105,116,116,105,110,103,87,104,111,108,101,87,111,114,100,66,114,101,97,107,115,84,104,101,114,101]), {  Arc::new(move || {
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
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(UStr::new(&[115,105,110,111,87,101,115,116,101,114,110,71,97,112,115,65,98,115,111,114,98,105,110,103,84,104,101,68,101,102,105,99,105,116,75,101,101,112,84,104,101,87,104,111,108,101,87,111,114,100]), {  Arc::new(move || {
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_hy(UStr::new(&[120]), 40 as f64, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![1]), Some(ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![1])), Some(8 as f64 as f64)).unwrap(), None).unwrap();
}) });
    });
}

#[test]
fn gapless_or_too_loose_lines_hyphenate_instead() {
    testlib::run("org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.gaplessOrTooLooseLinesHyphenateInstead", "org.tiqian.layout.ProgressiveBreakDecisionsCoverageTest.gaplessOrTooLooseLinesHyphenateInstead", || {
        ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_run(UStr::new(&[103,97,112,108,101,115,115,79,114,84,111,111,76,111,111,115,101,76,105,110,101,115,72,121,112,104,101,110,97,116,101,73,110,115,116,101,97,100]), {  Arc::new(move || {
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_hy(UStr::new(&[120]), 60 as f64, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![2]), None, None).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_hy(UStr::new(&[120]), 100 as f64, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![1]), None, None).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, ProgressiveBreakDecisions::progressive_break_decisions_decide_hyphen_break(0, 3, &vec![
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(0, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(1, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(2, None, None).unwrap()).clone(),
    (ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_c(3, None, None).unwrap()).clone(),
], 36 as f64, ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![3]), ProgressiveBreakDecisionsCoverageTestSupport::progressive_break_decisions_coverage_test_support_m(&vec![1]), 8 as f64, None, None), None).unwrap();
}) });
    });
}
