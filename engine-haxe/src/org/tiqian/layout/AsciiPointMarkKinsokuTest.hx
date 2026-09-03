package org.tiqian.layout;
import org.tiqian.core.*;
import org.tiqian.test.trace.*;
class AsciiPointMarkKinsokuTest {
@:test public static function LatinTokensAndAmbiguousAsciiCharactersKeepExistingSegmentation():Void {}
@:test public static function adjacentImpossibleGroupsDoNotShareHangProvenance():Void {}
@:test public static function authoredWhitespaceAndMandatoryBreakDoNotCreateContextualKinsoku():Void {}
@:test public static function cjkAttachedAsciiPointMarksCannotStartWrappedLinesAndStayLatin():Void {}
@:test public static function compressedClosingAndPointMarkPairDoesNotReportAnUnusedHangFallback():Void {}
@:test public static function contextualRunCanExtendAProfileHangOnlyWithinTheSameProtectedGroup():Void {}
@:test public static function firstLineIndentUsesTheSameImpossibleMeasureFallback():Void {}
@:test public static function impossibleMeasureHangsThePointMarkInsteadOfLeavingItAtLineStart():Void {}
@:test public static function kinsokuNoneDisablesClreqButKeepsTheUax14AsciiPointMarkBoundary():Void {}
@:test public static function leadingPointMarkRunIsSplitFromFollowingLatinText():Void {}
@:test public static function lineBreakGeometryIncludesBopomofoSpreadWhenChoosingTheFallback():Void {}
@:test public static function mandatoryBreakControlAfterAHungPointMarkStaysInTheTrailingSuffix():Void {}
@:test public static function pointMarkExposedByASecondStageLatinCutIsSplitFromItsSuffix():Void {}
@:test public static function pointMarkSplitFromAnOverlongLatinTokenStillCannotStartALine():Void {}
@:test public static function reportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiComma():Void {}
@:test public static function styledPointMarkRunCanExtendOneImpossibleMeasureHang():Void {}
}
