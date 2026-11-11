#![cfg(test)]

use crate::org::tiqian::layout::spacing_and_line_geometry_engine_test_support::SpacingAndLineGeometryEngineTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;


#[test]
fn auto_space_replaces_typed_space_at_cjk_latin_boundary() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceReplacesTypedSpaceAtCjkLatinBoundary", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceReplacesTypedSpaceAtCjkLatinBoundary", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[97,117,116,111,83,112,97,99,101,82,101,112,108,97,99,101,115,84,121,112,101,100,83,112,97,99,101,65,116,67,106,107,76,97,116,105,110,66,111,117,110,100,97,114,121]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[97,117,116,111,83,112,97,99,101,82,101,112,108,97,99,101,115,84,121,112,101,100,83,112,97,99,101,65,116,67,106,107,76,97,116,105,110,66,111,117,110,100,97,114,121])).unwrap();
    });
}

#[test]
fn auto_space_does_not_shrink_spaces_between_latin_words() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceDoesNotShrinkSpacesBetweenLatinWords", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceDoesNotShrinkSpacesBetweenLatinWords", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[97,117,116,111,83,112,97,99,101,68,111,101,115,78,111,116,83,104,114,105,110,107,83,112,97,99,101,115,66,101,116,119,101,101,110,76,97,116,105,110,87,111,114,100,115]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[97,117,116,111,83,112,97,99,101,68,111,101,115,78,111,116,83,104,114,105,110,107,83,112,97,99,101,115,66,101,116,119,101,101,110,76,97,116,105,110,87,111,114,100,115])).unwrap();
    });
}

#[test]
fn auto_space_disabled_keeps_typed_spaces_at_half_em() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceDisabledKeepsTypedSpacesAtHalfEm", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceDisabledKeepsTypedSpacesAtHalfEm", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[97,117,116,111,83,112,97,99,101,68,105,115,97,98,108,101,100,75,101,101,112,115,84,121,112,101,100,83,112,97,99,101,115,65,116,72,97,108,102,69,109]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[97,117,116,111,83,112,97,99,101,68,105,115,97,98,108,101,100,75,101,101,112,115,84,121,112,101,100,83,112,97,99,101,115,65,116,72,97,108,102,69,109])).unwrap();
    });
}

#[test]
fn uses_font_declared_typo_box_for_cjk_line_box() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.usesFontDeclaredTypoBoxForCjkLineBox", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.usesFontDeclaredTypoBoxForCjkLineBox", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[117,115,101,115,70,111,110,116,68,101,99,108,97,114,101,100,84,121,112,111,66,111,120,70,111,114,67,106,107,76,105,110,101,66,111,120]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[117,115,101,115,70,111,110,116,68,101,99,108,97,114,101,100,84,121,112,111,66,111,120,70,111,114,67,106,107,76,105,110,101,66,111,120])).unwrap();
    });
}

#[test]
fn auto_space_gap_at_line_end_is_trimmed_like_any_line_edge_blank() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceGapAtLineEndIsTrimmedLikeAnyLineEdgeBlank", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceGapAtLineEndIsTrimmedLikeAnyLineEdgeBlank", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[97,117,116,111,83,112,97,99,101,71,97,112,65,116,76,105,110,101,69,110,100,73,115,84,114,105,109,109,101,100,76,105,107,101,65,110,121,76,105,110,101,69,100,103,101,66,108,97,110,107]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[97,117,116,111,83,112,97,99,101,71,97,112,65,116,76,105,110,101,69,110,100,73,115,84,114,105,109,109,101,100,76,105,107,101,65,110,121,76,105,110,101,69,100,103,101,66,108,97,110,107])).unwrap();
    });
}

