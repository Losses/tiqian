#![cfg(test)]

use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakDecisions;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::progressive_technical_break_test_support::ProgressiveTechnicalBreakTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveTechnicalBreakTestSourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllableFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ProgressiveTechnicalBreakTestSourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllableFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveTechnicalBreakTestSourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllableFault) -> Self {
        match value {
            ProgressiveTechnicalBreakTestSourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllableFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveTechnicalBreakTestSourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllableFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveTechnicalBreakTestSourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllableFault) -> Self {
        match value {
            ProgressiveTechnicalBreakTestSourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllableFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveTechnicalBreakTestSourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllableFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveTechnicalBreakTestSourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllableFault) -> Self {
        match value {
            ProgressiveTechnicalBreakTestSourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllableFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveTechnicalBreakTestSourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllableFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveTechnicalBreakTestSourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllableFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveTechnicalBreakTestSourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllableFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveTechnicalBreakTestSourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllableFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveTechnicalBreakTestSourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllableFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveTechnicalBreakTestSourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllableFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressiveTechnicalBreakTestLookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCutFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ProgressiveTechnicalBreakTestLookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCutFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ProgressiveTechnicalBreakTestLookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCutFault) -> Self {
        match value {
            ProgressiveTechnicalBreakTestLookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCutFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveTechnicalBreakTestLookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCutFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ProgressiveTechnicalBreakTestLookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCutFault) -> Self {
        match value {
            ProgressiveTechnicalBreakTestLookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCutFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ProgressiveTechnicalBreakTestLookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCutFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ProgressiveTechnicalBreakTestLookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCutFault) -> Self {
        match value {
            ProgressiveTechnicalBreakTestLookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCutFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ProgressiveTechnicalBreakTestLookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCutFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ProgressiveTechnicalBreakTestLookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCutFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ProgressiveTechnicalBreakTestLookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCutFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ProgressiveTechnicalBreakTestLookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCutFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ProgressiveTechnicalBreakTestLookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCutFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ProgressiveTechnicalBreakTestLookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCutFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn source_whitespace_capacity_keeps_structural_tier_ahead_of_syllable() {
    testlib::run("org.tiqian.layout.ProgressiveTechnicalBreakTest.sourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllable", "org.tiqian.layout.ProgressiveTechnicalBreakTest.sourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllable", || {
        let mut t = TestTraceRecorder::new("ProgressiveTechnicalBreakTest");
        t.section(&"sourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllable");
        let span = TextRange::new(0u32, 6u32).unwrap();
        let c = vec![
    (ProgressiveTechnicalBreakTestSupport::progressive_technical_break_test_support_cluster(0, &"a", 20 as f64).unwrap()).clone(),
    (ProgressiveTechnicalBreakTestSupport::progressive_technical_break_test_support_cluster(1, &" ", 4 as f64).unwrap()).clone(),
    (ProgressiveTechnicalBreakTestSupport::progressive_technical_break_test_support_cluster(2, &"b", 28 as f64).unwrap()).clone(),
    (ProgressiveTechnicalBreakTestSupport::progressive_technical_break_test_support_cluster(3, &"/", 28 as f64).unwrap()).clone(),
    (ProgressiveTechnicalBreakTestSupport::progressive_technical_break_test_support_cluster(4, &"c", 2 as f64).unwrap()).clone(),
    (ProgressiveTechnicalBreakTestSupport::progressive_technical_break_test_support_cluster(5, &"d", 20 as f64).unwrap()).clone(),
];
        let o: SortedMapTable<u32, ProgressiveBreakOpportunity> = ProgressiveTechnicalBreakTestSupport::progressive_technical_break_test_support_opportunity_map(&vec![2, 4, 5], &vec![
    (ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Whitespace, (span).clone(), Some(4 as f64))).clone(),
    (ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Structural, (span).clone(), Some(0.0))).clone(),
    (ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Syllable, (span).clone(), Some(0.0))).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, ProgressiveBreakDecisions::progressive_break_decisions_decide_progressive_break(0, 5, (o).clone(), Some((c).clone()), Some(84 as f64 as f64), None, Some(8 as f64 as f64), None, None), None).unwrap();
    });
}

#[test]
fn lookahead_may_not_replace_selected_emergency_boundary_with_earlier_same_tier_cut() {
    testlib::run("org.tiqian.layout.ProgressiveTechnicalBreakTest.lookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCut", "org.tiqian.layout.ProgressiveTechnicalBreakTest.lookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCut", || {
        let mut t = TestTraceRecorder::new("ProgressiveTechnicalBreakTest");
        t.section(&"lookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCut");
        let span = TextRange::new(0u32, 5u32).unwrap();
        let mut _g: Vec<Cluster> = vec![];
        {
            _g.push(ProgressiveTechnicalBreakTestSupport::progressive_technical_break_test_support_cluster(0, &"a", 20 as f64).unwrap());
            _g.push(ProgressiveTechnicalBreakTestSupport::progressive_technical_break_test_support_cluster(1, &"b", 20 as f64).unwrap());
            _g.push(ProgressiveTechnicalBreakTestSupport::progressive_technical_break_test_support_cluster(2, &"c", 20 as f64).unwrap());
            _g.push(ProgressiveTechnicalBreakTestSupport::progressive_technical_break_test_support_cluster(3, &"d", 20 as f64).unwrap());
            _g.push(ProgressiveTechnicalBreakTestSupport::progressive_technical_break_test_support_cluster(4, &"e", 20 as f64).unwrap());
        }
        let c = (_g).clone();
        let o: SortedMapTable<u32, ProgressiveBreakOpportunity> = ProgressiveTechnicalBreakTestSupport::progressive_technical_break_test_support_opportunity_map(&vec![3, 4], &vec![
    (ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Emergency, (span).clone(), Some(0.0))).clone(),
    (ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Emergency, (span).clone(), Some(0.0))).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_false(ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_allowed(0, 4, 3, (o).clone(), Some((c).clone()), Some(90 as f64 as f64), None, Some(8 as f64 as f64), None, None), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(ProgressiveBreakDecisions::progressive_break_decisions_progressive_candidate_allowed(0, 4, 4, (o).clone(), Some((c).clone()), Some(90 as f64 as f64), None, Some(8 as f64 as f64), None, None), None).unwrap();
    });
}
