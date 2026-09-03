package org.tiqian.layout;

import org.tiqian.core.Cluster;
import org.tiqian.core.TextRange;
import org.tiqian.core.IntRange;
import org.tiqian.layout.LineOptimization.RepairOption;
import org.tiqian.layout.ProgressiveBreakDecisions.ShrinkOpportunity;
import org.tiqian.layout.ProgressiveBreakDecisions.ShrinkChannel;
import org.tiqian.layout.ProgressiveBreakDecisions.UnbreakableRanges;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;

class PushInLineWideCapacityTest {
    private static function cluster(s:Int,e:Int,text:String,a:Float):Cluster return new Cluster(new TextRange(s,e),text,"test",a);
    private static function solve(c:Array<Cluster>,w:Float,o:Array<ShrinkOpportunity>,?u:Null<UnbreakableRanges>):LineOptimization.LineSolution return new GreedyLineBreaker().breakLines(c,c,w,o,u);
    @:test public static function pushInAggregatesShrinkFromMultiplePrecedingClusters():Void {
        final t=new TestTraceRecorder("PushInLineWideCapacityTest");t.section("pushInAggregatesShrinkFromMultiplePrecedingClusters"); final c:Array<Cluster>=[];
        for(i in 0...5)c.push(cluster(i,i+1,"中",16)); c.push(cluster(5,6,"、",16)); for(i in 0...4)c.push(cluster(6+i,7+i,"文",16)); c.push(cluster(10,11,"。",16));
        final s=solve(c,160,[new ShrinkOpportunity(5,6,8,ShrinkChannel.TrailingGlue),new ShrinkOpportunity(10,6,8,ShrinkChannel.TrailingGlue)]); final l=s.lines[0];
        TracedAssertions.assertEqualsInt(1,s.lines.length);TracedAssertions.assertEqualsIntRange(new IntRange(0,10),l.clusterRange);TracedAssertions.assertEqualsFloat(160,l.adjustedWidth);TracedAssertions.assertTrue(l.repair!=null);
        final r:RepairOption=l.repair; switch(r){case PushIn(_,_,off,alloc,shrink,cap): TracedAssertions.assertEqualsInt(10,off);TracedAssertions.assertEqualsFloat(16,shrink);TracedAssertions.assertEqualsFloat(16,cap);TracedAssertions.assertEqualsInt(2,alloc.length); default: TracedAssertions.fail();}
    }
    @:test public static function pushInRejectsWhenLineWideCapacityStillInsufficient():Void {
        final t=new TestTraceRecorder("PushInLineWideCapacityTest");t.section("pushInRejectsWhenLineWideCapacityStillInsufficient"); final c=[for(i in 0...11) cluster(i,i+1,i==5?"、":i==10?"。":"文",16)]; final s=solve(c,160,[new ShrinkOpportunity(5,6,4,ShrinkChannel.TrailingGlue)]);
        TracedAssertions.assertEqualsInt(2,s.lines.length);TracedAssertions.assertEqualsIntRange(new IntRange(0,8),s.lines[0].clusterRange);TracedAssertions.assertEqualsIntRange(new IntRange(9,10),s.lines[1].clusterRange);
    }
    @:test public static function pushInOffenderOnlyCapacityStillWorksBackCompat():Void { final t=new TestTraceRecorder("PushInLineWideCapacityTest");t.section("pushInOffenderOnlyCapacityStillWorksBackCompat"); final c=[cluster(0,1,"中",16),cluster(1,2,"文",16),cluster(2,3,"中",16),cluster(3,4,"。",16)]; final s=solve(c,60,[new ShrinkOpportunity(3,6,4,ShrinkChannel.TrailingGlue)]); TracedAssertions.assertEqualsInt(1,s.lines.length); }
    @:test public static function pushInMergesOffenderThatFitsAfterChainedRepairs():Void { final t=new TestTraceRecorder("PushInLineWideCapacityTest");t.section("pushInMergesOffenderThatFitsAfterChainedRepairs"); final c=[for(i in 0...10) cluster(i,i+1,i==3?"」":i==4?"。":i==8?"、":"中",16)]; final s=solve(c,64,[new ShrinkOpportunity(3,6,8,ShrinkChannel.TrailingGlue),new ShrinkOpportunity(4,6,8,ShrinkChannel.TrailingGlue),new ShrinkOpportunity(8,6,8,ShrinkChannel.TrailingGlue)]); TracedAssertions.assertEqualsInt(3,s.lines.length); }
    @:test public static function carryPreviousRefusesToSplitUnbreakableSpan():Void { final t=new TestTraceRecorder("PushInLineWideCapacityTest");t.section("carryPreviousRefusesToSplitUnbreakableSpan"); final c=[for(i in 0...6) cluster(i,i+1,i==5?"。":i>=2?"王":"中",16)]; final s=solve(c,80,[new ShrinkOpportunity(5,6,8,ShrinkChannel.TrailingGlue)],new UnbreakableRanges([new IntRange(2,4)])); TracedAssertions.assertEqualsInt(2,s.lines.length); }
}
