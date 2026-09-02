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
        if (adjustedClusters.length == 0) return new LineSolution([]);
        if (naturalClusters.length != adjustedClusters.length) throw new IllegalStateException("naturalClusters and adjustedClusters must align cluster-for-cluster.");
        final shrink = shrinkOpportunities == null ? [] : shrinkOpportunities;
        final ranges = unbreakableRanges == null ? new UnbreakableRanges([]) : unbreakableRanges;
        final indent = firstLineIndent == null ? 0.0 : firstLineIndent;
        final forbidStart = forbiddenLineStartClusters;
        final forbidEnd = forbiddenLineEndClusters == null ? SortedSet.builder().build() : forbiddenLineEndClusters;
        final hyphens = hyphenBreakClusters == null ? SortedSet.builder().build() : hyphenBreakClusters;
        final cjk = cjkInterCharBoundaries == null ? SortedSet.builder().build() : cjkInterCharBoundaries;
        final sino = sinoWesternBoundaries == null ? SortedSet.builder().build() : sinoWesternBoundaries;
        final controls = nonRenderingControlClusters == null ? SortedSet.builder().build() : nonRenderingControlClusters;
        final progressive = progressiveBreakOpportunities == null ? SortedMap.builder().build() : progressiveBreakOpportunities;
        final hard = hardBreakAfterClusters == null ? SortedSet.builder().build() : hardBreakAfterClusters;
        final hangables = hangableClusters == null ? SortedSet.builder().build() : hangableClusters;
        final maxStretch = maxCjkStretchPerGap == null ? Math.POSITIVE_INFINITY : maxCjkStretchPerGap;
        final sinoCap = sinoWesternStretchCap == null ? 0.0 : sinoWesternStretchCap;
        final context = new DpContext(naturalClusters, adjustedClusters, maxWidth, shrink, ranges, indent, forbiddenLineStartClusters, forbidEnd, hyphens, cjk,
            maxStretch, sino, sinoCap, controls, cjk, maxStretch,
            lineAdjustmentPushIn == true, progressive);
        final committed:Array<LineCandidate> = [];
        final sortedBreaks:Array<Int> = [];
        if (hard.size() > 0) for (i in 0...hard.size()) sortedBreaks.push(hard.at(i));
        var cursor = 0; var segmentStart = 0;
        while (segmentStart < adjustedClusters.length) {
            while (cursor < sortedBreaks.length && sortedBreaks[cursor] < segmentStart) cursor++;
            final mandatory = cursor < sortedBreaks.length ? sortedBreaks[cursor] : null;
            final end = mandatory == null ? adjustedClusters.length : mandatory + 1;
            final ends = solveSegment(context, segmentStart, end, mandatory != null);
            commitSegment(committed, ends, segmentStart, mandatory, context, hard);
            segmentStart = end;
        }
        return LineRepair.applyKinsokuRepairs(committed, naturalClusters, adjustedClusters, maxWidth, kinsoku, shrink, pushInPenalty,
            carryPreviousPenalty, leaveRaggedPenalty, ranges, indent, hangables,
            extendableHangRanges == null ? [] : extendableHangRanges, 5, forbidStart);
    }

    private function candidateEnds(context:DpContext, start:Int, segmentEndExclusive:Int, endsWithMandatory:Bool):Array<Int> {
        final limit = ProgressiveBreakDecisions.lineLimit(context.maxWidth, context.firstLineIndent, start);
        final raw = findGreedyEnd(context.adjustedClusters, start, limit, segmentEndExclusive, context.nonRenderingControlClusters);
        if (raw >= segmentEndExclusive) return [segmentEndExclusive];
        final progressive = ProgressiveBreakDecisions.decideProgressiveBreak(start, raw, context.progressiveBreakOpportunities, context.adjustedClusters, limit,
            context.cjkInterCharBoundaries, context.maxCjkStretchPerGap, context.sinoWesternBoundaries, context.sinoWesternStretchCap);
        final baseline = ProgressiveBreakDecisions.adjustBreakForUnbreakables(ProgressiveBreakDecisions.decideHyphenBreak(start, progressive, context.adjustedClusters, limit,
            context.hyphenBreakClusters, context.cjkInterCharBoundaries, context.maxCjkStretchPerGap, context.sinoWesternBoundaries, context.sinoWesternStretchCap), start, context.unbreakableRanges);
        final pool:Array<Int> = [];
        var i = start + 1; while (i <= raw) { if (!context.unbreakableRanges.containsBoundary(i) && (!endsWithMandatory || i != segmentEndExclusive - 1) &&
            ProgressiveBreakDecisions.progressiveCandidateAllowed(start, raw, i, context.progressiveBreakOpportunities, context.adjustedClusters, limit, context.cjkInterCharBoundaries,
                context.maxCjkStretchPerGap, context.sinoWesternBoundaries, context.sinoWesternStretchCap) && rangeHasOnlyNonControlClusters(start, i, context.nonRenderingControlClusters)) pool.push(i); i++; }
        if (context.allowCompressionEdges) {
            var width = 0.0; i = start; while (i < raw) { width += context.adjustedClusters[i].advance; i++; }
            var e = raw + 1;
            while (e <= segmentEndExclusive && e - raw <= candidateWindow) {
                width += context.adjustedClusters[e - 1].advance;
                if (width - limit > context.shrinkCapacity(new IntRange(start, e - 1))) break;
                if (!context.unbreakableRanges.containsBoundary(e) && (!endsWithMandatory || e != segmentEndExclusive - 1)) pool.push(e);
                e++;
            }
        }
        final clean:Array<Int> = []; for (e in pool) if (e == segmentEndExclusive || (context.forbiddenLineStartClusters == null || !context.forbiddenLineStartClusters.has(e)) && !context.forbiddenLineEndClusters.has(e - 1)) clean.push(e);
        final result = clean.length > 0 ? clean : pool;
        if (baseline >= start + 1 && baseline <= segmentEndExclusive) result.push(baseline);
        final unique:Array<Int> = []; for (e in result) { var seen = false; for (v in unique) if (v == e) seen = true; if (!seen) unique.push(e); }
        return unique.length > 0 ? unique : [baseline < start + 1 ? start + 1 : baseline];
    }

    private function ParagraphDpLineBreakerCandidateWindow(context:DpContext, start:Int):Int return 8;
    private function edgeGeometry(context:DpContext, line:LineCandidate, isSegmentLast:Bool, hyphenEnd:Bool):EdgeGeometry {
        final limit = ProgressiveBreakDecisions.lineLimit(context.maxWidth, context.firstLineIndent, line.clusterRange.start);
        final inMeasure = line.clusterRange;
        final overflow = line.adjustedWidth - limit;
        final orphan = !isSegmentLast && inMeasure.start == inMeasure.end ? leaveRaggedPenalty : 0.0;
        final hyphen = hyphenEnd ? syntheticHyphenBreakPenalty : 0.0;
        final ref = context.dRef < 1.0 ? 1.0 : context.dRef;
        if (overflow > 0.0) {
            final gaps = context.gapCount(inMeasure) < 1 ? 1 : context.gapCount(inMeasure);
            final d = overflow / gaps * compressionVisibility;
            return new EdgeGeometry(orphan + hyphen + d * d / ref * raggednessWeight, false);
        }
        final deficit = isSegmentLast ? 0.0 : Math.max(limit - line.adjustedWidth, 0.0);
        final sinoGaps = context.sinoGapCount(inMeasure);
        final cjkGaps = context.cjkGapCount(inMeasure);
        final sinoFill = sinoGaps > 0 ? Math.min(deficit, sinoGaps * context.sinoWesternStretchCap) : 0.0;
        final dSino = sinoGaps > 0 ? sinoFill / sinoGaps : 0.0;
        final cjkDeficit = deficit - sinoFill;
        final dCjk = cjkGaps > 0 ? cjkDeficit / cjkGaps : 0.0;
        final residual = cjkGaps == 0 ? cjkDeficit : 0.0;
        return new EdgeGeometry(residual * raggednessWeight + orphan + hyphen + (dSino * dSino + dCjk * dCjk) / ref * raggednessWeight,
            Math.max(dSino, dCjk) > VISIBLE_STRETCH_FLOOR_PX);
    }
    private function solveSegment(context:DpContext, segmentStart:Int, segmentEndExclusive:Int, endsWithMandatory:Bool):Array<Int> { throw new IllegalStateException("TODO r5: solveSegment"); }
    private function greedyFallbackEnds(context:DpContext, segmentStart:Int, segmentEndExclusive:Int):Array<Int> {
        final ends:Array<Int> = []; var start = segmentStart;
        while (start < segmentEndExclusive) {
            final limit = ProgressiveBreakDecisions.lineLimit(context.maxWidth, context.firstLineIndent, start);
            final raw = findGreedyEnd(context.adjustedClusters, start, limit, segmentEndExclusive, context.nonRenderingControlClusters);
            var e = raw >= segmentEndExclusive ? segmentEndExclusive : ProgressiveBreakDecisions.adjustBreakForUnbreakables(
                ProgressiveBreakDecisions.decideHyphenBreak(start, raw, context.adjustedClusters, limit, context.hyphenBreakClusters,
                    context.cjkInterCharBoundaries, context.maxCjkStretchPerGap, context.sinoWesternBoundaries, context.sinoWesternStretchCap), start, context.unbreakableRanges);
            if (e <= start) e = start + 1;
            ends.push(e); start = e;
        }
        return ends;
    }
    private function commitSegment(committed:Array<LineCandidate>, ends:Array<Int>, segmentStart:Int, mandatoryEnd:Null<Int>, context:DpContext,
            hardBreakAfterClusters:SortedSet<Int>):Void { throw new IllegalStateException("TODO r5: commitSegment"); }
    private static function findGreedyEnd(clusters:Array<Cluster>, start:Int, limit:Float, endExclusive:Int, controls:SortedSet<Int>):Int {
        var width = 0.0; var i = start; var hasContent = false;
        while (i < endExclusive) { final next = width + clusters[i].advance; if (next > limit && hasContent) return i; width = next; if (!controls.has(i)) hasContent = true; i++; }
        return endExclusive;
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
    private final gapPrefix:Array<Int>;
    private final sinoPrefix:Array<Int>;
    private final cjkPrefix:Array<Int>;
    private final naturalPrefix:Array<Float>;
    private final adjustedPrefix:Array<Float>;
    private final shrinkPrefix:Array<Float>;
    private final lineEndOnlyCapacity:Array<Float>;
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
        final n = adjustedClusters.length;
        gapPrefix = [];
        sinoPrefix = [];
        cjkPrefix = [];
        naturalPrefix = [];
        adjustedPrefix = [];
        var init = 0;
        while (init <= n) {
            gapPrefix.push(0); sinoPrefix.push(0); cjkPrefix.push(0); adjustedPrefix.push(0.0);
            init++;
        }
        init = 0;
        while (init <= naturalClusters.length) { naturalPrefix.push(0.0); init++; }
        var k = 0;
        while (k < n) {
            gapPrefix[k + 1] = gapPrefix[k] + (gapBoundaries.has(k) ? 1 : 0);
            sinoPrefix[k + 1] = sinoPrefix[k] + (sinoWesternBoundaries.has(k) ? 1 : 0);
            cjkPrefix[k + 1] = cjkPrefix[k] + (cjkInterCharBoundaries.has(k) ? 1 : 0);
            naturalPrefix[k + 1] = naturalPrefix[k] + naturalClusters[k].advance;
            adjustedPrefix[k + 1] = adjustedPrefix[k] + adjustedClusters[k].advance;
            k++;
        }
        shrinkPrefix = [];
        lineEndOnlyCapacity = [];
        init = 0;
        while (init <= n) { shrinkPrefix.push(0.0); init++; }
        init = 0;
        while (init < n) { lineEndOnlyCapacity.push(0.0); init++; }
        for (opp in shrinkOpportunities) {
            if (opp.capacity <= 0 || opp.clusterIndex < 0 || opp.clusterIndex >= n) continue;
            if (opp.lineEndOnly) lineEndOnlyCapacity[opp.clusterIndex] += opp.capacity;
            else shrinkPrefix[opp.clusterIndex + 1] += opp.capacity;
        }
        k = 0;
        while (k < n) { shrinkPrefix[k + 1] += shrinkPrefix[k]; k++; }
    }
    public function buildLine(clusterRange:IntRange, endReason:LineEndReason):LineCandidate {
        return new LineCandidate(clusterRange, new TextRange(adjustedClusters[clusterRange.start].range.start, adjustedClusters[clusterRange.end].range.end),
            naturalPrefix[clusterRange.end + 1] - naturalPrefix[clusterRange.start], adjustedPrefix[clusterRange.end + 1] - adjustedPrefix[clusterRange.start], endReason);
    }
    public function gapCount(range:IntRange):Int return range.isEmpty ? 0 : gapPrefix[range.end] - gapPrefix[range.start];
    public function sinoGapCount(range:IntRange):Int return range.isEmpty ? 0 : sinoPrefix[range.end] - sinoPrefix[range.start];
    public function cjkGapCount(range:IntRange):Int return range.isEmpty ? 0 : cjkPrefix[range.end] - cjkPrefix[range.start];
    public function shrinkCapacity(range:IntRange):Float return shrinkPrefix[range.end + 1] - shrinkPrefix[range.start] + lineEndOnlyCapacity[range.end];
}

class EdgeState {
    public final start:Int; public final end:Int; public final hyphenRun:Int; public final stretchRun:Int; public final cost:Float; public final parent:Null<EdgeState>;
    public function new(start:Int, end:Int, hyphenRun:Int, stretchRun:Int, cost:Float, parent:Null<EdgeState>) { this.start=start; this.end=end; this.hyphenRun=hyphenRun; this.stretchRun=stretchRun; this.cost=cost; this.parent=parent; }
}
class EdgeGeometry {
    public final baseCost:Float; public final visibleStretch:Bool;
    public function new(baseCost:Float, visibleStretch:Bool) { this.baseCost=baseCost; this.visibleStretch=visibleStretch; }
}
