#![cfg(test)]

use crate::org::tiqian::layout::punctuation_geometry_engine_test_support::PunctuationGeometryEngineTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;


#[test]
fn builds_two_em_punctuation_atom_for_recommended_dash_codepoint() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.buildsTwoEmPunctuationAtomForRecommendedDashCodepoint", "org.tiqian.layout.PunctuationGeometryEngineTest.buildsTwoEmPunctuationAtomForRecommendedDashCodepoint", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[98,117,105,108,100,115,84,119,111,69,109,80,117,110,99,116,117,97,116,105,111,110,65,116,111,109,70,111,114,82,101,99,111,109,109,101,110,100,101,100,68,97,115,104,67,111,100,101,112,111,105,110,116]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[98,117,105,108,100,115,84,119,111,69,109,80,117,110,99,116,117,97,116,105,111,110,65,116,111,109,70,111,114,82,101,99,111,109,109,101,110,100,101,100,68,97,115,104,67,111,100,101,112,111,105,110,116])).unwrap();
    });
}

#[test]
fn ink_bounds_determine_compression_amount_and_sides() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.inkBoundsDetermineCompressionAmountAndSides", "org.tiqian.layout.PunctuationGeometryEngineTest.inkBoundsDetermineCompressionAmountAndSides", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[105,110,107,66,111,117,110,100,115,68,101,116,101,114,109,105,110,101,67,111,109,112,114,101,115,115,105,111,110,65,109,111,117,110,116,65,110,100,83,105,100,101,115]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[105,110,107,66,111,117,110,100,115,68,101,116,101,114,109,105,110,101,67,111,109,112,114,101,115,115,105,111,110,65,109,111,117,110,116,65,110,100,83,105,100,101,115])).unwrap();
    });
}

#[test]
fn records_ink_calibrated_punctuation_geometry_in_layout_debug() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.recordsInkCalibratedPunctuationGeometryInLayoutDebug", "org.tiqian.layout.PunctuationGeometryEngineTest.recordsInkCalibratedPunctuationGeometryInLayoutDebug", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[114,101,99,111,114,100,115,73,110,107,67,97,108,105,98,114,97,116,101,100,80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,73,110,76,97,121,111,117,116,68,101,98,117,103]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[114,101,99,111,114,100,115,73,110,107,67,97,108,105,98,114,97,116,101,100,80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,73,110,76,97,121,111,117,116,68,101,98,117,103])).unwrap();
    });
}

#[test]
fn push_in_keeps_font_centered_punctuation_compression_paired() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.pushInKeepsFontCenteredPunctuationCompressionPaired", "org.tiqian.layout.PunctuationGeometryEngineTest.pushInKeepsFontCenteredPunctuationCompressionPaired", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[112,117,115,104,73,110,75,101,101,112,115,70,111,110,116,67,101,110,116,101,114,101,100,80,117,110,99,116,117,97,116,105,111,110,67,111,109,112,114,101,115,115,105,111,110,80,97,105,114,101,100]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[112,117,115,104,73,110,75,101,101,112,115,70,111,110,116,67,101,110,116,101,114,101,100,80,117,110,99,116,117,97,116,105,111,110,67,111,109,112,114,101,115,115,105,111,110,80,97,105,114,101,100])).unwrap();
    });
}

#[test]
fn records_punctuation_atoms_in_layout_debug() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.recordsPunctuationAtomsInLayoutDebug", "org.tiqian.layout.PunctuationGeometryEngineTest.recordsPunctuationAtomsInLayoutDebug", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[114,101,99,111,114,100,115,80,117,110,99,116,117,97,116,105,111,110,65,116,111,109,115,73,110,76,97,121,111,117,116,68,101,98,117,103]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[114,101,99,111,114,100,115,80,117,110,99,116,117,97,116,105,111,110,65,116,111,109,115,73,110,76,97,121,111,117,116,68,101,98,117,103])).unwrap();
    });
}

