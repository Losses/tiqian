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
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum CoreUnitsGeometryTestTextRangeRejectsStartGreaterThanEndFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
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
        TestTraceRecorder::new("CoreUnitsGeometryTest").section(&"icPlusReturnsSum");
        let _ = TracedAssertions::traced_assertions_assert_equals_ic(Ic(5.0f64), Ic(2.0f64) + Ic(3.0f64), None).unwrap();
    });
}

#[test]
fn ic_unary_minus_returns_negated() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.icUnaryMinusReturnsNegated", "org.tiqian.core.CoreUnitsGeometryTest.icUnaryMinusReturnsNegated", || {
        TestTraceRecorder::new("CoreUnitsGeometryTest").section(&"icUnaryMinusReturnsNegated");
        let _ = TracedAssertions::traced_assertions_assert_equals_ic(Ic(-3.0f64), -Ic(3.0f64), None).unwrap();
    });
}

#[test]
fn float_ic_extension_creates_ic() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.floatIcExtensionCreatesIc", "org.tiqian.core.CoreUnitsGeometryTest.floatIcExtensionCreatesIc", || {
        TestTraceRecorder::new("CoreUnitsGeometryTest").section(&"floatIcExtensionCreatesIc");
        let _ = TracedAssertions::traced_assertions_assert_equals_ic(Ic(2.0f64), FloatIc::float_ic_ic(2.0f64), None).unwrap();
    });
}

#[test]
fn int_ic_extension_creates_ic() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.intIcExtensionCreatesIc", "org.tiqian.core.CoreUnitsGeometryTest.intIcExtensionCreatesIc", || {
        TestTraceRecorder::new("CoreUnitsGeometryTest").section(&"intIcExtensionCreatesIc");
        let _ = TracedAssertions::traced_assertions_assert_equals_ic(Ic(5.0f64), IntIc::int_ic_ic(5), None).unwrap();
    });
}

#[test]
fn ic_to_px_multiplies_by_em_size() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.icToPxMultipliesByEmSize", "org.tiqian.core.CoreUnitsGeometryTest.icToPxMultipliesByEmSize", || {
        TestTraceRecorder::new("CoreUnitsGeometryTest").section(&"icToPxMultipliesByEmSize");
        let _ = TracedAssertions::traced_assertions_assert_equals_float(24.0f64, Ic(3.0f64).to_px(8.0f64), None).unwrap();
    });
}

#[test]
fn rect_height_returns_difference() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.rectHeightReturnsDifference", "org.tiqian.core.CoreUnitsGeometryTest.rectHeightReturnsDifference", || {
        TestTraceRecorder::new("CoreUnitsGeometryTest").section(&"rectHeightReturnsDifference");
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20.0f64, Rect::new(0.0f64, 0.0f64, 10.0f64, 20.0f64).get_height(), None).unwrap();
    });
}

#[test]
fn rect_width_returns_difference() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.rectWidthReturnsDifference", "org.tiqian.core.CoreUnitsGeometryTest.rectWidthReturnsDifference", || {
        TestTraceRecorder::new("CoreUnitsGeometryTest").section(&"rectWidthReturnsDifference");
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, Rect::new(0.0f64, 0.0f64, 10.0f64, 20.0f64).get_width(), None).unwrap();
    });
}

#[test]
fn text_range_rejects_start_greater_than_end() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.textRangeRejectsStartGreaterThanEnd", "org.tiqian.core.CoreUnitsGeometryTest.textRangeRejectsStartGreaterThanEnd", || {
        TestTraceRecorder::new("CoreUnitsGeometryTest").section(&"textRangeRejectsStartGreaterThanEnd");
        let _ = CoreUnitsGeometryTestHelpers::core_units_geometry_test_helpers_expect_argument_failure({  Arc::new(move || {
        TextRange::new(5u32, 2u32).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn text_range_rejects_negative_start() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.textRangeRejectsNegativeStart", "org.tiqian.core.CoreUnitsGeometryTest.textRangeRejectsNegativeStart", || {
        TestTraceRecorder::new("CoreUnitsGeometryTest").section(&"textRangeRejectsNegativeStart");
        let _ = CoreUnitsGeometryTestHelpers::core_units_geometry_test_helpers_expect_argument_failure({  Arc::new(move || {
        TextRange::new(4294967295u32, 1u32).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn layout_constraints_rejects_non_positive_max_width() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.layoutConstraintsRejectsNonPositiveMaxWidth", "org.tiqian.core.CoreUnitsGeometryTest.layoutConstraintsRejectsNonPositiveMaxWidth", || {
        TestTraceRecorder::new("CoreUnitsGeometryTest").section(&"layoutConstraintsRejectsNonPositiveMaxWidth");
        let _ = CoreUnitsGeometryTestHelpers::core_units_geometry_test_helpers_expect_argument_failure({  Arc::new(move || {
        LayoutConstraints::new(-1.0f64, Some(f64::INFINITY), Some(2147483647)).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn layout_constraints_rejects_non_positive_max_height() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.layoutConstraintsRejectsNonPositiveMaxHeight", "org.tiqian.core.CoreUnitsGeometryTest.layoutConstraintsRejectsNonPositiveMaxHeight", || {
        TestTraceRecorder::new("CoreUnitsGeometryTest").section(&"layoutConstraintsRejectsNonPositiveMaxHeight");
        let _ = CoreUnitsGeometryTestHelpers::core_units_geometry_test_helpers_expect_argument_failure({  Arc::new(move || {
        LayoutConstraints::new(100.0f64, Some(-1.0f64), Some(2147483647)).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn layout_constraints_rejects_non_positive_max_lines() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.layoutConstraintsRejectsNonPositiveMaxLines", "org.tiqian.core.CoreUnitsGeometryTest.layoutConstraintsRejectsNonPositiveMaxLines", || {
        TestTraceRecorder::new("CoreUnitsGeometryTest").section(&"layoutConstraintsRejectsNonPositiveMaxLines");
        let _ = CoreUnitsGeometryTestHelpers::core_units_geometry_test_helpers_expect_argument_failure({  Arc::new(move || {
        LayoutConstraints::new(100.0f64, Some(100.0f64), Some(0)).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn max_lines_decision_info_records_truncation_details() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.maxLinesDecisionInfoRecordsTruncationDetails", "org.tiqian.core.CoreUnitsGeometryTest.maxLinesDecisionInfoRecordsTruncationDetails", || {
        TestTraceRecorder::new("CoreUnitsGeometryTest").section(&"maxLinesDecisionInfoRecordsTruncationDetails");
        let info = MaxLinesDecisionInfo::new(5u32, 3u32, Some("MaxLinesLineTruncation".to_string()));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(5, info.laid_out_lines, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, info.visible_lines, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"MaxLinesLineTruncation", (info.reason).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn layout_debug_info_accepts_max_lines_decision() {
    testlib::run("org.tiqian.core.CoreUnitsGeometryTest.layoutDebugInfoAcceptsMaxLinesDecision", "org.tiqian.core.CoreUnitsGeometryTest.layoutDebugInfoAcceptsMaxLinesDecision", || {
        TestTraceRecorder::new("CoreUnitsGeometryTest").section(&"layoutDebugInfoAcceptsMaxLinesDecision");
        let debug = LayoutDebugInfo::new(Some(MaxLinesDecisionInfo::new(5u32, 3u32, Some("MaxLinesLineTruncation".to_string()))), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
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
