#![cfg(test)]

use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::line_breaker::LineBreaker;
use crate::org::tiqian::layout::line_breaker::LineBreakerLines;
use crate::org::tiqian::layout::line_breaker::LookaheadLineBreaker;
use crate::org::tiqian::layout::line_optimization::LineCandidate;
use crate::org::tiqian::layout::line_optimization::LineSolution;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::UnbreakableRanges;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakerCoverage2TestTestRebuildLineEmptyRangeThrowsFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<LineBreakerCoverage2TestTestRebuildLineEmptyRangeThrowsFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: LineBreakerCoverage2TestTestRebuildLineEmptyRangeThrowsFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestRebuildLineEmptyRangeThrowsFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakerCoverage2TestTestRebuildLineEmptyRangeThrowsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakerCoverage2TestTestRebuildLineEmptyRangeThrowsFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestRebuildLineEmptyRangeThrowsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakerCoverage2TestTestRebuildLineEmptyRangeThrowsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: LineBreakerCoverage2TestTestRebuildLineEmptyRangeThrowsFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestRebuildLineEmptyRangeThrowsFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for LineBreakerCoverage2TestTestRebuildLineEmptyRangeThrowsFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        LineBreakerCoverage2TestTestRebuildLineEmptyRangeThrowsFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakerCoverage2TestTestRebuildLineEmptyRangeThrowsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakerCoverage2TestTestRebuildLineEmptyRangeThrowsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for LineBreakerCoverage2TestTestRebuildLineEmptyRangeThrowsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        LineBreakerCoverage2TestTestRebuildLineEmptyRangeThrowsFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakerCoverage2TestTestLookaheadOrphanAndSyntheticHyphenRunsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineBreakerCoverage2TestTestLookaheadOrphanAndSyntheticHyphenRunsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakerCoverage2TestTestLookaheadOrphanAndSyntheticHyphenRunsFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestLookaheadOrphanAndSyntheticHyphenRunsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakerCoverage2TestTestLookaheadOrphanAndSyntheticHyphenRunsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakerCoverage2TestTestLookaheadOrphanAndSyntheticHyphenRunsFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestLookaheadOrphanAndSyntheticHyphenRunsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakerCoverage2TestTestLookaheadOrphanAndSyntheticHyphenRunsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakerCoverage2TestTestLookaheadOrphanAndSyntheticHyphenRunsFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestLookaheadOrphanAndSyntheticHyphenRunsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakerCoverage2TestTestLookaheadOrphanAndSyntheticHyphenRunsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakerCoverage2TestTestLookaheadOrphanAndSyntheticHyphenRunsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakerCoverage2TestTestLookaheadOrphanAndSyntheticHyphenRunsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakerCoverage2TestTestLookaheadOrphanAndSyntheticHyphenRunsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakerCoverage2TestTestLookaheadOrphanAndSyntheticHyphenRunsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakerCoverage2TestTestLookaheadOrphanAndSyntheticHyphenRunsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakerCoverage2TestTestLookaheadLineBreakerPreconditionsFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<LineBreakerCoverage2TestTestLookaheadLineBreakerPreconditionsFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: LineBreakerCoverage2TestTestLookaheadLineBreakerPreconditionsFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestLookaheadLineBreakerPreconditionsFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakerCoverage2TestTestLookaheadLineBreakerPreconditionsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakerCoverage2TestTestLookaheadLineBreakerPreconditionsFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestLookaheadLineBreakerPreconditionsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakerCoverage2TestTestLookaheadLineBreakerPreconditionsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: LineBreakerCoverage2TestTestLookaheadLineBreakerPreconditionsFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestLookaheadLineBreakerPreconditionsFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for LineBreakerCoverage2TestTestLookaheadLineBreakerPreconditionsFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        LineBreakerCoverage2TestTestLookaheadLineBreakerPreconditionsFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakerCoverage2TestTestLookaheadLineBreakerPreconditionsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakerCoverage2TestTestLookaheadLineBreakerPreconditionsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for LineBreakerCoverage2TestTestLookaheadLineBreakerPreconditionsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        LineBreakerCoverage2TestTestLookaheadLineBreakerPreconditionsFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakerCoverage2TestTestLookaheadHardBreakAtEndAndMiddleFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineBreakerCoverage2TestTestLookaheadHardBreakAtEndAndMiddleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakerCoverage2TestTestLookaheadHardBreakAtEndAndMiddleFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestLookaheadHardBreakAtEndAndMiddleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakerCoverage2TestTestLookaheadHardBreakAtEndAndMiddleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakerCoverage2TestTestLookaheadHardBreakAtEndAndMiddleFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestLookaheadHardBreakAtEndAndMiddleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakerCoverage2TestTestLookaheadHardBreakAtEndAndMiddleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakerCoverage2TestTestLookaheadHardBreakAtEndAndMiddleFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestLookaheadHardBreakAtEndAndMiddleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakerCoverage2TestTestLookaheadHardBreakAtEndAndMiddleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakerCoverage2TestTestLookaheadHardBreakAtEndAndMiddleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakerCoverage2TestTestLookaheadHardBreakAtEndAndMiddleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakerCoverage2TestTestLookaheadHardBreakAtEndAndMiddleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakerCoverage2TestTestLookaheadHardBreakAtEndAndMiddleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakerCoverage2TestTestLookaheadHardBreakAtEndAndMiddleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakerCoverage2TestTestLookaheadCandidateFilteringWithNonRenderingControlClustersFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineBreakerCoverage2TestTestLookaheadCandidateFilteringWithNonRenderingControlClustersFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakerCoverage2TestTestLookaheadCandidateFilteringWithNonRenderingControlClustersFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestLookaheadCandidateFilteringWithNonRenderingControlClustersFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakerCoverage2TestTestLookaheadCandidateFilteringWithNonRenderingControlClustersFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakerCoverage2TestTestLookaheadCandidateFilteringWithNonRenderingControlClustersFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestLookaheadCandidateFilteringWithNonRenderingControlClustersFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakerCoverage2TestTestLookaheadCandidateFilteringWithNonRenderingControlClustersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakerCoverage2TestTestLookaheadCandidateFilteringWithNonRenderingControlClustersFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestLookaheadCandidateFilteringWithNonRenderingControlClustersFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakerCoverage2TestTestLookaheadCandidateFilteringWithNonRenderingControlClustersFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakerCoverage2TestTestLookaheadCandidateFilteringWithNonRenderingControlClustersFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakerCoverage2TestTestLookaheadCandidateFilteringWithNonRenderingControlClustersFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakerCoverage2TestTestLookaheadCandidateFilteringWithNonRenderingControlClustersFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakerCoverage2TestTestLookaheadCandidateFilteringWithNonRenderingControlClustersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakerCoverage2TestTestLookaheadCandidateFilteringWithNonRenderingControlClustersFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakerCoverage2TestTestLineCandidateEndsWithProgressiveBreakFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineBreakerCoverage2TestTestLineCandidateEndsWithProgressiveBreakFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakerCoverage2TestTestLineCandidateEndsWithProgressiveBreakFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestLineCandidateEndsWithProgressiveBreakFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakerCoverage2TestTestLineCandidateEndsWithProgressiveBreakFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakerCoverage2TestTestLineCandidateEndsWithProgressiveBreakFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestLineCandidateEndsWithProgressiveBreakFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakerCoverage2TestTestLineCandidateEndsWithProgressiveBreakFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakerCoverage2TestTestLineCandidateEndsWithProgressiveBreakFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestLineCandidateEndsWithProgressiveBreakFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakerCoverage2TestTestLineCandidateEndsWithProgressiveBreakFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakerCoverage2TestTestLineCandidateEndsWithProgressiveBreakFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakerCoverage2TestTestLineCandidateEndsWithProgressiveBreakFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakerCoverage2TestTestLineCandidateEndsWithProgressiveBreakFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakerCoverage2TestTestLineCandidateEndsWithProgressiveBreakFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakerCoverage2TestTestLineCandidateEndsWithProgressiveBreakFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakerCoverage2TestTestFindGreedyEndDefaultArgsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineBreakerCoverage2TestTestFindGreedyEndDefaultArgsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakerCoverage2TestTestFindGreedyEndDefaultArgsFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestFindGreedyEndDefaultArgsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakerCoverage2TestTestFindGreedyEndDefaultArgsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakerCoverage2TestTestFindGreedyEndDefaultArgsFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestFindGreedyEndDefaultArgsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakerCoverage2TestTestFindGreedyEndDefaultArgsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakerCoverage2TestTestFindGreedyEndDefaultArgsFault) -> Self {
        match value {
            LineBreakerCoverage2TestTestFindGreedyEndDefaultArgsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakerCoverage2TestTestFindGreedyEndDefaultArgsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakerCoverage2TestTestFindGreedyEndDefaultArgsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakerCoverage2TestTestFindGreedyEndDefaultArgsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakerCoverage2TestTestFindGreedyEndDefaultArgsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakerCoverage2TestTestFindGreedyEndDefaultArgsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakerCoverage2TestTestFindGreedyEndDefaultArgsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn test_line_breaker_strategy_name_default() {
    testlib::run("org.tiqian.layout.LineBreakerCoverage2Test.testLineBreakerStrategyNameDefault", "org.tiqian.layout.LineBreakerCoverage2Test.testLineBreakerStrategyNameDefault", || {
        let mut test_trace = TestTraceRecorder::new("LineBreakerCoverage2Test");
        test_trace.section(&"testLineBreakerStrategyNameDefault");
        let breaker: Box<dyn LineBreaker> = Box::new(LineBreakerCoverage2TestCustomBreaker::new());
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"custom", breaker.get_strategy_name().as_str(), None).unwrap();
    });
}

#[test]
fn test_lookahead_line_breaker_preconditions() {
    testlib::run("org.tiqian.layout.LineBreakerCoverage2Test.testLookaheadLineBreakerPreconditions", "org.tiqian.layout.LineBreakerCoverage2Test.testLookaheadLineBreakerPreconditions", || {
        let mut test_trace = TestTraceRecorder::new("LineBreakerCoverage2Test");
        test_trace.section(&"testLookaheadLineBreakerPreconditions");
        let clusters = LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_han_clusters(2, 16.0f64).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let clusters = (clusters).clone(); Arc::new(move || {
        LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)).break_lines(&LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_han_clusters(1,
16.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?, &clusters, 100.0f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let clusters = (clusters).clone(); Arc::new(move || {
        LookaheadLineBreaker::new(Some(4294967295u32), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)).break_lines(&clusters, &clusters, 100.0f64, None, None, None, None, None, None, None, None, None,
None, None, None, None, None, None, None, None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let clusters = (clusters).clone(); Arc::new(move || {
        LookaheadLineBreaker::new(Some(2), Some(4294967295u32), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)).break_lines(&clusters, &clusters, 100.0f64, None, None, None, None, None, None, None, None, None,
None, None, None, None, None, None, None, None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn test_lookahead_candidate_filtering_with_non_rendering_control_clusters() {
    testlib::run("org.tiqian.layout.LineBreakerCoverage2Test.testLookaheadCandidateFilteringWithNonRenderingControlClusters", "org.tiqian.layout.LineBreakerCoverage2Test.testLookaheadCandidateFilteringWithNonRenderingControlClusters", || {
        let mut test_trace = TestTraceRecorder::new("LineBreakerCoverage2Test");
        test_trace.section(&"testLookaheadCandidateFilteringWithNonRenderingControlClusters");
        let clusters = vec![
    (LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_cluster(0, &"​", 0.0f64).unwrap()).clone(),
    (LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_cluster(1, &"A", 20.0f64).unwrap()).clone(),
    (LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_cluster(2, &"B", 20.0f64).unwrap()).clone(),
];
        let solution = LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)).break_lines(&clusters, &clusters, 25.0f64, None, None, None, None, None, None, None, None, None,
None, None, None, None, None, None, Some(LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_ints(&vec![0])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
    });
}

#[test]
fn test_lookahead_hard_break_at_end_and_middle() {
    testlib::run("org.tiqian.layout.LineBreakerCoverage2Test.testLookaheadHardBreakAtEndAndMiddle", "org.tiqian.layout.LineBreakerCoverage2Test.testLookaheadHardBreakAtEndAndMiddle", || {
        let mut test_trace = TestTraceRecorder::new("LineBreakerCoverage2Test");
        test_trace.section(&"testLookaheadHardBreakAtEndAndMiddle");
        let end_solution = LookaheadLineBreaker::new(Some(1), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20),
Some(12.0)).break_lines(&LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_han_clusters(2, 16.0f64).unwrap(), &LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_han_clusters(2, 16.0f64).unwrap(), 20.0f64, None, None, None, None, None, None,
None, None, None, None, None, None, None, None, Some(LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_ints(&vec![1])), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((end_solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((end_solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::MandatoryBreak), &(end_solution.lines[0usize].end_reason), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(1u32, 0u32), ((end_solution.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::ParagraphEnd), &(end_solution.lines[1usize].end_reason), None).unwrap();
        let middle_solution = LookaheadLineBreaker::new(Some(1), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20),
Some(12.0)).break_lines(&LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_han_clusters(3, 16.0f64).unwrap(), &LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_han_clusters(3, 16.0f64).unwrap(), 20.0f64, None, None, None, None, None, None,
None, None, None, None, None, None, None, None, Some(LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_ints(&vec![0])), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((middle_solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 0u32), ((middle_solution.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(LineEndReason::MandatoryBreak), &(middle_solution.lines[0usize].end_reason), None).unwrap();
        let oversized = vec![
    (LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_cluster(0, &"A", 50.0f64).unwrap()).clone(),
    (LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_cluster(1, &"B", 10.0f64).unwrap()).clone(),
];
        let oversized_solution = LookaheadLineBreaker::new(Some(1), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)).break_lines(&oversized, &oversized, 20.0f64, None, None, None, None, None, None, None,
None, None, None, None, None, None, None, Some(LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_ints(&vec![0])), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((oversized_solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn test_line_candidate_ends_with_progressive_break() {
    testlib::run("org.tiqian.layout.LineBreakerCoverage2Test.testLineCandidateEndsWithProgressiveBreak", "org.tiqian.layout.LineBreakerCoverage2Test.testLineCandidateEndsWithProgressiveBreak", || {
        let mut test_trace = TestTraceRecorder::new("LineBreakerCoverage2Test");
        test_trace.section(&"testLineCandidateEndsWithProgressiveBreak");
        let c = LineCandidate::new(IntRange::new(0u32, 1u32), TextRange::new(0u32, 2u32).unwrap(), 32.0f64, 32.0f64, Some(LineEndReason::AutoWrap), None, Some(vec![]), Some(LineCandidate::line_candidate_empty_hanging())).unwrap();
        let opp = ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Syllable, TextRange::new(0u32, 4u32).unwrap(), Some(0.0));
        let _ = TracedAssertions::traced_assertions_assert_true(LineBreakerLines::line_breaker_lines_ends_with_progressive_break((c).clone(), LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_opp_map(&vec![2], &vec![(opp).clone()])), None).unwrap();
        let paragraph_end = LineCandidate::new((c.cluster_range).clone(), (c.source_range).clone(), c.natural_width, c.adjusted_width, Some(LineEndReason::ParagraphEnd), (c.repair).clone(), Some((c.repair_candidates).clone()), Some((c.hanging_cluster_indices).clone())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LineBreakerLines::line_breaker_lines_ends_with_progressive_break((paragraph_end).clone(), LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_opp_map(&vec![2], &vec![(opp).clone()])),
None).unwrap();
        let empty_range = LineCandidate::new(IntRange::new(1u32, 0u32), (c.source_range).clone(), c.natural_width, c.adjusted_width, Some(c.end_reason), (c.repair).clone(), Some((c.repair_candidates).clone()), Some((c.hanging_cluster_indices).clone())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LineBreakerLines::line_breaker_lines_ends_with_progressive_break((empty_range).clone(), LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_opp_map(&vec![2], &vec![(opp).clone()])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(LineBreakerLines::line_breaker_lines_ends_with_progressive_break((c).clone(), SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b|
SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).clone().build()), None).unwrap();
    });
}

#[test]
fn test_line_gap_count() {
    testlib::run("org.tiqian.layout.LineBreakerCoverage2Test.testLineGapCount", "org.tiqian.layout.LineBreakerCoverage2Test.testLineGapCount", || {
        let mut test_trace = TestTraceRecorder::new("LineBreakerCoverage2Test");
        test_trace.section(&"testLineGapCount");
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LineBreakerLines::line_breaker_lines_line_gap_count(IntRange::new(1u32, 0u32), LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_ints(&vec![0, 1])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, LineBreakerLines::line_breaker_lines_line_gap_count(IntRange::new(0u32, 2u32), LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_ints(&vec![1])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LineBreakerLines::line_breaker_lines_line_gap_count(IntRange::new(0u32, 2u32), LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_ints(&vec![2])), None).unwrap();
    });
}

#[test]
fn test_rebuild_line_empty_range_throws() {
    testlib::run("org.tiqian.layout.LineBreakerCoverage2Test.testRebuildLineEmptyRangeThrows", "org.tiqian.layout.LineBreakerCoverage2Test.testRebuildLineEmptyRangeThrows", || {
        let mut test_trace = TestTraceRecorder::new("LineBreakerCoverage2Test");
        test_trace.section(&"testRebuildLineEmptyRangeThrows");
        let clusters = LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_han_clusters(2, 16.0f64).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let clusters = (clusters).clone(); Arc::new(move || {
        LineBreakerLines::line_breaker_lines_rebuild_line(IntRange::new(1u32, 0u32), &clusters, &clusters, None, None, None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn test_find_greedy_end_default_args() {
    testlib::run("org.tiqian.layout.LineBreakerCoverage2Test.testFindGreedyEndDefaultArgs", "org.tiqian.layout.LineBreakerCoverage2Test.testFindGreedyEndDefaultArgs", || {
        let mut test_trace = TestTraceRecorder::new("LineBreakerCoverage2Test");
        test_trace.section(&"testFindGreedyEndDefaultArgs");
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, LineBreakerLines::line_breaker_lines_find_greedy_end(&LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_han_clusters(5, 10.0f64).unwrap(), 0, 25.0f64, None, None), None).unwrap();
    });
}

#[test]
fn test_lookahead_orphan_and_synthetic_hyphen_runs() {
    testlib::run("org.tiqian.layout.LineBreakerCoverage2Test.testLookaheadOrphanAndSyntheticHyphenRuns", "org.tiqian.layout.LineBreakerCoverage2Test.testLookaheadOrphanAndSyntheticHyphenRuns", || {
        let mut test_trace = TestTraceRecorder::new("LineBreakerCoverage2Test");
        test_trace.section(&"testLookaheadOrphanAndSyntheticHyphenRuns");
        let clusters = LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_han_clusters(4, 20.0f64).unwrap();
        let solution = LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0)).break_lines(&clusters, &clusters, 25.0f64, None, None, None, None, None, None, None,
Some(LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_ints(&vec![1, 2, 3])), None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, u32::try_from((solution.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[derive(Clone, PartialEq)]
pub struct LineBreakerCoverage2TestCustomBreaker {
}

impl LineBreakerCoverage2TestCustomBreaker {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn get_strategy_name(&self) -> String {
        return "custom".to_string();
    }

    pub fn break_lines(&self, _n: &Vec<Cluster>, _a: &Vec<Cluster>, _max_width: f64, _s: Option<Vec<ShrinkOpportunity>>, _u: Option<UnbreakableRanges>, _i: Option<f64>, _h: Option<SortedSetTable<u32>>, _e: Option<Vec<IntRange>>, _fs: Option<SortedSetTable<u32>>, _fe:
Option<SortedSetTable<u32>>, _hy: Option<SortedSetTable<u32>>, _cj: Option<SortedSetTable<u32>>, _mc: Option<f64>, _sw: Option<SortedSetTable<u32>>, _sc: Option<f64>, _p: Option<bool>, _bias: Option<f64>, _hb: Option<SortedSetTable<u32>>, _nc: Option<SortedSetTable<u32>>, _pr:
Option<SortedMapTable<u32, ProgressiveBreakOpportunity>>) -> Result<LineSolution, TextRangeError> {
        return Ok(LineSolution::new(Some(vec![]), Some(0 as f64))?);
    }
}

impl LineBreaker for LineBreakerCoverage2TestCustomBreaker {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.LineBreakerCoverage2Test.LineBreakerCoverage2TestCustomBreaker"
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

    fn break_lines(&self, _n: &Vec<Cluster>, _a: &Vec<Cluster>, _max_width: f64, _s: Option<Vec<ShrinkOpportunity>>, _u: Option<UnbreakableRanges>, _i: Option<f64>, _h: Option<SortedSetTable<u32>>, _e: Option<Vec<IntRange>>, _fs: Option<SortedSetTable<u32>>, _fe:
Option<SortedSetTable<u32>>, _hy: Option<SortedSetTable<u32>>, _cj: Option<SortedSetTable<u32>>, _mc: Option<f64>, _sw: Option<SortedSetTable<u32>>, _sc: Option<f64>, _p: Option<bool>, _bias: Option<f64>, _hb: Option<SortedSetTable<u32>>, _nc: Option<SortedSetTable<u32>>, _pr:
Option<SortedMapTable<u32, ProgressiveBreakOpportunity>>) -> Result<LineSolution, TextRangeError> {
        return Ok(LineSolution::new(Some(vec![]), Some(0 as f64))?);
    }
}

#[derive(Clone, Copy)]
pub struct LineBreakerCoverage2TestSupport;

impl LineBreakerCoverage2TestSupport {
    pub fn line_breaker_coverage2_test_support_cluster(index: u32, text: &str, advance: f64) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(index, u32::wrapping_add(index, 1))?, text, "test", advance, Some(text.to_string()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn line_breaker_coverage2_test_support_han_clusters(count: u32, advance: f64) -> Result<Vec<Cluster>, TextRangeError> {
        let mut result: Vec<Cluster> = vec![];
        for i in 0..count {
            result.push(LineBreakerCoverage2TestSupport::line_breaker_coverage2_test_support_cluster(i, &"中", advance)?);
        }
        return Ok(result);
    }

    pub fn line_breaker_coverage2_test_support_ints(values: &Vec<u32>) -> SortedSetTable<u32> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for &v in values {
            b.put(&(v));
        }
        return b.clone().build();
    }

    pub fn line_breaker_coverage2_test_support_opp_map(keys: &Vec<u32>, opps: &Vec<ProgressiveBreakOpportunity>) -> SortedMapTable<u32, ProgressiveBreakOpportunity> {
        let mut b: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..match u32::try_from(keys.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            b.put(&(keys[usize::try_from(i).unwrap_or(0)]), &((opps[usize::try_from(i).unwrap_or(0)]).clone()));
        }
        return b.clone().build();
    }
}
