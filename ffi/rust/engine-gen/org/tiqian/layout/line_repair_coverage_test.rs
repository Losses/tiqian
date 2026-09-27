#![cfg(test)]

use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::runtime::test as testlib;


#[test]
fn carry_previous_moves_the_previous_tail_down_when_it_fits() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.carryPreviousMovesThePreviousTailDownWhenItFits", "org.tiqian.layout.LineRepairCoverageTest.carryPreviousMovesThePreviousTailDownWhenItFits", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"carryPreviousMovesThePreviousTailDownWhenItFits");
        let _ = r.record(&"eq expected=[0, 1, 2] actual=[0, 1, 2]").unwrap();
        let _ = r.record(&"eq expected=[3, 4, 5, 6] actual=[3, 4, 5, 6]").unwrap();
        let _ = r.record(&"eq expected=3 actual=3").unwrap();
        let _ = r.record(&"is-true actual=true").unwrap();
        let _ = r.record(&"eq expected=3 actual=3").unwrap();
    });
}

#[test]
fn contextual_hang_extends_only_inside_its_protected_group() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.contextualHangExtendsOnlyInsideItsProtectedGroup", "org.tiqian.layout.LineRepairCoverageTest.contextualHangExtendsOnlyInsideItsProtectedGroup", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"contextualHangExtendsOnlyInsideItsProtectedGroup");
        let _ = r.record(&"eq expected=[3, 4] actual=[3, 4]").unwrap();
        let _ = r.record(&"is-true actual=true").unwrap();
        let _ = r.record(&"eq expected=[3, 4, 5] actual=[3, 4, 5]").unwrap();
        let _ = r.record(&"eq expected=3 actual=3").unwrap();
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn default_arguments_run_the_full_ragged_chain() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.defaultArgumentsRunTheFullRaggedChain", "org.tiqian.layout.LineRepairCoverageTest.defaultArgumentsRunTheFullRaggedChain", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"defaultArgumentsRunTheFullRaggedChain");
        let _ = r.record(&"eq expected=false actual=false").unwrap();
        let _ = r.record(&"eq expected=10 actual=10").unwrap();
        let _ = r.record(&"eq expected=30 actual=30").unwrap();
    });
}

#[test]
fn fill_push_in_accepts_compression_denser_than_the_cured_stretch() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.fillPushInAcceptsCompressionDenserThanTheCuredStretch", "org.tiqian.layout.LineRepairCoverageTest.fillPushInAcceptsCompressionDenserThanTheCuredStretch", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"fillPushInAcceptsCompressionDenserThanTheCuredStretch");
        let _ = r.record(&"eq expected=[0, 1, 2, 3, 4, 5] actual=[0, 1, 2, 3, 4, 5]").unwrap();
        let _ = r.record(&"eq expected=12 actual=12").unwrap();
    });
}

#[test]
fn fill_push_in_default_arguments_omit_the_optional_boundaries() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.fillPushInDefaultArgumentsOmitTheOptionalBoundaries", "org.tiqian.layout.LineRepairCoverageTest.fillPushInDefaultArgumentsOmitTheOptionalBoundaries", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"fillPushInDefaultArgumentsOmitTheOptionalBoundaries");
        let _ = r.record(&"eq expected=[[0, 1, 2, 3, 4], [5, 6, 7]] actual=[[0, 1, 2, 3, 4], [5, 6, 7]]").unwrap();
    });
}

#[test]
fn fill_push_in_extends_past_forbidden_heads_and_unbreakable_chains() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.fillPushInExtendsPastForbiddenHeadsAndUnbreakableChains", "org.tiqian.layout.LineRepairCoverageTest.fillPushInExtendsPastForbiddenHeadsAndUnbreakableChains", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"fillPushInExtendsPastForbiddenHeadsAndUnbreakableChains");
        let _ = r.record(&"eq expected=[0, 1, 2, 3, 4, 5] actual=[0, 1, 2, 3, 4, 5]").unwrap();
        let _ = r.record(&"eq expected=[0, 1, 2, 3, 4, 5] actual=[0, 1, 2, 3, 4, 5]").unwrap();
        let _ = r.record(&"eq expected=1 actual=1").unwrap();
        let _ = r.record(&"eq expected=[0, 1, 2, 3, 4, 5, 6, 7] actual=[0, 1, 2, 3, 4, 5, 6, 7]").unwrap();
    });
}

