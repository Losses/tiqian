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
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum GreedyLineBreakerTestSingleClusterFitsOnOneLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for GreedyLineBreakerTestSingleClusterFitsOnOneLineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreedyLineBreakerTestSingleClusterFitsOnOneLineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestSingleClusterFitsOnOneLineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestSingleClusterFitsOnOneLineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestRetreatsBreakSoLineDoesNotEndOnOpeningMarkFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestPushInStillPreferredOverHangWhenGlueCoversFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestNaturalAndAdjustedWidthsTrackIndependentlyFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for GreedyLineBreakerTestMisalignedClusterListsThrowFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreedyLineBreakerTestMisalignedClusterListsThrowFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestMisalignedClusterListsThrowFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestMisalignedClusterListsThrowFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestMandatoryBreakClosesLineAndPreservesTrailingEmptyLineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestMandatoryBreakBlocksKinsokuRepairAcrossBoundaryFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestKinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflowFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestKinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflowFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestKinsokuLeaveRaggedWhenPrevLineIsSingleClusterFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestKinsokuCarryPreviousMovesPrevClusterToNextLineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestKinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflowFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestKeepsOpenerAtLineEndWhenItIsTheLineSoleClusterFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestHangsPauseStopPastMeasureWhenEnabledAndPushInCannotFitFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestFillsLineUntilOverflowThenStartsNewLineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for GreedyLineBreakerTestEmptyInputProducesNoLinesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreedyLineBreakerTestEmptyInputProducesNoLinesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestEmptyInputProducesNoLinesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestEmptyInputProducesNoLinesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for GreedyLineBreakerTestDoesNotHangWhenDisabledFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreedyLineBreakerTestDoesNotHangWhenDisabledFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestDoesNotHangWhenDisabledFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestDoesNotHangWhenDisabledFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestCustomKinsokuRuleOverridesDefaultFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            GreedyLineBreakerTestClusterWiderThanMaxWidthGetsOwnLineRatherThanInfiniteLoopFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(UStr::new(&[101,109,112,116,121,73,110,112,117,116,80,114,111,100,117,99,101,115,78,111,76,105,110,101,115]));
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&vec![], &vec![], 100 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn single_cluster_fits_on_one_line() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.singleClusterFitsOnOneLine", "org.tiqian.layout.GreedyLineBreakerTest.singleClusterFitsOnOneLine", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(UStr::new(&[115,105,110,103,108,101,67,108,117,115,116,101,114,70,105,116,115,79,110,79,110,101,76,105,110,101]));
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, UStr::new(&[20013]), 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 64 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let line = (solution.lines[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 0u32), (line.cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,48,44,32,101,110,100,61,49,41]), GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_text_range((line.source_range).clone()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, line.natural_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, line.adjusted_width, None).unwrap();
    });
}

#[test]
fn fills_line_until_overflow_then_starts_new_line() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.fillsLineUntilOverflowThenStartsNewLine", "org.tiqian.layout.GreedyLineBreakerTest.fillsLineUntilOverflowThenStartsNewLine", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(UStr::new(&[102,105,108,108,115,76,105,110,101,85,110,116,105,108,79,118,101,114,102,108,111,119,84,104,101,110,83,116,97,114,116,115,78,101,119,76,105,110,101]));
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
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(UStr::new(&[110,97,116,117,114,97,108,65,110,100,65,100,106,117,115,116,101,100,87,105,100,116,104,115,84,114,97,99,107,73,110,100,101,112,101,110,100,101,110,116,108,121]));
        let natural = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, UStr::new(&[65292]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, UStr::new(&[12290]), 16 as f64, None).unwrap()).clone(),
];
        let adjusted = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, UStr::new(&[65292]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, UStr::new(&[12290]), 12 as f64, None).unwrap()).clone(),
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
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(UStr::new(&[99,108,117,115,116,101,114,87,105,100,101,114,84,104,97,110,77,97,120,87,105,100,116,104,71,101,116,115,79,119,110,76,105,110,101,82,97,116,104,101,114,84,104,97,110,73,110,102,105,110,105,116,101,76,111,111,112]));
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, UStr::new(&[20013]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 8, UStr::new(&[69,110,103,108,105,115,104]), 112 as f64, None).unwrap()).clone(),
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
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(UStr::new(&[107,105,110,115,111,107,117,67,97,114,114,121,80,114,101,118,105,111,117,115,77,111,118,101,115,80,114,101,118,67,108,117,115,116,101,114,84,111,78,101,120,116,76,105,110,101]));
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, UStr::new(&[97]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, UStr::new(&[98]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, UStr::new(&[99]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(3, 4, UStr::new(&[12290]), 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 48 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(2u32, 3u32), ((solution.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32 as f64, solution.lines[0usize].adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32 as f64, solution.lines[1usize].adjusted_width, None).unwrap();
        let repair = (solution.lines[1usize]).clone().repair;
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[116,114,117,101]), GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_is((repair).clone(), UStr::new(&[67,97,114,114,121,80,114,101,118,105,111,117,115])).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10 as f64, solution.total_badness, None).unwrap();
    });
}

