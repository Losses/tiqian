#![cfg(test)]

use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::line_optimization::RepairCandidate;
use crate::org::tiqian::layout::line_optimization::RepairOptions;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkChannel;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::UnbreakableRanges;
use crate::org::tiqian::layout::push_in_line_wide_capacity_test_support::PushInLineWideCapacityTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;


#[derive(Debug, Clone, PartialEq)]
pub enum PushInLineWideCapacityTestPushInRejectsWhenLineWideCapacityStillInsufficientFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PushInLineWideCapacityTestPushInRejectsWhenLineWideCapacityStillInsufficientFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PushInLineWideCapacityTestPushInRejectsWhenLineWideCapacityStillInsufficientFault) -> Self {
        match value {
            PushInLineWideCapacityTestPushInRejectsWhenLineWideCapacityStillInsufficientFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PushInLineWideCapacityTestPushInRejectsWhenLineWideCapacityStillInsufficientFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PushInLineWideCapacityTestPushInRejectsWhenLineWideCapacityStillInsufficientFault) -> Self {
        match value {
            PushInLineWideCapacityTestPushInRejectsWhenLineWideCapacityStillInsufficientFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PushInLineWideCapacityTestPushInRejectsWhenLineWideCapacityStillInsufficientFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PushInLineWideCapacityTestPushInRejectsWhenLineWideCapacityStillInsufficientFault) -> Self {
        match value {
            PushInLineWideCapacityTestPushInRejectsWhenLineWideCapacityStillInsufficientFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PushInLineWideCapacityTestPushInRejectsWhenLineWideCapacityStillInsufficientFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PushInLineWideCapacityTestPushInRejectsWhenLineWideCapacityStillInsufficientFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PushInLineWideCapacityTestPushInRejectsWhenLineWideCapacityStillInsufficientFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PushInLineWideCapacityTestPushInRejectsWhenLineWideCapacityStillInsufficientFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PushInLineWideCapacityTestPushInRejectsWhenLineWideCapacityStillInsufficientFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PushInLineWideCapacityTestPushInRejectsWhenLineWideCapacityStillInsufficientFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PushInLineWideCapacityTestPushInOffenderOnlyCapacityStillWorksBackCompatFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PushInLineWideCapacityTestPushInOffenderOnlyCapacityStillWorksBackCompatFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PushInLineWideCapacityTestPushInOffenderOnlyCapacityStillWorksBackCompatFault) -> Self {
        match value {
            PushInLineWideCapacityTestPushInOffenderOnlyCapacityStillWorksBackCompatFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PushInLineWideCapacityTestPushInOffenderOnlyCapacityStillWorksBackCompatFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PushInLineWideCapacityTestPushInOffenderOnlyCapacityStillWorksBackCompatFault) -> Self {
        match value {
            PushInLineWideCapacityTestPushInOffenderOnlyCapacityStillWorksBackCompatFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PushInLineWideCapacityTestPushInOffenderOnlyCapacityStillWorksBackCompatFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PushInLineWideCapacityTestPushInOffenderOnlyCapacityStillWorksBackCompatFault) -> Self {
        match value {
            PushInLineWideCapacityTestPushInOffenderOnlyCapacityStillWorksBackCompatFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PushInLineWideCapacityTestPushInOffenderOnlyCapacityStillWorksBackCompatFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PushInLineWideCapacityTestPushInOffenderOnlyCapacityStillWorksBackCompatFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PushInLineWideCapacityTestPushInOffenderOnlyCapacityStillWorksBackCompatFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PushInLineWideCapacityTestPushInOffenderOnlyCapacityStillWorksBackCompatFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PushInLineWideCapacityTestPushInOffenderOnlyCapacityStillWorksBackCompatFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PushInLineWideCapacityTestPushInOffenderOnlyCapacityStillWorksBackCompatFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PushInLineWideCapacityTestPushInMergesOffenderThatFitsAfterChainedRepairsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PushInLineWideCapacityTestPushInMergesOffenderThatFitsAfterChainedRepairsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PushInLineWideCapacityTestPushInMergesOffenderThatFitsAfterChainedRepairsFault) -> Self {
        match value {
            PushInLineWideCapacityTestPushInMergesOffenderThatFitsAfterChainedRepairsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PushInLineWideCapacityTestPushInMergesOffenderThatFitsAfterChainedRepairsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PushInLineWideCapacityTestPushInMergesOffenderThatFitsAfterChainedRepairsFault) -> Self {
        match value {
            PushInLineWideCapacityTestPushInMergesOffenderThatFitsAfterChainedRepairsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PushInLineWideCapacityTestPushInMergesOffenderThatFitsAfterChainedRepairsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PushInLineWideCapacityTestPushInMergesOffenderThatFitsAfterChainedRepairsFault) -> Self {
        match value {
            PushInLineWideCapacityTestPushInMergesOffenderThatFitsAfterChainedRepairsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PushInLineWideCapacityTestPushInMergesOffenderThatFitsAfterChainedRepairsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PushInLineWideCapacityTestPushInMergesOffenderThatFitsAfterChainedRepairsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PushInLineWideCapacityTestPushInMergesOffenderThatFitsAfterChainedRepairsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PushInLineWideCapacityTestPushInMergesOffenderThatFitsAfterChainedRepairsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PushInLineWideCapacityTestPushInMergesOffenderThatFitsAfterChainedRepairsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PushInLineWideCapacityTestPushInMergesOffenderThatFitsAfterChainedRepairsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PushInLineWideCapacityTestPushInAggregatesShrinkFromMultiplePrecedingClustersFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PushInLineWideCapacityTestPushInAggregatesShrinkFromMultiplePrecedingClustersFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PushInLineWideCapacityTestPushInAggregatesShrinkFromMultiplePrecedingClustersFault) -> Self {
        match value {
            PushInLineWideCapacityTestPushInAggregatesShrinkFromMultiplePrecedingClustersFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PushInLineWideCapacityTestPushInAggregatesShrinkFromMultiplePrecedingClustersFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PushInLineWideCapacityTestPushInAggregatesShrinkFromMultiplePrecedingClustersFault) -> Self {
        match value {
            PushInLineWideCapacityTestPushInAggregatesShrinkFromMultiplePrecedingClustersFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PushInLineWideCapacityTestPushInAggregatesShrinkFromMultiplePrecedingClustersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PushInLineWideCapacityTestPushInAggregatesShrinkFromMultiplePrecedingClustersFault) -> Self {
        match value {
            PushInLineWideCapacityTestPushInAggregatesShrinkFromMultiplePrecedingClustersFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PushInLineWideCapacityTestPushInAggregatesShrinkFromMultiplePrecedingClustersFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PushInLineWideCapacityTestPushInAggregatesShrinkFromMultiplePrecedingClustersFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PushInLineWideCapacityTestPushInAggregatesShrinkFromMultiplePrecedingClustersFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PushInLineWideCapacityTestPushInAggregatesShrinkFromMultiplePrecedingClustersFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PushInLineWideCapacityTestPushInAggregatesShrinkFromMultiplePrecedingClustersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PushInLineWideCapacityTestPushInAggregatesShrinkFromMultiplePrecedingClustersFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PushInLineWideCapacityTestCarryPreviousRefusesToSplitUnbreakableSpanFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PushInLineWideCapacityTestCarryPreviousRefusesToSplitUnbreakableSpanFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PushInLineWideCapacityTestCarryPreviousRefusesToSplitUnbreakableSpanFault) -> Self {
        match value {
            PushInLineWideCapacityTestCarryPreviousRefusesToSplitUnbreakableSpanFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PushInLineWideCapacityTestCarryPreviousRefusesToSplitUnbreakableSpanFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PushInLineWideCapacityTestCarryPreviousRefusesToSplitUnbreakableSpanFault) -> Self {
        match value {
            PushInLineWideCapacityTestCarryPreviousRefusesToSplitUnbreakableSpanFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PushInLineWideCapacityTestCarryPreviousRefusesToSplitUnbreakableSpanFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PushInLineWideCapacityTestCarryPreviousRefusesToSplitUnbreakableSpanFault) -> Self {
        match value {
            PushInLineWideCapacityTestCarryPreviousRefusesToSplitUnbreakableSpanFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PushInLineWideCapacityTestCarryPreviousRefusesToSplitUnbreakableSpanFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PushInLineWideCapacityTestCarryPreviousRefusesToSplitUnbreakableSpanFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PushInLineWideCapacityTestCarryPreviousRefusesToSplitUnbreakableSpanFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PushInLineWideCapacityTestCarryPreviousRefusesToSplitUnbreakableSpanFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PushInLineWideCapacityTestCarryPreviousRefusesToSplitUnbreakableSpanFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PushInLineWideCapacityTestCarryPreviousRefusesToSplitUnbreakableSpanFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn push_in_aggregates_shrink_from_multiple_preceding_clusters() {
    testlib::run("org.tiqian.layout.PushInLineWideCapacityTest.pushInAggregatesShrinkFromMultiplePrecedingClusters", "org.tiqian.layout.PushInLineWideCapacityTest.pushInAggregatesShrinkFromMultiplePrecedingClusters", || {
        let mut t = TestTraceRecorder::new("PushInLineWideCapacityTest");
        t.section(&"pushInAggregatesShrinkFromMultiplePrecedingClusters");
        let mut c: Vec<Cluster> = vec![];
        {
            c.push(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(0, 1, &"中", 16 as f64).unwrap());
            c.push(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(1, 2, &"中", 16 as f64).unwrap());
            c.push(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(2, 3, &"中", 16 as f64).unwrap());
            c.push(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(3, 4, &"中", 16 as f64).unwrap());
            c.push(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(4, 5, &"中", 16 as f64).unwrap());
        }
        c.push(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(5, 6, &"、", 16 as f64).unwrap());
        {
            c.push(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(6, 7, &"文", 16 as f64).unwrap());
            c.push(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(7, 8, &"文", 16 as f64).unwrap());
            c.push(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(8, 9, &"文", 16 as f64).unwrap());
            c.push(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(9, 10, &"文", 16 as f64).unwrap());
        }
        c.push(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(10, 11, &"。", 16 as f64).unwrap());
        let s = GreedyLineBreaker::new(None, None, None, None).break_lines(&c, &c, 160 as f64, Some(vec![
    (ShrinkOpportunity::new(5u32, 6u32, 8 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
    (ShrinkOpportunity::new(10u32, 6u32, 8 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
]), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let l = (s.lines[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 10u32), (l.cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(160 as f64, l.adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_not_null_rendered(l.repair != None, match &(l.repair) { None => "null".to_string(), Some(__option2) =>
TestTraceRender::test_trace_render_cap(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_push_in_string((*__option2).clone()).as_str()).unwrap().to_string() }.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_is_push_in((l.repair).as_ref().unwrap().clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(10, PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_push_in_offender_cluster_index((l.repair).as_ref().unwrap().clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_push_in_total_shrink((l.repair).as_ref().unwrap().clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_push_in_total_available_capacity((l.repair).as_ref().unwrap().clone()), None).unwrap();
        let mut indexes_m1: Vec<u32> = vec![];
        {
            let _g1 = PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_push_in_allocations((l.repair).as_ref().unwrap().clone());
            for a in &_g1 {
                indexes_m1.push(a.cluster_index);
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![10, 5], &indexes_m1, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_push_in_allocations((l.repair).as_ref().unwrap().clone())[0usize].shrink, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_push_in_allocations((l.repair).as_ref().unwrap().clone())[1usize].shrink, None).unwrap();
    });
}

#[test]
fn push_in_rejects_when_line_wide_capacity_still_insufficient() {
    testlib::run("org.tiqian.layout.PushInLineWideCapacityTest.pushInRejectsWhenLineWideCapacityStillInsufficient", "org.tiqian.layout.PushInLineWideCapacityTest.pushInRejectsWhenLineWideCapacityStillInsufficient", || {
        let mut t = TestTraceRecorder::new("PushInLineWideCapacityTest");
        t.section(&"pushInRejectsWhenLineWideCapacityStillInsufficient");
        let mut _g: Vec<Cluster> = vec![];
        for i in 0..11 {
            _g.push(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(i, u32::wrapping_add(i, 1), if i == 5 { "、".to_string() } else { if i == 10 { "。".to_string() } else { "文".to_string() }.to_string() }.as_str(), 16 as f64).unwrap());
        }
        let c = (_g).clone();
        let s = GreedyLineBreaker::new(None, None, None, None).break_lines(&c, &c, 160 as f64, Some(vec![
    (ShrinkOpportunity::new(5u32, 6u32, 4 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
]), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 8u32), ((s.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(9u32, 10u32), ((s.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_is_carry_previous(((s.lines[1usize]).clone().repair).as_ref().unwrap().clone()), None).unwrap();
        let mut p: Option<RepairCandidate> = None;
        {
            let _g1 = ((s.lines[1usize]).clone().repair_candidates).clone();
            for cand in &_g1 {
                if cand.kind.to_string() == "PushIn" {
                    p = Some(cand.clone());
                    break;
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_not_null_rendered(p.is_some(), PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_render_nullable_candidate((p).clone()).unwrap().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bool(false, p.as_ref().unwrap().accepted, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"insufficient-capacity", ((p.as_ref().unwrap().rejection_reason).clone()).as_deref().unwrap_or(""), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, p.as_ref().unwrap().available_capacity, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, p.as_ref().unwrap().required_shrink, None).unwrap();
    });
}

#[test]
fn push_in_offender_only_capacity_still_works_back_compat() {
    testlib::run("org.tiqian.layout.PushInLineWideCapacityTest.pushInOffenderOnlyCapacityStillWorksBackCompat", "org.tiqian.layout.PushInLineWideCapacityTest.pushInOffenderOnlyCapacityStillWorksBackCompat", || {
        let mut t = TestTraceRecorder::new("PushInLineWideCapacityTest");
        t.section(&"pushInOffenderOnlyCapacityStillWorksBackCompat");
        let c = vec![
    (PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(0, 1, &"中", 16 as f64).unwrap()).clone(),
    (PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(1, 2, &"文", 16 as f64).unwrap()).clone(),
    (PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(2, 3, &"中", 16 as f64).unwrap()).clone(),
    (PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(3, 4, &"。", 16 as f64).unwrap()).clone(),
];
        let s = GreedyLineBreaker::new(None, None, None, None).break_lines(&c, &c, 60 as f64, Some(vec![
    (ShrinkOpportunity::new(3u32, 6u32, 4 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
]), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_is_push_in(((s.lines[0usize]).clone().repair).as_ref().unwrap().clone()), None).unwrap();
        let mut indexes_m3: Vec<u32> = vec![];
        {
            let _g1 = PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_push_in_allocations(((s.lines[0usize]).clone().repair).as_ref().unwrap().clone());
            for a in &_g1 {
                indexes_m3.push(a.cluster_index);
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![3], &indexes_m3, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_push_in_total_shrink(((s.lines[0usize]).clone().repair).as_ref().unwrap().clone()), None).unwrap();
    });
}

#[test]
fn push_in_merges_offender_that_fits_after_chained_repairs() {
    testlib::run("org.tiqian.layout.PushInLineWideCapacityTest.pushInMergesOffenderThatFitsAfterChainedRepairs", "org.tiqian.layout.PushInLineWideCapacityTest.pushInMergesOffenderThatFitsAfterChainedRepairs", || {
        let mut t = TestTraceRecorder::new("PushInLineWideCapacityTest");
        t.section(&"pushInMergesOffenderThatFitsAfterChainedRepairs");
        let mut _g: Vec<Cluster> = vec![];
        for i in 0..10 {
            _g.push(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(i, u32::wrapping_add(i, 1), if i == 3 { "」".to_string() } else { if i == 4 { "。".to_string() } else { if i == 8 { "、".to_string() } else { "中".to_string() }.to_string()
}.to_string() }.as_str(), 16 as f64).unwrap());
        }
        let c = (_g).clone();
        let s = GreedyLineBreaker::new(None, None, None, None).break_lines(&c, &c, 64 as f64, Some(vec![
    (ShrinkOpportunity::new(3u32, 6u32, 8 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
    (ShrinkOpportunity::new(4u32, 6u32, 8 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
    (ShrinkOpportunity::new(8u32, 6u32, 8 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
]), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 4u32), ((s.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(5u32, 8u32), ((s.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(9u32, 9u32), ((s.lines[2usize]).clone().cluster_range).clone(), None).unwrap();
        {
            let _g1 = s.lines.clone();
            for line in &_g1 {
                let first = ((c[usize::try_from((line.cluster_range).clone().start).unwrap_or(0)]).clone().text).to_string();
                let _ = TracedAssertions::traced_assertions_assert_true(first == "中", Some((format!("{}{}{}",
            "line starts with forbidden '",
            first,
            "'"
        )).to_string())).unwrap();
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_is_push_in(((s.lines[1usize]).clone().repair).as_ref().unwrap().clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_push_in_total_shrink(((s.lines[1usize]).clone().repair).as_ref().unwrap().clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&RepairOptions::repair_options_reason(((s.lines[1usize]).clone().repair).as_ref().unwrap().clone()), "fits-no-shrink", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(64 as f64, s.lines[1usize].adjusted_width, None).unwrap();
    });
}

#[test]
fn carry_previous_refuses_to_split_unbreakable_span() {
    testlib::run("org.tiqian.layout.PushInLineWideCapacityTest.carryPreviousRefusesToSplitUnbreakableSpan", "org.tiqian.layout.PushInLineWideCapacityTest.carryPreviousRefusesToSplitUnbreakableSpan", || {
        let mut t = TestTraceRecorder::new("PushInLineWideCapacityTest");
        t.section(&"carryPreviousRefusesToSplitUnbreakableSpan");
        let c = vec![
    (PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(0, 1, &"中", 16 as f64).unwrap()).clone(),
    (PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(1, 2, &"中", 16 as f64).unwrap()).clone(),
    (PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(2, 3, &"王", 16 as f64).unwrap()).clone(),
    (PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(3, 4, &"小", 16 as f64).unwrap()).clone(),
    (PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(4, 5, &"明", 16 as f64).unwrap()).clone(),
    (PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_cluster(5, 6, &"。", 16 as f64).unwrap()).clone(),
];
        let s = GreedyLineBreaker::new(None, None, None, None).break_lines(&c, &c, 80 as f64, Some(vec![
    (ShrinkOpportunity::new(5u32, 6u32, 8 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
]), Some(UnbreakableRanges::new(vec![(IntRange::new(2u32, 4u32)).clone()].to_vec())), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((s.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_is_leave_ragged(((s.lines[1usize]).clone().repair).as_ref().unwrap().clone()), None).unwrap();
        let reason = RepairOptions::repair_options_reason(((s.lines[1usize]).clone().repair).as_ref().unwrap().clone());
        let _ = TracedAssertions::traced_assertions_assert_true(u_string::substr(&reason, i32::from_ne_bytes((u32::wrapping_sub(u_string::unit_count(&(reason)), u_string::unit_count(&("carry-would-split-mourning-span")))).to_ne_bytes()), None) == "carry-would-split-mourning-span",
None).unwrap();
        let mut carry: Option<RepairCandidate> = None;
        {
            let _g1 = ((s.lines[1usize]).clone().repair_candidates).clone();
            for cand in &_g1 {
                if cand.kind.to_string() == "CarryPrevious" {
                    carry = Some(cand.clone());
                    break;
                }
            }
        }
        if carry.is_none() {
            panic!("{}", TextRangeError::Message { text: "CarryPrevious candidate not found".to_string() });
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"carry-would-split-mourning-span", (((carry).as_ref().unwrap().rejection_reason).clone()).as_deref().unwrap_or(""), None).unwrap();
    });
}