#[test]
fn emphasis_span_produces_dot_anchors_for_han_and_skips_punctuation() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.emphasisSpanProducesDotAnchorsForHanAndSkipsPunctuation", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.emphasisSpanProducesDotAnchorsForHanAndSkipsPunctuation", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[101,109,112,104,97,115,105,115,83,112,97,110,80,114,111,100,117,99,101,115,68,111,116,65,110,99,104,111,114,115,70,111,114,72,97,110,65,110,100,83,107,105,112,115,80,117,110,99,116,117,97,116,105,111,110]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[101,109,112,104,97,115,105,115,83,112,97,110,80,114,111,100,117,99,101,115,68,111,116,65,110,99,104,111,114,115,70,111,114,72,97,110,65,110,100,83,107,105,112,115,80,117,110,99,116,117,97,116,105,111,110])).unwrap();
    });
}

#[test]
fn emphasis_dot_gap_is_explicit_and_independent_of_line_height() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.emphasisDotGapIsExplicitAndIndependentOfLineHeight", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.emphasisDotGapIsExplicitAndIndependentOfLineHeight", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[101,109,112,104,97,115,105,115,68,111,116,71,97,112,73,115,69,120,112,108,105,99,105,116,65,110,100,73,110,100,101,112,101,110,100,101,110,116,79,102,76,105,110,101,72,101,105,103,104,116]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[101,109,112,104,97,115,105,115,68,111,116,71,97,112,73,115,69,120,112,108,105,99,105,116,65,110,100,73,110,100,101,112,101,110,100,101,110,116,79,102,76,105,110,101,72,101,105,103,104,116])).unwrap();
    });
}

#[test]
fn mourning_span_is_kept_unbroken_and_framed_per_line() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.mourningSpanIsKeptUnbrokenAndFramedPerLine", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.mourningSpanIsKeptUnbrokenAndFramedPerLine", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[109,111,117,114,110,105,110,103,83,112,97,110,73,115,75,101,112,116,85,110,98,114,111,107,101,110,65,110,100,70,114,97,109,101,100,80,101,114,76,105,110,101]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[109,111,117,114,110,105,110,103,83,112,97,110,73,115,75,101,112,116,85,110,98,114,111,107,101,110,65,110,100,70,114,97,109,101,100,80,101,114,76,105,110,101])).unwrap();
    });
}

#[test]
fn mourning_span_wider_than_measure_splits_with_open_edges() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.mourningSpanWiderThanMeasureSplitsWithOpenEdges", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.mourningSpanWiderThanMeasureSplitsWithOpenEdges", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[109,111,117,114,110,105,110,103,83,112,97,110,87,105,100,101,114,84,104,97,110,77,101,97,115,117,114,101,83,112,108,105,116,115,87,105,116,104,79,112,101,110,69,100,103,101,115]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[109,111,117,114,110,105,110,103,83,112,97,110,87,105,100,101,114,84,104,97,110,77,101,97,115,117,114,101,83,112,108,105,116,115,87,105,116,104,79,112,101,110,69,100,103,101,115])).unwrap();
    });
}

#[test]
fn half_em_word_spaces_do_not_stretch_under_justification() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.halfEmWordSpacesDoNotStretchUnderJustification", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.halfEmWordSpacesDoNotStretchUnderJustification", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[104,97,108,102,69,109,87,111,114,100,83,112,97,99,101,115,68,111,78,111,116,83,116,114,101,116,99,104,85,110,100,101,114,74,117,115,116,105,102,105,99,97,116,105,111,110]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[104,97,108,102,69,109,87,111,114,100,83,112,97,99,101,115,68,111,78,111,116,83,116,114,101,116,99,104,85,110,100,101,114,74,117,115,116,105,102,105,99,97,116,105,111,110])).unwrap();
    });
}

