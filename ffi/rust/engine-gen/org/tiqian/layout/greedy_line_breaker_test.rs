#![cfg(test)]

use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::kinsoku_rule::KinsokuRule;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::line_optimization::PushInAllocation;
use crate::org::tiqian::layout::line_optimization::RepairCandidate;
use crate::org::tiqian::layout::line_optimization::RepairOption;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkChannel;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestSingleClusterFitsOnOneLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<GreedyLineBreakerTestSingleClusterFitsOnOneLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: GreedyLineBreakerTestSingleClusterFitsOnOneLineFault) -> Self {
        match value {
            GreedyLineBreakerTestSingleClusterFitsOnOneLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestSingleClusterFitsOnOneLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: GreedyLineBreakerTestSingleClusterFitsOnOneLineFault) -> Self {
        match value {
            GreedyLineBreakerTestSingleClusterFitsOnOneLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestSingleClusterFitsOnOneLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: GreedyLineBreakerTestSingleClusterFitsOnOneLineFault) -> Self {
        match value {
            GreedyLineBreakerTestSingleClusterFitsOnOneLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for GreedyLineBreakerTestSingleClusterFitsOnOneLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        GreedyLineBreakerTestSingleClusterFitsOnOneLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for GreedyLineBreakerTestSingleClusterFitsOnOneLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        GreedyLineBreakerTestSingleClusterFitsOnOneLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for GreedyLineBreakerTestSingleClusterFitsOnOneLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        GreedyLineBreakerTestSingleClusterFitsOnOneLineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault) -> Self {
        match value {
            GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault) -> Self {
        match value {
            GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault) -> Self {
        match value {
            GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault) -> Self {
        match value {
            GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault) -> Self {
        match value {
            GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault) -> Self {
        match value {
            GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault) -> Self {
        match value {
            GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault) -> Self {
        match value {
            GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault) -> Self {
        match value {
            GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestMisalignedClusterListsThrowFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<GreedyLineBreakerTestMisalignedClusterListsThrowFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: GreedyLineBreakerTestMisalignedClusterListsThrowFault) -> Self {
        match value {
            GreedyLineBreakerTestMisalignedClusterListsThrowFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestMisalignedClusterListsThrowFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: GreedyLineBreakerTestMisalignedClusterListsThrowFault) -> Self {
        match value {
            GreedyLineBreakerTestMisalignedClusterListsThrowFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestMisalignedClusterListsThrowFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: GreedyLineBreakerTestMisalignedClusterListsThrowFault) -> Self {
        match value {
            GreedyLineBreakerTestMisalignedClusterListsThrowFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for GreedyLineBreakerTestMisalignedClusterListsThrowFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        GreedyLineBreakerTestMisalignedClusterListsThrowFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for GreedyLineBreakerTestMisalignedClusterListsThrowFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        GreedyLineBreakerTestMisalignedClusterListsThrowFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for GreedyLineBreakerTestMisalignedClusterListsThrowFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        GreedyLineBreakerTestMisalignedClusterListsThrowFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault) -> Self {
        match value {
            GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault) -> Self {
        match value {
            GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault) -> Self {
        match value {
            GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault) -> Self {
        match value {
            GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault) -> Self {
        match value {
            GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault) -> Self {
        match value {
            GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault) -> Self {
        match value {
            GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault) -> Self {
        match value {
            GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault) -> Self {
        match value {
            GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault) -> Self {
        match value {
            GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault) -> Self {
        match value {
            GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault) -> Self {
        match value {
            GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault) -> Self {
        match value {
            GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault) -> Self {
        match value {
            GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault) -> Self {
        match value {
            GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault) -> Self {
        match value {
            GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault) -> Self {
        match value {
            GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault) -> Self {
        match value {
            GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault) -> Self {
        match value {
            GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault) -> Self {
        match value {
            GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault) -> Self {
        match value {
            GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault) -> Self {
        match value {
            GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault) -> Self {
        match value {
            GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault) -> Self {
        match value {
            GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault) -> Self {
        match value {
            GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault) -> Self {
        match value {
            GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault) -> Self {
        match value {
            GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault) -> Self {
        match value {
            GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault) -> Self {
        match value {
            GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault) -> Self {
        match value {
            GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestEmptyInputProducesNoLinesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<GreedyLineBreakerTestEmptyInputProducesNoLinesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: GreedyLineBreakerTestEmptyInputProducesNoLinesFault) -> Self {
        match value {
            GreedyLineBreakerTestEmptyInputProducesNoLinesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestEmptyInputProducesNoLinesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: GreedyLineBreakerTestEmptyInputProducesNoLinesFault) -> Self {
        match value {
            GreedyLineBreakerTestEmptyInputProducesNoLinesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestEmptyInputProducesNoLinesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: GreedyLineBreakerTestEmptyInputProducesNoLinesFault) -> Self {
        match value {
            GreedyLineBreakerTestEmptyInputProducesNoLinesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for GreedyLineBreakerTestEmptyInputProducesNoLinesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        GreedyLineBreakerTestEmptyInputProducesNoLinesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for GreedyLineBreakerTestEmptyInputProducesNoLinesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        GreedyLineBreakerTestEmptyInputProducesNoLinesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for GreedyLineBreakerTestEmptyInputProducesNoLinesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        GreedyLineBreakerTestEmptyInputProducesNoLinesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestDoesNotHangWhenDisabledFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<GreedyLineBreakerTestDoesNotHangWhenDisabledFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: GreedyLineBreakerTestDoesNotHangWhenDisabledFault) -> Self {
        match value {
            GreedyLineBreakerTestDoesNotHangWhenDisabledFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestDoesNotHangWhenDisabledFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: GreedyLineBreakerTestDoesNotHangWhenDisabledFault) -> Self {
        match value {
            GreedyLineBreakerTestDoesNotHangWhenDisabledFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestDoesNotHangWhenDisabledFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: GreedyLineBreakerTestDoesNotHangWhenDisabledFault) -> Self {
        match value {
            GreedyLineBreakerTestDoesNotHangWhenDisabledFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for GreedyLineBreakerTestDoesNotHangWhenDisabledFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        GreedyLineBreakerTestDoesNotHangWhenDisabledFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for GreedyLineBreakerTestDoesNotHangWhenDisabledFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        GreedyLineBreakerTestDoesNotHangWhenDisabledFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for GreedyLineBreakerTestDoesNotHangWhenDisabledFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        GreedyLineBreakerTestDoesNotHangWhenDisabledFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault) -> Self {
        match value {
            GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault) -> Self {
        match value {
            GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault) -> Self {
        match value {
            GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault) -> Self {
        match value {
            GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault) -> Self {
        match value {
            GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault) -> Self {
        match value {
            GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn empty_input_produces_no_lines() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.emptyInputProducesNoLines", "org.tiqian.layout.GreedyLineBreakerTest.emptyInputProducesNoLines", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(&"emptyInputProducesNoLines");
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&vec![], &vec![], 100 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn single_cluster_fits_on_one_line() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.singleClusterFitsOnOneLine", "org.tiqian.layout.GreedyLineBreakerTest.singleClusterFitsOnOneLine", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(&"singleClusterFitsOnOneLine");
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, &"中", 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 64 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let line = (solution.lines[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 0u32), (line.cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"TextRange(start=0, end=1)", GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_text_range((line.source_range).clone()).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, line.natural_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, line.adjusted_width, None).unwrap();
    });
}

#[test]
fn fills_line_until_overflow_then_starts_new_line() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.fillsLineUntilOverflowThenStartsNewLine", "org.tiqian.layout.GreedyLineBreakerTest.fillsLineUntilOverflowThenStartsNewLine", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(&"fillsLineUntilOverflowThenStartsNewLine");
        let clusters = GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_clusters_x(5).unwrap();
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 48 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 2u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(48 as f64, solution.lines[0usize].adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(3u32, 4u32), ((solution.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32 as f64, solution.lines[1usize].adjusted_width, None).unwrap();
    });
}

#[test]
fn natural_and_adjusted_widths_track_independently() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.naturalAndAdjustedWidthsTrackIndependently", "org.tiqian.layout.GreedyLineBreakerTest.naturalAndAdjustedWidthsTrackIndependently", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(&"naturalAndAdjustedWidthsTrackIndependently");
        let natural = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, &"，", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, &"。", 16 as f64, None).unwrap()).clone(),
];
        let adjusted = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, &"，", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, &"。", 12 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&natural, &adjusted, 64 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let line = (solution.lines[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32 as f64, line.natural_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(28 as f64, line.adjusted_width, None).unwrap();
    });
}

#[test]
fn cluster_wider_than_max_width_gets_own_line_rather_than_infinite_loop() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.clusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoop", "org.tiqian.layout.GreedyLineBreakerTest.clusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoop", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(&"clusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoop");
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, &"中", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 8, &"English", 112 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 80 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 0u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(1u32, 1u32), ((solution.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(112 as f64, solution.lines[1usize].adjusted_width, None).unwrap();
    });
}

#[test]
fn kinsoku_carry_previous_moves_prev_cluster_to_next_line() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.kinsokuCarryPreviousMovesPrevClusterToNextLine", "org.tiqian.layout.GreedyLineBreakerTest.kinsokuCarryPreviousMovesPrevClusterToNextLine", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(&"kinsokuCarryPreviousMovesPrevClusterToNextLine");
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, &"a", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, &"b", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, &"c", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(3, 4, &"。", 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 48 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(2u32, 3u32), ((solution.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32 as f64, solution.lines[0usize].adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32 as f64, solution.lines[1usize].adjusted_width, None).unwrap();
        let repair = (solution.lines[1usize]).clone().repair;
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"true", GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_is((repair).clone(), &"CarryPrevious").as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10 as f64, solution.total_badness, None).unwrap();
    });
}

#[test]
fn kinsoku_pushes_forbidden_punctuation_into_previous_line_when_glue_capacity_covers_overflow() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.kinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflow", "org.tiqian.layout.GreedyLineBreakerTest.kinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflow", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(&"kinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflow");
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, &"a", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, &"b", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, &"c", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(3, 4, &"。", 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 60 as f64, Some(vec![
    (ShrinkOpportunity::new(3u32, 6u32, 4 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
]), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let line = (solution.lines[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 3u32), (line.cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(64 as f64, line.natural_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(60 as f64, line.adjusted_width, None).unwrap();
        let repair = line.repair.clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"true", GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_is((repair).clone(), &"PushIn").as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(3, GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_push_in_offender_cluster_index((repair).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_push_in_total_shrink((repair).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_push_in_total_available_capacity((repair).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![3], &GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_allocation_cluster_indices(&GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_push_in_allocations((repair).clone())),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_push_in_allocations((repair).clone())[0usize].shrink, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_push_in_allocations((repair).clone())[0usize].available_capacity, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((line.repair_candidates.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"PushIn", ((line.repair_candidates[0usize]).clone().kind).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"true", if line.repair_candidates[0usize].accepted { "true".to_string() } else { "false".to_string() }.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, solution.total_badness, None).unwrap();
    });
}

#[test]
fn kinsoku_carries_previous_when_push_in_capacity_cannot_cover_overflow() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.kinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflow", "org.tiqian.layout.GreedyLineBreakerTest.kinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflow", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(&"kinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflow");
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, &"a", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, &"b", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, &"c", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(3, 4, &"。", 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 59 as f64, Some(vec![
    (ShrinkOpportunity::new(3u32, 6u32, 4 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
]), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(2u32, 3u32), ((solution.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"true", GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_is(((solution.lines[1usize]).clone().repair).clone(), &"CarryPrevious").as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from(((solution.lines[1usize]).clone().repair_candidates.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"PushIn", (((solution.lines[1usize]).clone().repair_candidates[0usize]).clone().kind).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"false", if solution.lines[1usize].clone().repair_candidates[0usize].accepted { "true".to_string() } else { "false".to_string() }.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"insufficient-capacity", (((solution.lines[1usize]).clone().repair_candidates[0usize]).clone().rejection_reason).as_deref().unwrap_or(""), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"CarryPrevious", (((solution.lines[1usize]).clone().repair_candidates[1usize]).clone().kind).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"true", if solution.lines[1usize].clone().repair_candidates[1usize].accepted { "true".to_string() } else { "false".to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn kinsoku_rejects_carry_previous_when_carried_line_would_overflow() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.kinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflow", "org.tiqian.layout.GreedyLineBreakerTest.kinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflow", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(&"kinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflow");
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, &"a", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, &"b", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, &"c", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(3, 4, &"d", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(4, 5, &"。", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(5, 6, &"e", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(6, 7, &"f", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(7, 8, &"g", 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 64 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 3u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(4u32, 7u32), ((solution.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(64 as f64, solution.lines[1usize].adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"true", GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_is(((solution.lines[1usize]).clone().repair).clone(), &"LeaveRagged").as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(3, u32::try_from(((solution.lines[1usize]).clone().repair_candidates.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"PushIn", (((solution.lines[1usize]).clone().repair_candidates[0usize]).clone().kind).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"false", if solution.lines[1usize].clone().repair_candidates[0usize].accepted { "true".to_string() } else { "false".to_string() }.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"insufficient-capacity", (((solution.lines[1usize]).clone().repair_candidates[0usize]).clone().rejection_reason).as_deref().unwrap_or(""), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"CarryPrevious", (((solution.lines[1usize]).clone().repair_candidates[1usize]).clone().kind).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"false", if solution.lines[1usize].clone().repair_candidates[1usize].accepted { "true".to_string() } else { "false".to_string() }.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"carry-overflows", (((solution.lines[1usize]).clone().repair_candidates[1usize]).clone().rejection_reason).as_deref().unwrap_or(""), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_int(Some(3), (solution.lines[1usize]).clone().repair_candidates[1usize].carried_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"LeaveRagged", (((solution.lines[1usize]).clone().repair_candidates[2usize]).clone().kind).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"true", if solution.lines[1usize].clone().repair_candidates[2usize].accepted { "true".to_string() } else { "false".to_string() }.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20 as f64, solution.total_badness, None).unwrap();
    });
}

#[test]
fn kinsoku_leave_ragged_when_prev_line_is_single_cluster() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.kinsokuLeaveRaggedWhenPrevLineIsSingleCluster", "org.tiqian.layout.GreedyLineBreakerTest.kinsokuLeaveRaggedWhenPrevLineIsSingleCluster", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(&"kinsokuLeaveRaggedWhenPrevLineIsSingleCluster");
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 7, &"English", 112 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(7, 8, &"。", 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 64 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let repair = (solution.lines[1usize]).clone().repair;
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"true", GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_is((repair).clone(), &"LeaveRagged").as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20 as f64, solution.total_badness, None).unwrap();
    });
}

#[test]
fn custom_kinsoku_rule_overrides_default() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.customKinsokuRuleOverridesDefault", "org.tiqian.layout.GreedyLineBreakerTest.customKinsokuRuleOverridesDefault", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(&"customKinsokuRuleOverridesDefault");
        let breaker = GreedyLineBreaker::new(Some(Box::new(NeverForbiddingKinsokuRule::new())), None, None, None);
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, &"a", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, &"b", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, &"c", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(3, 4, &"。", 16 as f64, None).unwrap()).clone(),
];
        let solution = breaker.break_lines(&clusters, &clusters, 48 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"-", GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_kind(((solution.lines[1usize]).clone().repair).clone()).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, solution.total_badness, None).unwrap();
    });
}

#[test]
fn misaligned_cluster_lists_throw() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.misalignedClusterListsThrow", "org.tiqian.layout.GreedyLineBreakerTest.misalignedClusterListsThrow", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(&"misalignedClusterListsThrow");
        let a = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, &"中", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, &"文", 16 as f64, None).unwrap()).clone(),
];
        let b = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, &"中", 16 as f64, None).unwrap()).clone(),
];
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let a = (a).clone(); let b = (b).clone(); Arc::new(move || {
        GreedyLineBreaker::new(None, None, None, None).break_lines(&a, &b, 100 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn hangs_pause_stop_past_measure_when_enabled_and_push_in_cannot_fit() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.hangsPauseStopPastMeasureWhenEnabledAndPushInCannotFit", "org.tiqian.layout.GreedyLineBreakerTest.hangsPauseStopPastMeasureWhenEnabledAndPushInCannotFit", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(&"hangsPauseStopPastMeasureWhenEnabledAndPushInCannotFit");
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, &"a", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, &"b", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, &"c", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(3, 4, &"d", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(4, 5, &"。", 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 64 as f64, None, None, None, Some(GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_set_ints(&vec![4])), None, None, None, None, None, None, None, None, None, None,
None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let line = (solution.lines[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 4u32), (line.cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_int(Some(4), line.get_hanging_cluster_index(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(64 as f64, line.adjusted_width, None).unwrap();
        let repair = line.repair.clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"true", GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_is((repair).clone(), &"Hang").as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(4, GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_hang_offender_cluster_index((repair).clone()), None).unwrap();
        let mut candidate: Option<RepairCandidate> = None;
        let mut ci = 0u32;
        while (i32::from_ne_bytes((ci).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((line.repair_candidates.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if line.repair_candidates[usize::try_from(ci).unwrap_or(0)].clone().kind.to_string() == "Hang" {
                candidate = Some((line.repair_candidates[usize::try_from(ci).unwrap_or(0)]).clone());
                break;
            }
            ci = u32::wrapping_add(ci, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"true", match &(candidate) { Some(__option5) => if __option5.accepted { "true".to_string() } else { "false".to_string() }, None => "false".to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn does_not_hang_when_disabled() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.doesNotHangWhenDisabled", "org.tiqian.layout.GreedyLineBreakerTest.doesNotHangWhenDisabled", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(&"doesNotHangWhenDisabled");
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, &"a", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, &"b", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, &"c", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(3, 4, &"d", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(4, 5, &"。", 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 64 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_int(None, (solution.lines[0usize]).clone().get_hanging_cluster_index(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"true", GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_is(((solution.lines[1usize]).clone().repair).clone(), &"CarryPrevious").as_str(), None).unwrap();
    });
}

#[test]
fn push_in_still_preferred_over_hang_when_glue_covers() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.pushInStillPreferredOverHangWhenGlueCovers", "org.tiqian.layout.GreedyLineBreakerTest.pushInStillPreferredOverHangWhenGlueCovers", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(&"pushInStillPreferredOverHangWhenGlueCovers");
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, &"a", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, &"b", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, &"c", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(3, 4, &"。", 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 60 as f64, Some(vec![
    (ShrinkOpportunity::new(3u32, 6u32, 8 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
]), None, None, Some(GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_set_ints(&vec![3])), None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_int(None, (solution.lines[0usize]).clone().get_hanging_cluster_index(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"true", GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_is(((solution.lines[0usize]).clone().repair).clone(), &"PushIn").as_str(), None).unwrap();
    });
}

#[test]
fn retreats_break_so_line_does_not_end_on_opening_mark() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.retreatsBreakSoLineDoesNotEndOnOpeningMark", "org.tiqian.layout.GreedyLineBreakerTest.retreatsBreakSoLineDoesNotEndOnOpeningMark", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(&"retreatsBreakSoLineDoesNotEndOnOpeningMark");
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, &"中", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, &"中", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, &"（", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(3, 4, &"中", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(4, 5, &"中", 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 48 as f64, None, None, None, None, None, None, Some(GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_set_ints(&vec![2])), None, None, None, None, None, None, None,
None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(2u32, 4u32), ((solution.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let repair = (solution.lines[0usize]).clone().repair;
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"true", GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_is((repair).clone(), &"CarryNext").as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_carry_next_moved_cluster_index((repair).clone()), None).unwrap();
    });
}

#[test]
fn keeps_opener_at_line_end_when_it_is_the_line_sole_cluster() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.keepsOpenerAtLineEndWhenItIsTheLineSoleCluster", "org.tiqian.layout.GreedyLineBreakerTest.keepsOpenerAtLineEndWhenItIsTheLineSoleCluster", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(&"keepsOpenerAtLineEndWhenItIsTheLineSoleCluster");
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, &"（", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, &"中", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, &"中", 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 16 as f64, None, None, None, None, None, None, Some(GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_set_ints(&vec![0])), None, None, None, None, None, None, None,
None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 0u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"-", GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_kind(((solution.lines[0usize]).clone().repair).clone()).as_str(), None).unwrap();
    });
}

#[test]
fn mandatory_break_closes_line_and_preserves_trailing_empty_line() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.mandatoryBreakClosesLineAndPreservesTrailingEmptyLine", "org.tiqian.layout.GreedyLineBreakerTest.mandatoryBreakClosesLineAndPreservesTrailingEmptyLine", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(&"mandatoryBreakClosesLineAndPreservesTrailingEmptyLine");
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, &"中", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, &concat!("\n",
""), 0 as f64, Some("".to_string())).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 160 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
Some(GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_set_ints(&vec![1])), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"MandatoryBreak", solution.lines[0usize].end_reason.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"TextRange(start=2, end=2)", GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_text_range(((solution.lines[1usize]).clone().source_range).clone()).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"ParagraphEnd", solution.lines[1usize].end_reason.name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn mandatory_break_blocks_kinsoku_repair_across_boundary() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.mandatoryBreakBlocksKinsokuRepairAcrossBoundary", "org.tiqian.layout.GreedyLineBreakerTest.mandatoryBreakBlocksKinsokuRepairAcrossBoundary", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(&"mandatoryBreakBlocksKinsokuRepairAcrossBoundary");
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, &"中", 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, &concat!("\n",
""), 0 as f64, Some("".to_string())).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, &"。", 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 160 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
Some(GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_set_ints(&vec![1])), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(2u32, 2u32), ((solution.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"-", GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_kind(((solution.lines[1usize]).clone().repair).clone()).as_str(), None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct GreedyLineBreakerTestSupport;

impl GreedyLineBreakerTestSupport {
    pub fn greedy_line_breaker_test_support_cluster(start: u32, end: u32, text: &str, advance: f64, display_text: Option<String>) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(start, end)?, text, "test", advance, display_text.clone(), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn greedy_line_breaker_test_support_clusters_x(count: u32) -> Result<Vec<Cluster>, TextRangeError> {
        let mut out: Vec<Cluster> = vec![];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((count).to_ne_bytes())) {
            out.push(GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(i, u32::wrapping_add(i, 1), &"x", 16 as f64, None)?);
            i = u32::wrapping_add(i, 1);
        }
        return Ok(out);
    }

    pub fn greedy_line_breaker_test_support_set_ints(values: &Vec<u32>) -> SortedSetTable<u32> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            b.put(&(values[usize::try_from(i).unwrap_or(0)]));
            i = u32::wrapping_add(i, 1);
        }
        return b.clone().build();
    }

    pub fn greedy_line_breaker_test_support_repair_is(o: Option<RepairOption>, name: &str) -> String {
        return if GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_kind((o).clone()) == name { "true".to_string() } else { "false".to_string() };
    }

    pub fn greedy_line_breaker_test_support_repair_kind(o: Option<RepairOption>) -> String {
        if o == None {
            return "-".to_string();
        }
        let v = (o).as_ref().unwrap().clone();
        return match v {
            RepairOption::PushIn { .. } => "PushIn".to_string(),
            RepairOption::Hang { .. } => "Hang".to_string(),
            RepairOption::CarryPrevious { .. } => "CarryPrevious".to_string(),
            RepairOption::CarryNext { .. } => "CarryNext".to_string(),
            RepairOption::LeaveRagged { .. } => "LeaveRagged".to_string(),
        };
    }

    pub fn greedy_line_breaker_test_support_push_in_offender_cluster_index(o: Option<RepairOption>) -> u32 {
        if o == None {
            return 4294967295u32;
        }
        let v = (o).as_ref().unwrap().clone();
        return match v {
            RepairOption::PushIn { penalty: _p0, reason: _p1, offender_cluster_index: _p2, .. } => _p2,
            RepairOption::Hang { .. } => 4294967295u32,
            RepairOption::CarryPrevious { .. } => 4294967295u32,
            RepairOption::CarryNext { .. } => 4294967295u32,
            RepairOption::LeaveRagged { .. } => 4294967295u32,
        };
    }

    pub fn greedy_line_breaker_test_support_push_in_total_shrink(o: Option<RepairOption>) -> f64 {
        if o == None {
            return i32::from_ne_bytes((4294967295u32).to_ne_bytes()) as f64;
        }
        let v = (o).as_ref().unwrap().clone();
        return match v {
            RepairOption::PushIn { penalty: _p0, reason: _p1, offender_cluster_index: _p2, allocations: _p3, total_shrink: _p4, .. } => _p4,
            RepairOption::Hang { .. } => i32::from_ne_bytes((4294967295u32).to_ne_bytes()) as f64,
            RepairOption::CarryPrevious { .. } => i32::from_ne_bytes((4294967295u32).to_ne_bytes()) as f64,
            RepairOption::CarryNext { .. } => i32::from_ne_bytes((4294967295u32).to_ne_bytes()) as f64,
            RepairOption::LeaveRagged { .. } => i32::from_ne_bytes((4294967295u32).to_ne_bytes()) as f64,
        };
    }

    pub fn greedy_line_breaker_test_support_push_in_total_available_capacity(o: Option<RepairOption>) -> f64 {
        if o == None {
            return i32::from_ne_bytes((4294967295u32).to_ne_bytes()) as f64;
        }
        let v = (o).as_ref().unwrap().clone();
        return match v {
            RepairOption::PushIn { penalty: _p0, reason: _p1, offender_cluster_index: _p2, allocations: _p3, total_shrink: _p4, total_available_capacity: _p5 } => _p5,
            RepairOption::Hang { .. } => i32::from_ne_bytes((4294967295u32).to_ne_bytes()) as f64,
            RepairOption::CarryPrevious { .. } => i32::from_ne_bytes((4294967295u32).to_ne_bytes()) as f64,
            RepairOption::CarryNext { .. } => i32::from_ne_bytes((4294967295u32).to_ne_bytes()) as f64,
            RepairOption::LeaveRagged { .. } => i32::from_ne_bytes((4294967295u32).to_ne_bytes()) as f64,
        };
    }

    pub fn greedy_line_breaker_test_support_push_in_allocations(o: Option<RepairOption>) -> Vec<PushInAllocation> {
        if o == None {
            return vec![];
        }
        let v = (o).as_ref().unwrap().clone();
        return match v {
            RepairOption::PushIn { penalty: _p0, reason: _p1, offender_cluster_index: _p2, allocations: _p3, .. } => _p3,
            RepairOption::Hang { .. } => vec![],
            RepairOption::CarryPrevious { .. } => vec![],
            RepairOption::CarryNext { .. } => vec![],
            RepairOption::LeaveRagged { .. } => vec![],
        };
    }

    pub fn greedy_line_breaker_test_support_hang_offender_cluster_index(o: Option<RepairOption>) -> u32 {
        if o == None {
            return 4294967295u32;
        }
        let v = (o).as_ref().unwrap().clone();
        return match v {
            RepairOption::PushIn { .. } => 4294967295u32,
            RepairOption::Hang { penalty: _p0, reason: _p1, offender_cluster_index: _p2 } => _p2,
            RepairOption::CarryPrevious { .. } => 4294967295u32,
            RepairOption::CarryNext { .. } => 4294967295u32,
            RepairOption::LeaveRagged { .. } => 4294967295u32,
        };
    }

    pub fn greedy_line_breaker_test_support_carry_next_moved_cluster_index(o: Option<RepairOption>) -> u32 {
        if o == None {
            return 4294967295u32;
        }
        let v = (o).as_ref().unwrap().clone();
        return match v {
            RepairOption::PushIn { .. } => 4294967295u32,
            RepairOption::Hang { .. } => 4294967295u32,
            RepairOption::CarryPrevious { .. } => 4294967295u32,
            RepairOption::CarryNext { penalty: _p0, reason: _p1, moved_cluster_index: _p2 } => _p2,
            RepairOption::LeaveRagged { .. } => 4294967295u32,
        };
    }

    pub fn greedy_line_breaker_test_support_allocation_cluster_indices(allocations: &Vec<PushInAllocation>) -> Vec<u32> {
        let mut out: Vec<u32> = vec![];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            out.push(allocations[usize::try_from(i).unwrap_or(0)].cluster_index);
            i = u32::wrapping_add(i, 1);
        }
        return out;
    }

    pub fn greedy_line_breaker_test_support_text_range(r: TextRange) -> String {
        return format!("{}{}{}{}{}",
            "TextRange(start=",
            crate::runtime::int_text::IntText::int_text(r.start),
            ", end=",
            crate::runtime::int_text::IntText::int_text(r.end),
            ")"
        );
    }

    pub fn greedy_line_breaker_test_support_start(n: &str) {
        TestTraceRecorder::new("GreedyLineBreakerTest").section(n);
    }
}

#[derive(Clone, PartialEq)]
pub struct NeverForbiddingKinsokuRule {
}

impl NeverForbiddingKinsokuRule {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn forbidden_at_line_start(&self, _cluster: Cluster) -> bool {
        return false;
    }

    pub fn forbidden_at_line_end(&self, _cluster: Cluster) -> bool {
        return false;
    }
}

impl KinsokuRule for NeverForbiddingKinsokuRule {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.GreedyLineBreakerTest.NeverForbiddingKinsokuRule"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn KinsokuRule> {
        Box::new(self.clone())
    }

    fn forbidden_at_line_start(&self, _cluster: Cluster) -> bool {
        return false;
    }

    fn forbidden_at_line_end(&self, _cluster: Cluster) -> bool {
        return false;
    }
}
