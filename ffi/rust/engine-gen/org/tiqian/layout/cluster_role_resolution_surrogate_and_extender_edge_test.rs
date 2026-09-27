#![cfg(test)]

use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::runtime::test as testlib;


#[test]
fn astral_variation_selector_after_an_attached_point_mark_ends_the_run() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.astralVariationSelectorAfterAnAttachedPointMarkEndsTheRun", "org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.astralVariationSelectorAfterAnAttachedPointMarkEndsTheRun", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionSurrogateAndExtenderEdgeTest");
        r.section(&"astralVariationSelectorAfterAnAttachedPointMarkEndsTheRun");
        let _ =
r.record(&"eq expected=3 actual=3 msg='[ResolvedClusterRange(range=TextRange(start=0, end=1), role=CjkText, mandatoryBreak=false, zeroWidthSoftBreak=false, roleOverride=null), ResolvedClusterRange(range=TextRange(start=1, end=4), role=LatinText, mandatoryBreak=false, zeroWidthSo~410#679463cd'").unwrap();
        let _ = r.record(&"eq expected=TextRange(start=1, end=4) actual=TextRange(start=1, end=4)").unwrap();
        let _ = r.record(&"eq expected=LatinText actual=LatinText").unwrap();
    });
}

#[test]
fn astral_variation_selector_between_base_and_modifier_keeps_the_sequence() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.astralVariationSelectorBetweenBaseAndModifierKeepsTheSequence",
"org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.astralVariationSelectorBetweenBaseAndModifierKeepsTheSequence", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionSurrogateAndExtenderEdgeTest");
        r.section(&"astralVariationSelectorBetweenBaseAndModifierKeepsTheSequence");
        let _ = r.record(&"eq expected=1 actual=1 msg='[ResolvedClusterRange(range=TextRange(start=0, end=5), role=Emoji, mandatoryBreak=false, zeroWidthSoftBreak=false, roleOverride=null)]'").unwrap();
        let _ = r.record(&"eq expected=TextRange(start=0, end=5) actual=TextRange(start=0, end=5)").unwrap();
        let _ = r.record(&"eq expected=Emoji actual=Emoji").unwrap();
    });
}

#[test]
fn astral_variation_selector_extends_the_run_before_it() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.astralVariationSelectorExtendsTheRunBeforeIt", "org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.astralVariationSelectorExtendsTheRunBeforeIt", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionSurrogateAndExtenderEdgeTest");
        r.section(&"astralVariationSelectorExtendsTheRunBeforeIt");
        let _ =
r.record(&"eq expected=2 actual=2 msg='[ResolvedClusterRange(range=TextRange(start=0, end=3), role=CjkText, mandatoryBreak=false, zeroWidthSoftBreak=false, roleOverride=null), ResolvedClusterRange(range=TextRange(start=3, end=4), role=CjkText, mandatoryBreak=false, zeroWidthSoft~272#2a105d37'").unwrap();
        let _ = r.record(&"eq expected=TextRange(start=0, end=3) actual=TextRange(start=0, end=3)").unwrap();
        let _ = r.record(&"eq expected=CjkText actual=CjkText").unwrap();
        let _ = r.record(&"eq expected=TextRange(start=3, end=4) actual=TextRange(start=3, end=4)").unwrap();
    });
}

#[test]
fn code_point_above_the_supplementary_selector_range_stands_alone() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.codePointAboveTheSupplementarySelectorRangeStandsAlone", "org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.codePointAboveTheSupplementarySelectorRangeStandsAlone", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionSurrogateAndExtenderEdgeTest");
        r.section(&"codePointAboveTheSupplementarySelectorRangeStandsAlone");
        let _ =
r.record(&"eq expected=3 actual=3 msg='[ResolvedClusterRange(range=TextRange(start=0, end=1), role=CjkText, mandatoryBreak=false, zeroWidthSoftBreak=false, roleOverride=null), ResolvedClusterRange(range=TextRange(start=1, end=3), role=Unknown, mandatoryBreak=false, zeroWidthSoft~408#b9ee8ab9'").unwrap();
        let _ = r.record(&"eq expected=TextRange(start=0, end=1) actual=TextRange(start=0, end=1)").unwrap();
        let _ = r.record(&"eq expected=TextRange(start=1, end=3) actual=TextRange(start=1, end=3)").unwrap();
        let _ = r.record(&"eq expected=TextRange(start=3, end=4) actual=TextRange(start=3, end=4)").unwrap();
    });
}

