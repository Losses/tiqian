package org.tiqian.core;

import org.tiqian.test.TestHelpers;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;

class SourceInteractionBoundariesCoverageTest {
    private static var testTrace:Null<TestTraceRecorder> = null;

    private static function currentTrace():TestTraceRecorder {
        if (testTrace == null) {
            testTrace = new TestTraceRecorder("SourceInteractionBoundariesCoverageTest");
        }
        return testTrace;
    }

    private static function boundaries(text:String):Array<Int> {
        return SourceInteractionBoundaries.interactionBoundaries(text, new TextRange(0, text.length));
    }

    private static function assertBoundaries(expected:String, text:String):Void {
        TracedAssertions.assertEqualsRendered(expected, renderInts(boundaries(text)));
    }

    private static function renderInts(values:Array<Int>):String {
        var output:String = "[";
        var index:Int = 0;
        while (index < values.length) {
            if (index > 0) {
                output += ", ";
            }
            output += Std.string(values[index]);
            index += 1;
        }
        return output + "]";
    }

    @:test
    public static function crlfStaysOneUnit():Void {
        currentTrace().section("crlfStaysOneUnit");
        assertBoundaries("[0, 2]", "\r\n");
        assertBoundaries("[0, 1]", "\r");
        assertBoundaries("[0, 1, 2]", "a\n");
    }

    @:test
    public static function regionalIndicatorsPairUp():Void {
        currentTrace().section("regionalIndicatorsPairUp");
        assertBoundaries("[0, 4]", TestHelpers.surrogateText([0xD83C, 0xDDE6, 0xD83C, 0xDDE8]));
        assertBoundaries("[0, 4, 6]", TestHelpers.surrogateText([0xD83C, 0xDDE6, 0xD83C, 0xDDE6, 0xD83C, 0xDDE6]));
        assertBoundaries("[0, 2, 3]", TestHelpers.surrogateText([0xD83C, 0xDDE6]) + "A");
        assertBoundaries("[0, 2]", TestHelpers.surrogateText([0xD83C, 0xDDE6]));
    }

    @:test
    public static function hangulJamoRunsMergeIntoSyllableBlocks():Void {
        currentTrace().section("hangulJamoRunsMergeIntoSyllableBlocks");
        assertBoundaries("[0, 3]", "\u1100\u1100\u1161");
        assertBoundaries("[0, 3]", "\u1100\u1161\u11A8");
        assertBoundaries("[0, 4]", "\u1100\u1161\u11A8\u11A8");
        assertBoundaries("[0, 1, 2]", "\u1100A");
        assertBoundaries("[0, 2, 3]", "\u1100\u1161A");
        assertBoundaries("[0, 2]", "\uA960\u1161");
        assertBoundaries("[0, 2]", "\u1100\uD7B0");
        assertBoundaries("[0, 3]", "\u1100\u1161\uD7CB");
    }

    @:test
    public static function precomposedHangulSyllablesAbsorbJamo():Void {
        currentTrace().section("precomposedHangulSyllablesAbsorbJamo");
        assertBoundaries("[0, 3]", "\uAC00\u1161\u11A8");
        assertBoundaries("[0, 2]", "\uAC01\u11A8");
        assertBoundaries("[0, 1, 2]", "\uAC01A");
        assertBoundaries("[0, 2]", "\uAC00\u11A8");
    }

    @:test
    public static function extendersAttachToThePrecedingUnit():Void {
        currentTrace().section("extendersAttachToThePrecedingUnit");
        assertBoundaries("[0, 2]", "a\u0301");
        assertBoundaries("[0, 2]", "a\uFE0F");
        assertBoundaries("[0, 3]", "a" + TestHelpers.surrogateText([0xDB40, 0xDD00]));
        final scotland:String = TestHelpers.surrogateText([
            0xD83C, 0xDFF4, 0xDB40, 0xDC67, 0xDB40, 0xDC62,
            0xDB40, 0xDC65, 0xDB40, 0xDC6E, 0xDB40, 0xDC67
        ]);
        assertBoundaries("[0, 12]", scotland);
        assertBoundaries("[0, 2]", "\uAC00\u200C");
        assertBoundaries("[0, 1, 2]", "aA");
    }

    @:test
    public static function bandEdgesAndGapsExerciseEveryRangeArm():Void {
        currentTrace().section("bandEdgesAndGapsExerciseEveryRangeArm");
        assertBoundaries("[0, 1, 2]", "\rA");
        assertBoundaries("[0, 1, 2]", "a\u1100");
        assertBoundaries("[0, 2, 3]", "\u1100\u1161\uE000");
        assertBoundaries("[0, 1, 2]", TestHelpers.surrogateText([0xD800, 0xE000]));
        assertBoundaries("[0, 1, 3]", "a" + TestHelpers.surrogateText([0xDB40, 0xDDF0]));
        assertBoundaries("[0, 1, 3]", "a" + TestHelpers.surrogateText([0xDB40, 0xDCA0]));
        assertBoundaries("[0, 2, 3]", TestHelpers.surrogateText([0xD83D, 0xDC4D]) + "\u7532");
        assertBoundaries("[0, 2, 4]", TestHelpers.surrogateText([0xD83D, 0xDC4D, 0xD83D, 0xDE00]));
    }

