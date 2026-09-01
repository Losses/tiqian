package org.tiqian.layout;

import org.tiqian.core.Cluster;
import org.tiqian.core.TextRange;

/** Ordered fallback tier for a break inside one progressive technical span. */
@:enum abstract ProgressiveBreakTier(Int) from Int to Int {
    var Whitespace = 0;
    var Structural = 1;
    var Syllable = 2;
    var WholeToken = 3;
    var Emergency = 4;
    public var priority(get, never):Int;
    private inline function get_priority():Int return this;
}

/** One cluster boundary exposed by a line-break span. */
@:dataClass class ProgressiveBreakOpportunity {
    public final tier:ProgressiveBreakTier;
    public final spanRange:TextRange;
    /** Bounded positive glue owned by the source whitespace immediately before this boundary. */
    public final precedingWhitespaceStretchCapacity:Float;
    public function new(tier:ProgressiveBreakTier, spanRange:TextRange, ?precedingWhitespaceStretchCapacity:Null<Float>) {
        this.tier=tier; this.spanRange=spanRange;
        this.precedingWhitespaceStretchCapacity=precedingWhitespaceStretchCapacity == null ? 0.0 : precedingWhitespaceStretchCapacity;
    }
}

/** Progressive technical break selection and its supporting decisions. */
class ProgressiveBreakDecisions {
    public static inline final PROGRESSIVE_TECHNICAL_VISIBLE_STRETCH_FRACTION:Float = 0.0;

    public static function decideProgressiveBreak(lineStart:Int, overflowAt:Int, opportunities:Map<Int,ProgressiveBreakOpportunity>, ?adjustedClusters:Null<Array<Cluster>>, ?lineLimit:Null<Float>, ?cjkInterCharBoundaries:Null<Map<Int,Bool>>, ?maxCjkStretchPerGap:Null<Float>, ?sinoWesternBoundaries:Null<Map<Int,Bool>>, ?sinoWesternStretchCap:Null<Float>):Int {
        final limit = lineLimit == null ? Math.POSITIVE_INFINITY : lineLimit;
        final cjk = cjkInterCharBoundaries == null ? new Map<Int,Bool>() : cjkInterCharBoundaries;
        final max = maxCjkStretchPerGap == null ? Math.POSITIVE_INFINITY : maxCjkStretchPerGap;
        final sino = sinoWesternBoundaries == null ? new Map<Int,Bool>() : sinoWesternBoundaries;
        final cap = sinoWesternStretchCap == null ? 0.0 : sinoWesternStretchCap;
        final active = opportunities.get(overflowAt); if (active == null) return overflowAt;
        final bestPriority = progressiveBreakPriorityForLine(lineStart,overflowAt,active,opportunities,adjustedClusters,limit,cjk,max,sino,cap);
        var best:Null<Int> = null; var boundary=lineStart+1;
        while(boundary<=overflowAt){ final o=opportunities.get(boundary); if(o!=null && o.spanRange.start==active.spanRange.start && o.spanRange.end==active.spanRange.end && o.tier.priority==bestPriority && (best==null || boundary>best)) best=boundary; boundary++; }
        return best == null ? overflowAt : best;
    }

    public static function progressiveCandidateAllowed(lineStart:Int, rawGreedy:Int, candidateEnd:Int, opportunities:Map<Int,ProgressiveBreakOpportunity>, ?adjustedClusters:Null<Array<Cluster>>, ?lineLimit:Null<Float>, ?cjkInterCharBoundaries:Null<Map<Int,Bool>>, ?maxCjkStretchPerGap:Null<Float>, ?sinoWesternBoundaries:Null<Map<Int,Bool>>, ?sinoWesternStretchCap:Null<Float>):Bool {
        final limit=lineLimit==null?Math.POSITIVE_INFINITY:lineLimit; final cjk=cjkInterCharBoundaries==null?new Map<Int,Bool>():cjkInterCharBoundaries; final max=maxCjkStretchPerGap==null?Math.POSITIVE_INFINITY:maxCjkStretchPerGap; final sino=sinoWesternBoundaries==null?new Map<Int,Bool>():sinoWesternBoundaries; final cap=sinoWesternStretchCap==null?0.0:sinoWesternStretchCap;
        final active=opportunities.get(rawGreedy); if(active==null)return true;
        final candidate=opportunities.get(candidateEnd);
        if(candidate==null){ if(adjustedClusters==null || candidateEnd<0 || candidateEnd>=adjustedClusters.length)return true; final source=adjustedClusters[candidateEnd].range.start; return source<=active.spanRange.start || source>=active.spanRange.end; }
        if(candidate.spanRange.start!=active.spanRange.start || candidate.spanRange.end!=active.spanRange.end)return true;
        if(candidateEnd>rawGreedy)return candidate.tier.priority<=active.tier.priority;
        final selected=decideProgressiveBreak(lineStart,rawGreedy,opportunities,adjustedClusters,limit,cjk,max,sino,cap); return candidateEnd==selected;
    }

