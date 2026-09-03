package org.tiqian.layout;

import org.tiqian.core.IntRange;
import org.tiqian.core.TextRange;
import org.tiqian.layout.LineOptimization.RepairOption;
import org.tiqian.layout.LineOptimization.RepairOptions;
import org.tiqian.layout.ProgressiveBreakDecisions.ProgressiveBreakOpportunity;
import org.tiqian.layout.ProgressiveBreakDecisions.ProgressiveBreakTier;
import org.tiqian.layout.ProgressiveBreakDecisions.ShrinkOpportunity;
import org.tiqian.layout.ProgressiveBreakDecisions.ShrinkChannel;
import org.tiqian.layout.ProgressiveBreakDecisions.UnbreakableRanges;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;
import org.tiqian.test.trace.TestTraceRender;

class ParagraphDpTierPromotionPoolTest {
    @:test public static function foreignSpanCandidateSurvivesThePromotionPoolPurge():Void {
        final t=new TestTraceRecorder("ParagraphDpTierPromotionPoolTest");t.section("foreignSpanCandidateSurvivesThePromotionPoolPurge"); final c=ParagraphDpTierPromotionPoolTestSupport.latinClusters(); final span=new TextRange(0,c.length);
        final s=new ParagraphDpLineBreaker().breakLines(c,c,80,[new ShrinkOpportunity(2,2,5,ShrinkChannel.RawAdvance)],null,null,null,null,null,null,null,null,null,null,null,true,null,null,null,ParagraphDpTierPromotionPoolTestSupport.opp([1,2,3],[new ProgressiveBreakOpportunity(ProgressiveBreakTier.Emergency,new TextRange(0,1)),new ProgressiveBreakOpportunity(ProgressiveBreakTier.Emergency,span),new ProgressiveBreakOpportunity(ProgressiveBreakTier.Whitespace,span)]));
        final repairs:Array<RepairOption>=[for (l in s.lines) l.repair]; TracedAssertions.assertTrue(ParagraphDpTierPromotionPoolTestSupport.repairReason(s.lines[0].repair).indexOf("ProgressiveTechnicalTierPromotion") == 0, TestTraceRender.cap(Std.string(repairs)));
    }
    @:test public static function committedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReason():Void {
        final t=new TestTraceRecorder("ParagraphDpTierPromotionPoolTest");t.section("committedCompressedLineWithForeignSpanOpportunitiesKeepsPlainPushInReason"); final c=ParagraphDpTierPromotionPoolTestSupport.hanClusters(4);
        final s=ParagraphDpLineBreakerTestSupport.solve(c,44,[new ShrinkOpportunity(2,1,4,ShrinkChannel.TrailingGlue,true)],null,true,null,ParagraphDpTierPromotionPoolTestSupport.opp([2,3],[new ProgressiveBreakOpportunity(ProgressiveBreakTier.Emergency,new TextRange(0,2)),new ProgressiveBreakOpportunity(ProgressiveBreakTier.Whitespace,new TextRange(2,4))]),null,[1],8);
        final repairs:Array<RepairOption>=[for (l in s.lines) l.repair]; TracedAssertions.assertEqualsIntRange(new IntRange(0,2),s.lines[0].clusterRange, TestTraceRender.cap(Std.string(s.lines))); TracedAssertions.assertTrue(ParagraphDpTierPromotionPoolTestSupport.repairReason(s.lines[0].repair).indexOf("LineAdjustmentPushIn") == 0, TestTraceRender.cap(Std.string(repairs)));
    }
    @:test public static function committedCompressedEndWithoutOpportunityKeepsPlainPushInReason():Void {
        final t=new TestTraceRecorder("ParagraphDpTierPromotionPoolTest");t.section("committedCompressedEndWithoutOpportunityKeepsPlainPushInReason"); final c=ParagraphDpTierPromotionPoolTestSupport.hanClusters(4);
        final s=ParagraphDpLineBreakerTestSupport.solve(c,44,[new ShrinkOpportunity(2,1,4,ShrinkChannel.TrailingGlue,true)],null,true,null,ParagraphDpTierPromotionPoolTestSupport.opp([2],[new ProgressiveBreakOpportunity(ProgressiveBreakTier.Emergency,new TextRange(0,2))]),null,[1],8);
        final repairs:Array<RepairOption>=[for (l in s.lines) l.repair]; TracedAssertions.assertEqualsIntRange(new IntRange(0,2),s.lines[0].clusterRange, TestTraceRender.cap(Std.string(s.lines))); TracedAssertions.assertTrue(ParagraphDpTierPromotionPoolTestSupport.repairReason(s.lines[0].repair).indexOf("LineAdjustmentPushIn") == 0, TestTraceRender.cap(Std.string(repairs)));
    }
}
