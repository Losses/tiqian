package org.tiqian.test.trace;

import org.tiqian.core.TiqianIllegalArgumentException;
import org.tiqian.core.IllegalStateException;
import org.tiqian.core.TiqianNoSuchElementException;
import org.tiqian.core.Ic;
import org.tiqian.core.EastAsianSpacingEdges;
import org.tiqian.clreq.BopomofoTone;
import org.tiqian.font.FontRole;
import org.tiqian.layout.QuotePairAnalyzer.QuotePair;
import org.tiqian.layout.QuotePairAnalyzer.QuoteType;
import org.tiqian.clreq.BopomofoReading;
import org.tiqian.clreq.ClreqProfile;
import org.tiqian.clreq.GlueSide;
import org.tiqian.clreq.HangingPunctuationStyle;
import org.tiqian.clreq.KinsokuLevel;
import org.tiqian.clreq.PunctuationClass;
import org.tiqian.clreq.PunctuationGluePlacement;
import std.ReadOnlyArray;
import org.tiqian.test.TestHelpers;
import std.StringBuf;

class TracedAssertions {
    public static function assertEquals(expected:Int, actual:Int, ?message:String):Void {
        recordEvent("eq", [
            field("expected", TestTraceRender.renderInt(expected)),
            field("actual", TestTraceRender.renderInt(actual)),
            msgField(message)
        ]);
        if (expected != actual) {
            fail(message == null ? "Expected values to be equal." : message);
        }
    }

    public static function assertEqualsString(expected:String, actual:String, ?message:String):Void {
        recordEvent("eq", [
            field("expected", TestTraceRender.renderString(expected)),
            field("actual", TestTraceRender.renderString(actual)),
            msgField(message)
        ]);
        if (expected != actual) {
            fail(message == null ? "Expected values to be equal." : message);
        }
    }

    public static function assertEqualsIc(expected:Ic, actual:Ic, ?message:String):Void {
        recordEvent("eq", [
            field("expected", expected.toString()),
            field("actual", actual.toString()),
            msgField(message)
        ]);
        if (expected.count != actual.count) {
            fail(message == null ? "Expected values to be equal." : message);
        }
    }

    public static function assertEqualsEastAsianSpacingEdges(expected:EastAsianSpacingEdges, actual:EastAsianSpacingEdges, ?message:String):Void {
        recordEvent("eq", [
            field("expected", expected.toString()),
            field("actual", actual.toString()),
            msgField(message)
        ]);
        if (expected.leading != actual.leading || expected.trailing != actual.trailing || expected.containsWide != actual.containsWide) {
            fail(message == null ? "Expected values to be equal." : message);
        }
    }

    public static function assertEqualsIntArray(expected:ReadOnlyArray<Int>, actual:ReadOnlyArray<Int>, ?message:String):Void {
        recordEvent("eq", [field("expected", renderInts(expected)), field("actual", renderInts(actual)), msgField(message)]);
        if (expected.length != actual.length) fail(message == null ? "Expected arrays to be equal." : message);
        var i = 0; while (i < expected.length) { if (expected[i] != actual[i]) fail(message == null ? "Expected arrays to be equal." : message); i++; }
    }

    private static function renderInts(values:ReadOnlyArray<Int>):String {
        final buf = new StringBuf(); buf.add("["); var i = 0;
        while (i < values.length) { if (i > 0) buf.add(", "); buf.add("" + values[i]); i++; }
        buf.add("]"); return buf.toString();
    }
    public static function assertEqualsStringArray(expected:ReadOnlyArray<String>, actual:ReadOnlyArray<String>, ?message:String):Void {
        recordEvent("eq", [
            field("expected", TestTraceRender.renderStringArray(expected)),
            field("actual", TestTraceRender.renderStringArray(actual)),
            msgField(message)
        ]);
        if (!sameStringArray(expected, actual)) {
            fail(message == null ? "Expected values to be equal." : message);
        }
    }

    public static function assertEqualsBopomofoTone(expected:BopomofoTone, actual:BopomofoTone, ?message:String):Void {
        recordEvent("eq", [
            field("expected", Std.string(expected)),
            field("actual", Std.string(actual)),
            msgField(message)
        ]);
        if (expected != actual) {
            fail(message == null ? "Expected values to be equal." : message);
        }
    }

    public static function assertEqualsFontRole(expected:FontRole, actual:Null<FontRole>, ?message:String):Void {
        recordEvent("eq", [field("expected", Std.string(expected)), field("actual", actual == null ? "null" : Std.string(actual)), msgField(message)]);
        if (actual == null || expected != actual) fail(message == null ? "Expected values to be equal." : message);
    }

    public static function assertEqualsQuoteType(expected:QuoteType, actual:QuoteType, ?message:String):Void {
        recordEvent("eq", [field("expected", Std.string(expected)), field("actual", Std.string(actual)), msgField(message)]);
        if (expected != actual) fail(message == null ? "Expected values to be equal." : message);
    }

