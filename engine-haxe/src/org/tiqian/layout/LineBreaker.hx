package org.tiqian.layout;

import org.tiqian.core.Cluster;
import org.tiqian.core.IntRange;
import org.tiqian.core.LineEndReason;
import org.tiqian.core.TextRange;
import org.tiqian.core.TiqianIllegalArgumentException;
import org.tiqian.core.TextRangeError.Message;
import org.tiqian.layout.LineOptimization.LineSolution;
import org.tiqian.layout.LineOptimization.LineCandidate;
import org.tiqian.layout.LineOptimization.RepairOption;
import org.tiqian.layout.LineOptimization.RepairCandidate;
import org.tiqian.layout.KinsokuRule.ClreqKinsokuRule;
import org.tiqian.layout.ProgressiveBreakDecisions.ShrinkOpportunity;
import org.tiqian.layout.ProgressiveBreakDecisions.UnbreakableRanges;
import org.tiqian.layout.ProgressiveBreakDecisions.ProgressiveBreakOpportunity;
import std.SortedSet;
import std.SortedMap;

/**
 * Haxe port of Kotlin LineBreaker.kt: the LineBreaker contract and the greedy
 * implementation. The lookahead implementation (LookaheadLineBreaker with
 * scoreCandidate, rawGreedyLinesFrom, badness) and the fill PushIn pass
 * (withFillPushIn) arrive with their own lanes; breakLines here stops after
 * the kinsoku repair pass, which matches Kotlin whenever
 * lineAdjustmentPushIn is false.
 */
interface LineBreaker {
    var strategyName(get, never):String;
    function breakLines(naturalClusters:Array<Cluster>, adjustedClusters:Array<Cluster>, maxWidth:Float,
        ?shrinkOpportunities:Array<ShrinkOpportunity>, ?unbreakableRanges:UnbreakableRanges,
        ?firstLineIndent:Float, ?hangableClusters:SortedSet<Int>, ?extendableHangRanges:Array<IntRange>,
        ?forbiddenLineStartClusters:Null<SortedSet<Int>>, ?forbiddenLineEndClusters:SortedSet<Int>,
        ?hyphenBreakClusters:SortedSet<Int>, ?cjkInterCharBoundaries:SortedSet<Int>, ?maxCjkStretchPerGap:Float,
        ?sinoWesternBoundaries:SortedSet<Int>, ?sinoWesternStretchCap:Float, ?lineAdjustmentPushIn:Bool,
        ?lineAdjustmentCompressBias:Float, ?hardBreakAfterClusters:SortedSet<Int>,
        ?nonRenderingControlClusters:SortedSet<Int>, ?progressiveBreakOpportunities:SortedMap<Int,ProgressiveBreakOpportunity>):LineSolution;
}

/**
 * GreedyLineBreaker: fills each line until the next cluster would overflow,
 * then starts a new line. The kinsoku pass then repairs breaks that would
 * place a forbidden-at-line-start cluster at a line start: PushIn first, then
 * Hang (opt-in), then CarryPrevious, falling back to LeaveRagged.
 */
class GreedyLineBreaker implements LineBreaker {
    var kinsoku:KinsokuRule; var pushInPenalty:Int; var carryPreviousPenalty:Int; var leaveRaggedPenalty:Int;
    public function new(?kinsoku:KinsokuRule,?pushInPenalty:Int,?carryPreviousPenalty:Int,?leaveRaggedPenalty:Int) {
        if(kinsoku==null)this.kinsoku=new ClreqKinsokuRule();else this.kinsoku=kinsoku;
        if(pushInPenalty==null)this.pushInPenalty=2;else this.pushInPenalty=pushInPenalty;
        if(carryPreviousPenalty==null)this.carryPreviousPenalty=10;else this.carryPreviousPenalty=carryPreviousPenalty;
        if(leaveRaggedPenalty==null)this.leaveRaggedPenalty=20;else this.leaveRaggedPenalty=leaveRaggedPenalty;
    }
    public var strategyName(get,never):String; function get_strategyName():String return "greedy";

