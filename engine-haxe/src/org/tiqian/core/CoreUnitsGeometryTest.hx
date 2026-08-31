package org.tiqian.core;

import org.tiqian.core.Ic;
import org.tiqian.core.Units.FloatIc;
import org.tiqian.core.Units.IntIc;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;

class CoreUnitsGeometryTest {
    private static var testTrace:TestTraceRecorder = null;

    private static function currentTrace():TestTraceRecorder {
        if (testTrace == null) {
            testTrace = new TestTraceRecorder("CoreUnitsGeometryTest");
        }
        return testTrace;
    }

    private static function expectArgumentFailure(block:()->Void):Void {
        TracedAssertions.assertFailsWith(null, block);
    }

    @:test
    public static function icPlusReturnsSum():Void {
        currentTrace().section("icPlusReturnsSum");
        TracedAssertions.assertEqualsIc(new Ic(5.0), Ic.plus(new Ic(2.0), new Ic(3.0)));
    }

    @:test
    public static function icUnaryMinusReturnsNegated():Void {
        currentTrace().section("icUnaryMinusReturnsNegated");
        TracedAssertions.assertEqualsIc(new Ic(-3.0), Ic.unaryMinus(new Ic(3.0)));
    }

    @:test
    public static function floatIcExtensionCreatesIc():Void {
        currentTrace().section("floatIcExtensionCreatesIc");
        TracedAssertions.assertEqualsIc(new Ic(2.0), FloatIc.ic(2.0));
    }

    @:test
    public static function intIcExtensionCreatesIc():Void {
        currentTrace().section("intIcExtensionCreatesIc");
        TracedAssertions.assertEqualsIc(new Ic(5.0), IntIc.ic(5));
    }

    @:test
    public static function icToPxMultipliesByEmSize():Void {
        currentTrace().section("icToPxMultipliesByEmSize");
        TracedAssertions.assertEqualsFloat(24.0, new Ic(3.0).toPx(8.0));
    }

    @:test
    public static function rectHeightReturnsDifference():Void {
        currentTrace().section("rectHeightReturnsDifference");
        TracedAssertions.assertEqualsFloat(20.0, new Rect(0.0, 0.0, 10.0, 20.0).height);
    }

    @:test
    public static function rectWidthReturnsDifference():Void {
        currentTrace().section("rectWidthReturnsDifference");
        TracedAssertions.assertEqualsFloat(10.0, new Rect(0.0, 0.0, 10.0, 20.0).width);
    }

    @:test
    public static function textRangeRejectsStartGreaterThanEnd():Void {
        currentTrace().section("textRangeRejectsStartGreaterThanEnd");
        expectArgumentFailure(() -> new TextRange(5, 2));
    }

    @:test
    public static function textRangeRejectsNegativeStart():Void {
        currentTrace().section("textRangeRejectsNegativeStart");
        expectArgumentFailure(() -> new TextRange(-1, 1));
    }

    @:test
    public static function layoutConstraintsRejectsNonPositiveMaxWidth():Void {
        currentTrace().section("layoutConstraintsRejectsNonPositiveMaxWidth");
        expectArgumentFailure(() -> new LayoutConstraints(-1.0, Math.POSITIVE_INFINITY, 2147483647));
    }

    @:test
    public static function layoutConstraintsRejectsNonPositiveMaxHeight():Void {
        currentTrace().section("layoutConstraintsRejectsNonPositiveMaxHeight");
        expectArgumentFailure(() -> new LayoutConstraints(100.0, -1.0, 2147483647));
    }

    @:test
    public static function layoutConstraintsRejectsNonPositiveMaxLines():Void {
        currentTrace().section("layoutConstraintsRejectsNonPositiveMaxLines");
        expectArgumentFailure(() -> new LayoutConstraints(100.0, 100.0, 0));
    }

    @:test
    public static function maxLinesDecisionInfoRecordsTruncationDetails():Void {
        currentTrace().section("maxLinesDecisionInfoRecordsTruncationDetails");
        final info = new MaxLinesDecisionInfo(5, 3, "MaxLinesLineTruncation");
        TracedAssertions.assertEqualsInt(5, info.laidOutLines);
        TracedAssertions.assertEqualsInt(3, info.visibleLines);
        TracedAssertions.assertEqualsString("MaxLinesLineTruncation", info.reason);
    }

    @:test
    public static function layoutDebugInfoAcceptsMaxLinesDecision():Void {
        currentTrace().section("layoutDebugInfoAcceptsMaxLinesDecision");
        final debug = new LayoutDebugInfo(new MaxLinesDecisionInfo(5, 3, "MaxLinesLineTruncation"), [], [], [], [], []);
        if (debug.maxLinesDecision == null) {
            TracedAssertions.fail();
            return;
        }
        final decision:MaxLinesDecisionInfo = debug.maxLinesDecision;
        TracedAssertions.assertEqualsInt(5, decision.laidOutLines);
        TracedAssertions.assertEqualsInt(3, decision.visibleLines);
    }

    public static function flushTestTrace():Void {
        currentTrace().flush();
    }
}
