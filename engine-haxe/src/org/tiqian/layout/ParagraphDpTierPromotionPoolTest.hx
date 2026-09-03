package org.tiqian.layout;

import org.tiqian.core.Cluster;
import org.tiqian.core.TextRange;
import org.tiqian.layout.LineOptimization.RepairOption;
import org.tiqian.layout.ProgressiveBreakDecisions.*;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;
import std.SortedMap;
import std.SortedSet;

class ParagraphDpTierPromotionPoolTest {
    private static function cluster(index:Int, text:String, advance:Float):Cluster return new Cluster(new TextRange(index,index+1),text,"test",advance);
    private static function hanClusters(n:Int):Array<Cluster> return [for (i in 0...n) cluster(i,"中",16)];
    private static function latinClusters():Array<Cluster> return [cluster(0,"a",30),cluster(1,"/",30),cluster(2,"b",25),cluster(3,"c",30),cluster(4,"d",30)];
    private static function opp(keys:Array<Int>, values:Array<ProgressiveBreakOpportunity>):SortedMap<Int,ProgressiveBreakOpportunity> { final b=SortedMap.builder(); for(i in 0...keys.length)b.put(keys[i],values[i]); return b.build(); }
    private static function repairReason(r:Null<RepairOption>):String return r == null ? "" : RepairOptions.reason(r);
    @:test public static function foreignSpanCandidateSurvivesThePromotionPoolPurge():Void {
        final t=new TestTraceRecorder("ParagraphDpTierPromotionPoolTest");t.section("foreignSpanCandidateSurvivesThePromotionPoolPurge"); final c=latinClusters(); final span=new TextRange(0,c.length);
        final s=new ParagraphDpLineBreaker().breakLines(c,c,80,[new ShrinkOpportunity(2,2,5,ShrinkChannel.RawAdvance)],null,null,null,null,null,null,null,null,null,null,null,true,null,null,null,opp([1,2,3],[new ProgressiveBreakOpportunity(Emergency,new TextRange(0,1)),new ProgressiveBreakOpportunity(Emergency,span),new ProgressiveBreakOpportunity(Whitespace,span)]));
        TracedAssertions.assertTrue(repairReason(s.lines[0].repair).indexOf("ProgressiveTechnicalTierPromotion") == 0, Std.string(s.lines));
    }
    @:test public static function committedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReason():Void {
        final t=new TestTraceRecorder("ParagraphDpTierPromotionPoolTest");t.section("committedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReason"); final c=hanClusters(4);
        final s=new ParagraphDpLineBreaker().breakLines(c,c,44,[new ShrinkOpportunity(2,1,4,ShrinkChannel.TrailingGlue,true)],null,null,null,null,null,null,null,null,null,[1],null,null,null,true,null,null,null,opp([2,3],[new ProgressiveBreakOpportunity(Emergency,new TextRange(0,2)),new ProgressiveBreakOpportunity(Whitespace,new TextRange(2,4))]));
        TracedAssertions.assertEqualsIntRange(new org.tiqian.core.IntRange(0,2),s.lines[0].clusterRange); TracedAssertions.assertTrue(repairReason(s.lines[0].repair).indexOf("LineAdjustmentPushIn") == 0);
    }
    @:test public static function committedCompressedEndWithoutOpportunityKeepsPlainPushInReason():Void {
        final t=new TestTraceRecorder("ParagraphDpTierPromotionPoolTest");t.section("committedCompressedEndWithoutOpportunityKeepsPlainPushInReason"); final c=hanClusters(4);
        final s=new ParagraphDpLineBreaker().breakLines(c,c,44,[new ShrinkOpportunity(2,1,4,ShrinkChannel.TrailingGlue,true)],null,null,null,null,null,null,null,null,[1],null,null,null,true,null,null,null,opp([2],[new ProgressiveBreakOpportunity(Emergency,new TextRange(0,2))]));
        TracedAssertions.assertEqualsIntRange(new org.tiqian.core.IntRange(0,2),s.lines[0].clusterRange); TracedAssertions.assertTrue(repairReason(s.lines[0].repair).indexOf("LineAdjustmentPushIn") == 0);
    }
}
