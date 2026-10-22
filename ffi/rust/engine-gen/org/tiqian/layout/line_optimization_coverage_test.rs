#![cfg(test)]

use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::line_optimization::BreakCandidate;
use crate::org::tiqian::layout::line_optimization::LineCandidate;
use crate::org::tiqian::layout::line_optimization::LineOptimizationStrategy;
use crate::org::tiqian::layout::line_optimization::LineSolution;
use crate::org::tiqian::layout::line_optimization::RepairCandidate;
use crate::org::tiqian::layout::line_optimization::RepairOption;
use crate::org::tiqian::layout::line_optimization::RepairOptions;
use crate::org::tiqian::linebreak::break_kind::BreakKind;
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
pub enum LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertNullRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault::TracedAssertionsAssertNullRenderedFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault) -> Self {
        match value {
            LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault) -> Self {
        match value {
            LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault {
    fn from(value: LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault) -> Self {
        match value {
            LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault::TracedAssertionsAssertNullRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault) -> Self {
        match value {
            LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault) -> Self {
        match value {
            LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault> for LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault) -> Self {
        LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault::TracedAssertionsAssertNullRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineOptimizationCoverageTestRepairCandidateDefaultsAreUsableFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault) -> Self {
        match value {
            LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault) -> Self {
        match value {
            LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault) -> Self {
        match value {
            LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault) -> Self {
        match value {
            LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineOptimizationCoverageTestLineSolutionDefaultsToZeroBadnessFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}
impl std::fmt::Display for LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault) -> Self {
        match value {
            LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault) -> Self {
        match value {
            LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault) -> Self {
        match value {
            LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        LineOptimizationCoverageTestLineCandidateRejectsHangingThatIsNotATrailingSuffixFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}
impl std::fmt::Display for LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault) -> Self {
        match value {
            LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault) -> Self {
        match value {
            LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault) -> Self {
        match value {
            LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        LineOptimizationCoverageTestLineCandidateRejectsDiscontiguousHangingFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsIntSetFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntSetFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault::TracedAssertionsAssertEqualsIntSetFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault) -> Self {
        match value {
            LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault) -> Self {
        match value {
            LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntSetFault {
    fn from(value: LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault) -> Self {
        match value {
            LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault::TracedAssertionsAssertEqualsIntSetFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault) -> Self {
        match value {
            LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntSetFault> for LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntSetFault) -> Self {
        LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault::TracedAssertionsAssertEqualsIntSetFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineOptimizationCoverageTestLineCandidateAcceptsAContiguousTrailingHangingSuffixFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsIntRangeFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault) -> Self {
        match value {
            LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault) -> Self {
        match value {
            LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault {
    fn from(value: LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault) -> Self {
        match value {
            LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault) -> Self {
        match value {
            LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault> for LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault) -> Self {
        LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineOptimizationCoverageTestInMeasureClusterRangeExcludesTheHangingSuffixFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsNullableIntFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableIntFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault::TracedAssertionsAssertEqualsNullableIntFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault) -> Self {
        match value {
            LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault) -> Self {
        match value {
            LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableIntFault {
    fn from(value: LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault) -> Self {
        match value {
            LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault::TracedAssertionsAssertEqualsNullableIntFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault) -> Self {
        match value {
            LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableIntFault> for LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsNullableIntFault) -> Self {
        LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault::TracedAssertionsAssertEqualsNullableIntFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineOptimizationCoverageTestHangingClusterIndexPrefersTheHangOffenderOverTheSuffixEndFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertNullRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault::TracedAssertionsAssertNullRenderedFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault) -> Self {
        match value {
            LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault) -> Self {
        match value {
            LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault {
    fn from(value: LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault) -> Self {
        match value {
            LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault::TracedAssertionsAssertNullRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault) -> Self {
        match value {
            LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault) -> Self {
        match value {
            LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault> for LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertNullRenderedFault) -> Self {
        LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault::TracedAssertionsAssertNullRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineOptimizationCoverageTestBreakCandidateDefaultsAreUsableFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertEqualsRepairOptionArrayFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRepairOptionArrayFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault::TracedAssertionsAssertEqualsRepairOptionArrayFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault) -> Self {
        match value {
            LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault) -> Self {
        match value {
            LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault) -> Self {
        match value {
            LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRepairOptionArrayFault {
    fn from(value: LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault) -> Self {
        match value {
            LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault::TracedAssertionsAssertEqualsRepairOptionArrayFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault) -> Self {
        match value {
            LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRepairOptionArrayFault> for LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRepairOptionArrayFault) -> Self {
        LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault::TracedAssertionsAssertEqualsRepairOptionArrayFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineOptimizationCoverageTestBreakCandidateCarriesExplicitForbiddenReasonAndRepairsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineOptimizationCoverageTestOptimizationStrategyEnumeratesAllThreeStrategiesFault {
    TracedAssertionsAssertEqualsRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineOptimizationCoverageTestOptimizationStrategyEnumeratesAllThreeStrategiesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineOptimizationCoverageTestOptimizationStrategyEnumeratesAllThreeStrategiesFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestOptimizationStrategyEnumeratesAllThreeStrategiesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineOptimizationCoverageTestOptimizationStrategyEnumeratesAllThreeStrategiesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: LineOptimizationCoverageTestOptimizationStrategyEnumeratesAllThreeStrategiesFault) -> Self {
        match value {
            LineOptimizationCoverageTestOptimizationStrategyEnumeratesAllThreeStrategiesFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestOptimizationStrategyEnumeratesAllThreeStrategiesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineOptimizationCoverageTestOptimizationStrategyEnumeratesAllThreeStrategiesFault) -> Self {
        match value {
            LineOptimizationCoverageTestOptimizationStrategyEnumeratesAllThreeStrategiesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault> for LineOptimizationCoverageTestOptimizationStrategyEnumeratesAllThreeStrategiesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault) -> Self {
        LineOptimizationCoverageTestOptimizationStrategyEnumeratesAllThreeStrategiesFault::TracedAssertionsAssertEqualsRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineOptimizationCoverageTestOptimizationStrategyEnumeratesAllThreeStrategiesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineOptimizationCoverageTestOptimizationStrategyEnumeratesAllThreeStrategiesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault {
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault::TracedAssertionsAssertEqualsFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault) -> Self {
        match value {
            LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault) -> Self {
        match value {
            LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault) -> Self {
        match value {
            LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineOptimizationCoverageTestCarryNextRecordsTheMovedMarkFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn break_candidate_defaults_are_usable() {
    testlib::run("org.tiqian.layout.LineOptimizationCoverageTest.breakCandidateDefaultsAreUsable", "org.tiqian.layout.LineOptimizationCoverageTest.breakCandidateDefaultsAreUsable", || {
        LineOptimizationCoverageSupport::line_optimization_coverage_support_start(UStr::new(&[98,114,101,97,107,67,97,110,100,105,100,97,116,101,68,101,102,97,117,108,116,115,65,114,101,85,115,97,98,108,101]));
        let candidate = BreakCandidate::new(3u32, BreakKind::Allowed, 16.0f64, 14.0f64, 18.0f64, None, Some(vec![])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(candidate.forbidden_reason.is_none(), UStr::new(&[45]), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((candidate.repair_options.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn break_candidate_carries_explicit_forbidden_reason_and_repairs() {
    testlib::run("org.tiqian.layout.LineOptimizationCoverageTest.breakCandidateCarriesExplicitForbiddenReasonAndRepairs", "org.tiqian.layout.LineOptimizationCoverageTest.breakCandidateCarriesExplicitForbiddenReasonAndRepairs", || {
        LineOptimizationCoverageSupport::line_optimization_coverage_support_start(UStr::new(&[98,114,101,97,107,67,97,110,100,105,100,97,116,101,67,97,114,114,105,101,115,69,120,112,108,105,99,105,116,70,111,114,98,105,100,100,101,110,82,101,97,115,111,110,65,110,100,82,101,112,97,105,114,115]));
        let repair = RepairOption::LeaveRagged { penalty: 30, reason: UString::from("ForbiddenAtLineStart:，:leave-ragged"), offender_cluster_index: 3 };
        let candidate = BreakCandidate::new(2u32, BreakKind::Problematic, 32.0f64, 28.0f64, 36.0f64, Some(UString::from("kinsoku")), Some(vec![(repair).clone()])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[107,105,110,115,111,107,117]), (candidate.forbidden_reason).as_deref().unwrap_or(UStr::new(&[])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_repair_option_array(&vec![(repair).clone()], &candidate.repair_options, None).unwrap();
    });
}

#[test]
fn line_candidate_rejects_hanging_that_is_not_a_trailing_suffix() {
    testlib::run("org.tiqian.layout.LineOptimizationCoverageTest.lineCandidateRejectsHangingThatIsNotATrailingSuffix", "org.tiqian.layout.LineOptimizationCoverageTest.lineCandidateRejectsHangingThatIsNotATrailingSuffix", || {
        LineOptimizationCoverageSupport::line_optimization_coverage_support_start(UStr::new(&[108,105,110,101,67,97,110,100,105,100,97,116,101,82,101,106,101,99,116,115,72,97,110,103,105,110,103,84,104,97,116,73,115,78,111,116,65,84,114,97,105,108,105,110,103,83,117,102,102,105,120]));
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        LineOptimizationCoverageSupport::line_optimization_coverage_support_line(Some(LineOptimizationCoverageSupport::line_optimization_coverage_support_set(&vec![0, 1])), None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        LineOptimizationCoverageSupport::line_optimization_coverage_support_line(Some(LineOptimizationCoverageSupport::line_optimization_coverage_support_set(&vec![2, 3])), None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        LineOptimizationCoverageSupport::line_optimization_coverage_support_line(Some(LineOptimizationCoverageSupport::line_optimization_coverage_support_set(&vec![7])), None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn line_candidate_rejects_discontiguous_hanging() {
    testlib::run("org.tiqian.layout.LineOptimizationCoverageTest.lineCandidateRejectsDiscontiguousHanging", "org.tiqian.layout.LineOptimizationCoverageTest.lineCandidateRejectsDiscontiguousHanging", || {
        LineOptimizationCoverageSupport::line_optimization_coverage_support_start(UStr::new(&[108,105,110,101,67,97,110,100,105,100,97,116,101,82,101,106,101,99,116,115,68,105,115,99,111,110,116,105,103,117,111,117,115,72,97,110,103,105,110,103]));
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        LineOptimizationCoverageSupport::line_optimization_coverage_support_line(Some(LineOptimizationCoverageSupport::line_optimization_coverage_support_set(&vec![2, 4])), None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn line_candidate_accepts_a_contiguous_trailing_hanging_suffix() {
    testlib::run("org.tiqian.layout.LineOptimizationCoverageTest.lineCandidateAcceptsAContiguousTrailingHangingSuffix", "org.tiqian.layout.LineOptimizationCoverageTest.lineCandidateAcceptsAContiguousTrailingHangingSuffix", || {
        LineOptimizationCoverageSupport::line_optimization_coverage_support_start(UStr::new(&[108,105,110,101,67,97,110,100,105,100,97,116,101,65,99,99,101,112,116,115,65,67,111,110,116,105,103,117,111,117,115,84,114,97,105,108,105,110,103,72,97,110,103,105,110,103,83,117,102,102,105,120]));
        let line = LineOptimizationCoverageSupport::line_optimization_coverage_support_line(Some(LineOptimizationCoverageSupport::line_optimization_coverage_support_set(&vec![3, 4])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_set(LineOptimizationCoverageSupport::line_optimization_coverage_support_set(&vec![3, 4]), (line.hanging_cluster_indices).clone(), None).unwrap();
    });
}

#[test]
fn hanging_cluster_index_prefers_the_hang_offender_over_the_suffix_end() {
    testlib::run("org.tiqian.layout.LineOptimizationCoverageTest.hangingClusterIndexPrefersTheHangOffenderOverTheSuffixEnd", "org.tiqian.layout.LineOptimizationCoverageTest.hangingClusterIndexPrefersTheHangOffenderOverTheSuffixEnd", || {
        LineOptimizationCoverageSupport::line_optimization_coverage_support_start(UStr::new(&[104,97,110,103,105,110,103,67,108,117,115,116,101,114,73,110,100,101,120,80,114,101,102,101,114,115,84,104,101,72,97,110,103,79,102,102,101,110,100,101,114,79,118,101,114,84,104,101,83,117,102,102,105,120,69,110,100]));
        let with_repair = LineOptimizationCoverageSupport::line_optimization_coverage_support_line(Some(LineOptimizationCoverageSupport::line_optimization_coverage_support_set(&vec![3, 4])), Some(RepairOption::Hang { penalty: 5, reason: UString::from("ForbiddenAtLineStart:，:hang"), offender_cluster_index: 3 })).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_int(Some(3), with_repair.get_hanging_cluster_index(), None).unwrap();
        let without_repair = LineOptimizationCoverageSupport::line_optimization_coverage_support_line(Some(LineOptimizationCoverageSupport::line_optimization_coverage_support_set(&vec![3, 4])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_int(Some(4), without_repair.get_hanging_cluster_index(), None).unwrap();
    });
}

#[test]
fn in_measure_cluster_range_excludes_the_hanging_suffix() {
    testlib::run("org.tiqian.layout.LineOptimizationCoverageTest.inMeasureClusterRangeExcludesTheHangingSuffix", "org.tiqian.layout.LineOptimizationCoverageTest.inMeasureClusterRangeExcludesTheHangingSuffix", || {
        LineOptimizationCoverageSupport::line_optimization_coverage_support_start(UStr::new(&[105,110,77,101,97,115,117,114,101,67,108,117,115,116,101,114,82,97,110,103,101,69,120,99,108,117,100,101,115,84,104,101,72,97,110,103,105,110,103,83,117,102,102,105,120]));
        let hanging = LineOptimizationCoverageSupport::line_optimization_coverage_support_line(Some(LineOptimizationCoverageSupport::line_optimization_coverage_support_set(&vec![3, 4])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 2u32), hanging.get_in_measure_cluster_range(), None).unwrap();
        let plain = LineOptimizationCoverageSupport::line_optimization_coverage_support_line(None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 4u32), plain.get_in_measure_cluster_range(), None).unwrap();
    });
}

#[test]
fn carry_next_records_the_moved_mark() {
    testlib::run("org.tiqian.layout.LineOptimizationCoverageTest.carryNextRecordsTheMovedMark", "org.tiqian.layout.LineOptimizationCoverageTest.carryNextRecordsTheMovedMark", || {
        LineOptimizationCoverageSupport::line_optimization_coverage_support_start(UStr::new(&[99,97,114,114,121,78,101,120,116,82,101,99,111,114,100,115,84,104,101,77,111,118,101,100,77,97,114,107]));
        let carry_next = RepairOption::CarryNext { penalty: 15, reason: UString::from("ForbiddenAtLineEnd:“:carry-next"), moved_cluster_index: 4 };
        let _ = TracedAssertions::traced_assertions_assert_equals(15, RepairOptions::repair_options_penalty((carry_next).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(4, LineOptimizationCoverageSupport::line_optimization_coverage_support_moved_index((carry_next).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[70,111,114,98,105,100,100,101,110,65,116,76,105,110,101,69,110,100,58,8220,58,99,97,114,114,121,45,110,101,120,116]), RepairOptions::repair_options_reason((carry_next).clone()).as_ustr(), None).unwrap();
    });
}

#[test]
fn repair_candidate_defaults_are_usable() {
    testlib::run("org.tiqian.layout.LineOptimizationCoverageTest.repairCandidateDefaultsAreUsable", "org.tiqian.layout.LineOptimizationCoverageTest.repairCandidateDefaultsAreUsable", || {
        LineOptimizationCoverageSupport::line_optimization_coverage_support_start(UStr::new(&[114,101,112,97,105,114,67,97,110,100,105,100,97,116,101,68,101,102,97,117,108,116,115,65,114,101,85,115,97,98,108,101]));
        let candidate = RepairCandidate::new(&(UStr::new(&[80,117,115,104,73,110])), &(UStr::new(&[70,111,114,98,105,100,100,101,110,65,116,76,105,110,101,83,116,97,114,116])), 4u32, 10u32, true, None, None, None, Some(0 as f64), Some(0 as f64), Some(0 as f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(candidate.rejection_reason.is_none(), UStr::new(&[45]), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(candidate.target_cluster_index.is_none(), UStr::new(&[45]), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(candidate.carried_cluster_index.is_none(), UStr::new(&[45]), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, candidate.shrink, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, candidate.required_shrink, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, candidate.available_capacity, None).unwrap();
    });
}

#[test]
fn line_solution_defaults_to_zero_badness() {
    testlib::run("org.tiqian.layout.LineOptimizationCoverageTest.lineSolutionDefaultsToZeroBadness", "org.tiqian.layout.LineOptimizationCoverageTest.lineSolutionDefaultsToZeroBadness", || {
        LineOptimizationCoverageSupport::line_optimization_coverage_support_start(UStr::new(&[108,105,110,101,83,111,108,117,116,105,111,110,68,101,102,97,117,108,116,115,84,111,90,101,114,111,66,97,100,110,101,115,115]));
        let solution = LineSolution::new(Some(vec![]), Some(0 as f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, solution.total_badness, None).unwrap();
    });
}

#[test]
fn optimization_strategy_enumerates_all_three_strategies() {
    testlib::run("org.tiqian.layout.LineOptimizationCoverageTest.optimizationStrategyEnumeratesAllThreeStrategies", "org.tiqian.layout.LineOptimizationCoverageTest.optimizationStrategyEnumeratesAllThreeStrategies", || {
        LineOptimizationCoverageSupport::line_optimization_coverage_support_start(UStr::new(&[111,112,116,105,109,105,122,97,116,105,111,110,83,116,114,97,116,101,103,121,69,110,117,109,101,114,97,116,101,115,65,108,108,84,104,114,101,101,83,116,114,97,116,101,103,105,101,115]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,71,114,101,101,100,121,44,32,76,111,111,107,97,104,101,97,100,44,32,80,97,114,97,103,114,97,112,104,68,121,110,97,109,105,99,80,114,111,103,114,97,109,109,105,110,103,93]), LineOptimizationCoverageSupport::line_optimization_coverage_support_render_strategies().as_ustr(), None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct LineOptimizationCoverageSupport;

impl LineOptimizationCoverageSupport {
    pub fn line_optimization_coverage_support_start(n: &UStr) {
        TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,79,112,116,105,109,105,122,97,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116]))).section(n);
    }

    pub fn line_optimization_coverage_support_set(values: &Vec<u32>) -> SortedSetTable<u32> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            b.put(&(values[usize::try_from(i).unwrap_or(0)]));
            i = u32::wrapping_add(i, 1);
        }
        return b.clone().build();
    }

    pub fn line_optimization_coverage_support_line(hanging: Option<SortedSetTable<u32>>, repair: Option<RepairOption>) -> Result<LineCandidate, TextRangeError> {
        return Ok(LineCandidate::new(IntRange::new(0u32, 4u32), TextRange::new(0u32, 5u32)?, 80.0f64, 80.0f64, Some(LineEndReason::AutoWrap), (repair).clone(), Some(vec![]), (hanging).clone())?);
    }

    pub fn line_optimization_coverage_support_moved_index(o: RepairOption) -> u32 {
        return match o {
            RepairOption::PushIn { .. } => 4294967295u32,
            RepairOption::Hang { .. } => 4294967295u32,
            RepairOption::CarryPrevious { .. } => 4294967295u32,
            RepairOption::CarryNext { penalty: _p0, reason: _p1, moved_cluster_index: _p2 } => _p2,
            RepairOption::LeaveRagged { .. } => 4294967295u32,
        };
    }

    pub fn line_optimization_coverage_support_render_strategies() -> UString {
        let values = LineOptimizationStrategy::ALL;
        let mut buf_b = UString::new();
        buf_b += &(UString::from("["));
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (3) {
            if i32::from_ne_bytes(((i) as i32).to_ne_bytes()) > (0) {
                buf_b += &(UString::from(", "));
            }
            {
                let x = UString::from(values[usize::try_from(i).unwrap_or(0)].name());
                buf_b += &(x.to_string());
            }
            i = u32::wrapping_add(i, 1);
        }
        buf_b += &(UString::from("]"));
        return buf_b;
    }
}
