package org.tiqian.layout;
import org.tiqian.layout.Justifier.CompressionPlan;
import org.tiqian.layout.ProgressiveBreakDecisions.ShrinkOpportunity;
import org.tiqian.layout.ProgressiveBreakDecisions.ShrinkChannel;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;

class JustifierCompressionTestSupport {
 public static function shrinkOf(plan:CompressionPlan, clusterIndex:Int):Null<Float> {
  var i = 0;
  while (i < plan.allocations.length) {
   if (plan.allocations[i].clusterIndex == clusterIndex) return plan.allocations[i].shrink;
   i++;
  }
  return null;
 }
}

class JustifierCompressionTest {
 @:test public static function consumesTiersInAscendingOrder():Void {
  new TestTraceRecorder("JustifierCompressionTest").section("consumesTiersInAscendingOrder");
  final justifier = new Justifier();
  final opps:Array<ShrinkOpportunity> = [
   new ShrinkOpportunity(0, 1, 2.0, ShrinkChannel.TrailingGlue),
   new ShrinkOpportunity(1, 2, 5.0, ShrinkChannel.TrailingGlue),
   new ShrinkOpportunity(2, 3, 5.0, ShrinkChannel.TrailingGlue),
  ];
  final plan = justifier.compress(3.0, opps);
  TracedAssertions.assertEqualsFloatTolerance(0.0, plan.unfilledSurplus, 0.0001);
  TracedAssertions.assertEqualsFloatTolerance(2.0, JustifierCompressionTestSupport.shrinkOf(plan, 0), 0.0001);
  TracedAssertions.assertEqualsFloatTolerance(1.0, JustifierCompressionTestSupport.shrinkOf(plan, 1), 0.0001);
  TracedAssertions.assertNullRendered(JustifierCompressionTestSupport.shrinkOf(plan, 2) == null, "-", "tier 3 must stay untouched while tier 2 has room");
 }
 @:test public static function nanSurplusEmitsNoAllocations():Void {
  new TestTraceRecorder("JustifierCompressionTest").section("nanSurplusEmitsNoAllocations");
  final justifier = new Justifier();
  final plan = justifier.compress(Math.NaN, [new ShrinkOpportunity(0, 1, 5.0, ShrinkChannel.TrailingGlue)]);
  TracedAssertions.assertTrue(plan.allocations.length == 0);
  TracedAssertions.assertTrue(Math.isNaN(plan.unfilledSurplus));
 }
}