#[test]
fn line_start_lenticular_bracket_consumes_opening_glue() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.lineStartLenticularBracketConsumesOpeningGlue", "org.tiqian.layout.PunctuationGeometryEngineTest.lineStartLenticularBracketConsumesOpeningGlue", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[108,105,110,101,83,116,97,114,116,76,101,110,116,105,99,117,108,97,114,66,114,97,99,107,101,116,67,111,110,115,117,109,101,115,79,112,101,110,105,110,103,71,108,117,101]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[108,105,110,101,83,116,97,114,116,76,101,110,116,105,99,117,108,97,114,66,114,97,99,107,101,116,67,111,110,115,117,109,101,115,79,112,101,110,105,110,103,71,108,117,101])).unwrap();
    });
}

#[test]
fn traditional_profile_centres_pause_stop_glue_on_both_sides() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.traditionalProfileCentresPauseStopGlueOnBothSides", "org.tiqian.layout.PunctuationGeometryEngineTest.traditionalProfileCentresPauseStopGlueOnBothSides", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[116,114,97,100,105,116,105,111,110,97,108,80,114,111,102,105,108,101,67,101,110,116,114,101,115,80,97,117,115,101,83,116,111,112,71,108,117,101,79,110,66,111,116,104,83,105,100,101,115]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[116,114,97,100,105,116,105,111,110,97,108,80,114,111,102,105,108,101,67,101,110,116,114,101,115,80,97,117,115,101,83,116,111,112,71,108,117,101,79,110,66,111,116,104,83,105,100,101,115])).unwrap();
    });
}

#[test]
fn applies_adjacent_punctuation_compression_to_drawable_geometry() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.appliesAdjacentPunctuationCompressionToDrawableGeometry", "org.tiqian.layout.PunctuationGeometryEngineTest.appliesAdjacentPunctuationCompressionToDrawableGeometry", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[97,112,112,108,105,101,115,65,100,106,97,99,101,110,116,80,117,110,99,116,117,97,116,105,111,110,67,111,109,112,114,101,115,115,105,111,110,84,111,68,114,97,119,97,98,108,101,71,101,111,109,101,116,114,121]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[97,112,112,108,105,101,115,65,100,106,97,99,101,110,116,80,117,110,99,116,117,97,116,105,111,110,67,111,109,112,114,101,115,115,105,111,110,84,111,68,114,97,119,97,98,108,101,71,101,111,109,101,116,114,121])).unwrap();
    });
}

#[test]
fn compresses_adjacent_cjk_single_quote_comma_sequence() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.compressesAdjacentCjkSingleQuoteCommaSequence", "org.tiqian.layout.PunctuationGeometryEngineTest.compressesAdjacentCjkSingleQuoteCommaSequence", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[99,111,109,112,114,101,115,115,101,115,65,100,106,97,99,101,110,116,67,106,107,83,105,110,103,108,101,81,117,111,116,101,67,111,109,109,97,83,101,113,117,101,110,99,101]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[99,111,109,112,114,101,115,115,101,115,65,100,106,97,99,101,110,116,67,106,107,83,105,110,103,108,101,81,117,111,116,101,67,111,109,109,97,83,101,113,117,101,110,99,101])).unwrap();
    });
}

#[test]
fn compresses_cjk_closing_before_ascii_point_mark_without_reclassifying_ascii() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.compressesCjkClosingBeforeAsciiPointMarkWithoutReclassifyingAscii", "org.tiqian.layout.PunctuationGeometryEngineTest.compressesCjkClosingBeforeAsciiPointMarkWithoutReclassifyingAscii", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[99,111,109,112,114,101,115,115,101,115,67,106,107,67,108,111,115,105,110,103,66,101,102,111,114,101,65,115,99,105,105,80,111,105,110,116,77,97,114,107,87,105,116,104,111,117,116,82,101,99,108,97,115,115,105,102,121,105,110,103,65,115,99,105,105]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[99,111,109,112,114,101,115,115,101,115,67,106,107,67,108,111,115,105,110,103,66,101,102,111,114,101,65,115,99,105,105,80,111,105,110,116,77,97,114,107,87,105,116,104,111,117,116,82,101,99,108,97,115,115,105,102,121,105,110,103,65,115,99,105,105])).unwrap();
    });
}

