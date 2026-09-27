#![cfg(test)]

use crate::org::tiqian::clreq::clreq_region::ClreqRegion;
use crate::org::tiqian::clreq::glue_side::GlueSide;
use crate::org::tiqian::clreq::punctuation_class::PunctuationClass;
use crate::org::tiqian::clreq::punctuation_glue_placement::PunctuationGluePlacement;
use crate::org::tiqian::clreq::punctuation_glue_placements::PunctuationGluePlacements;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[test]
fn mainland_anchors_closing_and_pause_stop_to_trailing() {
    testlib::run("org.tiqian.clreq.PunctuationGluePlacementTest.mainlandAnchorsClosingAndPauseStopToTrailing", "org.tiqian.clreq.PunctuationGluePlacementTest.mainlandAnchorsClosingAndPauseStopToTrailing", || {
        TestTraceRecorder::new("PunctuationGluePlacementTest").section(&"mainlandAnchorsClosingAndPauseStopToTrailing");
        let placement = PunctuationGluePlacement::MainlandSimplified;
        let _ = TracedAssertions::traced_assertions_assert_equals_glue_side(GlueSide::TrailingOnly, PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, PunctuationClass::Closing), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_glue_side(GlueSide::TrailingOnly, PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, PunctuationClass::PauseOrStop), None).unwrap();
    });
}

#[test]
fn mainland_anchors_opening_to_leading() {
    testlib::run("org.tiqian.clreq.PunctuationGluePlacementTest.mainlandAnchorsOpeningToLeading", "org.tiqian.clreq.PunctuationGluePlacementTest.mainlandAnchorsOpeningToLeading", || {
        TestTraceRecorder::new("PunctuationGluePlacementTest").section(&"mainlandAnchorsOpeningToLeading");
        let placement = PunctuationGluePlacement::MainlandSimplified;
        let _ = TracedAssertions::traced_assertions_assert_equals_glue_side(GlueSide::LeadingOnly, PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, PunctuationClass::Opening), None).unwrap();
    });
}

#[test]
fn mainland_splits_symmetric_punctuation_on_both_sides() {
    testlib::run("org.tiqian.clreq.PunctuationGluePlacementTest.mainlandSplitsSymmetricPunctuationOnBothSides", "org.tiqian.clreq.PunctuationGluePlacementTest.mainlandSplitsSymmetricPunctuationOnBothSides", || {
        TestTraceRecorder::new("PunctuationGluePlacementTest").section(&"mainlandSplitsSymmetricPunctuationOnBothSides");
        let placement = PunctuationGluePlacement::MainlandSimplified;
        let _ = TracedAssertions::traced_assertions_assert_equals_glue_side(GlueSide::BothSides, PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, PunctuationClass::MiddleDot), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_glue_side(GlueSide::BothSides, PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, PunctuationClass::Ellipsis), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_glue_side(GlueSide::BothSides, PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, PunctuationClass::Dash), None).unwrap();
    });
}

#[test]
fn traditional_centres_closing_and_pause_stop() {
    testlib::run("org.tiqian.clreq.PunctuationGluePlacementTest.traditionalCentresClosingAndPauseStop", "org.tiqian.clreq.PunctuationGluePlacementTest.traditionalCentresClosingAndPauseStop", || {
        TestTraceRecorder::new("PunctuationGluePlacementTest").section(&"traditionalCentresClosingAndPauseStop");
        let placement = PunctuationGluePlacement::Traditional;
        let _ = TracedAssertions::traced_assertions_assert_equals_glue_side(GlueSide::BothSides, PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, PunctuationClass::PauseOrStop), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_glue_side(GlueSide::BothSides, PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, PunctuationClass::Closing), None).unwrap();
    });
}

#[test]
fn traditional_centres_opening() {
    testlib::run("org.tiqian.clreq.PunctuationGluePlacementTest.traditionalCentresOpening", "org.tiqian.clreq.PunctuationGluePlacementTest.traditionalCentresOpening", || {
        TestTraceRecorder::new("PunctuationGluePlacementTest").section(&"traditionalCentresOpening");
        let placement = PunctuationGluePlacement::Traditional;
        let _ = TracedAssertions::traced_assertions_assert_equals_glue_side(GlueSide::BothSides, PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, PunctuationClass::Opening), None).unwrap();
    });
}

#[test]
fn for_region_maps_clreq_regions_to_correct_placement() {
    testlib::run("org.tiqian.clreq.PunctuationGluePlacementTest.forRegionMapsClreqRegionsToCorrectPlacement", "org.tiqian.clreq.PunctuationGluePlacementTest.forRegionMapsClreqRegionsToCorrectPlacement", || {
        TestTraceRecorder::new("PunctuationGluePlacementTest").section(&"forRegionMapsClreqRegionsToCorrectPlacement");
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::MainlandSimplified, PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::Mainland), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::Traditional, PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::Taiwan), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::Traditional, PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::HongKong), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::MainlandSimplified, PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::Custom), None).unwrap();
    });
}

#[test]
fn built_in_taiwan_and_hong_kong_profiles_use_traditional_placement() {
    testlib::run("org.tiqian.clreq.PunctuationGluePlacementTest.builtInTaiwanAndHongKongProfilesUseTraditionalPlacement", "org.tiqian.clreq.PunctuationGluePlacementTest.builtInTaiwanAndHongKongProfilesUseTraditionalPlacement", || {
        TestTraceRecorder::new("PunctuationGluePlacementTest").section(&"builtInTaiwanAndHongKongProfilesUseTraditionalPlacement");
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::Traditional, (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_TAIWAN_HORIZONTAL).clone().glue_placement, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::Traditional, (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_HONG_KONG_HORIZONTAL).clone().glue_placement, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::MainlandSimplified, (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().glue_placement, None).unwrap();
    });
}
