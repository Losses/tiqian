package org.tiqian.layout;

import org.tiqian.core.Cluster;
import org.tiqian.core.TextRange;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;
import std.SortedSet;
import org.tiqian.test.trace.ClassTestEntry;

class DecideHyphenBreakTest {
    static function c(i:Int, a:Float):Cluster
        return new Cluster(new TextRange(i, i + 1), "x", "k", a);

    static function cs():Array<Cluster>
        return [c(0, 16), c(1, 16), c(2, 32), c(3, 32), c(4, 32)];

    static function m(a:Array<Int>):SortedSet<Int> {
        final b = SortedSet.builder();
        var j = 0;
        while (j < a.length) {
            b.put(a[j]);
            j++;
        }
        return b.build();
    }

    public static function chargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnown():Void {
        new TestTraceRecorder("DecideHyphenBreakTest").section("chargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnown");
        TracedAssertions.assertEqualsInt(4, ProgressiveBreakDecisions.decideHyphenBreak(0, 4, cs(), 74, m([4]), m([1]), 8));
    }

    public static function discountsSinoWesternCapacityBeforeChargingCjkLooseness():Void {
        new TestTraceRecorder("DecideHyphenBreakTest").section("discountsSinoWesternCapacityBeforeChargingCjkLooseness");
        TracedAssertions.assertEqualsInt(3, ProgressiveBreakDecisions.decideHyphenBreak(0, 4, cs(), 74, m([4]), m([1]), 8, m([2]), 4));
    }

    public static function flush():Void
        new TestTraceRecorder("DecideHyphenBreakTest").flush();

    /**
        The conventional class test entry (boring feature spec 19).
        The class carries no test marker, so the Kotlin runner generated from
        that marker's collection never instantiates it; this entry is how the
        runner calls the class once. It registers no test id, so the cross-target
        test id set is unchanged. The body repeats the calls
        engine-haxe/tests/Main.hx makes for this class, through the same failure
        accounting Main.hx uses, and then flushes the class trace the way Main.hx
        does.
    **/
    public static function runTestEntries():Void {
        ClassTestEntry.run(chargesAllDeficitToCjkWhenNoSinoWesternCapacityIsKnown);
        ClassTestEntry.run(discountsSinoWesternCapacityBeforeChargingCjkLooseness);
        TestTraceRecorder.flushClass("DecideHyphenBreakTest");
    }
}