#[test]
fn halt_advance_from_shaper_drives_punctuation_body_end_to_end() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.haltAdvanceFromShaperDrivesPunctuationBodyEndToEnd", "org.tiqian.layout.PunctuationGeometryEngineTest.haltAdvanceFromShaperDrivesPunctuationBodyEndToEnd", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[104,97,108,116,65,100,118,97,110,99,101,70,114,111,109,83,104,97,112,101,114,68,114,105,118,101,115,80,117,110,99,116,117,97,116,105,111,110,66,111,100,121,69,110,100,84,111,69,110,100]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[104,97,108,116,65,100,118,97,110,99,101,70,114,111,109,83,104,97,112,101,114,68,114,105,118,101,115,80,117,110,99,116,117,97,116,105,111,110,66,111,100,121,69,110,100,84,111,69,110,100])).unwrap();
    });
}

#[test]
fn loose_line_end_style_keeps_full_width_punctuation() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.looseLineEndStyleKeepsFullWidthPunctuation", "org.tiqian.layout.PunctuationGeometryEngineTest.looseLineEndStyleKeepsFullWidthPunctuation", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[108,111,111,115,101,76,105,110,101,69,110,100,83,116,121,108,101,75,101,101,112,115,70,117,108,108,87,105,100,116,104,80,117,110,99,116,117,97,116,105,111,110]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[108,111,111,115,101,76,105,110,101,69,110,100,83,116,121,108,101,75,101,101,112,115,70,117,108,108,87,105,100,116,104,80,117,110,99,116,117,97,116,105,111,110])).unwrap();
    });
}

#[test]
fn inline_stop_compression_knob_limits_push_in_capacity() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.inlineStopCompressionKnobLimitsPushInCapacity", "org.tiqian.layout.PunctuationGeometryEngineTest.inlineStopCompressionKnobLimitsPushInCapacity", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[105,110,108,105,110,101,83,116,111,112,67,111,109,112,114,101,115,115,105,111,110,75,110,111,98,76,105,109,105,116,115,80,117,115,104,73,110,67,97,112,97,99,105,116,121]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[105,110,108,105,110,101,83,116,111,112,67,111,109,112,114,101,115,115,105,111,110,75,110,111,98,76,105,109,105,116,115,80,117,115,104,73,110,67,97,112,97,99,105,116,121])).unwrap();
    });
}

#[test]
fn sino_western_gap_knob_disables_stretch_and_shrink() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.sinoWesternGapKnobDisablesStretchAndShrink", "org.tiqian.layout.PunctuationGeometryEngineTest.sinoWesternGapKnobDisablesStretchAndShrink", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[115,105,110,111,87,101,115,116,101,114,110,71,97,112,75,110,111,98,68,105,115,97,98,108,101,115,83,116,114,101,116,99,104,65,110,100,83,104,114,105,110,107]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[115,105,110,111,87,101,115,116,101,114,110,71,97,112,75,110,111,98,68,105,115,97,98,108,101,115,83,116,114,101,116,99,104,65,110,100,83,104,114,105,110,107])).unwrap();
    });
}

#[test]
fn short_hyphen_connector_is_half_width_wavy_tilde_full_width() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.shortHyphenConnectorIsHalfWidthWavyTildeFullWidth", "org.tiqian.layout.PunctuationGeometryEngineTest.shortHyphenConnectorIsHalfWidthWavyTildeFullWidth", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[115,104,111,114,116,72,121,112,104,101,110,67,111,110,110,101,99,116,111,114,73,115,72,97,108,102,87,105,100,116,104,87,97,118,121,84,105,108,100,101,70,117,108,108,87,105,100,116,104]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[115,104,111,114,116,72,121,112,104,101,110,67,111,110,110,101,99,116,111,114,73,115,72,97,108,102,87,105,100,116,104,87,97,118,121,84,105,108,100,101,70,117,108,108,87,105,100,116,104])).unwrap();
    });
}

