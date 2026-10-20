#![cfg(test)]

use crate::org::tiqian::layout::justifier::CompressionPlan;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkChannel;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCompressionTestZeroSurplusIsNoOpFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for JustifierCompressionTestZeroSurplusIsNoOpFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifierCompressionTestZeroSurplusIsNoOpFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            JustifierCompressionTestZeroSurplusIsNoOpFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            JustifierCompressionTestZeroSurplusIsNoOpFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<JustifierCompressionTestZeroSurplusIsNoOpFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCompressionTestZeroSurplusIsNoOpFault) -> Self {
        match value {
            JustifierCompressionTestZeroSurplusIsNoOpFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCompressionTestZeroSurplusIsNoOpFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCompressionTestZeroSurplusIsNoOpFault) -> Self {
        match value {
            JustifierCompressionTestZeroSurplusIsNoOpFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCompressionTestZeroSurplusIsNoOpFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCompressionTestZeroSurplusIsNoOpFault) -> Self {
        match value {
            JustifierCompressionTestZeroSurplusIsNoOpFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCompressionTestZeroSurplusIsNoOpFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCompressionTestZeroSurplusIsNoOpFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCompressionTestZeroSurplusIsNoOpFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCompressionTestZeroSurplusIsNoOpFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCompressionTestZeroSurplusIsNoOpFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCompressionTestZeroSurplusIsNoOpFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCompressionTestSharesEqualFractionWithinATierFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for JustifierCompressionTestSharesEqualFractionWithinATierFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifierCompressionTestSharesEqualFractionWithinATierFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            JustifierCompressionTestSharesEqualFractionWithinATierFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            JustifierCompressionTestSharesEqualFractionWithinATierFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<JustifierCompressionTestSharesEqualFractionWithinATierFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCompressionTestSharesEqualFractionWithinATierFault) -> Self {
        match value {
            JustifierCompressionTestSharesEqualFractionWithinATierFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCompressionTestSharesEqualFractionWithinATierFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCompressionTestSharesEqualFractionWithinATierFault) -> Self {
        match value {
            JustifierCompressionTestSharesEqualFractionWithinATierFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCompressionTestSharesEqualFractionWithinATierFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCompressionTestSharesEqualFractionWithinATierFault) -> Self {
        match value {
            JustifierCompressionTestSharesEqualFractionWithinATierFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCompressionTestSharesEqualFractionWithinATierFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCompressionTestSharesEqualFractionWithinATierFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCompressionTestSharesEqualFractionWithinATierFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCompressionTestSharesEqualFractionWithinATierFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCompressionTestSharesEqualFractionWithinATierFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCompressionTestSharesEqualFractionWithinATierFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault) -> Self {
        match value {
            JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault) -> Self {
        match value {
            JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault) -> Self {
        match value {
            JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCompressionTestReportsUnfilledWhenCapacityExhaustedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCompressionTestNanSurplusEmitsNoAllocationsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for JustifierCompressionTestNanSurplusEmitsNoAllocationsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifierCompressionTestNanSurplusEmitsNoAllocationsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            JustifierCompressionTestNanSurplusEmitsNoAllocationsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            JustifierCompressionTestNanSurplusEmitsNoAllocationsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<JustifierCompressionTestNanSurplusEmitsNoAllocationsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCompressionTestNanSurplusEmitsNoAllocationsFault) -> Self {
        match value {
            JustifierCompressionTestNanSurplusEmitsNoAllocationsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCompressionTestNanSurplusEmitsNoAllocationsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCompressionTestNanSurplusEmitsNoAllocationsFault) -> Self {
        match value {
            JustifierCompressionTestNanSurplusEmitsNoAllocationsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCompressionTestNanSurplusEmitsNoAllocationsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCompressionTestNanSurplusEmitsNoAllocationsFault) -> Self {
        match value {
            JustifierCompressionTestNanSurplusEmitsNoAllocationsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCompressionTestNanSurplusEmitsNoAllocationsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCompressionTestNanSurplusEmitsNoAllocationsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCompressionTestNanSurplusEmitsNoAllocationsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCompressionTestNanSurplusEmitsNoAllocationsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCompressionTestNanSurplusEmitsNoAllocationsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCompressionTestNanSurplusEmitsNoAllocationsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierCompressionTestConsumesTiersInAscendingOrderFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for JustifierCompressionTestConsumesTiersInAscendingOrderFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifierCompressionTestConsumesTiersInAscendingOrderFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            JustifierCompressionTestConsumesTiersInAscendingOrderFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            JustifierCompressionTestConsumesTiersInAscendingOrderFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<JustifierCompressionTestConsumesTiersInAscendingOrderFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierCompressionTestConsumesTiersInAscendingOrderFault) -> Self {
        match value {
            JustifierCompressionTestConsumesTiersInAscendingOrderFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCompressionTestConsumesTiersInAscendingOrderFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierCompressionTestConsumesTiersInAscendingOrderFault) -> Self {
        match value {
            JustifierCompressionTestConsumesTiersInAscendingOrderFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierCompressionTestConsumesTiersInAscendingOrderFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierCompressionTestConsumesTiersInAscendingOrderFault) -> Self {
        match value {
            JustifierCompressionTestConsumesTiersInAscendingOrderFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierCompressionTestConsumesTiersInAscendingOrderFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierCompressionTestConsumesTiersInAscendingOrderFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierCompressionTestConsumesTiersInAscendingOrderFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierCompressionTestConsumesTiersInAscendingOrderFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierCompressionTestConsumesTiersInAscendingOrderFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierCompressionTestConsumesTiersInAscendingOrderFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct JustifierCompressionTestSupport;

impl JustifierCompressionTestSupport {
    pub fn justifier_compression_test_support_shrink_of(plan: CompressionPlan, cluster_index: u32) -> Option<f64> {
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((plan.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if plan.allocations[usize::try_from(i).unwrap_or(0)].cluster_index == cluster_index {
                return Some(plan.allocations[usize::try_from(i).unwrap_or(0)].shrink);
            }
            i = u32::wrapping_add(i, 1);
        }
        return None;
    }
}

#[test]
fn consumes_tiers_in_ascending_order() {
    testlib::run("org.tiqian.layout.JustifierCompressionTest.consumesTiersInAscendingOrder", "org.tiqian.layout.JustifierCompressionTest.consumesTiersInAscendingOrder", || {
        TestTraceRecorder::new(&(UStr::new(&[74,117,115,116,105,102,105,101,114,67,111,109,112,114,101,115,115,105,111,110,84,101,115,116]))).section(UStr::new(&[99,111,110,115,117,109,101,115,84,105,101,114,115,73,110,65,115,99,101,110,100,105,110,103,79,114,100,101,114]));
        let justifier = Justifier::new(Some(0.5), Some(0.25));
        let opps = vec![
    (ShrinkOpportunity::new(0u32, 1u32, 2.0f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
    (ShrinkOpportunity::new(1u32, 2u32, 5.0f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
    (ShrinkOpportunity::new(2u32, 3u32, 5.0f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
];
        let plan = justifier.compress(3.0f64, &opps).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0.0f64, plan.unfilled_surplus, 0.0001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(2.0f64, *(JustifierCompressionTestSupport::justifier_compression_test_support_shrink_of((plan).clone(), 0)).as_ref().unwrap(), 0.0001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(1.0f64, *(JustifierCompressionTestSupport::justifier_compression_test_support_shrink_of((plan).clone(), 1)).as_ref().unwrap(), 0.0001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(JustifierCompressionTestSupport::justifier_compression_test_support_shrink_of((plan).clone(), 2).is_none(), UStr::new(&[45]), Some(UString::from("tier 3 must stay untouched while tier 2 has room"))).unwrap();
    });
}

#[test]
fn shares_equal_fraction_within_a_tier() {
    testlib::run("org.tiqian.layout.JustifierCompressionTest.sharesEqualFractionWithinATier", "org.tiqian.layout.JustifierCompressionTest.sharesEqualFractionWithinATier", || {
        TestTraceRecorder::new(&(UStr::new(&[74,117,115,116,105,102,105,101,114,67,111,109,112,114,101,115,115,105,111,110,84,101,115,116]))).section(UStr::new(&[115,104,97,114,101,115,69,113,117,97,108,70,114,97,99,116,105,111,110,87,105,116,104,105,110,65,84,105,101,114]));
        let p = Justifier::new(Some(0.5), Some(0.25)).compress(4 as f64, &vec![
    (ShrinkOpportunity::new(0u32, 2u32, 2 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
    (ShrinkOpportunity::new(1u32, 2u32, 6 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(1 as f64, *(JustifierCompressionTestSupport::justifier_compression_test_support_shrink_of((p).clone(), 0)).as_ref().unwrap(), 0.0001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(3 as f64, *(JustifierCompressionTestSupport::justifier_compression_test_support_shrink_of((p).clone(), 1)).as_ref().unwrap(), 0.0001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0 as f64, p.unfilled_surplus, 0.0001f64, None).unwrap();
    });
}

#[test]
fn reports_unfilled_when_capacity_exhausted() {
    testlib::run("org.tiqian.layout.JustifierCompressionTest.reportsUnfilledWhenCapacityExhausted", "org.tiqian.layout.JustifierCompressionTest.reportsUnfilledWhenCapacityExhausted", || {
        TestTraceRecorder::new(&(UStr::new(&[74,117,115,116,105,102,105,101,114,67,111,109,112,114,101,115,115,105,111,110,84,101,115,116]))).section(UStr::new(&[114,101,112,111,114,116,115,85,110,102,105,108,108,101,100,87,104,101,110,67,97,112,97,99,105,116,121,69,120,104,97,117,115,116,101,100]));
        let p = Justifier::new(Some(0.5), Some(0.25)).compress(5 as f64, &vec![
    (ShrinkOpportunity::new(0u32, 1u32, 1 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
    (ShrinkOpportunity::new(1u32, 2u32, 1 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(3 as f64, p.unfilled_surplus, 0.0001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn zero_surplus_is_no_op() {
    testlib::run("org.tiqian.layout.JustifierCompressionTest.zeroSurplusIsNoOp", "org.tiqian.layout.JustifierCompressionTest.zeroSurplusIsNoOp", || {
        TestTraceRecorder::new(&(UStr::new(&[74,117,115,116,105,102,105,101,114,67,111,109,112,114,101,115,115,105,111,110,84,101,115,116]))).section(UStr::new(&[122,101,114,111,83,117,114,112,108,117,115,73,115,78,111,79,112]));
        let p = Justifier::new(Some(0.5), Some(0.25)).compress(0 as f64, &vec![
    (ShrinkOpportunity::new(0u32, 1u32, 5 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(0 as f64, p.unfilled_surplus, 0.0001f64, None).unwrap();
    });
}

#[test]
fn nan_surplus_emits_no_allocations() {
    testlib::run("org.tiqian.layout.JustifierCompressionTest.nanSurplusEmitsNoAllocations", "org.tiqian.layout.JustifierCompressionTest.nanSurplusEmitsNoAllocations", || {
        TestTraceRecorder::new(&(UStr::new(&[74,117,115,116,105,102,105,101,114,67,111,109,112,114,101,115,115,105,111,110,84,101,115,116]))).section(UStr::new(&[110,97,110,83,117,114,112,108,117,115,69,109,105,116,115,78,111,65,108,108,111,99,97,116,105,111,110,115]));
        let justifier = Justifier::new(Some(0.5), Some(0.25));
        let plan = justifier.compress(f64::NAN, &vec![(ShrinkOpportunity::new(0u32, 1u32, 5.0f64, ShrinkChannel::TrailingGlue, Some(false))).clone()]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((plan.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((plan.unfilled_surplus).is_nan(), None).unwrap();
    });
}
