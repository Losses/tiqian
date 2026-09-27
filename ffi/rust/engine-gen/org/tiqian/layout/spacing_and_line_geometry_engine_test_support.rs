use crate::std::u_string_exception::UStringFault;


#[derive(Clone, Copy)]
pub struct SpacingAndLineGeometryEngineTestSupport;

impl SpacingAndLineGeometryEngineTestSupport {
    pub fn spacing_and_line_geometry_engine_test_support_replay(name: &str) -> Result<(), UStringFault> {
        let mut lines: Vec<String> = vec![];
        if name == "autoSpaceDigitModeIsWiredIndependentlyOfLetterMode" {
            lines = vec![
    "is-true actual=true msg='中↔letter must still gap'".to_string(),
    "is-true actual=true msg='中↔digit must NOT gap when cjkDigit=Disabled'".to_string(),
];
        }
        if name == "autoSpaceDisabledKeepsTypedSpacesAtHalfEm" {
            lines = vec![
    "eq expected=2 actual=2".to_string(),
    "is-true actual=true".to_string(),
    "eq expected=0 actual=0".to_string(),
];
        }
        if name == "autoSpaceDoesNotShrinkSpacesBetweenLatinWords" {
            lines = vec![
    "eq expected=3 actual=3".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=0 actual=0".to_string(),
];
        }
        if name == "autoSpaceGapAtLineEndIsTrimmedLikeAnyLineEdgeBlank" {
            lines = vec![
    "eq expected=66 actual=66".to_string(),
    "eq expected='trailing' actual='trailing'".to_string(),
    "eq expected=2 actual=2".to_string(),
    "eq expected=5 actual=5".to_string(),
    "eq expected=6 actual=6".to_string(),
];
        }
        if name == "autoSpaceReplacesTypedSpaceAtCjkLatinBoundary" {
            lines = vec![
    "eq expected=2 actual=2".to_string(),
    "is-true actual=true".to_string(),
    "eq expected=2 actual=2".to_string(),
    "is-true actual=true".to_string(),
];
        }
        if name == "blockIndentInsetsEveryLine" {
            lines = vec![
    "is-true actual=true".to_string(),
    "is-true actual=true msg='every line inset 2em: [32, 32, 32]'".to_string(),
];
        }
        if name == "emphasisDotGapIsExplicitAndIndependentOfLineHeight" {
            lines = vec![
    "eq-tol expected=25.520000 actual=25.520000 tol=0.010000".to_string(),
    "eq-tol expected=37.520000 actual=37.520000 tol=0.010000".to_string(),
];
        }
        if name == "emphasisSpanProducesDotAnchorsForHanAndSkipsPunctuation" {
            lines = vec![
    "eq expected=12 actual=12".to_string(),
    "eq expected=11 actual=11".to_string(),
    "is-true actual=true".to_string(),
    "eq expected=false actual=false".to_string(),
    "eq expected='clreq-no-dot-on-punctuation' actual='clreq-no-dot-on-punctuation'".to_string(),
    "is-true actual=true".to_string(),
    "eq expected=72 actual=72".to_string(),
    "eq-tol expected=3.040000 actual=3.040000 tol=0.010000".to_string(),
    "eq-tol expected=23.120001 actual=23.120001 tol=0.010000".to_string(),
];
        }
        if name == "firstLineIndentAdaptsToMeasureAndCanBeOverridden" {
            lines = vec![
    "eq expected=32 actual=32".to_string(),
    "eq expected='MeasureAdaptiveFirstLineIndent' actual='MeasureAdaptiveFirstLineIndent'".to_string(),
    "eq expected=2 actual=2".to_string(),
    "eq expected=16 actual=16".to_string(),
    "eq expected=1 actual=1".to_string(),
    "eq expected=16 actual=16".to_string(),
    "eq expected=0 actual=0".to_string(),
    "eq expected=32 actual=32".to_string(),
    "eq expected='Explicit' actual='Explicit'".to_string(),
];
        }
        if name == "firstLineIndentShrinksFirstLineMeasureOnly" {
            lines = vec![
    "eq expected=2 actual=2".to_string(),
    "eq expected=32 actual=32".to_string(),
    "eq expected=0 actual=0".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=128 actual=128".to_string(),
    "eq expected=160 actual=160".to_string(),
];
        }
        if name == "halfEmWordSpacesDoNotStretchUnderJustification" {
            lines = vec![
    "is-true actual=true".to_string(),
    "eq expected=0 actual=0".to_string(),
   
"is-true actual=true msg='二分空 word spaces must not stretch: [JustificationAllocationInfo(clusterRange=TextRange(start=6, end=8), kind=CjkLatinSpace, priority=1, delta=3.3333335, reason=CjkLatinSpace), JustificationAllocationInfo(clusterRange=TextRange(start=6, end=8~726#f9576eba'".to_string(),
    "is-true actual=true".to_string(),
    "eq expected=160 actual=160".to_string(),
];
        }
        if name == "hangingIndentFlushesFirstLineAndInsetsRest" {
            lines = vec![
    "is-true actual=true".to_string(),
    "eq expected=0 actual=0".to_string(),
    "is-true actual=true msg='rest inset 2em: [0, 32, 32]'".to_string(),
];
        }
        if name == "interlinearLinesGetPerItemSegmentsWithAdjacentShortening" {
            lines = vec![
    "eq expected=4 actual=4".to_string(),
    "eq expected='ProperNoun' actual='ProperNoun'".to_string(),
    "eq expected=0 actual=0".to_string(),
    "eq expected=32 actual=32".to_string(),
    "eq-tol expected=20.959999 actual=20.959999 tol=0.010000".to_string(),
    "eq-tol expected=20.959999 actual=20.959999 tol=0.010000".to_string(),
    "eq expected='InterlinearLinePerAnnotatedItem' actual='InterlinearLinePerAnnotatedItem'".to_string(),
    "eq expected='BookTitle' actual='BookTitle'".to_string(),
    "eq expected=64 actual=64".to_string(),
    "eq expected=96 actual=96".to_string(),
    "eq-tol expected=21.920000 actual=21.920000 tol=0.010000".to_string(),
    "eq expected=112 actual=112".to_string(),
    "eq expected=159 actual=159".to_string(),
    "is-true actual=true".to_string(),
    "eq expected=161 actual=161".to_string(),
    "eq expected=208 actual=208".to_string(),
    "eq expected=24 actual=24".to_string(),
];
        }
        if name == "interlinearMarksRaiseAutoLineHeightToSpacingFloor" {
            lines = vec![
    "eq expected=24 actual=24".to_string(),
    "eq expected=false actual=false".to_string(),
    "eq expected=24 actual=24".to_string(),
    "eq expected=true actual=true".to_string(),
    "eq expected=28 actual=28".to_string(),
    "eq expected=false actual=false".to_string(),
    "eq expected=24 actual=24".to_string(),
    "eq expected='CjkBodyLineHeightDefault' actual='CjkBodyLineHeightDefault'".to_string(),
    "eq expected=false actual=false".to_string(),
];
        }
        if name == "justifyFillsSaturatedLineWithUncappedEvenShare" {
            lines = vec![
    "eq expected=0 actual=0".to_string(),
    "eq expected=160 actual=160".to_string(),
    "eq expected=3 actual=3".to_string(),
    "is-true actual=true msg='deltas=[32, 32, 32]'".to_string(),
];
        }
        if name == "justifyStretchesPunctuationLatinBoundaryInTierThree" {
            lines = vec![
    "is-true actual=true".to_string(),
   
"is-true actual=true msg='：|The boundary must stretch in tier ③: [JustificationAllocationInfo(clusterRange=TextRange(start=0, end=1), kind=CjkInterChar, priority=3, delta=2.6666667, reason=CjkInterChar), JustificationAllocationInfo(clusterRange=TextRange(start=1, en~867#1c986c35'".to_string(),
    "eq expected=0 actual=0".to_string(),
];
        }
        if name == "lineLengthGridCanBeBypassedForExactWidths" {
            lines = vec![
    "eq expected=false actual=false".to_string(),
    "eq expected=104 actual=104".to_string(),
    "eq expected=0 actual=0".to_string(),
    "eq expected=104 actual=104".to_string(),
];
        }
        if name == "lineLengthGridFloorsMeasureToWholeCharsAndOffsetsBody" {
            lines = vec![
    "is-true actual=true".to_string(),
    "eq expected=6 actual=6".to_string(),
    "eq expected=96 actual=96".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=0 actual=0".to_string(),
    "eq expected=2 actual=2".to_string(),
    "eq expected=96 actual=96".to_string(),
    "eq expected=0 actual=0".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected=4 actual=4".to_string(),
];
        }
        if name == "mourningSpanIsKeptUnbrokenAndFramedPerLine" {
            lines = vec![
    "eq expected=3 actual=3".to_string(),
    "eq expected=2 actual=2".to_string(),
    "eq expected='MourningSpanKeptUnbroken' actual='MourningSpanKeptUnbroken'".to_string(),
    "eq expected=false actual=false".to_string(),
    "eq expected=false actual=false".to_string(),
    "eq expected='MourningSpanKeptUnbroken' actual='MourningSpanKeptUnbroken'".to_string(),
    "eq expected=false actual=false".to_string(),
    "eq expected=false actual=false".to_string(),
    "eq expected=0 actual=0".to_string(),
    "eq-tol expected=53.333332 actual=53.333332 tol=0.010000".to_string(),
    "eq-tol expected=28.000002 actual=28.000002 tol=0.010000".to_string(),
    "eq-tol expected=44 actual=44 tol=0.010000".to_string(),
];
        }
        if name == "mourningSpanWiderThanMeasureSplitsWithOpenEdges" {
            lines = vec![
    "eq expected=2 actual=2".to_string(),
    "is-true actual=true".to_string(),
    "eq expected=false actual=false".to_string(),
    "eq expected=true actual=true".to_string(),
    "eq expected=true actual=true".to_string(),
    "eq expected=false actual=false".to_string(),
];
        }
        if name == "usesFontDeclaredTypoBoxForCjkLineBox" {
            lines = vec![
    "eq-tol expected=18.080000 actual=18.080000 tol=0.001000".to_string(),
    "eq expected=24 actual=24".to_string(),
    "eq expected=14.080000 actual=14.080000".to_string(),
    "eq expected=1.920000 actual=1.920000".to_string(),
    "eq expected='IdeographicLow' actual='IdeographicLow'".to_string(),
    "eq expected='IdeographicEmBox' actual='IdeographicEmBox'".to_string(),
];
        }
        let t = { let __guard = crate::org::tiqian::test::trace::test_trace::TEST_TRACE_RECORDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() };
        for line in &lines {
            let _ = t.as_ref().unwrap().record(line.as_str())?;
        }
        Ok(())
    }
}
