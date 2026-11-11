#![cfg(test)]

use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_debug_info::LayoutDebugInfo;
use crate::org::tiqian::core::max_lines_decision_info::MaxLinesDecisionInfo;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::units::FloatIc;
use crate::org::tiqian::core::units::IntIc;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}
impl std::fmt::Display for CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault) -> Self {
        match value {
            CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault) -> Self {
        match value {
            CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault) -> Self {
        match value {
            CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}
impl std::fmt::Display for CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault) -> Self {
        match value {
            CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault) -> Self {
        match value {
            CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault) -> Self {
        match value {
            CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        CoreUnitsGeometryTestTextRangeRejectsNegativeStartFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreUnitsGeometryTestLayoutDebugInfoAcceptsMaxLinesDecisionFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
}
impl std::fmt::Display for CoreUnitsGeometryTestLayoutDebugInfoAcceptsMaxLinesDecisionFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreUnitsGeometryTestLayoutDebugInfoAcceptsMaxLinesDecisionFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            CoreUnitsGeometryTestLayoutDebugInfoAcceptsMaxLinesDecisionFault::UStringFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<CoreUnitsGeometryTestLayoutDebugInfoAcceptsMaxLinesDecisionFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: CoreUnitsGeometryTestLayoutDebugInfoAcceptsMaxLinesDecisionFault) -> Self {
        match value {
            CoreUnitsGeometryTestLayoutDebugInfoAcceptsMaxLinesDecisionFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreUnitsGeometryTestLayoutDebugInfoAcceptsMaxLinesDecisionFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreUnitsGeometryTestLayoutDebugInfoAcceptsMaxLinesDecisionFault) -> Self {
        match value {
            CoreUnitsGeometryTestLayoutDebugInfoAcceptsMaxLinesDecisionFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for CoreUnitsGeometryTestLayoutDebugInfoAcceptsMaxLinesDecisionFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        CoreUnitsGeometryTestLayoutDebugInfoAcceptsMaxLinesDecisionFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreUnitsGeometryTestLayoutDebugInfoAcceptsMaxLinesDecisionFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreUnitsGeometryTestLayoutDebugInfoAcceptsMaxLinesDecisionFault::UStringFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}
impl std::fmt::Display for CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault) -> Self {
        match value {
            CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault) -> Self {
        match value {
            CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault) -> Self {
        match value {
            CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxWidthFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}
impl std::fmt::Display for CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault) -> Self {
        match value {
            CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault) -> Self {
        match value {
            CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault) -> Self {
        match value {
            CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxLinesFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}
impl std::fmt::Display for CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault) -> Self {
        match value {
            CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault) -> Self {
        match value {
            CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault) -> Self {
        match value {
            CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        CoreUnitsGeometryTestLayoutConstraintsRejectsNonPositiveMaxHeightFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[test]
fn ic_plus_returns_sum() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.icPlusReturnsSum", "org.tiqian.core.CoreUnitsGeometryTest.icPlusReturnsSum", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,85,110,105,116,115,71,101,111,109,101,116,114,121,84,101,115,116]))).section(UStr::new(&[105,99,80,108,117,115,82,101,116,117,114,110,115,83,117,109]));
        let _ = TracedAssertions::traced_assertions_assert_equals_ic(Ic(5.0f64), Ic(2.0f64) + Ic(3.0f64), None).unwrap();
    });
}

#[test]
fn ic_unary_minus_returns_negated() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.icUnaryMinusReturnsNegated", "org.tiqian.core.CoreUnitsGeometryTest.icUnaryMinusReturnsNegated", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,85,110,105,116,115,71,101,111,109,101,116,114,121,84,101,115,116]))).section(UStr::new(&[105,99,85,110,97,114,121,77,105,110,117,115,82,101,116,117,114,110,115,78,101,103,97,116,101,100]));
        let _ = TracedAssertions::traced_assertions_assert_equals_ic(Ic(-3.0f64), -Ic(3.0f64), None).unwrap();
    });
}

#[test]
fn float_ic_extension_creates_ic() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.floatIcExtensionCreatesIc", "org.tiqian.core.CoreUnitsGeometryTest.floatIcExtensionCreatesIc", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,85,110,105,116,115,71,101,111,109,101,116,114,121,84,101,115,116]))).section(UStr::new(&[102,108,111,97,116,73,99,69,120,116,101,110,115,105,111,110,67,114,101,97,116,101,115,73,99]));
        let _ = TracedAssertions::traced_assertions_assert_equals_ic(Ic(2.0f64), FloatIc::float_ic_ic(2.0f64), None).unwrap();
    });
}

#[test]
fn int_ic_extension_creates_ic() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.intIcExtensionCreatesIc", "org.tiqian.core.CoreUnitsGeometryTest.intIcExtensionCreatesIc", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,85,110,105,116,115,71,101,111,109,101,116,114,121,84,101,115,116]))).section(UStr::new(&[105,110,116,73,99,69,120,116,101,110,115,105,111,110,67,114,101,97,116,101,115,73,99]));
        let _ = TracedAssertions::traced_assertions_assert_equals_ic(Ic(5.0f64), IntIc::int_ic_ic(5), None).unwrap();
    });
}

#[test]
fn ic_to_px_multiplies_by_em_size() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.icToPxMultipliesByEmSize", "org.tiqian.core.CoreUnitsGeometryTest.icToPxMultipliesByEmSize", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,85,110,105,116,115,71,101,111,109,101,116,114,121,84,101,115,116]))).section(UStr::new(&[105,99,84,111,80,120,77,117,108,116,105,112,108,105,101,115,66,121,69,109,83,105,122,101]));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(24.0f64, Ic(3.0f64).to_px(8.0f64), None).unwrap();
    });
}