    public function breakLines(n:Array<Cluster>,a:Array<Cluster>,maxWidth:Float,?shrinkOpportunities:Array<ShrinkOpportunity>,?unbreakableRanges:UnbreakableRanges,?firstLineIndent:Float,?hangableClusters:SortedSet<Int>,?extendableHangRanges:Array<IntRange>,?forbiddenLineStartClusters:Null<SortedSet<Int>>,?forbiddenLineEndClusters:SortedSet<Int>,?hyphenBreakClusters:SortedSet<Int>,?cjkInterCharBoundaries:SortedSet<Int>,?maxCjkStretchPerGap:Float,?sinoWesternBoundaries:SortedSet<Int>,?sinoWesternStretchCap:Float,?lineAdjustmentPushIn:Bool,?lineAdjustmentCompressBias:Float,?hardBreakAfterClusters:SortedSet<Int>,?nonRenderingControlClusters:SortedSet<Int>,?progressiveBreakOpportunities:SortedMap<Int,ProgressiveBreakOpportunity>):LineSolution {
        if(a.length==0)return new LineSolution([]);
        if(n.length!=a.length)throw new TiqianIllegalArgumentException(Message("naturalClusters and adjustedClusters must align cluster-for-cluster."));
        final shrinkOps=shrinkOpportunities==null?[]:shrinkOpportunities;
        final ranges=unbreakableRanges==null?UnbreakableRanges.Empty:unbreakableRanges;
        final indent=firstLineIndent==null?0:firstLineIndent;
        final hangables=hangableClusters==null?LineBreakerLines.emptyIntSet():hangableClusters;
        final extendables=extendableHangRanges==null?[]:extendableHangRanges;
        final forbidEnd=forbiddenLineEndClusters==null?LineBreakerLines.emptyIntSet():forbiddenLineEndClusters;
        final hyphens=hyphenBreakClusters==null?LineBreakerLines.emptyIntSet():hyphenBreakClusters;
        final cjk=cjkInterCharBoundaries==null?LineBreakerLines.emptyIntSet():cjkInterCharBoundaries;
        final maxStretch=maxCjkStretchPerGap==null?Math.POSITIVE_INFINITY:maxCjkStretchPerGap;
        final sino=sinoWesternBoundaries==null?LineBreakerLines.emptyIntSet():sinoWesternBoundaries;
        final sinoCap=sinoWesternStretchCap==null?0:sinoWesternStretchCap;
        final hard=hardBreakAfterClusters==null?LineBreakerLines.emptyIntSet():hardBreakAfterClusters;
        final controls=nonRenderingControlClusters==null?LineBreakerLines.emptyIntSet():nonRenderingControlClusters;
        final progressive=progressiveBreakOpportunities==null?LineBreakerLines.emptyProgressiveMap():progressiveBreakOpportunities;
        final greedy=greedyFill(n,a,maxWidth,ranges,indent,forbidEnd,hyphens,cjk,maxStretch,sino,sinoCap,hard,controls,progressive);
        return LineRepair.applyKinsokuRepairs(greedy,n,a,maxWidth,this.kinsoku,shrinkOps,pushInPenalty,carryPreviousPenalty,leaveRaggedPenalty,ranges,indent,hangables,extendables,5,forbiddenLineStartClusters);
    }