#[test]
fn high_surrogate_before_plain_bmp_keeps_the_lone_half() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.highSurrogateBeforePlainBmpKeepsTheLoneHalf", "org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.highSurrogateBeforePlainBmpKeepsTheLoneHalf", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionSurrogateAndExtenderEdgeTest");
        r.section(&"highSurrogateBeforePlainBmpKeepsTheLoneHalf");
        let _ =
r.record(&"eq expected=2 actual=2 msg='[ResolvedClusterRange(range=TextRange(start=0, end=1), role=Unknown, mandatoryBreak=false, zeroWidthSoftBreak=false, roleOverride=null), ResolvedClusterRange(range=TextRange(start=1, end=2), role=CjkText, mandatoryBreak=false, zeroWidthSoft~272#c7546518'").unwrap();
        let _ = r.record(&"eq expected=TextRange(start=0, end=1) actual=TextRange(start=0, end=1)").unwrap();
        let _ = r.record(&"eq expected=TextRange(start=1, end=2) actual=TextRange(start=1, end=2)").unwrap();
    });
}

#[test]
fn high_surrogate_before_private_use_keeps_the_lone_half() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.highSurrogateBeforePrivateUseKeepsTheLoneHalf", "org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.highSurrogateBeforePrivateUseKeepsTheLoneHalf", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionSurrogateAndExtenderEdgeTest");
        r.section(&"highSurrogateBeforePrivateUseKeepsTheLoneHalf");
        let _ =
r.record(&"eq expected=3 actual=3 msg='[ResolvedClusterRange(range=TextRange(start=0, end=1), role=Unknown, mandatoryBreak=false, zeroWidthSoftBreak=false, roleOverride=null), ResolvedClusterRange(range=TextRange(start=1, end=2), role=Unknown, mandatoryBreak=false, zeroWidthSoft~408#655aab7f'").unwrap();
        let _ = r.record(&"eq expected=TextRange(start=0, end=1) actual=TextRange(start=0, end=1)").unwrap();
        let _ = r.record(&"eq expected=TextRange(start=1, end=2) actual=TextRange(start=1, end=2)").unwrap();
    });
}

#[test]
fn inline_object_over_the_cr_walks_the_lf_with_a_cr_behind_it() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.inlineObjectOverTheCrWalksTheLfWithACrBehindIt", "org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.inlineObjectOverTheCrWalksTheLfWithACrBehindIt", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionSurrogateAndExtenderEdgeTest");
        r.section(&"inlineObjectOverTheCrWalksTheLfWithACrBehindIt");
        let _ =
r.record(&"eq expected=2 actual=2 msg='[ResolvedClusterRange(range=TextRange(start=0, end=1), role=Unknown, mandatoryBreak=false, zeroWidthSoftBreak=false, roleOverride=null), ResolvedClusterRange(range=TextRange(start=1, end=2), role=Unknown, mandatoryBreak=false, zeroWidthSoft~272#36a39ab5'").unwrap();
        let _ = r.record(&"eq expected=TextRange(start=0, end=1) actual=TextRange(start=0, end=1)").unwrap();
        let _ = r.record(&"eq expected=TextRange(start=1, end=2) actual=TextRange(start=1, end=2)").unwrap();
        let _ =
r.record(&"is-false actual=false msg='[ResolvedClusterRange(range=TextRange(start=0, end=1), role=Unknown, mandatoryBreak=false, zeroWidthSoftBreak=false, roleOverride=null), ResolvedClusterRange(range=TextRange(start=1, end=2), role=Unknown, mandatoryBreak=false, zeroWidthSoft~272#36a39ab5'").unwrap();
        let _ =