    public static function assertEqualsQuotePair(expected:QuotePair, actual:QuotePair, ?message:String):Void {
        recordEvent("eq", [field("expected", expected.toString()), field("actual", actual.toString()), msgField(message)]);
        if (expected.openIndex != actual.openIndex || expected.closeIndex != actual.closeIndex || expected.quoteType != actual.quoteType) fail(message == null ? "Expected values to be equal." : message);
    }

    private static function renderQuotePairs(values:Array<QuotePair>):String {
        final buf = new StringBuf(); buf.add("["); var i = 0;
        while (i < values.length) { if (i > 0) buf.add(", "); buf.add(values[i].toString()); i++; }
        buf.add("]"); return buf.toString();
    }

    public static function assertEqualsQuotePairArray(expected:Array<QuotePair>, actual:Array<QuotePair>, ?message:String):Void {
        recordEvent("eq", [field("expected", renderQuotePairs(expected)), field("actual", renderQuotePairs(actual)), msgField(message)]);
        if (expected.length != actual.length) fail(message == null ? "Expected arrays to be equal." : message);
        var i = 0;
        while (i < expected.length) {
            if (expected[i].openIndex != actual[i].openIndex || expected[i].closeIndex != actual[i].closeIndex || expected[i].quoteType != actual[i].quoteType) fail(message == null ? "Expected arrays to be equal." : message);
            i++;
        }
    }

    public static function assertEqualsBopomofoReading(expected:BopomofoReading, actual:BopomofoReading, ?message:String):Void {
        recordEvent("eq", [
            field("expected", expected.toString()),
            field("actual", actual.toString()),
            msgField(message)
        ]);
        if (!sameStringArray(expected.symbols, actual.symbols) || expected.tone != actual.tone) {
            fail(message == null ? "Expected values to be equal." : message);
        }
    }

    public static function assertEqualsPunctuationClass(expected:PunctuationClass, actual:PunctuationClass, ?message:String):Void {
        recordEvent("eq", [
            field("expected", Std.string(expected)),
            field("actual", Std.string(actual)),
            msgField(message)
        ]);
        if (expected != actual) {
            fail(message == null ? "Expected values to be equal." : message);
        }
    }

    public static function assertEqualsGlueSide(expected:GlueSide, actual:GlueSide, ?message:String):Void {
        recordEvent("eq", [
            field("expected", Std.string(expected)),
            field("actual", Std.string(actual)),
            msgField(message)
        ]);
        if (expected != actual) {
            fail(message == null ? "Expected values to be equal." : message);
        }
    }

    public static function assertEqualsPunctuationGluePlacement(expected:PunctuationGluePlacement, actual:PunctuationGluePlacement, ?message:String):Void {
        recordEvent("eq", [
            field("expected", Std.string(expected)),
            field("actual", Std.string(actual)),
            msgField(message)
        ]);
        if (expected != actual) {
            fail(message == null ? "Expected values to be equal." : message);
        }
    }

    public static function assertEqualsKinsokuLevel(expected:KinsokuLevel, actual:KinsokuLevel, ?message:String):Void {
        recordEvent("eq", [
            field("expected", Std.string(expected)),
            field("actual", Std.string(actual)),
            msgField(message)
        ]);
        if (expected != actual) {
            fail(message == null ? "Expected values to be equal." : message);
        }
    }

    public static function assertEqualsHangingPunctuationStyle(expected:HangingPunctuationStyle, actual:HangingPunctuationStyle, ?message:String):Void {
        recordEvent("eq", [
            field("expected", Std.string(expected)),
            field("actual", Std.string(actual)),
            msgField(message)
        ]);
        if (expected != actual) {
            fail(message == null ? "Expected values to be equal." : message);
        }
    }

    public static function assertEqualsClreqProfile(expected:ClreqProfile, actual:ClreqProfile, ?message:String):Void {
        recordEvent("eq", [
            field("expected", TestTraceRender.cap(expected.toString())),
            field("actual", TestTraceRender.cap(actual.toString())),
            msgField(message)
        ]);
        if (!ClreqProfile.sameProfile(expected, actual)) {
            fail(message == null ? "Expected values to be equal." : message);
        }
    }

    private static function sameStringArray(first:ReadOnlyArray<String>, second:ReadOnlyArray<String>):Bool {
        if (first.length != second.length) {
            return false;
        }
        var index:Int = 0;
        while (index < first.length) {
            if (first[index] != second[index]) {
                return false;
            }
            index += 1;
        }
        return true;
    }

    public static function f32Literal(value:Float):Float {
        return TestHelpers.f32Literal(value);
    }

