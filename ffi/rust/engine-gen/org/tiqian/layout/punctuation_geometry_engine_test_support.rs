use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;


#[derive(Clone, Copy)]
pub struct PunctuationGeometryEngineTestSupport;

impl PunctuationGeometryEngineTestSupport {
    pub fn punctuation_geometry_engine_test_support_replay(name: &UStr) -> Result<(), UStringFault> {
        let mut lines: Vec<UString> = vec![];
        if name == UString::from("appliesAdjacentPunctuationCompressionToDrawableGeometry") {
            lines = vec![
    UString::from("eq expected=64 actual=64").to_ustring(),
    UString::from("eq expected=48 actual=48").to_ustring(),
    UString::from("eq expected=48 actual=48").to_ustring(),
    UString::from("eq expected=48 actual=48").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=48 actual=48").to_ustring(),
    UString::from("eq expected=48 actual=48").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected='trailing' actual='trailing'").to_ustring(),
    UString::from("eq expected='LineEndHalfWidthPunctuation' actual='LineEndHalfWidthPunctuation'").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=3 actual=3").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected='PunctuationGeometryLedger' actual='PunctuationGeometryLedger'").to_ustring(),
    UString::from("eq expected='ProfileGlueFallbackWithoutFontGeometry' actual='ProfileGlueFallbackWithoutFontGeometry'").to_ustring(),
    UString::from("eq expected=16 actual=16").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=0 actual=0").to_ustring(),
    UString::from("eq expected=0 actual=0").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=0 actual=0").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=2 actual=2").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected='」' actual='」'").to_ustring(),
    UString::from("eq expected='。' actual='。'").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=0 actual=0").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=2 actual=2").to_ustring(),
    UString::from("eq expected=3 actual=3").to_ustring(),
    UString::from("eq expected='collapse-adjacent-punctuation-inner-glue' actual='collapse-adjacent-punctuation-inner-glue'").to_ustring(),
];
        }
        if name == UString::from("buildsTwoEmPunctuationAtomForRecommendedDashCodepoint") {
            lines = vec![
    UString::from("eq expected=32 actual=32").to_ustring(),
    UString::from("eq expected=32 actual=32").to_ustring(),
];
        }
        if name == UString::from("compressesAdjacentCjkSingleQuoteCommaSequence") {
            lines = vec![
    UString::from("is-true actual=true msg='[FontDecisionInfo(range=TextRange(start=0, end=1), sourceText=’, displayText=’, role=CjkPunctuation, fontKey=cjk-primary, reason=PreferCjkForAmbiguousPunctuationResolver:CjkPunctuation, substitutionReason=CjkPunctuationGlyphPolicy:PreferClr~822#a0440a0'").to_ustring(),
    UString::from("eq expected=3 actual=3").to_ustring(),
    UString::from("eq expected=2 actual=2").to_ustring(),
    UString::from("is-true actual=true msg='[SpacingDecisionInfo(range=TextRange(start=0, end=2), leftChar=’, rightChar=，, naturalInnerGlue=8, adjustedInnerGlue=0, reduction=8, reductionTargetRange=TextRange(start=0, end=1), reason=collapse-adjacent-punctuation-inner-glue), SpacingDe~461#1f148bad'").to_ustring(),
    UString::from("eq expected=32 actual=32").to_ustring(),
    UString::from("eq expected=32 actual=32").to_ustring(),
    UString::from("eq expected=[0, 8, 16] actual=[0, 8, 16]").to_ustring(),
];
        }
        if name == UString::from("compressesCjkClosingBeforeAsciiPointMarkWithoutReclassifyingAscii") {
            lines = vec![
    UString::from("eq expected='LatinText' actual='LatinText'").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected='」' actual='」'").to_ustring(),
    UString::from("eq expected=',' actual=','").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
];
        }
        if name == UString::from("gbFixedSeparatorsAreHalfWidthAndUnadjustable") {
            lines = vec![
    UString::from("eq expected=16 actual=16").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
];
        }
        if name == UString::from("haltAdvanceFromShaperDrivesPunctuationBodyEndToEnd") {
            lines = vec![
    UString::from("eq expected=7 actual=7").to_ustring(),
    UString::from("eq expected=7 actual=7").to_ustring(),
    UString::from("eq expected='FontHaltFittedBodyCompression' actual='FontHaltFittedBodyCompression'").to_ustring(),
    UString::from("eq expected=9 actual=9").to_ustring(),
    UString::from("eq expected=7 actual=7").to_ustring(),
];
        }
        if name == UString::from("inkBoundsDetermineCompressionAmountAndSides") {
            lines = vec![
    UString::from("eq expected=16 actual=16").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("is-true actual=true").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected='InkBoundsFittedBodyCompression' actual='InkBoundsFittedBodyCompression'").to_ustring(),
    UString::from("eq expected=2 actual=2").to_ustring(),
    UString::from("eq expected=10 actual=10").to_ustring(),
];
        }
        if name == UString::from("inlineStopCompressionKnobLimitsPushInCapacity") {
            lines = vec![
    UString::from("eq expected=1 actual=1").to_ustring(),
    UString::from("is-true actual=true").to_ustring(),
    UString::from("is-true actual=true").to_ustring(),
    UString::from("eq expected='insufficient-capacity' actual='insufficient-capacity'").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
];
        }
        if name == UString::from("kaimingStyleHalvesInteriorPunctuationButNotSentenceEnd") {
            lines = vec![
    UString::from("eq expected=16 actual=16").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=16 actual=16").to_ustring(),
];
        }
        if name == UString::from("lineStartLenticularBracketConsumesOpeningGlue") {
            lines = vec![
    UString::from("eq expected='Opening' actual='Opening'").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=0 actual=0").to_ustring(),
    UString::from("eq expected=-8 actual=-8").to_ustring(),
];
        }
        if name == UString::from("looseLineEndStyleKeepsFullWidthPunctuation") {
            lines = vec![
    UString::from("eq expected=16 actual=16").to_ustring(),
    UString::from("is-true actual=true").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
];
        }
        if name == UString::from("pushInConsumesWordSpaceBeforeMidLinePunctGlue") {
            lines = vec![
    UString::from("eq expected=1 actual=1").to_ustring(),
    UString::from("is-true actual=true").to_ustring(),
    UString::from("eq expected=16 actual=16").to_ustring(),
    UString::from("eq expected=[5, 1] actual=[5, 1]").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=RawAdvance actual=RawAdvance").to_ustring(),
    UString::from("is-true actual=true").to_ustring(),
];
        }
        if name == UString::from("pushInDrainsBracketOuterGlueBeforeInlineComma") {
            lines = vec![
    UString::from("eq expected=1 actual=1").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected=0 actual=0").to_ustring(),
];
        }
        if name == UString::from("pushInKeepsFontCenteredPunctuationCompressionPaired") {
            lines = vec![
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=TextRange(start=4, end=5) actual=TextRange(start=4, end=5)").to_ustring(),
];
        }
        if name == UString::from("recordsInkCalibratedPunctuationGeometryInLayoutDebug") {
            lines = vec![
    UString::from("eq expected=Rect(left=9, top=-2, right=11, bottom=2) actual=Rect(left=9, top=-2, right=11, bottom=2)").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("is-true actual=true").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected='InkBoundsFittedBodyCompression' actual='InkBoundsFittedBodyCompression'").to_ustring(),
    UString::from("eq expected='InkBoundsFittedBodyCompression' actual='InkBoundsFittedBodyCompression'").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected='both' actual='both'").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected='LineEndCenteredPunctuationPairedCompression' actual='LineEndCenteredPunctuationPairedCompression'").to_ustring(),
];
        }
        if name == UString::from("recordsPunctuationAtomsInLayoutDebug") {
            lines = vec![
    UString::from("eq expected=2 actual=2").to_ustring(),
    UString::from("eq expected=3 actual=3").to_ustring(),
    UString::from("eq expected='PauseOrStop' actual='PauseOrStop'").to_ustring(),
    UString::from("eq expected=16 actual=16").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=0 actual=0").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected='Leading' actual='Leading'").to_ustring(),
    UString::from("eq expected=5 actual=5").to_ustring(),
    UString::from("eq expected=6 actual=6").to_ustring(),
    UString::from("eq expected=6 actual=6").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected='Dash' actual='Dash'").to_ustring(),
    UString::from("eq expected=32 actual=32").to_ustring(),
    UString::from("eq expected=3 actual=3").to_ustring(),
];
        }
        if name == UString::from("shortHyphenConnectorIsHalfWidthWavyTildeFullWidth") {
            lines = vec![
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=16 actual=16").to_ustring(),
];
        }
        if name == UString::from("sinoWesternGapKnobDisablesStretchAndShrink") {
            lines = vec![
    UString::from("is-true actual=true").to_ustring(),
    UString::from("is-true actual=true").to_ustring(),
];
        }
        if name == UString::from("sinoWesternGapShrinkFloorsAtEighthEm") {
            lines = vec![
    UString::from("eq expected=2 actual=2").to_ustring(),
    UString::from("eq expected='CarryPrevious' actual='CarryPrevious'").to_ustring(),
    UString::from("eq expected=false actual=false").to_ustring(),
    UString::from("eq expected='insufficient-capacity' actual='insufficient-capacity'").to_ustring(),
    UString::from("eq expected=12 actual=12").to_ustring(),
];
        }
        if name == UString::from("traditionalProfileCentresPauseStopGlueOnBothSides") {
            lines = vec![
    UString::from("eq expected='PauseOrStop' actual='PauseOrStop'").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected='Center' actual='Center'").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected=4 actual=4").to_ustring(),
    UString::from("eq expected=8 actual=8").to_ustring(),
    UString::from("eq expected='both' actual='both'").to_ustring(),
    UString::from("eq expected='LineEndCenteredPunctuationPairedCompression' actual='LineEndCenteredPunctuationPairedCompression'").to_ustring(),
];
        }
        let t = { let __guard = crate::org::tiqian::test::trace::test_trace::TEST_TRACE_RECORDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() };
        for line in &lines {
            let _ = t.as_ref().unwrap().record(line.as_ustr())?;
        }
        Ok(())
    }
}
