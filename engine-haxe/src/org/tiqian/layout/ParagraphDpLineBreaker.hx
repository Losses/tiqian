package org.tiqian.layout;

import org.tiqian.core.Cluster;
import org.tiqian.core.IntRange;
import org.tiqian.core.IllegalStateException;
import org.tiqian.core.LineEndReason;
import org.tiqian.core.TextRange;
import org.tiqian.layout.KinsokuRule;
import org.tiqian.layout.KinsokuRule.ClreqKinsokuRule;
import org.tiqian.layout.LineBreaker;
import org.tiqian.layout.LineOptimization.LineCandidate;
import org.tiqian.layout.LineOptimization.LineSolution;
import org.tiqian.layout.ProgressiveBreakDecisions.ProgressiveBreakOpportunity;
import org.tiqian.layout.ProgressiveBreakDecisions.ShrinkOpportunity;
import org.tiqian.layout.ProgressiveBreakDecisions.UnbreakableRanges;
import std.SortedMap;
import std.SortedSet;

/** Partial from-zero port. TODO markers deliberately identify members pending r3. */
class ParagraphDpLineBreaker implements LineBreaker {
    private final candidateWindow:Int;
    private final raggednessWeight:Float;
    private final kinsoku:KinsokuRule;
    private final pushInPenalty:Int;
    private final carryPreviousPenalty:Int;
    private final leaveRaggedPenalty:Int;
    private final syntheticHyphenBreakPenalty:Float;
    private final consecutiveSyntheticHyphenPenalty:Float;
    private final consecutiveStretchPenalty:Float;
    private final compressionVisibility:Float;

    public function new(?candidateWindow:Int, ?raggednessWeight:Float, ?kinsoku:KinsokuRule, ?pushInPenalty:Int, ?carryPreviousPenalty:Int,
            ?leaveRaggedPenalty:Int, ?syntheticHyphenBreakPenalty:Float, ?consecutiveSyntheticHyphenPenalty:Float,
            ?consecutiveStretchPenalty:Float, ?compressionVisibility:Float) {
        this.candidateWindow = candidateWindow == null ? 8 : candidateWindow;
        this.raggednessWeight = raggednessWeight == null ? 0.5 : raggednessWeight;
        this.kinsoku = kinsoku == null ? new ClreqKinsokuRule() : kinsoku;
        this.pushInPenalty = pushInPenalty == null ? 2 : pushInPenalty;
        this.carryPreviousPenalty = carryPreviousPenalty == null ? 10 : carryPreviousPenalty;
        this.leaveRaggedPenalty = leaveRaggedPenalty == null ? 20 : leaveRaggedPenalty;
        this.syntheticHyphenBreakPenalty = syntheticHyphenBreakPenalty == null ? 12 : syntheticHyphenBreakPenalty;
        this.consecutiveSyntheticHyphenPenalty = consecutiveSyntheticHyphenPenalty == null ? 12 : consecutiveSyntheticHyphenPenalty;
        this.consecutiveStretchPenalty = consecutiveStretchPenalty == null ? 3 : consecutiveStretchPenalty;
        this.compressionVisibility = compressionVisibility == null ? 1 : compressionVisibility;
    }

    public var strategyName(get, never):String;
    public function get_strategyName():String return "paragraph-dp";

    public function breakLines(naturalClusters:Array<Cluster>, adjustedClusters:Array<Cluster>, maxWidth:Float, ?shrinkOpportunities:Array<ShrinkOpportunity>,
            ?unbreakableRanges:UnbreakableRanges, ?firstLineIndent:Float, ?hangableClusters:SortedSet<Int>, ?extendableHangRanges:Array<IntRange>,
            ?forbiddenLineStartClusters:Null<SortedSet<Int>>, ?forbiddenLineEndClusters:SortedSet<Int>, ?hyphenBreakClusters:SortedSet<Int>,
            ?cjkInterCharBoundaries:SortedSet<Int>, ?maxCjkStretchPerGap:Float, ?sinoWesternBoundaries:SortedSet<Int>, ?sinoWesternStretchCap:Float,
            ?lineAdjustmentPushIn:Bool, ?lineAdjustmentCompressBias:Float, ?hardBreakAfterClusters:SortedSet<Int>, ?nonRenderingControlClusters:SortedSet<Int>,
            ?progressiveBreakOpportunities:SortedMap<Int, ProgressiveBreakOpportunity>):LineSolution {
        throw new IllegalStateException("TODO r3: breakLines");
    }

