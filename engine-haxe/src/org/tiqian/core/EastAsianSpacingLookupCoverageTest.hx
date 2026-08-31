package org.tiqian.core;

import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;

class EastAsianSpacingLookupCoverageTest {
    private static var testTrace:TestTraceRecorder = null;

    private static function currentTrace():TestTraceRecorder {
        if (testTrace == null) {
            testTrace = new TestTraceRecorder("EastAsianSpacingLookupCoverageTest");
        }
        return testTrace;
    }

    @:test
    public static function lookupCoversEveryGeneratedValueAndBothMissDirections():Void {
        currentTrace().section("lookupCoversEveryGeneratedValueAndBothMissDirections");
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Conditional, EastAsianSpacingData.lookup(0x21));
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Narrow, EastAsianSpacingData.lookup(0x41));
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Narrow, EastAsianSpacingData.lookup(0x30));
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Wide, EastAsianSpacingData.lookup(0x4E00));
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Wide, EastAsianSpacingData.lookup(0x9FFF));
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Other, EastAsianSpacingData.lookup(0x02));
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Other, EastAsianSpacingData.lookup(0x10FFFF));
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Other, EastAsianSpacingData.lookup(0x22));
    }

    public static function flushTestTrace():Void {
        currentTrace().flush();
    }
}