#[test]
fn kaiming_style_halves_interior_punctuation_but_not_sentence_end() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.kaimingStyleHalvesInteriorPunctuationButNotSentenceEnd", "org.tiqian.layout.PunctuationGeometryEngineTest.kaimingStyleHalvesInteriorPunctuationButNotSentenceEnd", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[107,97,105,109,105,110,103,83,116,121,108,101,72,97,108,118,101,115,73,110,116,101,114,105,111,114,80,117,110,99,116,117,97,116,105,111,110,66,117,116,78,111,116,83,101,110,116,101,110,99,101,69,110,100]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[107,97,105,109,105,110,103,83,116,121,108,101,72,97,108,118,101,115,73,110,116,101,114,105,111,114,80,117,110,99,116,117,97,116,105,111,110,66,117,116,78,111,116,83,101,110,116,101,110,99,101,69,110,100])).unwrap();
    });
}

#[test]
fn gb_fixed_separators_are_half_width_and_unadjustable() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.gbFixedSeparatorsAreHalfWidthAndUnadjustable", "org.tiqian.layout.PunctuationGeometryEngineTest.gbFixedSeparatorsAreHalfWidthAndUnadjustable", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[103,98,70,105,120,101,100,83,101,112,97,114,97,116,111,114,115,65,114,101,72,97,108,102,87,105,100,116,104,65,110,100,85,110,97,100,106,117,115,116,97,98,108,101]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[103,98,70,105,120,101,100,83,101,112,97,114,97,116,111,114,115,65,114,101,72,97,108,102,87,105,100,116,104,65,110,100,85,110,97,100,106,117,115,116,97,98,108,101])).unwrap();
    });
}

#[test]
fn push_in_drains_bracket_outer_glue_before_inline_comma() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.pushInDrainsBracketOuterGlueBeforeInlineComma", "org.tiqian.layout.PunctuationGeometryEngineTest.pushInDrainsBracketOuterGlueBeforeInlineComma", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[112,117,115,104,73,110,68,114,97,105,110,115,66,114,97,99,107,101,116,79,117,116,101,114,71,108,117,101,66,101,102,111,114,101,73,110,108,105,110,101,67,111,109,109,97]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[112,117,115,104,73,110,68,114,97,105,110,115,66,114,97,99,107,101,116,79,117,116,101,114,71,108,117,101,66,101,102,111,114,101,73,110,108,105,110,101,67,111,109,109,97])).unwrap();
    });
}

#[test]
fn sino_western_gap_shrink_floors_at_eighth_em() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.sinoWesternGapShrinkFloorsAtEighthEm", "org.tiqian.layout.PunctuationGeometryEngineTest.sinoWesternGapShrinkFloorsAtEighthEm", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[115,105,110,111,87,101,115,116,101,114,110,71,97,112,83,104,114,105,110,107,70,108,111,111,114,115,65,116,69,105,103,104,116,104,69,109]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[115,105,110,111,87,101,115,116,101,114,110,71,97,112,83,104,114,105,110,107,70,108,111,111,114,115,65,116,69,105,103,104,116,104,69,109])).unwrap();
    });
}

#[test]
fn push_in_consumes_word_space_before_mid_line_punct_glue() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.pushInConsumesWordSpaceBeforeMidLinePunctGlue", "org.tiqian.layout.PunctuationGeometryEngineTest.pushInConsumesWordSpaceBeforeMidLinePunctGlue", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[112,117,115,104,73,110,67,111,110,115,117,109,101,115,87,111,114,100,83,112,97,99,101,66,101,102,111,114,101,77,105,100,76,105,110,101,80,117,110,99,116,71,108,117,101]));
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(UStr::new(&[112,117,115,104,73,110,67,111,110,115,117,109,101,115,87,111,114,100,83,112,97,99,101,66,101,102,111,114,101,77,105,100,76,105,110,101,80,117,110,99,116,71,108,117,101])).unwrap();
    });
}
