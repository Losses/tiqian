#![cfg(test)]

use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::layout::line_optimization::LineCandidate;
use crate::org::tiqian::layout::line_optimization::RepairOptions;
use crate::org::tiqian::layout::paragraph_dp_line_breaker_test_support::ParagraphDpLineBreakerTestSupport;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkChannel;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::UnbreakableRanges;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestTrailingMandatoryBreakEmitsParagraphEndLineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestTilesAllClustersInOrderFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerTestTilesAllClustersInOrderFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestTilesAllClustersInOrderFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestTilesAllClustersInOrderFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestTilesAllClustersInOrderFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestTilesAllClustersInOrderFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestTilesAllClustersInOrderFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestTilesAllClustersInOrderFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestTilesAllClustersInOrderFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestTilesAllClustersInOrderFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestTilesAllClustersInOrderFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestTilesAllClustersInOrderFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestTilesAllClustersInOrderFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestTilesAllClustersInOrderFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestTilesAllClustersInOrderFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestTilesAllClustersInOrderFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestSingleLineWhenEverythingFitsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestOverWideSingleClusterStillProgressesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestNeverBreaksInsideUnbreakableRangeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestMandatoryBreakBindsControlToPreviousLineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestKinsokuAvoidanceRoutesAroundForbiddenLineStartFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestCompressionEdgeRecordsPushInRepairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestCompressionDisabledWithoutPushInFlagFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault) -> Self {
        match value {
            ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphDpLineBreakerTestCompressedSameTierBoundaryIsNotReportedAsPromotionFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn compressed_same_tier_boundary_is_not_reported_as_promotion() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.compressedSameTierBoundaryIsNotReportedAsPromotion", "org.tiqian.layout.ParagraphDpLineBreakerTest.compressedSameTierBoundaryIsNotReportedAsPromotion", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(&"compressedSameTierBoundaryIsNotReportedAsPromotion");
        let cs = vec![
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(0, &"a", 30 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(1, &"/", 30 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(2, &"b", 25 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(3, &"c", 30 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(4, &"d", 30 as f64).unwrap()).clone(),
];
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve(&cs, 80 as f64, Some(vec![
    (ShrinkOpportunity::new(2u32, 2u32, 5 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
]), None, Some(true), None, Some(ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_opportunities(&vec![2, 3], &vec![(TextRange::new(0u32, 5u32).unwrap()).clone(), (TextRange::new(0u32, 5u32).unwrap()).clone()], &vec![ProgressiveBreakTier::Emergency,
ProgressiveBreakTier::Emergency])), None, Some(vec![]), Some(8.0f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 2u32), ((s.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(match &((s.lines[0usize]).clone().repair) { Some(__option1) => (RepairOptions::repair_options_reason((*__option1).clone())).starts_with(&"LineAdjustmentPushIn"), None => false }, None).unwrap();
    });
}

#[test]
fn tiles_all_clusters_in_order() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.tilesAllClustersInOrder", "org.tiqian.layout.ParagraphDpLineBreakerTest.tilesAllClustersInOrder", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(&"tilesAllClustersInOrder");
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve_test_defaults(&ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_han(23, None).unwrap(), 100 as f64, None, None, None, None, None, None).unwrap();
        let _ = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_tiles((s).clone(), 23).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"ParagraphEnd", s.lines[usize::try_from(u32::wrapping_sub(u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)].end_reason.name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn single_line_when_everything_fits() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.singleLineWhenEverythingFits", "org.tiqian.layout.ParagraphDpLineBreakerTest.singleLineWhenEverythingFits", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(&"singleLineWhenEverythingFits");
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve_test_defaults(&ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_han(4, None).unwrap(), 400 as f64, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"ParagraphEnd", s.lines[0usize].end_reason.name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn mandatory_break_binds_control_to_previous_line() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.mandatoryBreakBindsControlToPreviousLine", "org.tiqian.layout.ParagraphDpLineBreakerTest.mandatoryBreakBindsControlToPreviousLine", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(&"mandatoryBreakBindsControlToPreviousLine");
        let cs = vec![
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(0, &"中", 16 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(1, &"中", 16 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(2, &concat!("\n",
""), 0 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(3, &"中", 16 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(4, &"中", 16 as f64).unwrap()).clone(),
];
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve_test_defaults(&cs, 200 as f64, None, Some(vec![2]), None, None, None, None).unwrap();
        let _ = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_tiles((s).clone(), 5).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"MandatoryBreak", s.lines[0usize].end_reason.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 2u32), ((s.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"ParagraphEnd", s.lines[1usize].end_reason.name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn trailing_mandatory_break_emits_paragraph_end_line() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.trailingMandatoryBreakEmitsParagraphEndLine", "org.tiqian.layout.ParagraphDpLineBreakerTest.trailingMandatoryBreakEmitsParagraphEndLine", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(&"trailingMandatoryBreakEmitsParagraphEndLine");
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve_test_defaults(&vec![
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(0, &"中", 16 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(1, &concat!("\n",
""), 0 as f64).unwrap()).clone(),
], 200 as f64, None, Some(vec![1]), None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"MandatoryBreak", s.lines[0usize].end_reason.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"ParagraphEnd", s.lines[usize::try_from(u32::wrapping_sub(u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)].end_reason.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((s.lines[usize::try_from(u32::wrapping_sub(u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone().cluster_range.start).to_ne_bytes())) >
(i32::from_ne_bytes(((s.lines[usize::try_from(u32::wrapping_sub(u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone().cluster_range.end).to_ne_bytes())), None).unwrap();
    });
}

#[test]
fn never_breaks_inside_unbreakable_range() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.neverBreaksInsideUnbreakableRange", "org.tiqian.layout.ParagraphDpLineBreakerTest.neverBreaksInsideUnbreakableRange", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(&"neverBreaksInsideUnbreakableRange");
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve_test_defaults(&ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_han(10, None).unwrap(), 64 as f64, None, None, None,
Some(UnbreakableRanges::new(vec![(IntRange::new(3u32, 6u32)).clone()].to_vec())), None, None).unwrap();
        let _ = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_tiles((s).clone(), 10).unwrap();
        {
            let _g1 = s.lines.clone();
            for l in &_g1 {
                if i32::from_ne_bytes(((l.cluster_range).clone().start).to_ne_bytes()) <= i32::from_ne_bytes(((l.cluster_range).clone().end).to_ne_bytes()) {
                    let _ = TracedAssertions::traced_assertions_assert_true(!((i32::from_ne_bytes(((l.cluster_range).clone().start).to_ne_bytes())) >= 4 && (i32::from_ne_bytes(((l.cluster_range).clone().start).to_ne_bytes())) <= 6), Some((format!("{}{}{}{}",
            "break inside unbreakable range: ",
            crate::runtime::int_text::IntText::int_text((l.cluster_range).clone().start),
            "..",
            crate::runtime::int_text::IntText::int_text((l.cluster_range).clone().end)
        )).to_string())).unwrap();
                }
            }
        }
    });
}

#[test]
fn kinsoku_avoidance_routes_around_forbidden_line_start() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.kinsokuAvoidanceRoutesAroundForbiddenLineStart", "org.tiqian.layout.ParagraphDpLineBreakerTest.kinsokuAvoidanceRoutesAroundForbiddenLineStart", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(&"kinsokuAvoidanceRoutesAroundForbiddenLineStart");
        let mut cs = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_han(7, None).unwrap();
        cs[6usize] = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(6, &"。", 16 as f64).unwrap();
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve_test_defaults(&cs, 48 as f64, None, None, None, None, None, Some(vec![6])).unwrap();
        let _ = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_tiles((s).clone(), 7).unwrap();
        {
            let _g1 = s.lines.clone();
            for l in &_g1 {
                if i32::from_ne_bytes(((l.cluster_range).clone().start).to_ne_bytes()) <= i32::from_ne_bytes(((l.cluster_range).clone().end).to_ne_bytes()) {
                    let _ = TracedAssertions::traced_assertions_assert_true((l.cluster_range).clone().start != 6 || (l.repair).clone() != None, Some("。 must not start a line without a recorded repair".to_string())).unwrap();
                }
            }
        }
    });
}

#[test]
fn compression_edge_records_push_in_repair() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.compressionEdgeRecordsPushInRepair", "org.tiqian.layout.ParagraphDpLineBreakerTest.compressionEdgeRecordsPushInRepair", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(&"compressionEdgeRecordsPushInRepair");
        let mut cs = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_han(7, None).unwrap();
        cs[3usize] = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(3, &"，", 16 as f64).unwrap();
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve_test_defaults(&cs, 56 as f64, Some(vec![
    (ShrinkOpportunity::new(3u32, 5u32, 8 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
]), None, Some(true), None, None, None).unwrap();
        let _ = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_tiles((s).clone(), 7).unwrap();
        let mut compressed: Option<LineCandidate> = None;
        {
            let _g1 = s.lines.clone();
            for l in &_g1 {
                if compressed.is_none() && ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_push_in_reason((l.repair).clone()).is_some() {
                    compressed = Some(l.clone());
                }
            }
        }
        if compressed.is_none() {
            let _ = TracedAssertions::traced_assertions_assert_true(false, Some((format!("{}{}",
            "expected a PushIn-compressed line, got ",
            ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_repairs_string((s).clone())
        )).to_string())).unwrap();
            return;
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 3u32), ((compressed).as_ref().unwrap().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((compressed).as_ref().unwrap().adjusted_width) <= 56.01f64, Some("compressed line must fit the measure".to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_push_in_reason(((compressed).as_ref().unwrap().repair).clone())).as_ref().unwrap()).starts_with(&"LineAdjustmentPushIn"),
Some("compression must be recorded as the fill-pass reason code".to_string())).unwrap();
    });
}

#[test]
fn compression_disabled_without_push_in_flag() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.compressionDisabledWithoutPushInFlag", "org.tiqian.layout.ParagraphDpLineBreakerTest.compressionDisabledWithoutPushInFlag", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(&"compressionDisabledWithoutPushInFlag");
        let mut cs = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_han(5, None).unwrap();
        cs[3usize] = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(3, &"，", 16 as f64).unwrap();
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve_test_defaults(&cs, 56 as f64, Some(vec![
    (ShrinkOpportunity::new(3u32, 5u32, 8 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
]), None, Some(false), None, None, None).unwrap();
        let _ = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_tiles((s).clone(), 5).unwrap();
        let mut none = true;
        {
            let _g1 = s.lines.clone();
            for l in &_g1 {
                let r = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_push_in_reason((l.repair).clone());
                match &(r) {
                    Some(__option2) => {
                        if __option2.starts_with(&"LineAdjustmentPushIn") {
                        none = false;
                        }
                    }
                    None => {
                    }
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none, Some("PushOutOnly must not produce fill push-ins".to_string())).unwrap();
    });
}

#[test]
fn over_wide_single_cluster_still_progresses() {
    testlib::run("org.tiqian.layout.ParagraphDpLineBreakerTest.overWideSingleClusterStillProgresses", "org.tiqian.layout.ParagraphDpLineBreakerTest.overWideSingleClusterStillProgresses", || {
        ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_rec(&"overWideSingleClusterStillProgresses");
        let s = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve_test_defaults(&vec![
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(0, &"中", 16 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(1, &"Ｗ", 300 as f64).unwrap()).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_c(2, &"中", 16 as f64).unwrap()).clone(),
], 48 as f64, None, None, None, None, None, None).unwrap();
        let _ = ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_tiles((s).clone(), 3).unwrap();
    });
}