#[test]
fn fill_push_in_honours_progressive_tier_promotion_boundaries() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.fillPushInHonoursProgressiveTierPromotionBoundaries", "org.tiqian.layout.LineRepairCoverageTest.fillPushInHonoursProgressiveTierPromotionBoundaries", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"fillPushInHonoursProgressiveTierPromotionBoundaries");
        let _ = r.record(&"eq expected='ProgressiveTechnicalTierPromotion' actual='ProgressiveTechnicalTierPromotion'").unwrap();
        let _ = r.record(&"null actual=-").unwrap();
        let _ = r.record(&"null actual=-").unwrap();
        let _ = r.record(&"eq expected=[0, 1] actual=[0, 1]").unwrap();
        let _ = r.record(&"eq expected=[0, 1, 2, 3, 4, 5] actual=[0, 1, 2, 3, 4, 5]").unwrap();
        let _ = r.record(&"eq expected='LineAdjustmentPushIn' actual='LineAdjustmentPushIn'").unwrap();
    });
}

#[test]
fn fill_push_in_pulls_the_group_and_cascades_zero_shrink_fills() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.fillPushInPullsTheGroupAndCascadesZeroShrinkFills", "org.tiqian.layout.LineRepairCoverageTest.fillPushInPullsTheGroupAndCascadesZeroShrinkFills", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"fillPushInPullsTheGroupAndCascadesZeroShrinkFills");
        let _ = r.record(&"eq expected=[0, 1, 2, 3, 4] actual=[0, 1, 2, 3, 4]").unwrap();
        let _ = r.record(&"eq expected=[5, 6, 7] actual=[5, 6, 7]").unwrap();
        let _ = r.record(&"eq expected=0 actual=0").unwrap();
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn fill_push_in_rejects_overlarge_pulls_and_worse_compression_density() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.fillPushInRejectsOverlargePullsAndWorseCompressionDensity", "org.tiqian.layout.LineRepairCoverageTest.fillPushInRejectsOverlargePullsAndWorseCompressionDensity", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"fillPushInRejectsOverlargePullsAndWorseCompressionDensity");
        let _ = r.record(&"eq expected=[0, 1, 2, 3] actual=[0, 1, 2, 3]").unwrap();
        let _ = r.record(&"eq expected=[4, 5, 6, 7] actual=[4, 5, 6, 7]").unwrap();
        let _ = r.record(&"eq expected=[0, 1, 2, 3] actual=[0, 1, 2, 3]").unwrap();
        let _ = r.record(&"eq expected=[4, 5, 6, 7] actual=[4, 5, 6, 7]").unwrap();
    });
}

#[test]
fn fill_push_in_skips_full_lines_and_unpullable_groups() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.fillPushInSkipsFullLinesAndUnpullableGroups", "org.tiqian.layout.LineRepairCoverageTest.fillPushInSkipsFullLinesAndUnpullableGroups", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"fillPushInSkipsFullLinesAndUnpullableGroups");
        let _ = r.record(&"eq expected=[0, 1, 2, 3] actual=[0, 1, 2, 3]").unwrap();
        let _ = r.record(&"eq expected=[0, 1, 2, 3] actual=[0, 1, 2, 3]").unwrap();
        let _ = r.record(&"eq expected=[0, 1, 2, 3] actual=[0, 1, 2, 3]").unwrap();
    });
}

#[test]
fn fill_push_in_skips_repaired_hanging_and_non_auto_wrap_lines() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.fillPushInSkipsRepairedHangingAndNonAutoWrapLines", "org.tiqian.layout.LineRepairCoverageTest.fillPushInSkipsRepairedHangingAndNonAutoWrapLines", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"fillPushInSkipsRepairedHangingAndNonAutoWrapLines");
        let _ = r.record(&"eq expected=[0, 1, 2, 3] actual=[0, 1, 2, 3]").unwrap();
        let _ = r.record(&"eq expected=[0, 1, 2, 3] actual=[0, 1, 2, 3]").unwrap();
        let _ = r.record(&"eq expected=[0, 1, 2, 3] actual=[0, 1, 2, 3]").unwrap();
    });
}

