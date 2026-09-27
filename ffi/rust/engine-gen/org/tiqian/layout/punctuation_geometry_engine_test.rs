#![cfg(test)]

use crate::org::tiqian::layout::punctuation_geometry_engine_test_support::PunctuationGeometryEngineTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::runtime::test as testlib;


#[test]
fn builds_two_em_punctuation_atom_for_recommended_dash_codepoint() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.buildsTwoEmPunctuationAtomForRecommendedDashCodepoint", "org.tiqian.layout.PunctuationGeometryEngineTest.buildsTwoEmPunctuationAtomForRecommendedDashCodepoint", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"buildsTwoEmPunctuationAtomForRecommendedDashCodepoint");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"buildsTwoEmPunctuationAtomForRecommendedDashCodepoint").unwrap();
    });
}

#[test]
fn ink_bounds_determine_compression_amount_and_sides() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.inkBoundsDetermineCompressionAmountAndSides", "org.tiqian.layout.PunctuationGeometryEngineTest.inkBoundsDetermineCompressionAmountAndSides", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"inkBoundsDetermineCompressionAmountAndSides");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"inkBoundsDetermineCompressionAmountAndSides").unwrap();
    });
}

#[test]
fn records_ink_calibrated_punctuation_geometry_in_layout_debug() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.recordsInkCalibratedPunctuationGeometryInLayoutDebug", "org.tiqian.layout.PunctuationGeometryEngineTest.recordsInkCalibratedPunctuationGeometryInLayoutDebug", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"recordsInkCalibratedPunctuationGeometryInLayoutDebug");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"recordsInkCalibratedPunctuationGeometryInLayoutDebug").unwrap();
    });
}

#[test]
fn push_in_keeps_font_centered_punctuation_compression_paired() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.pushInKeepsFontCenteredPunctuationCompressionPaired", "org.tiqian.layout.PunctuationGeometryEngineTest.pushInKeepsFontCenteredPunctuationCompressionPaired", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"pushInKeepsFontCenteredPunctuationCompressionPaired");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"pushInKeepsFontCenteredPunctuationCompressionPaired").unwrap();
    });
}

#[test]
fn records_punctuation_atoms_in_layout_debug() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.recordsPunctuationAtomsInLayoutDebug", "org.tiqian.layout.PunctuationGeometryEngineTest.recordsPunctuationAtomsInLayoutDebug", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"recordsPunctuationAtomsInLayoutDebug");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"recordsPunctuationAtomsInLayoutDebug").unwrap();
    });
}

#[test]
fn line_start_lenticular_bracket_consumes_opening_glue() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.lineStartLenticularBracketConsumesOpeningGlue", "org.tiqian.layout.PunctuationGeometryEngineTest.lineStartLenticularBracketConsumesOpeningGlue", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"lineStartLenticularBracketConsumesOpeningGlue");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"lineStartLenticularBracketConsumesOpeningGlue").unwrap();
    });
}

#[test]
fn traditional_profile_centres_pause_stop_glue_on_both_sides() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.traditionalProfileCentresPauseStopGlueOnBothSides", "org.tiqian.layout.PunctuationGeometryEngineTest.traditionalProfileCentresPauseStopGlueOnBothSides", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"traditionalProfileCentresPauseStopGlueOnBothSides");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"traditionalProfileCentresPauseStopGlueOnBothSides").unwrap();
    });
}

#[test]
fn applies_adjacent_punctuation_compression_to_drawable_geometry() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.appliesAdjacentPunctuationCompressionToDrawableGeometry", "org.tiqian.layout.PunctuationGeometryEngineTest.appliesAdjacentPunctuationCompressionToDrawableGeometry", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"appliesAdjacentPunctuationCompressionToDrawableGeometry");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"appliesAdjacentPunctuationCompressionToDrawableGeometry").unwrap();
    });
}

#[test]
fn compresses_adjacent_cjk_single_quote_comma_sequence() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.compressesAdjacentCjkSingleQuoteCommaSequence", "org.tiqian.layout.PunctuationGeometryEngineTest.compressesAdjacentCjkSingleQuoteCommaSequence", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"compressesAdjacentCjkSingleQuoteCommaSequence");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"compressesAdjacentCjkSingleQuoteCommaSequence").unwrap();
    });
}

#[test]
fn compresses_cjk_closing_before_ascii_point_mark_without_reclassifying_ascii() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.compressesCjkClosingBeforeAsciiPointMarkWithoutReclassifyingAscii", "org.tiqian.layout.PunctuationGeometryEngineTest.compressesCjkClosingBeforeAsciiPointMarkWithoutReclassifyingAscii", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"compressesCjkClosingBeforeAsciiPointMarkWithoutReclassifyingAscii");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"compressesCjkClosingBeforeAsciiPointMarkWithoutReclassifyingAscii").unwrap();
    });
}