    public static function assertEqualsFloat(expected:Float, actual:Float, ?message:String):Void {
        recordEvent("eq", [
            field("expected", TestTraceRender.renderFloat(expected)),
            field("actual", TestTraceRender.renderFloat(actual)),
            msgField(message)
        ]);
        if (expected != actual) {
            fail(message == null ? "Expected values to be equal." : message);
        }
    }

    public static function assertEqualsFloatTolerance(expected:Float, actual:Float, tolerance:Float, ?message:String):Void {
        recordEvent("eq-tol", [
            field("expected", TestTraceRender.renderFloat(expected)),
            field("actual", TestTraceRender.renderFloat(actual)),
            field("tol", TestTraceRender.renderFloat(tolerance)),
            msgField(message)
        ]);
        if (expected != expected || actual != actual || Math.abs(expected - actual) > tolerance) {
            fail(message == null ? "Expected values to be equal within tolerance." : message);
        }
    }

    public static function assertEqualsRendered(expected:String, actual:String, ?message:String):Void {
        recordEvent("eq", [
            field("expected", TestTraceRender.cap(expected)),
            field("actual", TestTraceRender.cap(actual)),
            msgField(message)
        ]);
        if (expected != actual) {
            fail(message == null ? "Expected rendered values to be equal." : message);
        }
    }

    public static function assertEqualsInt(expected:Int, actual:Int, ?message:String):Void {
        recordEvent("eq", [
            field("expected", TestTraceRender.renderInt(expected)),
            field("actual", TestTraceRender.renderInt(actual)),
            msgField(message)
        ]);
        if (expected != actual) {
            fail(message == null ? "Expected values to be equal." : message);
        }
    }

    public static function assertTrue(actual:Bool, ?message:String):Void {
        recordEvent("is-true", [field("actual", TestTraceRender.renderBool(actual)), msgField(message)]);
        if (!actual) {
            fail(message == null ? "Expected value to be true." : message);
        }
    }

    public static function assertFalse(actual:Bool, ?message:String):Void {
        recordEvent("is-false", [field("actual", TestTraceRender.renderBool(actual)), msgField(message)]);
        if (actual) {
            fail(message == null ? "Expected value to be false." : message);
        }
    }

    public static function assertNullRendered(wasNull:Bool, renderedActual:String, ?message:String):Void {
        recordEvent("null", [field("actual", renderedActual), msgField(message)]);
        if (!wasNull) {
            fail(message == null ? "Expected value to be null." : message);
        }
    }

    public static function assertNotNullRendered(wasNotNull:Bool, renderedActual:String, ?message:String):Void {
        recordEvent("not-null", [field("actual", renderedActual), msgField(message)]);
        if (!wasNotNull) {
            fail(message == null ? "Expected value to be non-null." : message);
        }
    }

    public static function assertFailsWith(?message:String, block:()->Void):TiqianIllegalArgumentException {
        try {
            block();
        } catch (error:TiqianIllegalArgumentException) {
            recordEvent("raises", [
                field("exception", Std.isOfType(error, IllegalStateException) ? "IllegalStateException" : "TiqianIllegalArgumentException"),
                field("thrown", TestTraceRender.renderString(error.message)),
                msgField(message)
            ]);
            return error;
        }
        fail(message == null ? "Expected an exception." : message);
        return null;
    }

    public static function assertFailsWithNoSuchElement(?message:String, block:()->Void):TiqianNoSuchElementException {
        try {
            block();
        } catch (error:TiqianNoSuchElementException) {
            recordEvent("raises", [
                field("exception", "TiqianNoSuchElementException"),
                field("thrown", TestTraceRender.renderString(error.message)),
                msgField(message)
            ]);
            return error;
        }
        fail(message == null ? "Expected an exception." : message);
        return null;
    }

    public static function fail(?message:String, ?cause:haxe.Exception):Void {
        final text = message == null ? "Assertion failed." : message;
        recordEvent("fail", [msgField(message)]);
        throw new TraceAssertionException(TraceAssertionError.AssertionFailed(text));
    }

    public static function assertDoesNotThrow<T>(block:()->T):T {
        final result = block();
        recordEvent("no-throw", []);
        return result;
    }

    private static function recordEvent(name:String, fields:Array<Null<TraceField>>):Void {
        if (!TestTrace.updateMode) {
            return;
        }
        final line = new StringBuf();
        line.add(name);
        var index = 0;
        while (index < fields.length) {
            final current = fields[index];
            if (current != null) {
                line.add(" ");
                line.add(current.key);
                line.add("=");
                line.add(TestTraceRender.canonicalNumbers(current.value));
            }
            index += 1;
        }
        final recorder = TestTrace.currentRecorder();
        if (recorder != null) {
            recorder.record(line.toString());
        }
    }

    private static function field(key:String, value:String):TraceField {
        return {key: key, value: value};
    }

    private static function msgField(message:Null<String>):Null<TraceField> {
        if (message == null) {
            return null;
        }
        return field("msg", "'" + TestTraceRender.escapeOperand(message) + "'");
    }
}
