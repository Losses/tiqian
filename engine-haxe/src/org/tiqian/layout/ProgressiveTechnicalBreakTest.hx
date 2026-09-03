package org.tiqian.layout;

import org.tiqian.core.Cluster;
import org.tiqian.core.TextRange;
import org.tiqian.layout.ProgressiveBreakDecisions.ProgressiveBreakOpportunity;
import org.tiqian.layout.ProgressiveBreakDecisions.ProgressiveBreakTier;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;
import std.SortedMap;

class ProgressiveTechnicalBreakTest {
    private static function cluster(index:Int, text:String, advance:Float):Cluster
        return new Cluster(new TextRange(index, index + 1), text, text, "test", advance);
    private static function map(keys:Array<Int>, values:Array<ProgressiveBreakOpportunity>):SortedMap<Int,ProgressiveBreakOpportunity> {
        final b = SortedMap.builder();
        for (i in 0...keys.length) b.put(keys[i], values[i]);
        return b.build();
    }
    @:test public static function sourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllable():Void {
        final t = new TestTraceRecorder("ProgressiveTechnicalBreakTest"); t.section("sourceWhitespaceCapacityKeepsStructuralTierAheadOfSyllable");
        final span = new TextRange(0, 6);
        final c = [cluster(0,"a",20),cluster(1," ",4),cluster(2,"b",28),cluster(3,"/",28),cluster(4,"c",2),cluster(5,"d",20)];
        final o = map([2,4,5], [new ProgressiveBreakOpportunity(ProgressiveBreakTier.Whitespace,span,4),new ProgressiveBreakOpportunity(ProgressiveBreakTier.Structural,span),new ProgressiveBreakOpportunity(ProgressiveBreakTier.Syllable,span)]);
        TracedAssertions.assertEqualsInt(4, ProgressiveBreakDecisions.decideProgressiveBreak(0,5,o,c,84,null,8));
    }
    @:test public static function lookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCut():Void {
        final t = new TestTraceRecorder("ProgressiveTechnicalBreakTest"); t.section("lookaheadMayNotReplaceSelectedEmergencyBoundaryWithEarlierSameTierCut");
        final span = new TextRange(0,5); final c = [for (i in 0...5) cluster(i,String.fromCharCode(97+i),20)];
        final o = map([3,4],[new ProgressiveBreakOpportunity(ProgressiveBreakTier.Emergency,span),new ProgressiveBreakOpportunity(ProgressiveBreakTier.Emergency,span)]);
        TracedAssertions.assertFalse(ProgressiveBreakDecisions.progressiveCandidateAllowed(0,4,3,o,c,90,null,8));
        TracedAssertions.assertTrue(ProgressiveBreakDecisions.progressiveCandidateAllowed(0,4,4,o,c,90,null,8));
    }
}