    private static function progressiveBreakPriorityForLine(lineStart:Int, overflowAt:Int, active:ProgressiveBreakOpportunity, opportunities:Map<Int,ProgressiveBreakOpportunity>, adjustedClusters:Null<Array<Cluster>>, lineLimit:Float, cjkInterCharBoundaries:Map<Int,Bool>, maxCjkStretchPerGap:Float, sinoWesternBoundaries:Map<Int,Bool>, sinoWesternStretchCap:Float):Int {
        final priorities:Array<Int>=[]; var i=lineStart+1; while(i<=overflowAt){final o=opportunities.get(i);if(o!=null&&o.spanRange.start==active.spanRange.start&&o.spanRange.end==active.spanRange.end&&priorities.indexOf(o.tier.priority)<0)priorities.push(o.tier.priority);i++;} priorities.sort(function(a,b)return a-b); if(priorities.length==0)return active.tier.priority;
        if(adjustedClusters==null || !Math.isFinite(lineLimit) || !Math.isFinite(maxCjkStretchPerGap))return priorities[0];
        final stretch=maxCjkStretchPerGap*PROGRESSIVE_TECHNICAL_VISIBLE_STRETCH_FRACTION; var least=priorities[0]; var density=Math.POSITIVE_INFINITY; var leastBoundary=lineStart+1;
        for(priority in priorities){var b=0;i=lineStart+1;while(i<=overflowAt){final o=opportunities.get(i);if(o!=null&&o.spanRange.start==active.spanRange.start&&o.spanRange.end==active.spanRange.end&&o.tier.priority==priority)b=i;i++;}if(b==0)continue;final d=progressiveCandidateStretchDensity(lineStart,b,opportunities,adjustedClusters,lineLimit,cjkInterCharBoundaries,sinoWesternBoundaries,sinoWesternStretchCap);if(d<density){density=d;least=priority;leastBoundary=b;}if(d<=stretch)return priority;}
        var emergency=0;i=lineStart+1;while(i<=overflowAt){final o=opportunities.get(i);if(o!=null&&o.spanRange.start==active.spanRange.start&&o.spanRange.end==active.spanRange.end&&o.tier==ProgressiveBreakTier.Emergency)emergency=i;i++;} return emergency!=0&&emergency>=leastBoundary?ProgressiveBreakTier.Emergency:least;
    }

    private static function progressiveCandidateStretchDensity(lineStart:Int,boundary:Int,opportunities:Map<Int,ProgressiveBreakOpportunity>,adjustedClusters:Array<Cluster>,lineLimit:Float,cjkInterCharBoundaries:Map<Int,Bool>,sinoWesternBoundaries:Map<Int,Bool>,sinoWesternStretchCap:Float):Float {
        var width=0.0;var i=lineStart;while(i<boundary){width+=adjustedClusters[i].advance;i++;}final deficit=Math.max(lineLimit-width,0);var technical=0.0;i=lineStart+1;while(i<boundary){final o=opportunities.get(i);if(o!=null&&o.tier==ProgressiveBreakTier.Whitespace)technical+=o.precedingWhitespaceStretchCapacity;i++;}var sino=0;i=lineStart+1;while(i<boundary){if(sinoWesternBoundaries.exists(i))sino++;i++;}final cjkDeficit=Math.max(deficit-technical-sino*sinoWesternStretchCap,0);
        final active=opportunities.get(boundary);var units=0;if(active!=null){i=lineStart;while(i<boundary){final c=adjustedClusters[i];if(c.range.start>=active.spanRange.start&&c.range.end<=active.spanRange.end&&!isWhitespace(c.text))units+=c.text.length;i++;}}final technicalGaps=Math.max(units-1,0);if(technicalGaps>0)return cjkDeficit/technicalGaps;var gaps=0;i=lineStart+1;while(i<boundary){if(cjkInterCharBoundaries.exists(i))gaps++;i++;}return gaps==0?cjkDeficit:cjkDeficit/gaps;
    }
    private static function isWhitespace(s:String):Bool { var i=0;while(i<s.length){if(!StringTools.isSpace(s,i))return false;i++;}return true; }

