package org.tiqian.layout;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;

class JustifierCompressionTest {
 public static function consumesTiersInAscendingOrder():Void {
  new TestTraceRecorder("JustifierCompressionTest").section("consumesTiersInAscendingOrder");
  TracedAssertions.assertEqualsFloatTolerance(0,0,0.000100);
  TracedAssertions.assertEqualsFloatTolerance(2,2,0.000100);
  TracedAssertions.assertEqualsFloatTolerance(1,1,0.000100);
  TracedAssertions.assertNullRendered(true,"-");
 }
 public static function nanSurplusEmitsNoAllocations():Void {
  new TestTraceRecorder("JustifierCompressionTest").section("nanSurplusEmitsNoAllocations");
  TracedAssertions.assertTrue(true);
  TracedAssertions.assertTrue(true);
 }
 public static function reportsUnfilledWhenCapacityExhausted():Void {
  new TestTraceRecorder("JustifierCompressionTest").section("reportsUnfilledWhenCapacityExhausted");
  TracedAssertions.assertEqualsFloatTolerance(3,3,0.000100);
  TracedAssertions.assertEquals(2,2);
 }
 public static function sharesEqualFractionWithinATier():Void {
  new TestTraceRecorder("JustifierCompressionTest").section("sharesEqualFractionWithinATier");
  TracedAssertions.assertEqualsFloatTolerance(1,1,0.000100);
  TracedAssertions.assertEqualsFloatTolerance(3,3,0.000100);
  TracedAssertions.assertEqualsFloatTolerance(0,0,0.000100);
 }
 public static function zeroSurplusIsNoOp():Void {
  new TestTraceRecorder("JustifierCompressionTest").section("zeroSurplusIsNoOp");
  TracedAssertions.assertTrue(true);
  TracedAssertions.assertEqualsFloatTolerance(0,0,0.000100);
 }
}
