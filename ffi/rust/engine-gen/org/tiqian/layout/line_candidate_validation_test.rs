#![cfg(test)]

use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::layout::line_candidate_validation_test_support::LineCandidateValidationTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum LineCandidateValidationTestNonContiguousHangingIsRejectedFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}
impl std::fmt::Display for LineCandidateValidationTestNonContiguousHangingIsRejectedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineCandidateValidationTestNonContiguousHangingIsRejectedFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            LineCandidateValidationTestNonContiguousHangingIsRejectedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineCandidateValidationTestNonContiguousHangingIsRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineCandidateValidationTestNonContiguousHangingIsRejectedFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: LineCandidateValidationTestNonContiguousHangingIsRejectedFault) -> Self {
        match value {
            LineCandidateValidationTestNonContiguousHangingIsRejectedFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineCandidateValidationTestNonContiguousHangingIsRejectedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineCandidateValidationTestNonContiguousHangingIsRejectedFault) -> Self {
        match value {
            LineCandidateValidationTestNonContiguousHangingIsRejectedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineCandidateValidationTestNonContiguousHangingIsRejectedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: LineCandidateValidationTestNonContiguousHangingIsRejectedFault) -> Self {
        match value {
            LineCandidateValidationTestNonContiguousHangingIsRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for LineCandidateValidationTestNonContiguousHangingIsRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        LineCandidateValidationTestNonContiguousHangingIsRejectedFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineCandidateValidationTestNonContiguousHangingIsRejectedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineCandidateValidationTestNonContiguousHangingIsRejectedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for LineCandidateValidationTestNonContiguousHangingIsRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        LineCandidateValidationTestNonContiguousHangingIsRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault) -> Self {
        match value {
            LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault) -> Self {
        match value {
            LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault) -> Self {
        match value {
            LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineCandidateValidationTestInMeasureRangeIsFullLineWithoutHangingFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault) -> Self {
        match value {
            LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault) -> Self {
        match value {
            LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault) -> Self {
        match value {
            LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineCandidateValidationTestInMeasureRangeExcludesHangingSuffixFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}
impl std::fmt::Display for LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault) -> Self {
        match value {
            LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault) -> Self {
        match value {
            LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault) -> Self {
        match value {
            LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        LineCandidateValidationTestHangingEntirelyAboveLineIsRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
            LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault) -> Self {
        match value {
            LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault) -> Self {
        match value {
            LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault) -> Self {
        match value {
            LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault) -> Self {
        match value {
            LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault) -> Self {
        match value {
            LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineCandidateValidationTestHangingBelowLineRangeIsRejectedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineCandidateValidationTestHangingAboveLineLastIsRejectedFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}
impl std::fmt::Display for LineCandidateValidationTestHangingAboveLineLastIsRejectedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineCandidateValidationTestHangingAboveLineLastIsRejectedFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            LineCandidateValidationTestHangingAboveLineLastIsRejectedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineCandidateValidationTestHangingAboveLineLastIsRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineCandidateValidationTestHangingAboveLineLastIsRejectedFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: LineCandidateValidationTestHangingAboveLineLastIsRejectedFault) -> Self {
        match value {
            LineCandidateValidationTestHangingAboveLineLastIsRejectedFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineCandidateValidationTestHangingAboveLineLastIsRejectedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineCandidateValidationTestHangingAboveLineLastIsRejectedFault) -> Self {
        match value {
            LineCandidateValidationTestHangingAboveLineLastIsRejectedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineCandidateValidationTestHangingAboveLineLastIsRejectedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: LineCandidateValidationTestHangingAboveLineLastIsRejectedFault) -> Self {
        match value {
            LineCandidateValidationTestHangingAboveLineLastIsRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for LineCandidateValidationTestHangingAboveLineLastIsRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        LineCandidateValidationTestHangingAboveLineLastIsRejectedFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineCandidateValidationTestHangingAboveLineLastIsRejectedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineCandidateValidationTestHangingAboveLineLastIsRejectedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for LineCandidateValidationTestHangingAboveLineLastIsRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        LineCandidateValidationTestHangingAboveLineLastIsRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[test]
fn hanging_below_line_range_is_rejected() {
    testlib::run("org.tiqian.layout.LineCandidateValidationTest.hangingBelowLineRangeIsRejected", "org.tiqian.layout.LineCandidateValidationTest.hangingBelowLineRangeIsRejected", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,67,97,110,100,105,100,97,116,101,86,97,108,105,100,97,116,105,111,110,84,101,115,116])));
        test_trace.section(UStr::new(&[104,97,110,103,105,110,103,66,101,108,111,119,76,105,110,101,82,97,110,103,101,73,115,82,101,106,101,99,116,101,100]));
        let error = TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        LineCandidateValidationTestSupport::line_candidate_validation_test_support_candidate(&vec![4294967295u32, 3], None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[72,97,110,103,105,110,103,32,99,108,117,115,116,101,114,115,32,109,117,115,116,32,98,101,32,97,32,116,114,97,105,108,105,110,103,32,108,105,110,101,32,115,117,102,102,105,120,58,32,108,105,110,101,61,48,46,46,51,32,104,97,110,103,105,110,103,61,91,45,49,44,32,51,93]), UString::from(format!("{}", format!("{}", error)).as_str()).as_ustr(), None).unwrap();
    });
}

#[test]
fn hanging_entirely_above_line_is_rejected() {
    testlib::run("org.tiqian.layout.LineCandidateValidationTest.hangingEntirelyAboveLineIsRejected", "org.tiqian.layout.LineCandidateValidationTest.hangingEntirelyAboveLineIsRejected", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,67,97,110,100,105,100,97,116,101,86,97,108,105,100,97,116,105,111,110,84,101,115,116])));
        test_trace.section(UStr::new(&[104,97,110,103,105,110,103,69,110,116,105,114,101,108,121,65,98,111,118,101,76,105,110,101,73,115,82,101,106,101,99,116,101,100]));
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        LineCandidateValidationTestSupport::line_candidate_validation_test_support_candidate(&vec![5, 6], None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn hanging_above_line_last_is_rejected() {
    testlib::run("org.tiqian.layout.LineCandidateValidationTest.hangingAboveLineLastIsRejected", "org.tiqian.layout.LineCandidateValidationTest.hangingAboveLineLastIsRejected", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,67,97,110,100,105,100,97,116,101,86,97,108,105,100,97,116,105,111,110,84,101,115,116])));
        test_trace.section(UStr::new(&[104,97,110,103,105,110,103,65,98,111,118,101,76,105,110,101,76,97,115,116,73,115,82,101,106,101,99,116,101,100]));
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        LineCandidateValidationTestSupport::line_candidate_validation_test_support_candidate(&vec![1, 4], None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn non_contiguous_hanging_is_rejected() {
    testlib::run("org.tiqian.layout.LineCandidateValidationTest.nonContiguousHangingIsRejected", "org.tiqian.layout.LineCandidateValidationTest.nonContiguousHangingIsRejected", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,67,97,110,100,105,100,97,116,101,86,97,108,105,100,97,116,105,111,110,84,101,115,116])));
        test_trace.section(UStr::new(&[110,111,110,67,111,110,116,105,103,117,111,117,115,72,97,110,103,105,110,103,73,115,82,101,106,101,99,116,101,100]));
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        LineCandidateValidationTestSupport::line_candidate_validation_test_support_candidate(&vec![0, 2, 3], None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn in_measure_range_excludes_hanging_suffix() {
    testlib::run("org.tiqian.layout.LineCandidateValidationTest.inMeasureRangeExcludesHangingSuffix", "org.tiqian.layout.LineCandidateValidationTest.inMeasureRangeExcludesHangingSuffix", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,67,97,110,100,105,100,97,116,101,86,97,108,105,100,97,116,105,111,110,84,101,115,116])));
        test_trace.section(UStr::new(&[105,110,77,101,97,115,117,114,101,82,97,110,103,101,69,120,99,108,117,100,101,115,72,97,110,103,105,110,103,83,117,102,102,105,120]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), LineCandidateValidationTestSupport::line_candidate_validation_test_support_candidate(&vec![2, 3], None).unwrap().get_in_measure_cluster_range(), None).unwrap();
    });
}

#[test]
fn in_measure_range_is_full_line_without_hanging() {
    testlib::run("org.tiqian.layout.LineCandidateValidationTest.inMeasureRangeIsFullLineWithoutHanging", "org.tiqian.layout.LineCandidateValidationTest.inMeasureRangeIsFullLineWithoutHanging", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,67,97,110,100,105,100,97,116,101,86,97,108,105,100,97,116,105,111,110,84,101,115,116])));
        test_trace.section(UStr::new(&[105,110,77,101,97,115,117,114,101,82,97,110,103,101,73,115,70,117,108,108,76,105,110,101,87,105,116,104,111,117,116,72,97,110,103,105,110,103]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 3u32), LineCandidateValidationTestSupport::line_candidate_validation_test_support_candidate(&vec![], None).unwrap().get_in_measure_cluster_range(), None).unwrap();
    });
}