    private function candidateEnds(context:DpContext, start:Int, segmentEndExclusive:Int, endsWithMandatory:Bool):Array<Int> {
        throw new IllegalStateException("TODO r3: candidateEnds");
    }
    private function edgeGeometry(context:DpContext, line:LineCandidate, isSegmentLast:Bool, hyphenEnd:Bool):EdgeGeometry {
        throw new IllegalStateException("TODO r3: edgeGeometry");
    }
    private function solveSegment(context:DpContext, segmentStart:Int, segmentEndExclusive:Int, endsWithMandatory:Bool):Array<Int> {
        throw new IllegalStateException("TODO r3: solveSegment");
    }
    private function greedyFallbackEnds(context:DpContext, segmentStart:Int, segmentEndExclusive:Int):Array<Int> {
        throw new IllegalStateException("TODO r3: greedyFallbackEnds");
    }
    private function commitSegment(committed:Array<LineCandidate>, ends:Array<Int>, segmentStart:Int, mandatoryEnd:Null<Int>, context:DpContext,
            hardBreakAfterClusters:SortedSet<Int>):Void {
        throw new IllegalStateException("TODO r3: commitSegment");
    }
    private static function rangeHasOnlyNonControlClusters(start:Int, endExclusive:Int, set:SortedSet<Int>):Bool {
        var i = start;
        while (i < endExclusive) { if (!set.has(i)) return true; i++; }
        return false;
    }
    private static final HYPHEN_RUN_STATE_CAP:Int = 3;
    private static final STRETCH_RUN_STATE_CAP:Int = 3;
    private static final VISIBLE_STRETCH_FLOOR_PX:Float = 0.5;
}

class DpContext {
    public final naturalClusters:Array<Cluster>;
    public final adjustedClusters:Array<Cluster>;
    public final maxWidth:Float;
    public final shrinkOpportunities:Array<ShrinkOpportunity>;
    public final unbreakableRanges:UnbreakableRanges;
    public final firstLineIndent:Float;
    public final forbiddenLineStartClusters:Null<SortedSet<Int>>;
    public final forbiddenLineEndClusters:SortedSet<Int>;
    public final hyphenBreakClusters:SortedSet<Int>;
    public final cjkInterCharBoundaries:SortedSet<Int>;
    public final maxCjkStretchPerGap:Float;
    public final sinoWesternBoundaries:SortedSet<Int>;
    public final sinoWesternStretchCap:Float;
    public final nonRenderingControlClusters:SortedSet<Int>;
    public final gapBoundaries:SortedSet<Int>;
    public final dRef:Float;
    public final allowCompressionEdges:Bool;
    public final progressiveBreakOpportunities:SortedMap<Int, ProgressiveBreakOpportunity>;
    public function new(naturalClusters:Array<Cluster>, adjustedClusters:Array<Cluster>, maxWidth:Float, shrinkOpportunities:Array<ShrinkOpportunity>,
            unbreakableRanges:UnbreakableRanges, firstLineIndent:Float, forbiddenLineStartClusters:Null<SortedSet<Int>>, forbiddenLineEndClusters:SortedSet<Int>,
            hyphenBreakClusters:SortedSet<Int>, cjkInterCharBoundaries:SortedSet<Int>, maxCjkStretchPerGap:Float, sinoWesternBoundaries:SortedSet<Int>,
            sinoWesternStretchCap:Float, nonRenderingControlClusters:SortedSet<Int>, gapBoundaries:SortedSet<Int>, dRef:Float, allowCompressionEdges:Bool,
            progressiveBreakOpportunities:SortedMap<Int, ProgressiveBreakOpportunity>) {
        this.naturalClusters=naturalClusters; this.adjustedClusters=adjustedClusters; this.maxWidth=maxWidth; this.shrinkOpportunities=shrinkOpportunities;
        this.unbreakableRanges=unbreakableRanges; this.firstLineIndent=firstLineIndent; this.forbiddenLineStartClusters=forbiddenLineStartClusters;
        this.forbiddenLineEndClusters=forbiddenLineEndClusters; this.hyphenBreakClusters=hyphenBreakClusters; this.cjkInterCharBoundaries=cjkInterCharBoundaries;
        this.maxCjkStretchPerGap=maxCjkStretchPerGap; this.sinoWesternBoundaries=sinoWesternBoundaries; this.sinoWesternStretchCap=sinoWesternStretchCap;
        this.nonRenderingControlClusters=nonRenderingControlClusters; this.gapBoundaries=gapBoundaries; this.dRef=dRef; this.allowCompressionEdges=allowCompressionEdges;
        this.progressiveBreakOpportunities=progressiveBreakOpportunities;
    }
    public function buildLine(clusterRange:IntRange, endReason:LineEndReason):LineCandidate throw new IllegalStateException("TODO r3: DpContext.buildLine");
    public function gapCount(range:IntRange):Int throw new IllegalStateException("TODO r3: DpContext.gapCount");
    public function sinoGapCount(range:IntRange):Int throw new IllegalStateException("TODO r3: DpContext.sinoGapCount");
    public function cjkGapCount(range:IntRange):Int throw new IllegalStateException("TODO r3: DpContext.cjkGapCount");
    public function shrinkCapacity(range:IntRange):Float throw new IllegalStateException("TODO r3: DpContext.shrinkCapacity");
}

class EdgeState {
    public final start:Int; public final end:Int; public final hyphenRun:Int; public final stretchRun:Int; public final cost:Float; public final parent:Null<EdgeState>;
    public function new(start:Int, end:Int, hyphenRun:Int, stretchRun:Int, cost:Float, parent:Null<EdgeState>) { this.start=start; this.end=end; this.hyphenRun=hyphenRun; this.stretchRun=stretchRun; this.cost=cost; this.parent=parent; }
}
class EdgeGeometry {
    public final baseCost:Float; public final visibleStretch:Bool;
    public function new(baseCost:Float, visibleStretch:Bool) { this.baseCost=baseCost; this.visibleStretch=visibleStretch; }
}
