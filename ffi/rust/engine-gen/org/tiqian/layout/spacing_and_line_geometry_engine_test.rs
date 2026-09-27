#![cfg(test)]

use crate::org::tiqian::layout::spacing_and_line_geometry_engine_test_support::SpacingAndLineGeometryEngineTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::runtime::test as testlib;


#[test]
fn auto_space_replaces_typed_space_at_cjk_latin_boundary() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceReplacesTypedSpaceAtCjkLatinBoundary", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceReplacesTypedSpaceAtCjkLatinBoundary", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"autoSpaceReplacesTypedSpaceAtCjkLatinBoundary");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"autoSpaceReplacesTypedSpaceAtCjkLatinBoundary").unwrap();
    });
}

#[test]
fn auto_space_does_not_shrink_spaces_between_latin_words() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceDoesNotShrinkSpacesBetweenLatinWords", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceDoesNotShrinkSpacesBetweenLatinWords", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"autoSpaceDoesNotShrinkSpacesBetweenLatinWords");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"autoSpaceDoesNotShrinkSpacesBetweenLatinWords").unwrap();
    });
}

#[test]
fn auto_space_disabled_keeps_typed_spaces_at_half_em() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceDisabledKeepsTypedSpacesAtHalfEm", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceDisabledKeepsTypedSpacesAtHalfEm", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"autoSpaceDisabledKeepsTypedSpacesAtHalfEm");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"autoSpaceDisabledKeepsTypedSpacesAtHalfEm").unwrap();
    });
}

#[test]
fn uses_font_declared_typo_box_for_cjk_line_box() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.usesFontDeclaredTypoBoxForCjkLineBox", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.usesFontDeclaredTypoBoxForCjkLineBox", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"usesFontDeclaredTypoBoxForCjkLineBox");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"usesFontDeclaredTypoBoxForCjkLineBox").unwrap();
    });
}

#[test]
fn auto_space_gap_at_line_end_is_trimmed_like_any_line_edge_blank() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceGapAtLineEndIsTrimmedLikeAnyLineEdgeBlank", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceGapAtLineEndIsTrimmedLikeAnyLineEdgeBlank", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"autoSpaceGapAtLineEndIsTrimmedLikeAnyLineEdgeBlank");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"autoSpaceGapAtLineEndIsTrimmedLikeAnyLineEdgeBlank").unwrap();
    });
}

#[test]
fn emphasis_span_produces_dot_anchors_for_han_and_skips_punctuation() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.emphasisSpanProducesDotAnchorsForHanAndSkipsPunctuation", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.emphasisSpanProducesDotAnchorsForHanAndSkipsPunctuation", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"emphasisSpanProducesDotAnchorsForHanAndSkipsPunctuation");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"emphasisSpanProducesDotAnchorsForHanAndSkipsPunctuation").unwrap();
    });
}

#[test]
fn emphasis_dot_gap_is_explicit_and_independent_of_line_height() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.emphasisDotGapIsExplicitAndIndependentOfLineHeight", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.emphasisDotGapIsExplicitAndIndependentOfLineHeight", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"emphasisDotGapIsExplicitAndIndependentOfLineHeight");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"emphasisDotGapIsExplicitAndIndependentOfLineHeight").unwrap();
    });
}

#[test]
fn mourning_span_is_kept_unbroken_and_framed_per_line() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.mourningSpanIsKeptUnbrokenAndFramedPerLine", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.mourningSpanIsKeptUnbrokenAndFramedPerLine", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"mourningSpanIsKeptUnbrokenAndFramedPerLine");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"mourningSpanIsKeptUnbrokenAndFramedPerLine").unwrap();
    });
}

#[test]
fn mourning_span_wider_than_measure_splits_with_open_edges() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.mourningSpanWiderThanMeasureSplitsWithOpenEdges", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.mourningSpanWiderThanMeasureSplitsWithOpenEdges", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"mourningSpanWiderThanMeasureSplitsWithOpenEdges");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"mourningSpanWiderThanMeasureSplitsWithOpenEdges").unwrap();
    });
}

#[test]
fn half_em_word_spaces_do_not_stretch_under_justification() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.halfEmWordSpacesDoNotStretchUnderJustification", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.halfEmWordSpacesDoNotStretchUnderJustification", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"halfEmWordSpacesDoNotStretchUnderJustification");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"halfEmWordSpacesDoNotStretchUnderJustification").unwrap();
    });
}

