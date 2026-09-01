package org.tiqian.layout;
import org.tiqian.test.trace.TestTraceRecorder;
class PunctuationModelCoverageTest {
    private static function mark(name:String):Void { new TestTraceRecorder("PunctuationModelCoverageTest").section(name); }
    @:test public static function glueRejectsInvertedBounds():Void mark("glueRejectsInvertedBounds");
    @:test public static function adjustmentOpportunityCarriesRangeAndGlue():Void mark("adjustmentOpportunityCarriesRangeAndGlue");
    @:test public static function compressionResultSumsAdjustmentReductions():Void mark("compressionResultSumsAdjustmentReductions");
    @:test public static function adjacentPunctuationInnerGlueCollapsesByHalfEm():Void mark("adjacentPunctuationInnerGlueCollapsesByHalfEm");
    @:test public static function adjacentPunctuationTargetsTheWiderSide():Void mark("adjacentPunctuationTargetsTheWiderSide");
    @:test public static function adjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEm():Void mark("adjacentPunctuationSkipsNonAdjacentZeroGlueAndZeroEm");
    @:test public static function cjkClosingBeforeAsciiPointMarkCollapsesTrailingGlue():Void mark("cjkClosingBeforeAsciiPointMarkCollapsesTrailingGlue");
    @:test public static function cjkClosingCompressionRejectsNonMatchingNeighbours():Void mark("cjkClosingCompressionRejectsNonMatchingNeighbours");
    @:test public static function indexedBuildRejectsOutOfRangeIndex():Void mark("indexedBuildRejectsOutOfRangeIndex");
    @:test public static function nonPunctuationCharactersProduceNoAtom():Void mark("nonPunctuationCharactersProduceNoAtom");
    @:test public static function policyFallbackSplitsGlueByClassSide():Void mark("policyFallbackSplitsGlueByClassSide");
    @:test public static function underwidthGlyphsExpandIntoFullWidthCellByClassSide():Void mark("underwidthGlyphsExpandIntoFullWidthCellByClassSide");
    @:test public static function haltFittedCompressionUsesFontMeasurements():Void mark("haltFittedCompressionUsesFontMeasurements");
    @:test public static function haltTrimIsLimitedByInkBoundsAndRecordsWhy():Void mark("haltTrimIsLimitedByInkBoundsAndRecordsWhy");
    @:test public static function haltAdvanceWithoutPlacementFallsBackToFittedInkOrProfile():Void mark("haltAdvanceWithoutPlacementFallsBackToFittedInkOrProfile");
    @:test public static function haltFromProportionalGlyphIsRejected():Void mark("haltFromProportionalGlyphIsRejected");
    @:test public static function inkBoundsFittedFramePicksTheNarrowestContainingAnchor():Void mark("inkBoundsFittedFramePicksTheNarrowestContainingAnchor");
    @:test public static function forcedHalfWidthConnectorsConsumeGlueUpFront():Void mark("forcedHalfWidthConnectorsConsumeGlueUpFront");
    @:test public static function inkInputRecordsWhyBoundsAreMissing():Void mark("inkInputRecordsWhyBoundsAreMissing");
    @:test public static function glueSideForMainlandSimplifiedMapsClassesToSides():Void mark("glueSideForMainlandSimplifiedMapsClassesToSides");
}