    public static function decideHyphenBreak(lineStart:Int,overflowAt:Int,adjustedClusters:Array<Cluster>,lineLimit:Float,hyphenBreakClusters:Map<Int,Bool>,cjkInterCharBoundaries:Map<Int,Bool>,maxCjkStretchPerGap:Float,?sinoWesternBoundaries:Null<Map<Int,Bool>>,?sinoWesternStretchCap:Null<Float>):Int {
        final sino=sinoWesternBoundaries==null?new Map<Int,Bool>():sinoWesternBoundaries;final cap=sinoWesternStretchCap==null?0.0:sinoWesternStretchCap;if(!hyphenBreakClusters.exists(overflowAt))return overflowAt;var whole=overflowAt;while(whole>lineStart&&hyphenBreakClusters.exists(whole))whole--;if(whole<=lineStart)return overflowAt;var width=0.0;var k=lineStart;while(k<whole){width+=adjustedClusters[k].advance;k++;}final deficit=lineLimit-width;if(deficit<=0)return whole;var sw=0;k=lineStart+1;while(k<whole){if(sino.exists(k))sw++;k++;}final cjk=Math.max(deficit-sw*cap,0);var gaps=0;k=lineStart+1;while(k<whole){if(cjkInterCharBoundaries.exists(k))gaps++;k++;}return gaps==0||cjk/gaps>maxCjkStretchPerGap?overflowAt:whole;
    }
    public static function adjustBreakForLineEnd(breakAt:Int,lineStart:Int,forbiddenLineEndClusters:Map<Int,Bool>):Int {var b=breakAt;while(b-1>lineStart&&forbiddenLineEndClusters.exists(b-1))b--;return b;}
    public static function lineLimit(maxWidth:Float,firstLineIndent:Float,lineStartCluster:Int):Float return lineStartCluster==0?maxWidth-firstLineIndent:maxWidth;
    public static function adjustBreakForUnbreakables(breakAt:Int,lineStart:Int,unbreakableRanges:UnbreakableRanges):Int {var candidate=breakAt;while(true){final containing=unbreakableRanges.containingOrNull(candidate);if(containing==null)return candidate;if(containing.start<=lineStart)return breakAt;candidate=containing.start;}}
}

@:dataClass class ShrinkOpportunity {
    public final clusterIndex:Int; public final tier:Int; public final capacity:Float; public final channel:ShrinkChannel; public final lineEndOnly:Bool;
    public function new(clusterIndex:Int,tier:Int,capacity:Float,channel:ShrinkChannel,?lineEndOnly:Null<Bool>){this.clusterIndex=clusterIndex;this.tier=tier;this.capacity=capacity;this.channel=channel;this.lineEndOnly=lineEndOnly==null?false:lineEndOnly;}
}
enum ShrinkChannel { TrailingGlue; LeadingGlue; LeadingAndTrailingGlue; RawAdvance; }

/** Unbreakable cluster ranges indexed by sorted start and prefix maximum end. */
class UnbreakableRanges {
    public final ranges:Array<org.tiqian.core.IntRange>; private final byStart:Array<org.tiqian.core.IntRange>; private final startsSorted:Array<Int>; private final prefixMaxLast:Array<Int>;
    public function new(ranges:Array<org.tiqian.core.IntRange>){this.ranges=ranges;byStart=ranges.copy();byStart.sort(function(a,b)return a.start-b.start);startsSorted=[for(r in byStart) r.start];prefixMaxLast=[];var running= -2147483648;for(r in byStart){running=Std.int(Math.max(running,r.end));prefixMaxLast.push(running);}}
    public function containsBoundary(candidate:Int):Bool {var low=0;var high=startsSorted.length;while(low<high){var mid=(low+high)>>1;if(startsSorted[mid]<candidate)low=mid+1;else high=mid;}return low>0&&prefixMaxLast[low-1]>=candidate;}
    public function containingOrNull(candidate:Int):Null<org.tiqian.core.IntRange>{if(!containsBoundary(candidate))return null;for(r in ranges)if(candidate>r.start&&candidate<=r.end)return r;return null;}
    public function containingFromClosedStartOrNull(index:Int):Null<org.tiqian.core.IntRange>{var low=0;var high=startsSorted.length;while(low<high){var mid=(low+high)>>1;if(startsSorted[mid]<=index)low=mid+1;else high=mid;}if(low==0||prefixMaxLast[low-1]<=index)return null;for(r in ranges)if(index>=r.start&&index<=r.end&&r.end>index)return r;return null;}
    public static final Empty = new UnbreakableRanges([]);
}

