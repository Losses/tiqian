#![cfg(test)]

use crate::org::tiqian::clreq::clreq_region::ClreqRegion;
use crate::org::tiqian::clreq::glue_side::GlueSide;
use crate::org::tiqian::clreq::punctuation_class::PunctuationClass;
use crate::org::tiqian::clreq::punctuation_glue_placement::PunctuationGluePlacement;
use crate::org::tiqian::clreq::punctuation_glue_placements::PunctuationGluePlacements;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;


#[test]
fn mainland_anchors_closing_and_pause_stop_to_trailing() {
    testlib::run("org.tiqian.clreq.PunctuationGluePlacementTest.mainlandAnchorsClosingAndPauseStopToTrailing", "org.tiqian.clreq.PunctuationGluePlacementTest.mainlandAnchorsClosingAndPauseStopToTrailing", || {
        TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,108,117,101,80,108,97,99,101,109,101,110,116,84,101,115,116]))).section(UStr::new(&[109,97,105,110,108,97,110,100,65,110,99,104,111,114,115,67,108,111,115,105,110,103,65,110,100,80,97,117,115,101,83,116,111,112,84,111,84,114,97,105,108,105,110,103]));
        let placement = PunctuationGluePlacement::MainlandSimplified;
        let _ = TracedAssertions::traced_assertions_assert_equals_glue_side(GlueSide::TrailingOnly, PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, PunctuationClass::Closing), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_glue_side(GlueSide::TrailingOnly, PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, PunctuationClass::PauseOrStop), None).unwrap();
    });
}

#[test]
fn mainland_anchors_opening_to_leading() {
    testlib::run("org.tiqian.clreq.PunctuationGluePlacementTest.mainlandAnchorsOpeningToLeading", "org.tiqian.clreq.PunctuationGluePlacementTest.mainlandAnchorsOpeningToLeading", || {
        TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,108,117,101,80,108,97,99,101,109,101,110,116,84,101,115,116]))).section(UStr::new(&[109,97,105,110,108,97,110,100,65,110,99,104,111,114,115,79,112,101,110,105,110,103,84,111,76,101,97,100,105,110,103]));
        let placement = PunctuationGluePlacement::MainlandSimplified;
        let _ = TracedAssertions::traced_assertions_assert_equals_glue_side(GlueSide::LeadingOnly, PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, PunctuationClass::Opening), None).unwrap();
    });
}

#[test]
fn mainland_splits_symmetric_punctuation_on_both_sides() {
    testlib::run("org.tiqian.clreq.PunctuationGluePlacementTest.mainlandSplitsSymmetricPunctuationOnBothSides", "org.tiqian.clreq.PunctuationGluePlacementTest.mainlandSplitsSymmetricPunctuationOnBothSides", || {
        TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,108,117,101,80,108,97,99,101,109,101,110,116,84,101,115,116]))).section(UStr::new(&[109,97,105,110,108,97,110,100,83,112,108,105,116,115,83,121,109,109,101,116,114,105,99,80,117,110,99,116,117,97,116,105,111,110,79,110,66,111,116,104,83,105,100,101,115]));
        let placement = PunctuationGluePlacement::MainlandSimplified;
        let _ = TracedAssertions::traced_assertions_assert_equals_glue_side(GlueSide::BothSides, PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, PunctuationClass::MiddleDot), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_glue_side(GlueSide::BothSides, PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, PunctuationClass::Ellipsis), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_glue_side(GlueSide::BothSides, PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, PunctuationClass::Dash), None).unwrap();
    });
}

#[test]
fn traditional_centres_closing_and_pause_stop() {
    testlib::run("org.tiqian.clreq.PunctuationGluePlacementTest.traditionalCentresClosingAndPauseStop", "org.tiqian.clreq.PunctuationGluePlacementTest.traditionalCentresClosingAndPauseStop", || {
        TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,108,117,101,80,108,97,99,101,109,101,110,116,84,101,115,116]))).section(UStr::new(&[116,114,97,100,105,116,105,111,110,97,108,67,101,110,116,114,101,115,67,108,111,115,105,110,103,65,110,100,80,97,117,115,101,83,116,111,112]));
        let placement = PunctuationGluePlacement::Traditional;
        let _ = TracedAssertions::traced_assertions_assert_equals_glue_side(GlueSide::BothSides, PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, PunctuationClass::PauseOrStop), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_glue_side(GlueSide::BothSides, PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, PunctuationClass::Closing), None).unwrap();
    });
}

#[test]
fn traditional_centres_opening() {
    testlib::run("org.tiqian.clreq.PunctuationGluePlacementTest.traditionalCentresOpening", "org.tiqian.clreq.PunctuationGluePlacementTest.traditionalCentresOpening", || {
        TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,108,117,101,80,108,97,99,101,109,101,110,116,84,101,115,116]))).section(UStr::new(&[116,114,97,100,105,116,105,111,110,97,108,67,101,110,116,114,101,115,79,112,101,110,105,110,103]));
        let placement = PunctuationGluePlacement::Traditional;
        let _ = TracedAssertions::traced_assertions_assert_equals_glue_side(GlueSide::BothSides, PunctuationGluePlacements::punctuation_glue_placements_glue_side_for(placement, PunctuationClass::Opening), None).unwrap();
    });
}

#[test]
fn for_region_maps_clreq_regions_to_correct_placement() {
    testlib::run("org.tiqian.clreq.PunctuationGluePlacementTest.forRegionMapsClreqRegionsToCorrectPlacement", "org.tiqian.clreq.PunctuationGluePlacementTest.forRegionMapsClreqRegionsToCorrectPlacement", || {
        TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,108,117,101,80,108,97,99,101,109,101,110,116,84,101,115,116]))).section(UStr::new(&[102,111,114,82,101,103,105,111,110,77,97,112,115,67,108,114,101,113,82,101,103,105,111,110,115,84,111,67,111,114,114,101,99,116,80,108,97,99,101,109,101,110,116]));
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::MainlandSimplified, PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::Mainland), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::Traditional, PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::Taiwan), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::Traditional, PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::HongKong), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::MainlandSimplified, PunctuationGluePlacements::punctuation_glue_placements_for_region(ClreqRegion::Custom), None).unwrap();
    });
}

#[test]
fn built_in_taiwan_and_hong_kong_profiles_use_traditional_placement() {
    testlib::run("org.tiqian.clreq.PunctuationGluePlacementTest.builtInTaiwanAndHongKongProfilesUseTraditionalPlacement", "org.tiqian.clreq.PunctuationGluePlacementTest.builtInTaiwanAndHongKongProfilesUseTraditionalPlacement", || {
        TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,108,117,101,80,108,97,99,101,109,101,110,116,84,101,115,116]))).section(UStr::new(&[98,117,105,108,116,73,110,84,97,105,119,97,110,65,110,100,72,111,110,103,75,111,110,103,80,114,111,102,105,108,101,115,85,115,101,84,114,97,100,105,116,105,111,110,97,108,80,108,97,99,101,109,101,110,116]));
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::Traditional, (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_TAIWAN_HORIZONTAL).clone().glue_placement, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::Traditional, (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_HONG_KONG_HORIZONTAL).clone().glue_placement, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_punctuation_glue_placement(PunctuationGluePlacement::MainlandSimplified, (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone().glue_placement, None).unwrap();
    });
}