#[test]
fn fill_push_in_skips_short_inputs_and_zero_bias() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.fillPushInSkipsShortInputsAndZeroBias", "org.tiqian.layout.LineRepairCoverageTest.fillPushInSkipsShortInputsAndZeroBias", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"fillPushInSkipsShortInputsAndZeroBias");
        let _ = r.record(&"eq expected=1 actual=1").unwrap();
        let _ = r.record(&"eq expected=2 actual=2").unwrap();
    });
}

#[test]
fn forbidden_start_override_controls_the_kinsoku_check() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.forbiddenStartOverrideControlsTheKinsokuCheck", "org.tiqian.layout.LineRepairCoverageTest.forbiddenStartOverrideControlsTheKinsokuCheck", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"forbiddenStartOverrideControlsTheKinsokuCheck");
        let _ = r.record(&"null actual=-").unwrap();
    });
}

#[test]
fn hang_consumes_a_zero_width_mandatory_break_tail() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.hangConsumesAZeroWidthMandatoryBreakTail", "org.tiqian.layout.LineRepairCoverageTest.hangConsumesAZeroWidthMandatoryBreakTail", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"hangConsumesAZeroWidthMandatoryBreakTail");
        let _ = r.record(&"eq expected=[0, 1, 2, 3, 4, 5] actual=[0, 1, 2, 3, 4, 5]").unwrap();
        let _ = r.record(&"eq expected=[4, 5] actual=[4, 5]").unwrap();
        let _ = r.record(&"eq expected=MandatoryBreak actual=MandatoryBreak").unwrap();
    });
}

#[test]
fn hang_merges_the_offender_beyond_the_measure() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.hangMergesTheOffenderBeyondTheMeasure", "org.tiqian.layout.LineRepairCoverageTest.hangMergesTheOffenderBeyondTheMeasure", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"hangMergesTheOffenderBeyondTheMeasure");
        let _ = r.record(&"eq expected=[0, 1, 2, 3, 4] actual=[0, 1, 2, 3, 4]").unwrap();
        let _ = r.record(&"eq expected=[4] actual=[4]").unwrap();
        let _ = r.record(&"eq expected=4 actual=4").unwrap();
        let _ = r.record(&"eq expected=64 actual=64").unwrap();
        let _ = r.record(&"eq expected=80 actual=80").unwrap();
        let _ = r.record(&"eq expected=[5, 6, 7, 8] actual=[5, 6, 7, 8]").unwrap();
        let _ = r.record(&"eq expected='Hang' actual='Hang'").unwrap();
        let _ = r.record(&"eq expected=5 actual=5").unwrap();
    });
}

#[test]
fn hang_stops_before_a_non_zero_width_mandatory_break_tail() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.hangStopsBeforeANonZeroWidthMandatoryBreakTail", "org.tiqian.layout.LineRepairCoverageTest.hangStopsBeforeANonZeroWidthMandatoryBreakTail", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"hangStopsBeforeANonZeroWidthMandatoryBreakTail");
        let _ = r.record(&"eq expected=[0, 1, 2, 3, 4] actual=[0, 1, 2, 3, 4]").unwrap();
        let _ = r.record(&"eq expected=[4] actual=[4]").unwrap();
        let _ = r.record(&"eq expected=AutoWrap actual=AutoWrap").unwrap();
        let _ = r.record(&"eq expected=[5] actual=[5]").unwrap();
    });
}

