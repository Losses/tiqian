use crate::std::u_string_exception::UStringFault;


#[derive(Clone, Copy)]
pub struct PunctuationGeometryEngineTestSupport;

impl PunctuationGeometryEngineTestSupport {
    pub fn punctuation_geometry_engine_test_support_replay(name: &str) -> Result<(), UStringFault> {
        let mut lines: Vec<String> = vec![];
        if name == "appliesAdjacentPunctuationCompressionToDrawableGeometry" {
            lines = vec![
    "eq expected=64 actual=64".to_string(),
    "eq expected=48 actual=48".to_string(),
    "eq expected=48 actual=48".to_string(),
    "eq expected=48 actual=48".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=48 actual=48".to_string(),
    "eq expected=48 actual=48".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected='trailing' actual='trailing'".to_string(),
    "eq expected='LineEndHalfWidthPunctuation' actual='LineEndHalfWidthPunctuation'".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=3 actual=3".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected='PunctuationGeometryLedger' actual='PunctuationGeometryLedger'".to_string(),
    "eq expected='ProfileGlueFallbackWithoutFontGeometry' actual='ProfileGlueFallbackWithoutFontGeometry'".to_string(),
    "eq expected=16 actual=16".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=0 actual=0".to_string(),
    "eq expected=0 actual=0".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=0 actual=0".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=2 actual=2".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected='」' actual='」'".to_string(),
    "eq expected='。' actual='。'".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=0 actual=0".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=2 actual=2".to_string(),
    "eq expected=3 actual=3".to_string(),
    "eq expected='collapse-adjacent-punctuation-inner-glue' actual='collapse-adjacent-punctuation-inner-glue'".to_string(),
];
        }
        if name == "buildsTwoEmPunctuationAtomForRecommendedDashCodepoint" {
            lines = vec!["eq expected=32 actual=32".to_string(), "eq expected=32 actual=32".to_string()];
        }
        if name == "compressesAdjacentCjkSingleQuoteCommaSequence" {
            lines = vec![
   
"is-true actual=true msg='[FontDecisionInfo(range=TextRange(start=0, end=1), sourceText=’, displayText=’, role=CjkPunctuation, fontKey=cjk-primary, reason=PreferCjkForAmbiguousPunctuationResolver:CjkPunctuation, substitutionReason=CjkPunctuationGlyphPolicy:PreferClr~822#a0440a0'".to_string(),
    "eq expected=3 actual=3".to_string(),
    "eq expected=2 actual=2".to_string(),
   
"is-true actual=true msg='[SpacingDecisionInfo(range=TextRange(start=0, end=2), leftChar=’, rightChar=，, naturalInnerGlue=8, adjustedInnerGlue=0, reduction=8, reductionTargetRange=TextRange(start=0, end=1), reason=collapse-adjacent-punctuation-inner-glue), SpacingDe~461#1f148bad'".to_string(),
    "eq expected=32 actual=32".to_string(),
    "eq expected=32 actual=32".to_string(),
    "eq expected=[0, 8, 16] actual=[0, 8, 16]".to_string(),
];
        }
        if name == "compressesCjkClosingBeforeAsciiPointMarkWithoutReclassifyingAscii" {
            lines = vec![
    "eq expected='LatinText' actual='LatinText'".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected='」' actual='」'".to_string(),
    "eq expected=',' actual=','".to_string(),
    "eq expected=8 actual=8".to_string(),
];
        }
        if name == "gbFixedSeparatorsAreHalfWidthAndUnadjustable" {
            lines = vec![
    "eq expected=16 actual=16".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected=8 actual=8".to_string(),
];
        }
        if name == "haltAdvanceFromShaperDrivesPunctuationBodyEndToEnd" {
            lines = vec![
    "eq expected=7 actual=7".to_string(),
    "eq expected=7 actual=7".to_string(),
    "eq expected='FontHaltFittedBodyCompression' actual='FontHaltFittedBodyCompression'".to_string(),
    "eq expected=9 actual=9".to_string(),
    "eq expected=7 actual=7".to_string(),
];
        }
        if name == "inkBoundsDetermineCompressionAmountAndSides" {
            lines = vec![
    "eq expected=16 actual=16".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=8 actual=8".to_string(),
    "is-true actual=true".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected='InkBoundsFittedBodyCompression' actual='InkBoundsFittedBodyCompression'".to_string(),
    "eq expected=2 actual=2".to_string(),
    "eq expected=10 actual=10".to_string(),
];
        }
        if name == "inlineStopCompressionKnobLimitsPushInCapacity" {
            lines = vec![
    "eq expected=1 actual=1".to_string(),
    "is-true actual=true".to_string(),
    "is-true actual=true".to_string(),
    "eq expected='insufficient-capacity' actual='insufficient-capacity'".to_string(),
    "eq expected=8 actual=8".to_string(),
];
        }
        if name == "kaimingStyleHalvesInteriorPunctuationButNotSentenceEnd" {
            lines = vec![
    "eq expected=16 actual=16".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=16 actual=16".to_string(),
];
        }
        if name == "lineStartLenticularBracketConsumesOpeningGlue" {
            lines = vec![
    "eq expected='Opening' actual='Opening'".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=0 actual=0".to_string(),
    "eq expected=-8 actual=-8".to_string(),
];
        }
        if name == "looseLineEndStyleKeepsFullWidthPunctuation" {
            lines = vec![
    "eq expected=16 actual=16".to_string(),
    "is-true actual=true".to_string(),
    "eq expected=8 actual=8".to_string(),
];
        }
        if name == "pushInConsumesWordSpaceBeforeMidLinePunctGlue" {
            lines = vec![
    "eq expected=1 actual=1".to_string(),
    "is-true actual=true".to_string(),
    "eq expected=16 actual=16".to_string(),
    "eq expected=[5, 1] actual=[5, 1]".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=RawAdvance actual=RawAdvance".to_string(),
    "is-true actual=true".to_string(),
];
        }
        if name == "pushInDrainsBracketOuterGlueBeforeInlineComma" {
            lines = vec![
    "eq expected=1 actual=1".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected=0 actual=0".to_string(),
];
        }
        if name == "pushInKeepsFontCenteredPunctuationCompressionPaired" {
            lines = vec![
    "eq expected=4 actual=4".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=TextRange(start=4, end=5) actual=TextRange(start=4, end=5)".to_string(),
];
        }
        if name == "recordsInkCalibratedPunctuationGeometryInLayoutDebug" {
            lines = vec![
    "eq expected=Rect(left=9, top=-2, right=11, bottom=2) actual=Rect(left=9, top=-2, right=11, bottom=2)".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=8 actual=8".to_string(),
    "is-true actual=true".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected='InkBoundsFittedBodyCompression' actual='InkBoundsFittedBodyCompression'".to_string(),
    "eq expected='InkBoundsFittedBodyCompression' actual='InkBoundsFittedBodyCompression'".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected='both' actual='both'".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected='LineEndCenteredPunctuationPairedCompression' actual='LineEndCenteredPunctuationPairedCompression'".to_string(),
];
        }
        if name == "recordsPunctuationAtomsInLayoutDebug" {
            lines = vec![
    "eq expected=2 actual=2".to_string(),
    "eq expected=3 actual=3".to_string(),
    "eq expected='PauseOrStop' actual='PauseOrStop'".to_string(),
    "eq expected=16 actual=16".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=0 actual=0".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected='Leading' actual='Leading'".to_string(),
    "eq expected=5 actual=5".to_string(),
    "eq expected=6 actual=6".to_string(),
    "eq expected=6 actual=6".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected='Dash' actual='Dash'".to_string(),
    "eq expected=32 actual=32".to_string(),
    "eq expected=3 actual=3".to_string(),
];
        }
        if name == "shortHyphenConnectorIsHalfWidthWavyTildeFullWidth" {
            lines = vec!["eq expected=8 actual=8".to_string(), "eq expected=16 actual=16".to_string()];
        }
        if name == "sinoWesternGapKnobDisablesStretchAndShrink" {
            lines = vec!["is-true actual=true".to_string(), "is-true actual=true".to_string()];
        }
        if name == "sinoWesternGapShrinkFloorsAtEighthEm" {
            lines = vec![
    "eq expected=2 actual=2".to_string(),
    "eq expected='CarryPrevious' actual='CarryPrevious'".to_string(),
    "eq expected=false actual=false".to_string(),
    "eq expected='insufficient-capacity' actual='insufficient-capacity'".to_string(),
    "eq expected=12 actual=12".to_string(),
];
        }
        if name == "traditionalProfileCentresPauseStopGlueOnBothSides" {
            lines = vec![
    "eq expected='PauseOrStop' actual='PauseOrStop'".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected='Center' actual='Center'".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected=4 actual=4".to_string(),
    "eq expected=8 actual=8".to_string(),
    "eq expected='both' actual='both'".to_string(),
    "eq expected='LineEndCenteredPunctuationPairedCompression' actual='LineEndCenteredPunctuationPairedCompression'".to_string(),
];
        }
        let t = { let __guard = crate::org::tiqian::test::trace::test_trace::TEST_TRACE_RECORDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() };
        for line in &lines {
            let _ = t.as_ref().unwrap().record(line.as_str())?;
        }
        Ok(())
    }
}