#[test]
fn justify_stretches_punctuation_latin_boundary_in_tier_three() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.justifyStretchesPunctuationLatinBoundaryInTierThree", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.justifyStretchesPunctuationLatinBoundaryInTierThree", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[106,117,115,116,105,102,121,83,116,114,101,116,99,104,101,115,80,117,110,99,116,117,97,116,105,111,110,76,97,116,105,110,66,111,117,110,100,97,114,121,73,110,84,105,101,114,84,104,114,101,101]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[106,117,115,116,105,102,121,83,116,114,101,116,99,104,101,115,80,117,110,99,116,117,97,116,105,111,110,76,97,116,105,110,66,111,117,110,100,97,114,121,73,110,84,105,101,114,84,104,114,101,101])).unwrap();
    });
}

#[test]
fn block_indent_insets_every_line() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.blockIndentInsetsEveryLine", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.blockIndentInsetsEveryLine", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[98,108,111,99,107,73,110,100,101,110,116,73,110,115,101,116,115,69,118,101,114,121,76,105,110,101]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[98,108,111,99,107,73,110,100,101,110,116,73,110,115,101,116,115,69,118,101,114,121,76,105,110,101])).unwrap();
    });
}

#[test]
fn hanging_indent_flushes_first_line_and_insets_rest() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.hangingIndentFlushesFirstLineAndInsetsRest", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.hangingIndentFlushesFirstLineAndInsetsRest", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[104,97,110,103,105,110,103,73,110,100,101,110,116,70,108,117,115,104,101,115,70,105,114,115,116,76,105,110,101,65,110,100,73,110,115,101,116,115,82,101,115,116]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[104,97,110,103,105,110,103,73,110,100,101,110,116,70,108,117,115,104,101,115,70,105,114,115,116,76,105,110,101,65,110,100,73,110,115,101,116,115,82,101,115,116])).unwrap();
    });
}

#[test]
fn justify_fills_saturated_line_with_uncapped_even_share() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.justifyFillsSaturatedLineWithUncappedEvenShare", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.justifyFillsSaturatedLineWithUncappedEvenShare", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[106,117,115,116,105,102,121,70,105,108,108,115,83,97,116,117,114,97,116,101,100,76,105,110,101,87,105,116,104,85,110,99,97,112,112,101,100,69,118,101,110,83,104,97,114,101]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[106,117,115,116,105,102,121,70,105,108,108,115,83,97,116,117,114,97,116,101,100,76,105,110,101,87,105,116,104,85,110,99,97,112,112,101,100,69,118,101,110,83,104,97,114,101])).unwrap();
    });
}

#[test]
fn auto_space_digit_mode_is_wired_independently_of_letter_mode() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceDigitModeIsWiredIndependentlyOfLetterMode", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceDigitModeIsWiredIndependentlyOfLetterMode", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[97,117,116,111,83,112,97,99,101,68,105,103,105,116,77,111,100,101,73,115,87,105,114,101,100,73,110,100,101,112,101,110,100,101,110,116,108,121,79,102,76,101,116,116,101,114,77,111,100,101]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[97,117,116,111,83,112,97,99,101,68,105,103,105,116,77,111,100,101,73,115,87,105,114,101,100,73,110,100,101,112,101,110,100,101,110,116,108,121,79,102,76,101,116,116,101,114,77,111,100,101])).unwrap();
    });
}

#[test]
fn line_length_grid_floors_measure_to_whole_chars_and_offsets_body() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.lineLengthGridFloorsMeasureToWholeCharsAndOffsetsBody", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.lineLengthGridFloorsMeasureToWholeCharsAndOffsetsBody", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[108,105,110,101,76,101,110,103,116,104,71,114,105,100,70,108,111,111,114,115,77,101,97,115,117,114,101,84,111,87,104,111,108,101,67,104,97,114,115,65,110,100,79,102,102,115,101,116,115,66,111,100,121]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[108,105,110,101,76,101,110,103,116,104,71,114,105,100,70,108,111,111,114,115,77,101,97,115,117,114,101,84,111,87,104,111,108,101,67,104,97,114,115,65,110,100,79,102,102,115,101,116,115,66,111,100,121])).unwrap();
    });
}

