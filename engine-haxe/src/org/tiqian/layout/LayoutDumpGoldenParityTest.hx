package org.tiqian.layout;

import org.tiqian.test.LayoutFixtures.EarlyLayoutFixtures;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions.assertTrue;
import org.tiqian.test.trace.ClassTestEntry;

class LayoutDumpGoldenParityTest {
    public static function layoutDecisionDumpsMatchEmbeddedGolden():Void {
        final t = new TestTraceRecorder("LayoutDumpGoldenParityTest");
        t.section("layoutDecisionDumpsMatchEmbeddedGolden");
        final failures = [];
        for (fixture in EarlyLayoutFixtures.all) {
            final golden = LayoutDumpGoldens.byId().get(fixture.id);
            if (golden == null) {
                failures.push("missing embedded golden for fixture '" + fixture.id + "' — run with TIQIAN_UPDATE_GOLDEN=1 on the JVM, then rebuild");
                continue;
            }
            final actual = LayoutDumpFormat.layoutFixtureDump(fixture);
            if (golden != actual)
                failures.push(LayoutDumpFormat.layoutDumpDiffMessage(fixture.id, golden, actual));
        }
        assertTrue(failures.length == 0,
            failures.join("\n\n") + "\n\nIf the change is intentional, regenerate with TIQIAN_UPDATE_GOLDEN=1 on the JVM and review the golden diff.");
    }

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
        ClassTestEntry.run(layoutDecisionDumpsMatchEmbeddedGolden);
        TestTraceRecorder.flushClass("LayoutDumpGoldenParityTest");
    }
}
