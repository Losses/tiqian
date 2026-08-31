package org.tiqian.core;

import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;

class TextRangeTest {
    private static var testTrace:TestTraceRecorder = null;

    private static function currentTrace():TestTraceRecorder {
        if (testTrace == null) {
            testTrace = new TestTraceRecorder("TextRangeTest");
        }
        return testTrace;
    }

    @:test
    public static function exposesLength():Void {
        currentTrace().section("exposesLength");
        TracedAssertions.assertEquals(3, new TextRange(2, 5).length);
    }

    @:test
    public static function rejectsNegativeStart():Void {
        currentTrace().section("rejectsNegativeStart");
        TracedAssertions.assertFailsWith(null, () -> {
            new TextRange(-1, 1);
        });
    }

    public static function flushTestTrace():Void {
        currentTrace().flush();
    }
}