    @:test
    public static function emojiModifiersOnlyAttachToBases():Void {
        currentTrace().section("emojiModifiersOnlyAttachToBases");
        assertBoundaries("[0, 4]", TestHelpers.surrogateText([0xD83D, 0xDC4D, 0xD83C, 0xDFFB]));
        assertBoundaries("[0, 5]", TestHelpers.surrogateText([0xD83D, 0xDC4D, 0xD83C, 0xDFFB]) + "\uFE0F");
        assertBoundaries("[0, 1, 3]", "a" + TestHelpers.surrogateText([0xD83C, 0xDFFB]));
        assertBoundaries("[0, 2]", TestHelpers.surrogateText([0xD83D, 0xDC4D]));
    }

    @:test
    public static function zwjChainsJoinOnlyExtendedPictographic():Void {
        currentTrace().section("zwjChainsJoinOnlyExtendedPictographic");
        assertBoundaries("[0, 8]", TestHelpers.surrogateText([0xD83D, 0xDC69, 0x200D, 0xD83D, 0xDC69, 0x200D, 0xD83D, 0xDC66]));
        assertBoundaries("[0, 2]", "a\u200D");
        assertBoundaries("[0, 3, 4]", TestHelpers.surrogateText([0xD83D, 0xDC69]) + "\u200Da");
        assertBoundaries("[0, 2, 3]", "a\u200Da");
        assertBoundaries("[0, 7]", TestHelpers.surrogateText([0xD83D, 0xDC4D, 0x200D, 0xD83D, 0xDC4D, 0xD83C, 0xDFFB]));
    }

    @:test
    public static function unpairedSurrogatesFallBackToSingleUnits():Void {
        currentTrace().section("unpairedSurrogatesFallBackToSingleUnits");
        assertBoundaries("[0, 1, 2]", TestHelpers.surrogateText([0x61, 0xD800]));
        assertBoundaries("[0, 1, 2, 3]", TestHelpers.surrogateText([0x61, 0xD800, 0x41]));
        assertBoundaries("[0, 2, 3]", TestHelpers.surrogateText([0xD83D, 0xDE00]) + "A");
    }

    @:test
    public static function codePointAtCompatCoversEverySurrogateCase():Void {
        currentTrace().section("codePointAtCompatCoversEverySurrogateCase");
        TracedAssertions.assertEqualsInt(97, SourceInteractionBoundaries.codePointAtCompat("a", 0, 1));
        TracedAssertions.assertEqualsInt(0x1F600, SourceInteractionBoundaries.codePointAtCompat(TestHelpers.surrogateText([0xD83D, 0xDE00]), 0, 2));
        TracedAssertions.assertEqualsInt(0xD800, SourceInteractionBoundaries.codePointAtCompat(TestHelpers.surrogateText([0x61, 0xD800]), 1, 2));
        TracedAssertions.assertEqualsInt(0xD800, SourceInteractionBoundaries.codePointAtCompat(TestHelpers.surrogateText([0x61, 0xD800, 0x41]), 1, 3));
    }

    @:test
    public static function rangeBoundariesRespectTheRequestedWindow():Void {
        currentTrace().section("rangeBoundariesRespectTheRequestedWindow");
        TracedAssertions.assertEqualsRendered("[1, 2, 3]", renderInts(SourceInteractionBoundaries.interactionBoundaries("abcd", new TextRange(1, 3))));
        TracedAssertions.assertEqualsRendered("[2]", renderInts(SourceInteractionBoundaries.interactionBoundaries("ab", new TextRange(5, 9))));
        final emojiB:String = TestHelpers.surrogateText([0xD83D, 0xDE00]) + "b";
        TracedAssertions.assertEqualsRendered("[0, 2, 3]", renderInts(SourceInteractionBoundaries.sourceGraphemeBoundaries(emojiB, new TextRange(0, 3))));
    }

    @:test
    public static function coercionHonoursEveryBiasAndEdgeCase():Void {
        currentTrace().section("coercionHonoursEveryBiasAndEdgeCase");
        final family:String = TestHelpers.surrogateText([
            0xD83D, 0xDC68, 0x200D, 0xD83D, 0xDC69, 0x200D,
            0xD83D, 0xDC67, 0x200D, 0xD83D, 0xDC67
        ]);
        TracedAssertions.assertEqualsInt(11, family.length);
        final familyRange:TextRange = new TextRange(0, 11);
        TracedAssertions.assertEqualsInt(0, SourceInteractionBoundaries.coerceToInteractionBoundary(family, 2, familyRange, SourceBoundaryBias.Nearest));
        TracedAssertions.assertEqualsInt(0, SourceInteractionBoundaries.coerceToInteractionBoundary(family, 2, familyRange, SourceBoundaryBias.Backward));
        TracedAssertions.assertEqualsInt(11, SourceInteractionBoundaries.coerceToInteractionBoundary(family, 2, familyRange, SourceBoundaryBias.Forward));
        final emojiB:String = TestHelpers.surrogateText([0xD83D, 0xDE00]) + "b";
        final emojiRange:TextRange = new TextRange(0, 3);
        TracedAssertions.assertEqualsInt(2, SourceInteractionBoundaries.coerceToInteractionBoundary(emojiB, 2, emojiRange, SourceBoundaryBias.Nearest));
        TracedAssertions.assertEqualsInt(3, SourceInteractionBoundaries.coerceToInteractionBoundary(emojiB, 9, emojiRange, SourceBoundaryBias.Backward));
        TracedAssertions.assertEqualsInt(0, SourceInteractionBoundaries.coerceToInteractionBoundary(emojiB, -1, emojiRange, SourceBoundaryBias.Forward));
    }

    public static function flushTestTrace():Void {
        currentTrace().flush();
    }
}