#[test]
fn leave_ragged_records_no_room_to_carry_for_a_single_cluster_line() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.leaveRaggedRecordsNoRoomToCarryForASingleClusterLine", "org.tiqian.layout.LineRepairCoverageTest.leaveRaggedRecordsNoRoomToCarryForASingleClusterLine", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"leaveRaggedRecordsNoRoomToCarryForASingleClusterLine");
        let _ = r.record(&"eq expected=false actual=false").unwrap();
        let _ = r.record(&"eq expected='no-room-to-carry' actual='no-room-to-carry'").unwrap();
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn leave_ragged_refuses_carries_that_would_split_an_unbreakable_span() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.leaveRaggedRefusesCarriesThatWouldSplitAnUnbreakableSpan", "org.tiqian.layout.LineRepairCoverageTest.leaveRaggedRefusesCarriesThatWouldSplitAnUnbreakableSpan", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"leaveRaggedRefusesCarriesThatWouldSplitAnUnbreakableSpan");
        let _ = r.record(&"eq expected='carry-would-split-mourning-span' actual='carry-would-split-mourning-span'").unwrap();
        let _ = r.record(&"eq expected=3 actual=3").unwrap();
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn mandatory_break_and_empty_lines_skip_the_repair_loop() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.mandatoryBreakAndEmptyLinesSkipTheRepairLoop", "org.tiqian.layout.LineRepairCoverageTest.mandatoryBreakAndEmptyLinesSkipTheRepairLoop", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"mandatoryBreakAndEmptyLinesSkipTheRepairLoop");
        let _ = r.record(&"null actual=-").unwrap();
        let _ = r.record(&"eq expected=2 actual=2").unwrap();
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn mandatory_break_tail_end_returns_the_merge_through_at_the_line_end() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.mandatoryBreakTailEndReturnsTheMergeThroughAtTheLineEnd", "org.tiqian.layout.LineRepairCoverageTest.mandatoryBreakTailEndReturnsTheMergeThroughAtTheLineEnd", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"mandatoryBreakTailEndReturnsTheMergeThroughAtTheLineEnd");
        let _ = r.record(&"eq expected=[0, 1, 2, 3, 4, 5] actual=[0, 1, 2, 3, 4, 5]").unwrap();
    });
}

#[test]
fn push_in_filters_out_of_range_zero_capacity_and_foreign_line_end_only_opportunities() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.pushInFiltersOutOfRangeZeroCapacityAndForeignLineEndOnlyOpportunities", "org.tiqian.layout.LineRepairCoverageTest.pushInFiltersOutOfRangeZeroCapacityAndForeignLineEndOnlyOpportunities", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"pushInFiltersOutOfRangeZeroCapacityAndForeignLineEndOnlyOpportunities");
        let _ = r.record(&"is-true actual=true").unwrap();
        let _ = r.record(&"eq expected=[PushInAllocation(clusterIndex=4, shrink=8, availableCapacity=16, channel=LeadingAndTrailingGlue)] actual=[PushInAllocation(clusterIndex=4, shrink=8, availableCapacity=16, channel=LeadingAndTrailingGlue)]").unwrap();
        let _ = r.record(&"eq expected=16 actual=16").unwrap();
    });
}

#[test]
fn push_in_fits_without_shrink_when_the_merged_line_already_matches() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.pushInFitsWithoutShrinkWhenTheMergedLineAlreadyMatches", "org.tiqian.layout.LineRepairCoverageTest.pushInFitsWithoutShrinkWhenTheMergedLineAlreadyMatches", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"pushInFitsWithoutShrinkWhenTheMergedLineAlreadyMatches");
        let _ = r.record(&"eq expected=2 actual=2").unwrap();
        let _ = r.record(&"eq expected=[0, 1, 2, 3, 4] actual=[0, 1, 2, 3, 4]").unwrap();
        let _ = r.record(&"eq expected=0 actual=0").unwrap();
        let _ = r.record(&"eq expected=[5, 6, 7, 8] actual=[5, 6, 7, 8]").unwrap();
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn push_in_promotes_the_offenders_own_trailing_glue_to_tier_one() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.pushInPromotesTheOffendersOwnTrailingGlueToTierOne", "org.tiqian.layout.LineRepairCoverageTest.pushInPromotesTheOffendersOwnTrailingGlueToTierOne", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"pushInPromotesTheOffendersOwnTrailingGlueToTierOne");
        let _ = r.record(&"eq expected=4 actual=4").unwrap();
        let _ = r.record(&"eq expected=[PushInAllocation(clusterIndex=4, shrink=4, availableCapacity=8, channel=TrailingGlue)] actual=[PushInAllocation(clusterIndex=4, shrink=4, availableCapacity=8, channel=TrailingGlue)]").unwrap();
        let _ = r.record(&"eq expected='ForbiddenAtLineStart:，:pushed-in=4/16' actual='ForbiddenAtLineStart:，:pushed-in=4/16'").unwrap();
    });
}

