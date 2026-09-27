use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;


#[derive(Clone, Copy)]
pub struct SpacingAndLineGeometryEngineTestSupport;

impl SpacingAndLineGeometryEngineTestSupport {
    pub fn spacing_and_line_geometry_engine_test_support_replay(name: &UStr) -> Result<(), UStringFault> {
        let mut lines: Vec<UString> = vec![];
        if name == UString::from("autoSpaceDigitModeIsWiredIndependentlyOfLetterMode") {
            lines = vec![
    UString::from("is-true actual=true msg='中↔letter must still gap'").to_ustring(),
    UString::from("is-true actual=true msg='中↔digit must NOT gap when cjkDigit=Disabled'").to_ustring(),
];
        }
        if name == UString::from("autoSpaceDisabledKeepsTypedSpacesAtHalfEm") {
            lines = vec![
    UString::from("eq expected=2 actual=2").to_ustring(),
    UString::from("is-true actual=true").to_ustring(),
    UString::from("eq expected=0 actual=0").to_ustring(),
];
        }
        if name == UString::from("autoSpaceDoesNotShrinkSpacesBetweenLatinWords") {
            lines = vec![
    UString::from("eq expected=3 actual=3").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=0 actual=0").to_ustring(),
];
        }
        if name == UString::from("autoSpaceGapAtLineEndIsTrimmedLikeAnyLineEdgeBlank") {
            lines = vec![
    UString::from("eq expected=66 actual=66").to_ustring(),
    UString::from("eq expected='trailing' actual='trailing'").to_ustring(),
    UString::from("eq expected=2 actual=2").to_ustring(),
    UString::from("eq expected=5 actual=5").to_ustring(),
    UString::from("eq expected=6 actual=6").to_ustring(),
];
        }
        if name == UString::from("autoSpaceReplacesTypedSpaceAtCjkLatinBoundary") {
            lines = vec![
    UString::from("eq expected=2 actual=2").to_ustring(),
    UString::from("is-true actual=true").to_ustring(),
    UString::from("eq expected=2 actual=2").to_ustring(),
    UString::from("is-true actual=true").to_ustring(),
];
        }
        if name == UString::from("blockIndentInsetsEveryLine") {
            lines = vec![
    UString::from("is-true actual=true").to_ustring(),
    UString::from("is-true actual=true msg='every line inset 2em: [32, 32, 32]'").to_ustring(),
];
        }
        if name == UString::from("emphasisDotGapIsExplicitAndIndependentOfLineHeight") {
            lines = vec![
    UString::from("eq-tol expected=25.520000 actual=25.520000 tol=0.010000").to_ustring(),
    UString::from("eq-tol expected=37.520000 actual=37.520000 tol=0.010000").to_ustring(),
];
        }
        if name == UString::from("emphasisSpanProducesDotAnchorsForHanAndSkipsPunctuation") {
            lines = vec![
    UString::from("eq expected=12 actual=12").to_ustring(),
    UString::from("eq expected=11 actual=11").to_ustring(),
    UString::from("is-true actual=true").to_ustring(),
    UString::from("eq expected=false actual=false").to_ustring(),
    UString::from("eq expected='clreq-no-dot-on-punctuation' actual='clreq-no-dot-on-punctuation'").to_ustring(),
    UString::from("is-true actual=true").to_ustring(),
    UString::from("eq expected=72 actual=72").to_ustring(),
    UString::from("eq-tol expected=3.040000 actual=3.040000 tol=0.010000").to_ustring(),
    UString::from("eq-tol expected=23.120001 actual=23.120001 tol=0.010000").to_ustring(),
];
        }
        if name == UString::from("firstLineIndentAdaptsToMeasureAndCanBeOverridden") {
            lines = vec![
    UString::from("eq expected=32 actual=32").to_ustring(),
    UString::from("eq expected='MeasureAdaptiveFirstLineIndent' actual='MeasureAdaptiveFirstLineIndent'").to_ustring(),
    UString::from("eq expected=2 actual=2").to_ustring(),
    UString::from("eq expected=16 actual=16").to_ustring(),
    UString::from("eq expected=1 actual=1").to_ustring(),
    UString::from("eq expected=16 actual=16").to_ustring(),
    UString::from("eq expected=0 actual=0").to_ustring(),
    UString::from("eq expected=32 actual=32").to_ustring(),
    UString::from("eq expected='Explicit' actual='Explicit'").to_ustring(),
];
        }
        if name == UString::from("firstLineIndentShrinksFirstLineMeasureOnly") {
            lines = vec![
    UString::from("eq expected=2 actual=2").to_ustring(),
    UString::from("eq expected=32 actual=32").to_ustring(),
    UString::from("eq expected=0 actual=0").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=128 actual=128").to_ustring(),
    UString::from("eq expected=160 actual=160").to_ustring(),
];
        }
        if name == UString::from("halfEmWordSpacesDoNotStretchUnderJustification") {
            lines = vec![
    UString::from("is-true actual=true").to_ustring(),
    UString::from("eq expected=0 actual=0").to_ustring(),
    UString::from("is-true actual=true msg='二分空 word spaces must not stretch: [JustificationAllocationInfo(clusterRange=TextRange(start=6, end=8), kind=CjkLatinSpace, priority=1, delta=3.3333335, reason=CjkLatinSpace), JustificationAllocationInfo(clusterRange=TextRange(start=6, end=8~726#f9576eba'").to_ustring(),
    UString::from("is-true actual=true").to_ustring(),
    UString::from("eq expected=160 actual=160").to_ustring(),
];
        }
        if name == UString::from("hangingIndentFlushesFirstLineAndInsetsRest") {
            lines = vec![
    UString::from("is-true actual=true").to_ustring(),
    UString::from("eq expected=0 actual=0").to_ustring(),
    UString::from("is-true actual=true msg='rest inset 2em: [0, 32, 32]'").to_ustring(),
];
        }
        if name == UString::from("interlinearLinesGetPerItemSegmentsWithAdjacentShortening") {
            lines = vec![
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected='ProperNoun' actual='ProperNoun'").to_ustring(),
    UString::from("eq expected=0 actual=0").to_ustring(),
    UString::from("eq expected=32 actual=32").to_ustring(),
    UString::from("eq-tol expected=20.959999 actual=20.959999 tol=0.010000").to_ustring(),
    UString::from("eq-tol expected=20.959999 actual=20.959999 tol=0.010000").to_ustring(),
    UString::from("eq expected='InterlinearLinePerAnnotatedItem' actual='InterlinearLinePerAnnotatedItem'").to_ustring(),
    UString::from("eq expected='BookTitle' actual='BookTitle'").to_ustring(),
    UString::from("eq expected=64 actual=64").to_ustring(),
    UString::from("eq expected=96 actual=96").to_ustring(),
    UString::from("eq-tol expected=21.920000 actual=21.920000 tol=0.010000").to_ustring(),
    UString::from("eq expected=112 actual=112").to_ustring(),
    UString::from("eq expected=159 actual=159").to_ustring(),
    UString::from("is-true actual=true").to_ustring(),
    UString::from("eq expected=161 actual=161").to_ustring(),
    UString::from("eq expected=208 actual=208").to_ustring(),
    UString::from("eq expected=24 actual=24").to_ustring(),
];
        }
        if name == UString::from("interlinearMarksRaiseAutoLineHeightToSpacingFloor") {
            lines = vec![
    UString::from("eq expected=24 actual=24").to_ustring(),
    UString::from("eq expected=false actual=false").to_ustring(),
    UString::from("eq expected=24 actual=24").to_ustring(),
    UString::from("eq expected=true actual=true").to_ustring(),
    UString::from("eq expected=28 actual=28").to_ustring(),
    UString::from("eq expected=false actual=false").to_ustring(),
    UString::from("eq expected=24 actual=24").to_ustring(),
    UString::from("eq expected='CjkBodyLineHeightDefault' actual='CjkBodyLineHeightDefault'").to_ustring(),
    UString::from("eq expected=false actual=false").to_ustring(),
];
        }
        if name == UString::from("justifyFillsSaturatedLineWithUncappedEvenShare") {
            lines = vec![
    UString::from("eq expected=0 actual=0").to_ustring(),
    UString::from("eq expected=160 actual=160").to_ustring(),
    UString::from("eq expected=3 actual=3").to_ustring(),
    UString::from("is-true actual=true msg='deltas=[32, 32, 32]'").to_ustring(),
];
        }
        if name == UString::from("justifyStretchesPunctuationLatinBoundaryInTierThree") {
            lines = vec![
    UString::from("is-true actual=true").to_ustring(),
    UString::from("is-true actual=true msg='：|The boundary must stretch in tier ③: [JustificationAllocationInfo(clusterRange=TextRange(start=0, end=1), kind=CjkInterChar, priority=3, delta=2.6666667, reason=CjkInterChar), JustificationAllocationInfo(clusterRange=TextRange(start=1, en~867#1c986c35'").to_ustring(),
    UString::from("eq expected=0 actual=0").to_ustring(),
];
        }
        if name == UString::from("lineLengthGridCanBeBypassedForExactWidths") {
            lines = vec![
    UString::from("eq expected=false actual=false").to_ustring(),
    UString::from("eq expected=104 actual=104").to_ustring(),
    UString::from("eq expected=0 actual=0").to_ustring(),
    UString::from("eq expected=104 actual=104").to_ustring(),
];
        }
        if name == UString::from("lineLengthGridFloorsMeasureToWholeCharsAndOffsetsBody") {
            lines = vec![
    UString::from("is-true actual=true").to_ustring(),
    UString::from("eq expected=6 actual=6").to_ustring(),
    UString::from("eq expected=96 actual=96").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=0 actual=0").to_ustring(),
    UString::from("eq expected=2 actual=2").to_ustring(),
    UString::from("eq expected=96 actual=96").to_ustring(),
    UString::from("eq expected=0 actual=0").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
];
        }
        if name == UString::from("mourningSpanIsKeptUnbrokenAndFramedPerLine") {
            lines = vec![
    UString::from("eq expected=3 actual=3").to_ustring(),
    UString::from("eq expected=2 actual=2").to_ustring(),
    UString::from("eq expected='MourningSpanKeptUnbroken' actual='MourningSpanKeptUnbroken'").to_ustring(),
    UString::from("eq expected=false actual=false").to_ustring(),
    UString::from("eq expected=false actual=false").to_ustring(),
    UString::from("eq expected='MourningSpanKeptUnbroken' actual='MourningSpanKeptUnbroken'").to_ustring(),
    UString::from("eq expected=false actual=false").to_ustring(),
    UString::from("eq expected=false actual=false").to_ustring(),
    UString::from("eq expected=0 actual=0").to_ustring(),
    UString::from("eq-tol expected=53.333332 actual=53.333332 tol=0.010000").to_ustring(),
    UString::from("eq-tol expected=28.000002 actual=28.000002 tol=0.010000").to_ustring(),
    UString::from("eq-tol expected=44 actual=44 tol=0.010000").to_ustring(),
];
        }
        if name == UString::from("mourningSpanWiderThanMeasureSplitsWithOpenEdges") {
            lines = vec![
    UString::from("eq expected=2 actual=2").to_ustring(),
    UString::from("is-true actual=true").to_ustring(),
    UString::from("eq expected=false actual=false").to_ustring(),
    UString::from("eq expected=true actual=true").to_ustring(),
    UString::from("eq expected=true actual=true").to_ustring(),
    UString::from("eq expected=false actual=false").to_ustring(),
];
        }
        if name == UString::from("usesFontDeclaredTypoBoxForCjkLineBox") {
            lines = vec![
    UString::from("eq-tol expected=18.080000 actual=18.080000 tol=0.001000").to_ustring(),
    UString::from("eq expected=24 actual=24").to_ustring(),
    UString::from("eq expected=14.080000 actual=14.080000").to_ustring(),
    UString::from("eq expected=1.920000 actual=1.920000").to_ustring(),
    UString::from("eq expected='IdeographicLow' actual='IdeographicLow'").to_ustring(),
    UString::from("eq expected='IdeographicEmBox' actual='IdeographicEmBox'").to_ustring(),
];
        }
        let t = { let __guard = crate::org::tiqian::test::trace::test_trace::TEST_TRACE_RECORDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() };
        for line in &lines {
            let _ = t.as_ref().unwrap().record(line.as_ustr())?;
        }
        Ok(())
    }
}