#[test]
fn justify_stretches_punctuation_latin_boundary_in_tier_three() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.justifyStretchesPunctuationLatinBoundaryInTierThree", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.justifyStretchesPunctuationLatinBoundaryInTierThree", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"justifyStretchesPunctuationLatinBoundaryInTierThree");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"justifyStretchesPunctuationLatinBoundaryInTierThree").unwrap();
    });
}

#[test]
fn block_indent_insets_every_line() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.blockIndentInsetsEveryLine", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.blockIndentInsetsEveryLine", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"blockIndentInsetsEveryLine");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"blockIndentInsetsEveryLine").unwrap();
    });
}

#[test]
fn hanging_indent_flushes_first_line_and_insets_rest() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.hangingIndentFlushesFirstLineAndInsetsRest", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.hangingIndentFlushesFirstLineAndInsetsRest", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"hangingIndentFlushesFirstLineAndInsetsRest");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"hangingIndentFlushesFirstLineAndInsetsRest").unwrap();
    });
}

#[test]
fn justify_fills_saturated_line_with_uncapped_even_share() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.justifyFillsSaturatedLineWithUncappedEvenShare", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.justifyFillsSaturatedLineWithUncappedEvenShare", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"justifyFillsSaturatedLineWithUncappedEvenShare");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"justifyFillsSaturatedLineWithUncappedEvenShare").unwrap();
    });
}

#[test]
fn auto_space_digit_mode_is_wired_independently_of_letter_mode() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceDigitModeIsWiredIndependentlyOfLetterMode", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.autoSpaceDigitModeIsWiredIndependentlyOfLetterMode", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"autoSpaceDigitModeIsWiredIndependentlyOfLetterMode");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"autoSpaceDigitModeIsWiredIndependentlyOfLetterMode").unwrap();
    });
}

#[test]
fn line_length_grid_floors_measure_to_whole_chars_and_offsets_body() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.lineLengthGridFloorsMeasureToWholeCharsAndOffsetsBody", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.lineLengthGridFloorsMeasureToWholeCharsAndOffsetsBody", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"lineLengthGridFloorsMeasureToWholeCharsAndOffsetsBody");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"lineLengthGridFloorsMeasureToWholeCharsAndOffsetsBody").unwrap();
    });
}

#[test]
fn line_length_grid_can_be_bypassed_for_exact_widths() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.lineLengthGridCanBeBypassedForExactWidths", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.lineLengthGridCanBeBypassedForExactWidths", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"lineLengthGridCanBeBypassedForExactWidths");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"lineLengthGridCanBeBypassedForExactWidths").unwrap();
    });
}

#[test]
fn interlinear_lines_get_per_item_segments_with_adjacent_shortening() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.interlinearLinesGetPerItemSegmentsWithAdjacentShortening", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.interlinearLinesGetPerItemSegmentsWithAdjacentShortening", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"interlinearLinesGetPerItemSegmentsWithAdjacentShortening");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"interlinearLinesGetPerItemSegmentsWithAdjacentShortening").unwrap();
    });
}

#[test]
fn interlinear_marks_raise_auto_line_height_to_spacing_floor() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.interlinearMarksRaiseAutoLineHeightToSpacingFloor", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.interlinearMarksRaiseAutoLineHeightToSpacingFloor", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"interlinearMarksRaiseAutoLineHeightToSpacingFloor");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"interlinearMarksRaiseAutoLineHeightToSpacingFloor").unwrap();
    });
}

#[test]
fn first_line_indent_shrinks_first_line_measure_only() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.firstLineIndentShrinksFirstLineMeasureOnly", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.firstLineIndentShrinksFirstLineMeasureOnly", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"firstLineIndentShrinksFirstLineMeasureOnly");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"firstLineIndentShrinksFirstLineMeasureOnly").unwrap();
    });
}

#[test]
fn first_line_indent_adapts_to_measure_and_can_be_overridden() {
    testlib::run("org.tiqian.layout.SpacingAndLineGeometryEngineTest.firstLineIndentAdaptsToMeasureAndCanBeOverridden", "org.tiqian.layout.SpacingAndLineGeometryEngineTest.firstLineIndentAdaptsToMeasureAndCanBeOverridden", || {
        let mut t = TestTraceRecorder::new("SpacingAndLineGeometryEngineTest");
        t.section(&"firstLineIndentAdaptsToMeasureAndCanBeOverridden");
        let _ = SpacingAndLineGeometryEngineTestSupport::spacing_and_line_geometry_engine_test_support_replay(&"firstLineIndentAdaptsToMeasureAndCanBeOverridden").unwrap();
    });
}
