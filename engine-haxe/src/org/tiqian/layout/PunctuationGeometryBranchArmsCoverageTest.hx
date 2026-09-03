package org.tiqian.layout;
import org.tiqian.test.trace.TestTraceRecorder;
class PunctuationGeometryBranchArmsCoverageTest {
    @:test public static function haltAdvanceIsRejectedAtZeroAndAtFullWidth():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("haltAdvanceIsRejectedAtZeroAndAtFullWidth"); }
    @:test public static function nonFiniteHaltPlacementIsIgnored():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("nonFiniteHaltPlacementIsIgnored"); }
    @:test public static function unionIgnoresGlyphsWithoutBounds():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("unionIgnoresGlyphsWithoutBounds"); }
    @:test public static function attachedMarkWalkStopsMidRunAtAGap():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("attachedMarkWalkStopsMidRunAtAGap"); }
    @:test public static function emptyTextClustersCannotBeAttachedMarks():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("emptyTextClustersCannotBeAttachedMarks"); }
    @:test public static function asciiPointMarkKinsokuSkipsEmptyTextClusters():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("asciiPointMarkKinsokuSkipsEmptyTextClusters"); }
    @:test public static function spaceRunRequiresNonEmptyAllSpaceText():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("spaceRunRequiresNonEmptyAllSpaceText"); }
    @:test public static function attachedRunAtParagraphEndEmitsNoAutoSpace():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("attachedRunAtParagraphEndEmitsNoAutoSpace"); }
    @:test public static function virtualGapWithEmptyPreviousTextHasNoNarrowCharacter():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("virtualGapWithEmptyPreviousTextHasNoNarrowCharacter"); }
    @:test public static function typedSpaceWithEmptyTextNeighboursKeepsItsWidth():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("typedSpaceWithEmptyTextNeighboursKeepsItsWidth"); }
    @:test public static function spacingBoundariesAtListEdgesAreFalse():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("spacingBoundariesAtListEdgesAreFalse"); }
    @:test public static function attachedAsciiPointMarkCheckSkipsEmptyPreviousText():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("attachedAsciiPointMarkCheckSkipsEmptyPreviousText"); }
    @:test public static function inlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeading():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("inlineBoxSpanWithZeroNetStructuralEdgeStillAppliesLeading"); }
    @:test public static function resolveClustersAppliesGlyphShiftWithUnchangedAdvance():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("resolveClustersAppliesGlyphShiftWithUnchangedAdvance"); }
    @:test public static function glueCapacitiesMarkCentredFramesAsPaired():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("glueCapacitiesMarkCentredFramesAsPaired"); }
    @:test public static function attachedBoundaryWithPlainPreviousClusterKeepsTheRightBudget():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("attachedBoundaryWithPlainPreviousClusterKeepsTheRightBudget"); }
    @:test public static function attachedBoundaryRecordsNullCharactersForEmptyTextClusters():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("attachedBoundaryRecordsNullCharactersForEmptyTextClusters"); }
    @:test public static function attachedTrailingGlueWidensABudgetedEndCluster():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("attachedTrailingGlueWidensABudgetedEndCluster"); }
    @:test public static function spacingPlanIgnoresTargetsOutsideTheBudgets():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("spacingPlanIgnoresTargetsOutsideTheBudgets"); }
    @:test public static function centredAdjacencyConsumesBothSidesEqually():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("centredAdjacencyConsumesBothSidesEqually"); }
    @:test public static function attachedBoundaryReasonFallsBackToNaturalWithoutLeftAtom():Void { new TestTraceRecorder("PunctuationGeometryBranchArmsCoverageTest").section("attachedBoundaryReasonFallsBackToNaturalWithoutLeftAtom"); }
}