r.record(&"is-true actual=true msg='[ResolvedClusterRange(range=TextRange(start=0, end=1), role=Unknown, mandatoryBreak=false, zeroWidthSoftBreak=false, roleOverride=null), ResolvedClusterRange(range=TextRange(start=1, end=2), role=Unknown, mandatoryBreak=false, zeroWidthSoft~272#36a39ab5'").unwrap();
    });
}

#[test]
fn modifier_base_with_a_bmp_selector_walks_the_selector_true_arm() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.modifierBaseWithABmpSelectorWalksTheSelectorTrueArm", "org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.modifierBaseWithABmpSelectorWalksTheSelectorTrueArm", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionSurrogateAndExtenderEdgeTest");
        r.section(&"modifierBaseWithABmpSelectorWalksTheSelectorTrueArm");
        let _ = r.record(&"eq expected=1 actual=1 msg='[ResolvedClusterRange(range=TextRange(start=0, end=4), role=Emoji, mandatoryBreak=false, zeroWidthSoftBreak=false, roleOverride=null)]'").unwrap();
        let _ = r.record(&"eq expected=TextRange(start=0, end=4) actual=TextRange(start=0, end=4)").unwrap();
        let _ = r.record(&"eq expected=Emoji actual=Emoji").unwrap();
    });
}

#[test]
fn modifier_base_with_only_a_selector_ends_the_walk_at_the_cluster_end() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.modifierBaseWithOnlyASelectorEndsTheWalkAtTheClusterEnd", "org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.modifierBaseWithOnlyASelectorEndsTheWalkAtTheClusterEnd", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionSurrogateAndExtenderEdgeTest");
        r.section(&"modifierBaseWithOnlyASelectorEndsTheWalkAtTheClusterEnd");
        let _ = r.record(&"eq expected=1 actual=1 msg='[ResolvedClusterRange(range=TextRange(start=0, end=3), role=Emoji, mandatoryBreak=false, zeroWidthSoftBreak=false, roleOverride=null)]'").unwrap();
        let _ = r.record(&"eq expected=TextRange(start=0, end=3) actual=TextRange(start=0, end=3)").unwrap();
    });
}

#[test]
fn span_boundary_after_a_space_let_the_point_mark_see_its_whitespace_neighbour() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.spanBoundaryAfterASpaceLetThePointMarkSeeItsWhitespaceNeighbour",
"org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.spanBoundaryAfterASpaceLetThePointMarkSeeItsWhitespaceNeighbour", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionSurrogateAndExtenderEdgeTest");
        r.section(&"spanBoundaryAfterASpaceLetThePointMarkSeeItsWhitespaceNeighbour");
        let _ =
r.record(&"eq expected=2 actual=2 msg='[ResolvedClusterRange(range=TextRange(start=0, end=2), role=LatinText, mandatoryBreak=false, zeroWidthSoftBreak=false, roleOverride=null), ResolvedClusterRange(range=TextRange(start=2, end=3), role=LatinText, mandatoryBreak=false, zeroWidth~276#317cf548'").unwrap();
        let _ = r.record(&"eq expected=TextRange(start=0, end=2) actual=TextRange(start=0, end=2)").unwrap();
        let _ = r.record(&"eq expected=TextRange(start=2, end=3) actual=TextRange(start=2, end=3)").unwrap();
    });
}

#[test]
fn zwj_member_inside_a_modifier_base_cluster_breaks_the_walk_below_the_range() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.zwjMemberInsideAModifierBaseClusterBreaksTheWalkBelowTheRange",
"org.tiqian.layout.ClusterRoleResolutionSurrogateAndExtenderEdgeTest.zwjMemberInsideAModifierBaseClusterBreaksTheWalkBelowTheRange", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionSurrogateAndExtenderEdgeTest");
        r.section(&"zwjMemberInsideAModifierBaseClusterBreaksTheWalkBelowTheRange");
        let _ = r.record(&"eq expected=1 actual=1 msg='[ResolvedClusterRange(range=TextRange(start=0, end=4), role=Emoji, mandatoryBreak=false, zeroWidthSoftBreak=false, roleOverride=null)]'").unwrap();
        let _ = r.record(&"eq expected=TextRange(start=0, end=4) actual=TextRange(start=0, end=4)").unwrap();
    });
}