#[test]
fn kinsoku_pushes_forbidden_punctuation_into_previous_line_when_glue_capacity_covers_overflow() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.kinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflow", "org.tiqian.layout.GreedyLineBreakerTest.kinsokuPushesForbiddenPunctuationIntoPreviousLineWhenGlueCapacityCoversOverflow", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(UStr::new(&[107,105,110,115,111,107,117,80,117,115,104,101,115,70,111,114,98,105,100,100,101,110,80,117,110,99,116,117,97,116,105,111,110,73,110,116,111,80,114,101,118,105,111,117,115,76,105,110,101,87,104,101,110,71,108,117,101,67,97,112,97,99,105,116,121,67,111,118,101,114,115,79,118,101,114,102,108,111,119]));
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, UStr::new(&[97]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, UStr::new(&[98]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, UStr::new(&[99]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(3, 4, UStr::new(&[12290]), 16 as f64, None).unwrap()).clone(),
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
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[116,114,117,101]), GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_is((repair).clone(), UStr::new(&[80,117,115,104,73,110])).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(3, GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_push_in_offender_cluster_index((repair).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_push_in_total_shrink((repair).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_push_in_total_available_capacity((repair).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![3], &GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_allocation_cluster_indices(&GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_push_in_allocations((repair).clone())), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_push_in_allocations((repair).clone())[0usize].shrink, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_push_in_allocations((repair).clone())[0usize].available_capacity, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((line.repair_candidates.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[80,117,115,104,73,110]), ((line.repair_candidates[0usize]).clone().kind).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[116,114,117,101]), if line.repair_candidates[0usize].accepted { UString::from("true") } else { UString::from("false") }.as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, solution.total_badness, None).unwrap();
    });
}

#[test]
fn kinsoku_carries_previous_when_push_in_capacity_cannot_cover_overflow() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.kinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflow", "org.tiqian.layout.GreedyLineBreakerTest.kinsokuCarriesPreviousWhenPushInCapacityCannotCoverOverflow", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(UStr::new(&[107,105,110,115,111,107,117,67,97,114,114,105,101,115,80,114,101,118,105,111,117,115,87,104,101,110,80,117,115,104,73,110,67,97,112,97,99,105,116,121,67,97,110,110,111,116,67,111,118,101,114,79,118,101,114,102,108,111,119]));
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, UStr::new(&[97]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, UStr::new(&[98]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, UStr::new(&[99]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(3, 4, UStr::new(&[12290]), 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 59 as f64, Some(vec![
    (ShrinkOpportunity::new(3u32, 6u32, 4 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
]), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(2u32, 3u32), ((solution.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[116,114,117,101]), GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_is(((solution.lines[1usize]).clone().repair).clone(), UStr::new(&[67,97,114,114,121,80,114,101,118,105,111,117,115])).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from(((solution.lines[1usize]).clone().repair_candidates.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[80,117,115,104,73,110]), (((solution.lines[1usize]).clone().repair_candidates[0usize]).clone().kind).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[102,97,108,115,101]), if solution.lines[1usize].clone().repair_candidates[0usize].accepted { UString::from("true") } else { UString::from("false") }.as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[105,110,115,117,102,102,105,99,105,101,110,116,45,99,97,112,97,99,105,116,121]), (((solution.lines[1usize]).clone().repair_candidates[0usize]).clone().rejection_reason).as_deref().unwrap_or(UStr::new(&[])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[67,97,114,114,121,80,114,101,118,105,111,117,115]), (((solution.lines[1usize]).clone().repair_candidates[1usize]).clone().kind).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[116,114,117,101]), if solution.lines[1usize].clone().repair_candidates[1usize].accepted { UString::from("true") } else { UString::from("false") }.as_ustr(), None).unwrap();
    });
}

#[test]
fn kinsoku_rejects_carry_previous_when_carried_line_would_overflow() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.kinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflow", "org.tiqian.layout.GreedyLineBreakerTest.kinsokuRejectsCarryPreviousWhenCarriedLineWouldOverflow", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(UStr::new(&[107,105,110,115,111,107,117,82,101,106,101,99,116,115,67,97,114,114,121,80,114,101,118,105,111,117,115,87,104,101,110,67,97,114,114,105,101,100,76,105,110,101,87,111,117,108,100,79,118,101,114,102,108,111,119]));
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, UStr::new(&[97]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, UStr::new(&[98]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, UStr::new(&[99]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(3, 4, UStr::new(&[100]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(4, 5, UStr::new(&[12290]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(5, 6, UStr::new(&[101]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(6, 7, UStr::new(&[102]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(7, 8, UStr::new(&[103]), 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 64 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 3u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(4u32, 7u32), ((solution.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(64 as f64, solution.lines[1usize].adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[116,114,117,101]), GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_is(((solution.lines[1usize]).clone().repair).clone(), UStr::new(&[76,101,97,118,101,82,97,103,103,101,100])).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(3, u32::try_from(((solution.lines[1usize]).clone().repair_candidates.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[80,117,115,104,73,110]), (((solution.lines[1usize]).clone().repair_candidates[0usize]).clone().kind).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[102,97,108,115,101]), if solution.lines[1usize].clone().repair_candidates[0usize].accepted { UString::from("true") } else { UString::from("false") }.as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[105,110,115,117,102,102,105,99,105,101,110,116,45,99,97,112,97,99,105,116,121]), (((solution.lines[1usize]).clone().repair_candidates[0usize]).clone().rejection_reason).as_deref().unwrap_or(UStr::new(&[])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[67,97,114,114,121,80,114,101,118,105,111,117,115]), (((solution.lines[1usize]).clone().repair_candidates[1usize]).clone().kind).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[102,97,108,115,101]), if solution.lines[1usize].clone().repair_candidates[1usize].accepted { UString::from("true") } else { UString::from("false") }.as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[99,97,114,114,121,45,111,118,101,114,102,108,111,119,115]), (((solution.lines[1usize]).clone().repair_candidates[1usize]).clone().rejection_reason).as_deref().unwrap_or(UStr::new(&[])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_int(Some(3), (solution.lines[1usize]).clone().repair_candidates[1usize].carried_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[76,101,97,118,101,82,97,103,103,101,100]), (((solution.lines[1usize]).clone().repair_candidates[2usize]).clone().kind).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[116,114,117,101]), if solution.lines[1usize].clone().repair_candidates[2usize].accepted { UString::from("true") } else { UString::from("false") }.as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20 as f64, solution.total_badness, None).unwrap();
    });
}

#[test]
fn kinsoku_leave_ragged_when_prev_line_is_single_cluster() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.kinsokuLeaveRaggedWhenPrevLineIsSingleCluster", "org.tiqian.layout.GreedyLineBreakerTest.kinsokuLeaveRaggedWhenPrevLineIsSingleCluster", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(UStr::new(&[107,105,110,115,111,107,117,76,101,97,118,101,82,97,103,103,101,100,87,104,101,110,80,114,101,118,76,105,110,101,73,115,83,105,110,103,108,101,67,108,117,115,116,101,114]));
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 7, UStr::new(&[69,110,103,108,105,115,104]), 112 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(7, 8, UStr::new(&[12290]), 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 64 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let repair = (solution.lines[1usize]).clone().repair;
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[116,114,117,101]), GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_is((repair).clone(), UStr::new(&[76,101,97,118,101,82,97,103,103,101,100])).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20 as f64, solution.total_badness, None).unwrap();
    });
}

#[test]
fn custom_kinsoku_rule_overrides_default() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.customKinsokuRuleOverridesDefault", "org.tiqian.layout.GreedyLineBreakerTest.customKinsokuRuleOverridesDefault", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(UStr::new(&[99,117,115,116,111,109,75,105,110,115,111,107,117,82,117,108,101,79,118,101,114,114,105,100,101,115,68,101,102,97,117,108,116]));
        let breaker = GreedyLineBreaker::new(Some(Box::new(NeverForbiddingKinsokuRule::new())), None, None, None);
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, UStr::new(&[97]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, UStr::new(&[98]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, UStr::new(&[99]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(3, 4, UStr::new(&[12290]), 16 as f64, None).unwrap()).clone(),
];
        let solution = breaker.break_lines(&clusters, &clusters, 48 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[45]), GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_kind(((solution.lines[1usize]).clone().repair).clone()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, solution.total_badness, None).unwrap();
    });
}

#[test]
fn misaligned_cluster_lists_throw() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.misalignedClusterListsThrow", "org.tiqian.layout.GreedyLineBreakerTest.misalignedClusterListsThrow", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(UStr::new(&[109,105,115,97,108,105,103,110,101,100,67,108,117,115,116,101,114,76,105,115,116,115,84,104,114,111,119]));
        let a = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, UStr::new(&[20013]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, UStr::new(&[25991]), 16 as f64, None).unwrap()).clone(),
];
        let b = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, UStr::new(&[20013]), 16 as f64, None).unwrap()).clone(),
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
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(UStr::new(&[104,97,110,103,115,80,97,117,115,101,83,116,111,112,80,97,115,116,77,101,97,115,117,114,101,87,104,101,110,69,110,97,98,108,101,100,65,110,100,80,117,115,104,73,110,67,97,110,110,111,116,70,105,116]));
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, UStr::new(&[97]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, UStr::new(&[98]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, UStr::new(&[99]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(3, 4, UStr::new(&[100]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(4, 5, UStr::new(&[12290]), 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 64 as f64, None, None, None, Some(GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_set_ints(&vec![4])), None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let line = (solution.lines[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 4u32), (line.cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_int(Some(4), line.get_hanging_cluster_index(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(64 as f64, line.adjusted_width, None).unwrap();
        let repair = line.repair.clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[116,114,117,101]), GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_is((repair).clone(), UStr::new(&[72,97,110,103])).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(4, GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_hang_offender_cluster_index((repair).clone()), None).unwrap();
        let mut candidate: Option<RepairCandidate> = None;
        let mut ci = 0u32;
        while (i32::from_ne_bytes(((ci) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((line.repair_candidates.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if line.repair_candidates[usize::try_from(ci).unwrap_or(0)].clone().kind.to_ustring() == UString::from("Hang") {
                candidate = Some((line.repair_candidates[usize::try_from(ci).unwrap_or(0)]).clone());
                break;
            }
            ci = u32::wrapping_add(ci, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[116,114,117,101]), match &(candidate) { Some(__option5) => if __option5.accepted { UString::from("true") } else { UString::from("false") }, None => UString::from("false") }.as_ustr(), None).unwrap();
    });
}

#[test]
fn does_not_hang_when_disabled() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.doesNotHangWhenDisabled", "org.tiqian.layout.GreedyLineBreakerTest.doesNotHangWhenDisabled", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(UStr::new(&[100,111,101,115,78,111,116,72,97,110,103,87,104,101,110,68,105,115,97,98,108,101,100]));
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, UStr::new(&[97]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, UStr::new(&[98]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, UStr::new(&[99]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(3, 4, UStr::new(&[100]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(4, 5, UStr::new(&[12290]), 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 64 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_int(None, (solution.lines[0usize]).clone().get_hanging_cluster_index(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[116,114,117,101]), GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_is(((solution.lines[1usize]).clone().repair).clone(), UStr::new(&[67,97,114,114,121,80,114,101,118,105,111,117,115])).as_ustr(), None).unwrap();
    });
}

#[test]
fn push_in_still_preferred_over_hang_when_glue_covers() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.pushInStillPreferredOverHangWhenGlueCovers", "org.tiqian.layout.GreedyLineBreakerTest.pushInStillPreferredOverHangWhenGlueCovers", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(UStr::new(&[112,117,115,104,73,110,83,116,105,108,108,80,114,101,102,101,114,114,101,100,79,118,101,114,72,97,110,103,87,104,101,110,71,108,117,101,67,111,118,101,114,115]));
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, UStr::new(&[97]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, UStr::new(&[98]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, UStr::new(&[99]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(3, 4, UStr::new(&[12290]), 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 60 as f64, Some(vec![
    (ShrinkOpportunity::new(3u32, 6u32, 8 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
]), None, None, Some(GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_set_ints(&vec![3])), None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_int(None, (solution.lines[0usize]).clone().get_hanging_cluster_index(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[116,114,117,101]), GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_is(((solution.lines[0usize]).clone().repair).clone(), UStr::new(&[80,117,115,104,73,110])).as_ustr(), None).unwrap();
    });
}

#[test]
fn retreats_break_so_line_does_not_end_on_opening_mark() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.retreatsBreakSoLineDoesNotEndOnOpeningMark", "org.tiqian.layout.GreedyLineBreakerTest.retreatsBreakSoLineDoesNotEndOnOpeningMark", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(UStr::new(&[114,101,116,114,101,97,116,115,66,114,101,97,107,83,111,76,105,110,101,68,111,101,115,78,111,116,69,110,100,79,110,79,112,101,110,105,110,103,77,97,114,107]));
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, UStr::new(&[20013]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, UStr::new(&[20013]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, UStr::new(&[65288]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(3, 4, UStr::new(&[20013]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(4, 5, UStr::new(&[20013]), 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 48 as f64, None, None, None, None, None, None, Some(GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_set_ints(&vec![2])), None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(2u32, 4u32), ((solution.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let repair = (solution.lines[0usize]).clone().repair;
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[116,114,117,101]), GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_is((repair).clone(), UStr::new(&[67,97,114,114,121,78,101,120,116])).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_carry_next_moved_cluster_index((repair).clone()), None).unwrap();
    });
}

#[test]
fn keeps_opener_at_line_end_when_it_is_the_line_sole_cluster() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.keepsOpenerAtLineEndWhenItIsTheLineSoleCluster", "org.tiqian.layout.GreedyLineBreakerTest.keepsOpenerAtLineEndWhenItIsTheLineSoleCluster", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(UStr::new(&[107,101,101,112,115,79,112,101,110,101,114,65,116,76,105,110,101,69,110,100,87,104,101,110,73,116,73,115,84,104,101,76,105,110,101,83,111,108,101,67,108,117,115,116,101,114]));
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, UStr::new(&[65288]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, UStr::new(&[20013]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, UStr::new(&[20013]), 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 16 as f64, None, None, None, None, None, None, Some(GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_set_ints(&vec![0])), None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 0u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[45]), GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_kind(((solution.lines[0usize]).clone().repair).clone()).as_ustr(), None).unwrap();
    });
}

#[test]
fn mandatory_break_closes_line_and_preserves_trailing_empty_line() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.mandatoryBreakClosesLineAndPreservesTrailingEmptyLine", "org.tiqian.layout.GreedyLineBreakerTest.mandatoryBreakClosesLineAndPreservesTrailingEmptyLine", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(UStr::new(&[109,97,110,100,97,116,111,114,121,66,114,101,97,107,67,108,111,115,101,115,76,105,110,101,65,110,100,80,114,101,115,101,114,118,101,115,84,114,97,105,108,105,110,103,69,109,112,116,121,76,105,110,101]));
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, UStr::new(&[20013]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, UStr::new(&[10]), 0 as f64, Some(UString::from(""))).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 160 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_set_ints(&vec![1])), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[77,97,110,100,97,116,111,114,121,66,114,101,97,107]), UString::from(solution.lines[0usize].end_reason.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,50,44,32,101,110,100,61,50,41]), GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_text_range(((solution.lines[1usize]).clone().source_range).clone()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[80,97,114,97,103,114,97,112,104,69,110,100]), UString::from(solution.lines[1usize].end_reason.name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn mandatory_break_blocks_kinsoku_repair_across_boundary() {
    testlib::run("org.tiqian.layout.GreedyLineBreakerTest.mandatoryBreakBlocksKinsokuRepairAcrossBoundary", "org.tiqian.layout.GreedyLineBreakerTest.mandatoryBreakBlocksKinsokuRepairAcrossBoundary", || {
        GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_start(UStr::new(&[109,97,110,100,97,116,111,114,121,66,114,101,97,107,66,108,111,99,107,115,75,105,110,115,111,107,117,82,101,112,97,105,114,65,99,114,111,115,115,66,111,117,110,100,97,114,121]));
        let clusters = vec![
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(0, 1, UStr::new(&[20013]), 16 as f64, None).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(1, 2, UStr::new(&[10]), 0 as f64, Some(UString::from(""))).unwrap()).clone(),
    (GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(2, 3, UStr::new(&[12290]), 16 as f64, None).unwrap()).clone(),
];
        let solution = GreedyLineBreaker::new(None, None, None, None).break_lines(&clusters, &clusters, 160 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_set_ints(&vec![1])), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(2u32, 2u32), ((solution.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[45]), GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_kind(((solution.lines[1usize]).clone().repair).clone()).as_ustr(), None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct GreedyLineBreakerTestSupport;

impl GreedyLineBreakerTestSupport {
    pub fn greedy_line_breaker_test_support_cluster(start: u32, end: u32, text: &UStr, advance: f64, display_text: Option<UString>) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(start, end)?, text, &(UStr::new(&[116,101,115,116])), advance, display_text.clone(), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn greedy_line_breaker_test_support_clusters_x(count: u32) -> Result<Vec<Cluster>, TextRangeError> {
        let mut out: Vec<Cluster> = vec![];
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((count) as i32).to_ne_bytes())) {
            out.push(GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_cluster(i, u32::wrapping_add(i, 1), UStr::new(&[120]), 16 as f64, None)?);
            i = u32::wrapping_add(i, 1);
        }
        return Ok(out);
    }

    pub fn greedy_line_breaker_test_support_set_ints(values: &Vec<u32>) -> SortedSetTable<u32> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            b.put(&(values[usize::try_from(i).unwrap_or(0)]));
            i = u32::wrapping_add(i, 1);
        }
        return b.clone().build();
    }

    pub fn greedy_line_breaker_test_support_repair_is(o: Option<RepairOption>, name: &UStr) -> UString {
        return if GreedyLineBreakerTestSupport::greedy_line_breaker_test_support_repair_kind((o).clone()) == name { UString::from("true") } else { UString::from("false") };
    }

    pub fn greedy_line_breaker_test_support_repair_kind(o: Option<RepairOption>) -> UString {
        if o == None {
            return UString::from("-").to_ustring();
        }
        let v = (o).as_ref().unwrap().clone();
        return match v {
            RepairOption::PushIn { .. } => UString::from("PushIn").to_ustring(),
            RepairOption::Hang { .. } => UString::from("Hang").to_ustring(),
            RepairOption::CarryPrevious { .. } => UString::from("CarryPrevious").to_ustring(),
            RepairOption::CarryNext { .. } => UString::from("CarryNext").to_ustring(),
            RepairOption::LeaveRagged { .. } => UString::from("LeaveRagged").to_ustring(),
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
            return i32::from_ne_bytes(((4294967295u32) as i32).to_ne_bytes()) as f64;
        }
        let v = (o).as_ref().unwrap().clone();
        return match v {
            RepairOption::PushIn { penalty: _p0, reason: _p1, offender_cluster_index: _p2, allocations: _p3, total_shrink: _p4, .. } => _p4,
            RepairOption::Hang { .. } => i32::from_ne_bytes(((4294967295u32) as i32).to_ne_bytes()) as f64,
            RepairOption::CarryPrevious { .. } => i32::from_ne_bytes(((4294967295u32) as i32).to_ne_bytes()) as f64,
            RepairOption::CarryNext { .. } => i32::from_ne_bytes(((4294967295u32) as i32).to_ne_bytes()) as f64,
            RepairOption::LeaveRagged { .. } => i32::from_ne_bytes(((4294967295u32) as i32).to_ne_bytes()) as f64,
        };
    }

    pub fn greedy_line_breaker_test_support_push_in_total_available_capacity(o: Option<RepairOption>) -> f64 {
        if o == None {
            return i32::from_ne_bytes(((4294967295u32) as i32).to_ne_bytes()) as f64;
        }
        let v = (o).as_ref().unwrap().clone();
        return match v {
            RepairOption::PushIn { penalty: _p0, reason: _p1, offender_cluster_index: _p2, allocations: _p3, total_shrink: _p4, total_available_capacity: _p5 } => _p5,
            RepairOption::Hang { .. } => i32::from_ne_bytes(((4294967295u32) as i32).to_ne_bytes()) as f64,
            RepairOption::CarryPrevious { .. } => i32::from_ne_bytes(((4294967295u32) as i32).to_ne_bytes()) as f64,
            RepairOption::CarryNext { .. } => i32::from_ne_bytes(((4294967295u32) as i32).to_ne_bytes()) as f64,
            RepairOption::LeaveRagged { .. } => i32::from_ne_bytes(((4294967295u32) as i32).to_ne_bytes()) as f64,
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
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            out.push(allocations[usize::try_from(i).unwrap_or(0)].cluster_index);
            i = u32::wrapping_add(i, 1);
        }
        return out;
    }

    pub fn greedy_line_breaker_test_support_text_range(r: TextRange) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("TextRange(start=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(r.start)).as_str())); __s += &(UString::from(", end=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(r.end)).as_str())); __s += &(UString::from(")")); __s }).as_str());
    }

    pub fn greedy_line_breaker_test_support_start(n: &UStr) {
        TestTraceRecorder::new(&(UStr::new(&[71,114,101,101,100,121,76,105,110,101,66,114,101,97,107,101,114,84,101,115,116]))).section(n);
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