#[test]
fn rect_height_returns_difference() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.rectHeightReturnsDifference", "org.tiqian.core.CoreUnitsGeometryTest.rectHeightReturnsDifference", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,85,110,105,116,115,71,101,111,109,101,116,114,121,84,101,115,116]))).section(UStr::new(&[114,101,99,116,72,101,105,103,104,116,82,101,116,117,114,110,115,68,105,102,102,101,114,101,110,99,101]));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20.0f64, Rect::new(0.0f64, 0.0f64, 10.0f64, 20.0f64).get_height(), None).unwrap();
    });
}

#[test]
fn rect_width_returns_difference() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.rectWidthReturnsDifference", "org.tiqian.core.CoreUnitsGeometryTest.rectWidthReturnsDifference", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,85,110,105,116,115,71,101,111,109,101,116,114,121,84,101,115,116]))).section(UStr::new(&[114,101,99,116,87,105,100,116,104,82,101,116,117,114,110,115,68,105,102,102,101,114,101,110,99,101]));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, Rect::new(0.0f64, 0.0f64, 10.0f64, 20.0f64).get_width(), None).unwrap();
    });
}

#[test]
fn text_range_rejects_start_greater_than_end() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.textRangeRejectsStartGreaterThanEnd", "org.tiqian.core.CoreUnitsGeometryTest.textRangeRejectsStartGreaterThanEnd", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,85,110,105,116,115,71,101,111,109,101,116,114,121,84,101,115,116]))).section(UStr::new(&[116,101,120,116,82,97,110,103,101,82,101,106,101,99,116,115,83,116,97,114,116,71,114,101,97,116,101,114,84,104,97,110,69,110,100]));
        let _ = CoreUnitsGeometryTestHelpers::core_units_geometry_test_helpers_expect_argument_failure({  Arc::new(move || {
        TextRange::new(5u32, 2u32).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn text_range_rejects_negative_start() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.textRangeRejectsNegativeStart", "org.tiqian.core.CoreUnitsGeometryTest.textRangeRejectsNegativeStart", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,85,110,105,116,115,71,101,111,109,101,116,114,121,84,101,115,116]))).section(UStr::new(&[116,101,120,116,82,97,110,103,101,82,101,106,101,99,116,115,78,101,103,97,116,105,118,101,83,116,97,114,116]));
        let _ = CoreUnitsGeometryTestHelpers::core_units_geometry_test_helpers_expect_argument_failure({  Arc::new(move || {
        TextRange::new(4294967295u32, 1u32).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn layout_constraints_rejects_non_positive_max_width() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.layoutConstraintsRejectsNonPositiveMaxWidth", "org.tiqian.core.CoreUnitsGeometryTest.layoutConstraintsRejectsNonPositiveMaxWidth", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,85,110,105,116,115,71,101,111,109,101,116,114,121,84,101,115,116]))).section(UStr::new(&[108,97,121,111,117,116,67,111,110,115,116,114,97,105,110,116,115,82,101,106,101,99,116,115,78,111,110,80,111,115,105,116,105,118,101,77,97,120,87,105,100,116,104]));
        let _ = CoreUnitsGeometryTestHelpers::core_units_geometry_test_helpers_expect_argument_failure({  Arc::new(move || {
        LayoutConstraints::new(-1.0f64, Some(f64::INFINITY), Some(2147483647)).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn layout_constraints_rejects_non_positive_max_height() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.layoutConstraintsRejectsNonPositiveMaxHeight", "org.tiqian.core.CoreUnitsGeometryTest.layoutConstraintsRejectsNonPositiveMaxHeight", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,85,110,105,116,115,71,101,111,109,101,116,114,121,84,101,115,116]))).section(UStr::new(&[108,97,121,111,117,116,67,111,110,115,116,114,97,105,110,116,115,82,101,106,101,99,116,115,78,111,110,80,111,115,105,116,105,118,101,77,97,120,72,101,105,103,104,116]));
        let _ = CoreUnitsGeometryTestHelpers::core_units_geometry_test_helpers_expect_argument_failure({  Arc::new(move || {
        LayoutConstraints::new(100.0f64, Some(-1.0f64), Some(2147483647)).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn layout_constraints_rejects_non_positive_max_lines() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.layoutConstraintsRejectsNonPositiveMaxLines", "org.tiqian.core.CoreUnitsGeometryTest.layoutConstraintsRejectsNonPositiveMaxLines", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,85,110,105,116,115,71,101,111,109,101,116,114,121,84,101,115,116]))).section(UStr::new(&[108,97,121,111,117,116,67,111,110,115,116,114,97,105,110,116,115,82,101,106,101,99,116,115,78,111,110,80,111,115,105,116,105,118,101,77,97,120,76,105,110,101,115]));
        let _ = CoreUnitsGeometryTestHelpers::core_units_geometry_test_helpers_expect_argument_failure({  Arc::new(move || {
        LayoutConstraints::new(100.0f64, Some(100.0f64), Some(0)).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn max_lines_decision_info_records_truncation_details() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.maxLinesDecisionInfoRecordsTruncationDetails", "org.tiqian.core.CoreUnitsGeometryTest.maxLinesDecisionInfoRecordsTruncationDetails", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,85,110,105,116,115,71,101,111,109,101,116,114,121,84,101,115,116]))).section(UStr::new(&[109,97,120,76,105,110,101,115,68,101,99,105,115,105,111,110,73,110,102,111,82,101,99,111,114,100,115,84,114,117,110,99,97,116,105,111,110,68,101,116,97,105,108,115]));
        let info = MaxLinesDecisionInfo::new(5u32, 3u32, Some(UString::from("MaxLinesLineTruncation")));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(5, info.laid_out_lines, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, info.visible_lines, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[77,97,120,76,105,110,101,115,76,105,110,101,84,114,117,110,99,97,116,105,111,110]), (info.reason).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn layout_debug_info_accepts_max_lines_decision() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.layoutDebugInfoAcceptsMaxLinesDecision", "org.tiqian.core.CoreUnitsGeometryTest.layoutDebugInfoAcceptsMaxLinesDecision", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,85,110,105,116,115,71,101,111,109,101,116,114,121,84,101,115,116]))).section(UStr::new(&[108,97,121,111,117,116,68,101,98,117,103,73,110,102,111,65,99,99,101,112,116,115,77,97,120,76,105,110,101,115,68,101,99,105,115,105,111,110]));
        let debug = LayoutDebugInfo::new(Some(MaxLinesDecisionInfo::new(5u32, 3u32, Some(UString::from("MaxLinesLineTruncation")))), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        if debug.max_lines_decision.is_none() {
            let _ = TracedAssertions::traced_assertions_fail(None, None).unwrap();
            return;
        }
        let decision = debug.max_lines_decision.clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(5, (decision).as_ref().unwrap().laid_out_lines, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, (decision).as_ref().unwrap().visible_lines, None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct CoreUnitsGeometryTestHelpers;

impl CoreUnitsGeometryTestHelpers {
    pub fn core_units_geometry_test_helpers_expect_argument_failure(block: Arc<dyn Fn() -> Result<(), IllegalStateException> + Send + Sync + 'static>) -> Result<(), TracedAssertionsAssertFailsWithFault> {
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), (block).clone())?;
        Ok(())
    }
}