#[test]
fn halt_advance_from_shaper_drives_punctuation_body_end_to_end() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.haltAdvanceFromShaperDrivesPunctuationBodyEndToEnd", "org.tiqian.layout.PunctuationGeometryEngineTest.haltAdvanceFromShaperDrivesPunctuationBodyEndToEnd", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"haltAdvanceFromShaperDrivesPunctuationBodyEndToEnd");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"haltAdvanceFromShaperDrivesPunctuationBodyEndToEnd").unwrap();
    });
}

#[test]
fn loose_line_end_style_keeps_full_width_punctuation() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.looseLineEndStyleKeepsFullWidthPunctuation", "org.tiqian.layout.PunctuationGeometryEngineTest.looseLineEndStyleKeepsFullWidthPunctuation", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"looseLineEndStyleKeepsFullWidthPunctuation");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"looseLineEndStyleKeepsFullWidthPunctuation").unwrap();
    });
}

#[test]
fn inline_stop_compression_knob_limits_push_in_capacity() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.inlineStopCompressionKnobLimitsPushInCapacity", "org.tiqian.layout.PunctuationGeometryEngineTest.inlineStopCompressionKnobLimitsPushInCapacity", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"inlineStopCompressionKnobLimitsPushInCapacity");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"inlineStopCompressionKnobLimitsPushInCapacity").unwrap();
    });
}

#[test]
fn sino_western_gap_knob_disables_stretch_and_shrink() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.sinoWesternGapKnobDisablesStretchAndShrink", "org.tiqian.layout.PunctuationGeometryEngineTest.sinoWesternGapKnobDisablesStretchAndShrink", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"sinoWesternGapKnobDisablesStretchAndShrink");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"sinoWesternGapKnobDisablesStretchAndShrink").unwrap();
    });
}

#[test]
fn short_hyphen_connector_is_half_width_wavy_tilde_full_width() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.shortHyphenConnectorIsHalfWidthWavyTildeFullWidth", "org.tiqian.layout.PunctuationGeometryEngineTest.shortHyphenConnectorIsHalfWidthWavyTildeFullWidth", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"shortHyphenConnectorIsHalfWidthWavyTildeFullWidth");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"shortHyphenConnectorIsHalfWidthWavyTildeFullWidth").unwrap();
    });
}

#[test]
fn kaiming_style_halves_interior_punctuation_but_not_sentence_end() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.kaimingStyleHalvesInteriorPunctuationButNotSentenceEnd", "org.tiqian.layout.PunctuationGeometryEngineTest.kaimingStyleHalvesInteriorPunctuationButNotSentenceEnd", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"kaimingStyleHalvesInteriorPunctuationButNotSentenceEnd");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"kaimingStyleHalvesInteriorPunctuationButNotSentenceEnd").unwrap();
    });
}

#[test]
fn gb_fixed_separators_are_half_width_and_unadjustable() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.gbFixedSeparatorsAreHalfWidthAndUnadjustable", "org.tiqian.layout.PunctuationGeometryEngineTest.gbFixedSeparatorsAreHalfWidthAndUnadjustable", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"gbFixedSeparatorsAreHalfWidthAndUnadjustable");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"gbFixedSeparatorsAreHalfWidthAndUnadjustable").unwrap();
    });
}

#[test]
fn push_in_drains_bracket_outer_glue_before_inline_comma() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.pushInDrainsBracketOuterGlueBeforeInlineComma", "org.tiqian.layout.PunctuationGeometryEngineTest.pushInDrainsBracketOuterGlueBeforeInlineComma", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"pushInDrainsBracketOuterGlueBeforeInlineComma");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"pushInDrainsBracketOuterGlueBeforeInlineComma").unwrap();
    });
}

#[test]
fn sino_western_gap_shrink_floors_at_eighth_em() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.sinoWesternGapShrinkFloorsAtEighthEm", "org.tiqian.layout.PunctuationGeometryEngineTest.sinoWesternGapShrinkFloorsAtEighthEm", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"sinoWesternGapShrinkFloorsAtEighthEm");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"sinoWesternGapShrinkFloorsAtEighthEm").unwrap();
    });
}

#[test]
fn push_in_consumes_word_space_before_mid_line_punct_glue() {
    testlib::run("org.tiqian.layout.PunctuationGeometryEngineTest.pushInConsumesWordSpaceBeforeMidLinePunctGlue", "org.tiqian.layout.PunctuationGeometryEngineTest.pushInConsumesWordSpaceBeforeMidLinePunctGlue", || {
        let mut t = TestTraceRecorder::new("PunctuationGeometryEngineTest");
        t.section(&"pushInConsumesWordSpaceBeforeMidLinePunctGlue");
        let _ = PunctuationGeometryEngineTestSupport::punctuation_geometry_engine_test_support_replay(&"pushInConsumesWordSpaceBeforeMidLinePunctGlue").unwrap();
    });
}
