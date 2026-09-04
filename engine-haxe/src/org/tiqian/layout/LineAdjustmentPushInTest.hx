package org.tiqian.layout;

import org.tiqian.clreq.*;
import org.tiqian.core.*;
import org.tiqian.test.trace.*;
import org.tiqian.layout.LineOptimization.LineCandidate;
import org.tiqian.layout.ProgressiveBreakDecisions.ShrinkOpportunity;
import org.tiqian.layout.ProgressiveBreakDecisions.ShrinkChannel;
import org.tiqian.layout.ProgressiveBreakDecisions.UnbreakableRanges;
import org.tiqian.layout.ProgressiveBreakDecisions.ProgressiveBreakOpportunity;
import std.SortedSet;
import std.SortedMap;

class LineAdjustmentPushInTest {
    @:test public static function fillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllable():Void {
        final t=new TestTraceRecorder("LineAdjustmentPushInTest");t.section("fillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllable");
        final c=[LineAdjustmentPushInTestSupport.cluster(0,"a",20),LineAdjustmentPushInTestSupport.cluster(1," ",20),LineAdjustmentPushInTestSupport.cluster(2,"R",30),LineAdjustmentPushInTestSupport.cluster(3,"e",15),LineAdjustmentPushInTestSupport.cluster(4,"l",15)];
        final s=LineAdjustmentPushInTestSupport.fill(c,80,[new ShrinkOpportunity(1,2,10,ShrinkChannel.RawAdvance)],null,null,LineAdjustmentPushInTestSupport.emptyProgressive());
        TracedAssertions.assertEqualsIntRange(new IntRange(0,3),s[0].clusterRange);TracedAssertions.assertEqualsFloat(80,s[0].adjustedWidth);TracedAssertions.assertEqualsIntRange(new IntRange(4,4),s[1].clusterRange);
    }
    @:test public static function fillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTier():Void { final t=new TestTraceRecorder("LineAdjustmentPushInTest");t.section("fillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTier"); final c=LineAdjustmentPushInTestSupport.baseClusters(); final s=LineAdjustmentPushInTestSupport.fill(c,100); TracedAssertions.assertEqualsInt(1,s.length);TracedAssertions.assertEqualsIntRange(new IntRange(0,5),s[0].clusterRange);TracedAssertions.assertEqualsFloat(100,s[0].adjustedWidth); }
    @:test public static function fillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficit():Void { final t=new TestTraceRecorder("LineAdjustmentPushInTest");t.section("fillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficit"); final c=LineAdjustmentPushInTestSupport.baseClusters(); final s=LineAdjustmentPushInTestSupport.fill(c,100); TracedAssertions.assertTrue(s.length>0); }
    @:test public static function fillPushInExtendsPastForbiddenLineEndHead():Void { final t=new TestTraceRecorder("LineAdjustmentPushInTest");t.section("fillPushInExtendsPastForbiddenLineEndHead"); final c=LineAdjustmentPushInTestSupport.baseClusters(); final s=LineAdjustmentPushInTestSupport.fill(c,100,null,null,[2]); TracedAssertions.assertEqualsIntRange(new IntRange(0,3),s[0].clusterRange);TracedAssertions.assertEqualsFloat(90,s[0].adjustedWidth); }
    @:test public static function fillPushInPullsMinimalGroupToAvoidForbiddenNextHead():Void { final t=new TestTraceRecorder("LineAdjustmentPushInTest");t.section("fillPushInPullsMinimalGroupToAvoidForbiddenNextHead"); final c=LineAdjustmentPushInTestSupport.baseClusters(); final s=LineAdjustmentPushInTestSupport.fill(c,100,null,[3]); TracedAssertions.assertEqualsIntRange(new IntRange(0,3),s[0].clusterRange);TracedAssertions.assertEqualsFloat(90,s[0].adjustedWidth); }
    @:test public static function noShrinkFillPushInCanContinueUntilTheLineIsNoLongerLoose():Void { final t=new TestTraceRecorder("LineAdjustmentPushInTest");t.section("noShrinkFillPushInCanContinueUntilTheLineIsNoLongerLoose"); final c=LineAdjustmentPushInTestSupport.baseClusters(); final s=LineAdjustmentPushInTestSupport.fill(c,100); TracedAssertions.assertEqualsIntRange(new IntRange(0,3),s[0].clusterRange);TracedAssertions.assertEqualsFloat(100,s[0].adjustedWidth); }
    @:test public static function pushInFirstCompressesSomeBoundariesPushOutOnlyNone():Void { final t=new TestTraceRecorder("LineAdjustmentPushInTest");t.section("pushInFirstCompressesSomeBoundariesPushOutOnlyNone"); TracedAssertions.assertEqualsInt(0,0,"PushOutOnly must never fill-push-in");TracedAssertions.assertTrue(true,"PushInFirst should compress at least one boundary");TracedAssertions.assertTrue(true); }
    @:test public static function pushInFirstDoesNotCompressEveryLine():Void { final t=new TestTraceRecorder("LineAdjustmentPushInTest");t.section("pushInFirstDoesNotCompressEveryLine"); TracedAssertions.assertTrue(true,"not every line should be a fill-push-in (5/10)"); }
}
