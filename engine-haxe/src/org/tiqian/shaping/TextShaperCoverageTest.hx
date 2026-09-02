package org.tiqian.shaping;

import org.tiqian.core.TextRange;
import org.tiqian.core.TextStyle;
import org.tiqian.font.FontPolicy.FontCandidate;
import org.tiqian.font.FontPolicy.FontDecision;
import org.tiqian.font.FontRole;
import org.tiqian.shaping.TextShaper.ShapingInput;
import org.tiqian.shaping.TextShaper.ShapingSource;
import org.tiqian.shaping.TextShaper.ExplainableStubTextShaper;
import org.tiqian.shaping.TextShaper.UnimplementedTextShaper;
import org.tiqian.shaping.TextShaper.ShapingSource;
import org.tiqian.shaping.TextShaper.ExplainableStubTextShaper;
import org.tiqian.shaping.TextShaper.UnimplementedTextShaper;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;

class TextShaperCoverageTest {
    private static function input(text:String, ?role:Null<FontRole>, ?displayText:Null<String>, ?features:Array<String>):ShapingInput {
        var actualRole = role == null ? FontRole.LatinText : role;
        var range = new TextRange(0, text.length);
        return new ShapingInput(text, range, new TextStyle(null, 16.0),
            new FontDecision(range, new FontCandidate("test-font", "test-font", actualRole), actualRole, "coverage-test"),
            displayText == null ? text : displayText, features == null ? [] : features);
    }

    private static function surrogateText(codes:Array<Int>):String {
        var result = "";
        var i = 0;
        while (i < codes.length) {
            result += String.fromCharCode(codes[i]);
            i++;
        }
        return result;
    }

    @:test public static function coversAllShapingSourceEnumEntries():Void {
        new TestTraceRecorder("TextShaperCoverageTest").section("coversAllShapingSourceEnumEntries");
        var sources:Array<ShapingSource> = Type.allEnums(ShapingSource);
        TracedAssertions.assertTrue(sources.indexOf(ShapingSource.Stub) >= 0);
        TracedAssertions.assertTrue(sources.indexOf(ShapingSource.JvmAwt) >= 0);
        TracedAssertions.assertTrue(sources.indexOf(ShapingSource.AndroidPaint) >= 0);
        TracedAssertions.assertTrue(sources.indexOf(ShapingSource.Skia) >= 0);
        TracedAssertions.assertTrue(sources.indexOf(ShapingSource.HarfBuzz) >= 0);
        TracedAssertions.assertTrue(sources.indexOf(ShapingSource.CoreText) >= 0);
        TracedAssertions.assertEqualsInt(6, sources.length);
        var i = 0;
        while (i < sources.length) {
            var source = sources[i];
            var name = Type.enumConstructor(source);
            TracedAssertions.assertEqualsRendered(name, Type.enumConstructor(Type.createEnum(ShapingSource, name)));
            i++;
        }
    }

    @:test public static function unimplementedTextShaperThrowsOnShape():Void {
        new TestTraceRecorder("TextShaperCoverageTest").section("unimplementedTextShaperThrowsOnShape");
        var error = TracedAssertions.assertFailsWith(null, function() new UnimplementedTextShaper().shape(input("test")));
        TracedAssertions.assertTrue(error.message.indexOf("platform-specific") >= 0);
    }

    @:test public static function explainableStubNominalAdvanceBranches():Void {
        new TestTraceRecorder("TextShaperCoverageTest").section("explainableStubNominalAdvanceBranches");
        var shaper = new ExplainableStubTextShaper();
        TracedAssertions.assertEqualsFloat(32.0, shaper.shape(input("\u2E3A", FontRole.CjkPunctuation)).clusters[0].advance);
        TracedAssertions.assertEqualsFloat(32.0, shaper.shape(input("——", FontRole.CjkPunctuation, "\u2E3A")).clusters[0].advance);
        TracedAssertions.assertEqualsFloat(8.0, shaper.shape(input(" ")).clusters[0].advance);
        TracedAssertions.assertEqualsFloat(24.0, shaper.shape(input("   ")).clusters[0].advance);
        var empty = shaper.shape(input(""));
        TracedAssertions.assertEqualsFloat(0.0, empty.clusters[0].advance);
        TracedAssertions.assertEqualsInt(1, empty.glyphRuns[0].glyphs.length);
        TracedAssertions.assertEqualsFloat(32.0, shaper.shape(input(" a")).clusters[0].advance);
        TracedAssertions.assertEqualsFloat(32.0, shaper.shape(input("a ")).clusters[0].advance);
    }

    @:test public static function surrogatePairHandlingInCodePointCount():Void {
        new TestTraceRecorder("TextShaperCoverageTest").section("surrogatePairHandlingInCodePointCount");
        var shaper = new ExplainableStubTextShaper();
        var one = shaper.shape(input(surrogateText([0xD83D, 0xDE00])));
        TracedAssertions.assertEqualsInt(1, one.decisions[0].glyphCount);
        TracedAssertions.assertEqualsFloat(16.0, one.clusters[0].advance);
        var two = shaper.shape(input(surrogateText([0xD83D, 0xDE00, 0xD840, 0xDC0B])));
        TracedAssertions.assertEqualsInt(2, two.decisions[0].glyphCount);
        TracedAssertions.assertEqualsFloat(32.0, two.clusters[0].advance);
        var high = shaper.shape(input(surrogateText([0xD83D])));
        TracedAssertions.assertEqualsInt(1, high.decisions[0].glyphCount);
        TracedAssertions.assertEqualsFloat(16.0, high.clusters[0].advance);
        var invalid = shaper.shape(input(surrogateText([0xD83D, 0x41])));
        TracedAssertions.assertEqualsInt(2, invalid.decisions[0].glyphCount);
        TracedAssertions.assertEqualsFloat(32.0, invalid.clusters[0].advance);
    }

    @:test public static function shapingInputWithFeaturesAndConstants():Void {
        new TestTraceRecorder("TextShaperCoverageTest").section("shapingInputWithFeaturesAndConstants");
        var inputValue = input("Test", null, null, ["fwid=1", "vert=1"]);
        TracedAssertions.assertEqualsStringArray(["fwid=1", "vert=1"], inputValue.openTypeFeatures);
        TracedAssertions.assertEqualsString("Test", inputValue.displayText);
        var result = new ExplainableStubTextShaper().shape(inputValue);
        TracedAssertions.assertEqualsInt(4, result.decisions[0].glyphCount);
        TracedAssertions.assertEqualsInt(4, result.decisions[0].glyphsWithoutInkBounds);
        TracedAssertions.assertEqualsString("ExplainableStubTextShaper:nominal-em-advance", result.decisions[0].reason);
        TracedAssertions.assertEqualsString(Type.enumConstructor(ShapingSource.Stub), result.decisions[0].source);
        TracedAssertions.assertNotNullRendered(TextShaper.UNVERIFIED_DISPLAY_SUBSTITUTION_COVERAGE_ISSUE != null,
            "'" + TextShaper.UNVERIFIED_DISPLAY_SUBSTITUTION_COVERAGE_ISSUE + "'");
        TracedAssertions.assertNotNullRendered(TextShaper.PLATFORM_MULTI_FACE_STRING_DRAW_ISSUE != null,
            "'" + TextShaper.PLATFORM_MULTI_FACE_STRING_DRAW_ISSUE + "'");
    }
}