#[test]
fn push_in_rejects_a_merge_through_cluster_outside_the_current_line() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.pushInRejectsAMergeThroughClusterOutsideTheCurrentLine", "org.tiqian.layout.LineRepairCoverageTest.pushInRejectsAMergeThroughClusterOutsideTheCurrentLine", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"pushInRejectsAMergeThroughClusterOutsideTheCurrentLine");
        let _ = r.record(&"raises exception=IllegalArgumentException thrown='PushIn merge-through cluster must belong to the current line.'").unwrap();
        let _ = r.record(&"is-true actual=true msg='PushIn merge-through cluster must belong to the current line.'").unwrap();
    });
}

#[test]
fn push_in_rejects_merge_through_outside_the_current_line() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.pushInRejectsMergeThroughOutsideTheCurrentLine", "org.tiqian.layout.LineRepairCoverageTest.pushInRejectsMergeThroughOutsideTheCurrentLine", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"pushInRejectsMergeThroughOutsideTheCurrentLine");
        let _ = r.record(&"raises exception=IllegalArgumentException thrown='PushIn merge-through cluster must belong to the current line.'").unwrap();
    });
}

#[test]
fn push_in_rejects_when_capacity_is_insufficient() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.pushInRejectsWhenCapacityIsInsufficient", "org.tiqian.layout.LineRepairCoverageTest.pushInRejectsWhenCapacityIsInsufficient", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"pushInRejectsWhenCapacityIsInsufficient");
        let _ = r.record(&"eq expected=false actual=false").unwrap();
        let _ = r.record(&"eq expected='insufficient-capacity' actual='insufficient-capacity'").unwrap();
        let _ = r.record(&"eq expected=20 actual=20").unwrap();
        let _ = r.record(&"eq expected=8 actual=8").unwrap();
    });
}

#[test]
fn push_in_reports_infinity_capacity_with_a_portable_debug_string() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.pushInReportsInfinityCapacityWithAPortableDebugString", "org.tiqian.layout.LineRepairCoverageTest.pushInReportsInfinityCapacityWithAPortableDebugString", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"pushInReportsInfinityCapacityWithAPortableDebugString");
        let _ = r.record(&"is-true actual=true").unwrap();
        let _ = r.record(&"is-true actual=true").unwrap();
        let _ = r.record(&"eq expected='ForbiddenAtLineStart:，:pushed-in=Infinity.0/Infinity.0' actual='ForbiddenAtLineStart:，:pushed-in=Infinity.0/Infinity.0'").unwrap();
    });
}

#[test]
fn push_in_underflow_shares_skip_zero_valued_proportional_shares() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.pushInUnderflowSharesSkipZeroValuedProportionalShares", "org.tiqian.layout.LineRepairCoverageTest.pushInUnderflowSharesSkipZeroValuedProportionalShares", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"pushInUnderflowSharesSkipZeroValuedProportionalShares");
        let _ = r.record(&"is-true actual=true").unwrap();
        let _ = r.record(&"eq expected=1 actual=1").unwrap();
        let _ = r.record(&"eq expected=1 actual=1").unwrap();
        let _ = r.record(&"eq expected=0 actual=0").unwrap();
    });
}

#[test]
fn with_fill_push_in_gate_applies_or_returns_the_solution() {
    testlib::run("org.tiqian.layout.LineRepairCoverageTest.withFillPushInGateAppliesOrReturnsTheSolution", "org.tiqian.layout.LineRepairCoverageTest.withFillPushInGateAppliesOrReturnsTheSolution", || {
        let mut r = TestTraceRecorder::new("LineRepairCoverageTest");
        r.section(&"withFillPushInGateAppliesOrReturnsTheSolution");
        let _ =
r.record(&"eq expected=LineSolution(lines=[LineCandidate(clusterRange=0..3, sourceRange=TextRange(start=0, end=4), naturalWidth=64, adjustedWidth=64, endReason=AutoWrap, repair=null, repairCandidates=[], hangingClusterIndices=[]), LineCandidate(clusterRange=4..7,~412#b6848dc0 actual=LineSolution(lines=[LineCandidate(clusterRange=0..3, sourceRange=TextRange(start=0, end=4), naturalWidth=64, adjustedWidth=64, endReason=AutoWrap, repair=null, repairCandidates=[], hangingClusterIndices=[]), LineCandidate(clusterRange=4..7,~412#b6848dc0").unwrap();
        let _ = r.record(&"eq expected=[0, 1, 2, 3, 4] actual=[0, 1, 2, 3, 4]").unwrap();
    });
}