#[test]
fn line_length_grid_can_be_bypassed_for_exact_widths() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.lineLengthGridCanBeBypassedForExactWidths", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.lineLengthGridCanBeBypassedForExactWidths", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[108,105,110,101,76,101,110,103,116,104,71,114,105,100,67,97,110,66,101,66,121,112,97,115,115,101,100,70,111,114,69,120,97,99,116,87,105,100,116,104,115]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[108,105,110,101,76,101,110,103,116,104,71,114,105,100,67,97,110,66,101,66,121,112,97,115,115,101,100,70,111,114,69,120,97,99,116,87,105,100,116,104,115])).unwrap();
    });
}

#[test]
fn interlinear_lines_get_per_item_segments_with_adjacent_shortening() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.interlinearLinesGetPerItemSegmentsWithAdjacentShortening", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.interlinearLinesGetPerItemSegmentsWithAdjacentShortening", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[105,110,116,101,114,108,105,110,101,97,114,76,105,110,101,115,71,101,116,80,101,114,73,116,101,109,83,101,103,109,101,110,116,115,87,105,116,104,65,100,106,97,99,101,110,116,83,104,111,114,116,101,110,105,110,103]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[105,110,116,101,114,108,105,110,101,97,114,76,105,110,101,115,71,101,116,80,101,114,73,116,101,109,83,101,103,109,101,110,116,115,87,105,116,104,65,100,106,97,99,101,110,116,83,104,111,114,116,101,110,105,110,103])).unwrap();
    });
}

#[test]
fn interlinear_marks_raise_auto_line_height_to_spacing_floor() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.interlinearMarksRaiseAutoLineHeightToSpacingFloor", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.interlinearMarksRaiseAutoLineHeightToSpacingFloor", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[105,110,116,101,114,108,105,110,101,97,114,77,97,114,107,115,82,97,105,115,101,65,117,116,111,76,105,110,101,72,101,105,103,104,116,84,111,83,112,97,99,105,110,103,70,108,111,111,114]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[105,110,116,101,114,108,105,110,101,97,114,77,97,114,107,115,82,97,105,115,101,65,117,116,111,76,105,110,101,72,101,105,103,104,116,84,111,83,112,97,99,105,110,103,70,108,111,111,114])).unwrap();
    });
}

#[test]
fn first_line_indent_shrinks_first_line_measure_only() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.firstLineIndentShrinksFirstLineMeasureOnly", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.firstLineIndentShrinksFirstLineMeasureOnly", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[102,105,114,115,116,76,105,110,101,73,110,100,101,110,116,83,104,114,105,110,107,115,70,105,114,115,116,76,105,110,101,77,101,97,115,117,114,101,79,110,108,121]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[102,105,114,115,116,76,105,110,101,73,110,100,101,110,116,83,104,114,105,110,107,115,70,105,114,115,116,76,105,110,101,77,101,97,115,117,114,101,79,110,108,121])).unwrap();
    });
}

#[test]
fn first_line_indent_adapts_to_measure_and_can_be_overridden() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.firstLineIndentAdaptsToMeasureAndCanBeOverridden", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.firstLineIndentAdaptsToMeasureAndCanBeOverridden", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[83,112,97,99,105,110,103,65,110,100,76,105,110,101,71,101,111,109,101,116,114,121,69,110,103,105,110,101,84,101,115,116])));
        t.section(UStr::new(&[102,105,114,115,116,76,105,110,101,73,110,100,101,110,116,65,100,97,112,116,115,84,111,77,101,97,115,117,114,101,65,110,100,67,97,110,66,101,79,118,101,114,114,105,100,100,101,110]));
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(UStr::new(&[102,105,114,115,116,76,105,110,101,73,110,100,101,110,116,65,100,97,112,116,115,84,111,77,101,97,115,117,114,101,65,110,100,67,97,110,66,101,79,118,101,114,114,105,100,100,101,110])).unwrap();
    });
}