    function greedyFill(n:Array<Cluster>,a:Array<Cluster>,maxWidth:Float,unbreakableRanges:UnbreakableRanges,firstLineIndent:Float,forbiddenLineEndClusters:SortedSet<Int>,hyphenBreakClusters:SortedSet<Int>,cjkInterCharBoundaries:SortedSet<Int>,maxCjkStretchPerGap:Float,sinoWesternBoundaries:SortedSet<Int>,sinoWesternStretchCap:Float,hardBreakAfterClusters:SortedSet<Int>,nonRenderingControlClusters:SortedSet<Int>,progressiveBreakOpportunities:SortedMap<Int,ProgressiveBreakOpportunity>):Array<LineCandidate> {
        final lines:Array<LineCandidate>=[];
        var lineStart=0;
        var adjustedAccum=0.0;
        var naturalAccum=0.0;
        var hasRenderingContent=false;
        var i=0;
        while(i<a.length){
            final nextAdjusted=adjustedAccum+a[i].advance;
            final limit=ProgressiveBreakDecisions.lineLimit(maxWidth,firstLineIndent,lineStart);
            if(nextAdjusted>limit&&hasRenderingContent){
                final progressive=ProgressiveBreakDecisions.decideProgressiveBreak(lineStart,i,progressiveBreakOpportunities,a,limit,cjkInterCharBoundaries,maxCjkStretchPerGap,sinoWesternBoundaries,sinoWesternStretchCap);
                final decided=ProgressiveBreakDecisions.decideHyphenBreak(lineStart,progressive,a,limit,hyphenBreakClusters,cjkInterCharBoundaries,maxCjkStretchPerGap,sinoWesternBoundaries,sinoWesternStretchCap);
                final afterUnbreak=ProgressiveBreakDecisions.adjustBreakForUnbreakables(decided,lineStart,unbreakableRanges);
                final breakAt=ProgressiveBreakDecisions.adjustBreakForLineEnd(afterUnbreak,lineStart,forbiddenLineEndClusters);
                lines.push(LineBreakerLines.closeFilledLine(new IntRange(lineStart,breakAt-1),afterUnbreak,n,a));
                lineStart=breakAt;
                adjustedAccum=a[breakAt].advance;
                naturalAccum=n[breakAt].advance;
                hasRenderingContent=!nonRenderingControlClusters.has(breakAt);
                i=breakAt+1;
            }else{
                adjustedAccum=nextAdjusted;
                naturalAccum+=n[i].advance;
                if(!nonRenderingControlClusters.has(i))hasRenderingContent=true;
                if(hardBreakAfterClusters.has(i)){
                    lines.push(LineBreakerLines.rebuildLine(new IntRange(lineStart,i),n,a,LineEndReason.MandatoryBreak));
                    lineStart=i+1;
                    adjustedAccum=0;
                    naturalAccum=0;
                    hasRenderingContent=false;
                }
                i++;
            }
        }
        if(lineStart<a.length)lines.push(LineBreakerLines.rebuildLine(new IntRange(lineStart,a.length-1),n,a,LineEndReason.ParagraphEnd));
        else if(hardBreakAfterClusters.has(a.length-1))lines.push(LineBreakerLines.emptyLineCandidate(a[a.length-1].range.end,LineEndReason.ParagraphEnd));
        return lines;
    }
}

/**
 * Kotlin LineBreaker.kt keeps rebuildLine, emptyLineCandidate and
 * closeFilledLine as package-level internal functions shared with
 * LineRepair.kt; the port groups them as statics of this class.
 */
class LineBreakerLines {
    /**
     * Builds a line for [range]; if the break retreated from [naturalBreakAt]
     * (line-end kinsoku), records CarryNext for the mark moved to the next
     * line.
     */
    public static function closeFilledLine(range:IntRange,naturalBreakAt:Int,n:Array<Cluster>,a:Array<Cluster>):LineCandidate {
        final line=rebuildLine(range,n,a);
        if(range.end+1==naturalBreakAt)return line;
        final moved=range.end+1;
        return new LineCandidate(line.clusterRange,line.sourceRange,line.naturalWidth,line.adjustedWidth,line.endReason,RepairOption.CarryNext(0,"ForbiddenAtLineEnd:"+a[moved].text+":moved-to-next-line",moved),line.repairCandidates,line.hangingClusterIndices);
    }

    public static function rebuildLine(clusterRange:IntRange,n:Array<Cluster>,a:Array<Cluster>,?endReason:Null<LineEndReason>,?repair:Null<RepairOption>,?repairCandidates:Null<Array<RepairCandidate>>):LineCandidate {
        if(clusterRange.isEmpty)throw new TiqianIllegalArgumentException(Message("Use emptyLineCandidate for an empty line."));
        var natural=0.0;
        var adjusted=0.0;
        var idx=clusterRange.start;
        while(idx<=clusterRange.end){natural+=n[idx].advance;adjusted+=a[idx].advance;idx++;}
        return new LineCandidate(clusterRange,new TextRange(a[clusterRange.start].range.start,a[clusterRange.end].range.end),natural,adjusted,endReason,repair,repairCandidates);
    }

    public static function emptyLineCandidate(sourceOffset:Int,?endReason:Null<LineEndReason>):LineCandidate {
        return new LineCandidate(new IntRange(1,0),new TextRange(sourceOffset,sourceOffset),0,0,endReason);
    }

    public static function emptyIntSet():SortedSet<Int> {final b=SortedSet.builder();return b.build();}

    public static function emptyProgressiveMap():SortedMap<Int,ProgressiveBreakOpportunity> {final b=SortedMap.builder();return b.build();}
}
