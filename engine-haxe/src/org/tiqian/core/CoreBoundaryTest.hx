package org.tiqian.core;

import org.tiqian.test.TestHelpers;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;

class CoreBoundaryTest {
    private static var testTrace:Null<TestTraceRecorder> = null;

    private static function currentTrace():TestTraceRecorder {
        if (testTrace == null) {
            testTrace = new TestTraceRecorder("CoreBoundaryTest");
        }
        return testTrace;
    }

    private static function interactionResult(text:String):LayoutResult {
        final clusters:Array<Cluster> = [
            new Cluster(new TextRange(0, 1), "a", "latin", 10.0, ("a"), 0.0, 0.0, 0.0),
            new Cluster(new TextRange(1, 2), "b", "latin", 10.0, ("b"), 0.0, 0.0, 0.0),
            new Cluster(new TextRange(2, 3), "c", "latin", 10.0, ("c"), 0.0, 0.0, 0.0)
        ];
        final line:LineBox = new LineBox(
            new TextRange(0, 3), new IntRange(0, 2), 15.0, 0.0, 20.0,
            30.0, 30.0, 30.0, 0.0, 0.0, LineEndReason.ParagraphEnd, 0.0, [], new LineDebugInfo(null, [])
        );
        final input:LayoutInput = new LayoutInput(
            new TiqianTextContent(text, [], [], [], []),
            new LayoutConstraints(100.0, Math.POSITIVE_INFINITY, 2147483647),
            new TextStyle(16.0, "zh-Hans", 400, false, 0.0, InlineAttachment.None, []), new ParagraphStyle(LastLineAlignment.Start, WritingMode.HorizontalTb, null, null, Ic.Zero, new MeasureAdaptiveFirstLineIndent(14.0, 1.0, 2.0), new LineLengthGrid(true, null), RubyLineHeightMode.PerLine, ParagraphStyle.DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM, ParagraphStyle.DEFAULT_EMPHASIS_DOT_GAP_EM), BuiltInLayoutProfiles.ClreqHorizontal, [], [], [], []
        );
        return new LayoutResult(input, new Size(30.0, 20.0), clusters, [], [line], new LayoutDebugInfo(null, [], [], [], [], []));
    }

    @:test
    public static function coerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEnd():Void {
        currentTrace().section("coerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEnd");
        TracedAssertions.assertEqualsInt(3, SourceInteractionBoundaries.coerceToInteractionBoundary("abc", 3, new TextRange(0, 3), SourceBoundaryBias.Backward));
    }

    @:test
    public static function coerceToInteractionBoundaryForwardReturnsNextBoundary():Void {
        currentTrace().section("coerceToInteractionBoundaryForwardReturnsNextBoundary");
        TracedAssertions.assertEqualsInt(3, SourceInteractionBoundaries.coerceToInteractionBoundary("abc", 3, new TextRange(0, 3), SourceBoundaryBias.Forward));
    }

    @:test
    public static function coerceToInteractionBoundaryNearestChoosesCloser():Void {
        currentTrace().section("coerceToInteractionBoundaryNearestChoosesCloser");
        TracedAssertions.assertEqualsInt(3, SourceInteractionBoundaries.coerceToInteractionBoundary("abcdef", 3, new TextRange(0, 6), SourceBoundaryBias.Nearest));
    }

    @:test
    public static function coerceToInteractionBoundaryWithSurrogatePair():Void {
        currentTrace().section("coerceToInteractionBoundaryWithSurrogatePair");
        final emoji:String = TestHelpers.surrogateText([0xD83D, 0xDE00]);
        final text:String = "a" + emoji + "b";
        TracedAssertions.assertEqualsInt(3, SourceInteractionBoundaries.coerceToInteractionBoundary(text, 3, new TextRange(0, text.length), SourceBoundaryBias.Nearest));
    }

    @:test
    public static function coerceToInteractionBoundaryWithInvalidSurrogatePair():Void {
        currentTrace().section("coerceToInteractionBoundaryWithInvalidSurrogatePair");
        final text:String = TestHelpers.surrogateText([0xD800]) + "A";
        TracedAssertions.assertEqualsInt(1, SourceInteractionBoundaries.coerceToInteractionBoundary(text, 1, new TextRange(0, text.length), SourceBoundaryBias.Nearest));
    }

    @:test
    public static function sourceGraphemeBoundariesWithHangulLeadingJamo():Void {
        currentTrace().section("sourceGraphemeBoundariesWithHangulLeadingJamo");
        final text:String = "\u1100\u1161\u11A8";
        final boundaries:Array<Int> = SourceInteractionBoundaries.sourceGraphemeBoundaries(text, new TextRange(0, 3));
        TracedAssertions.assertTrue(boundaries.indexOf(3) >= 0);
    }

    @:test
    public static function sourceGraphemeBoundariesWithHangulSyllable():Void {
        currentTrace().section("sourceGraphemeBoundariesWithHangulSyllable");
        final text:String = "\uAC00";
        final boundaries:Array<Int> = SourceInteractionBoundaries.sourceGraphemeBoundaries(text, new TextRange(0, 1));
        TracedAssertions.assertEqualsInt(2, boundaries.length);
        TracedAssertions.assertEqualsInt(0, boundaries[0]);
        TracedAssertions.assertEqualsInt(1, boundaries[boundaries.length - 1]);
    }

    @:test
    public static function sourceGraphemeBoundariesWithRegionalIndicator():Void {
        currentTrace().section("sourceGraphemeBoundariesWithRegionalIndicator");
        final text:String = TestHelpers.surrogateText([0xD83C, 0xDDE8, 0xD83C, 0xDDE6]);
        final boundaries:Array<Int> = SourceInteractionBoundaries.sourceGraphemeBoundaries(text, new TextRange(0, text.length));
        TracedAssertions.assertTrue(boundaries.indexOf(text.length) >= 0);
    }

    @:test
    public static function sourceGraphemeBoundariesWithEmojiZwjSequence():Void {
        currentTrace().section("sourceGraphemeBoundariesWithEmojiZwjSequence");
        final text:String = TestHelpers.surrogateText([0xD83D, 0xDC69, 0x200D, 0xD83D, 0xDC69]);
        final boundaries:Array<Int> = SourceInteractionBoundaries.sourceGraphemeBoundaries(text, new TextRange(0, text.length));
        TracedAssertions.assertEqualsInt(2, boundaries.length);
        TracedAssertions.assertEqualsInt(0, boundaries[0]);
    }

    @:test
    public static function sourceGraphemeBoundariesWithEmojiModifier():Void {
        currentTrace().section("sourceGraphemeBoundariesWithEmojiModifier");
        final text:String = TestHelpers.surrogateText([0xD83D, 0xDC69, 0xD83C, 0xDFFB]);
        final boundaries:Array<Int> = SourceInteractionBoundaries.sourceGraphemeBoundaries(text, new TextRange(0, text.length));
        TracedAssertions.assertTrue(boundaries.indexOf(text.length) >= 0);
    }

    @:test
    public static function sourceGraphemeBoundariesReturnsSingleBoundaryForEmptyText():Void {
        currentTrace().section("sourceGraphemeBoundariesReturnsSingleBoundaryForEmptyText");
        final boundaries:Array<Int> = SourceInteractionBoundaries.sourceGraphemeBoundaries("", new TextRange(0, 0));
        TracedAssertions.assertEqualsInt(1, boundaries.length);
        TracedAssertions.assertEqualsInt(0, boundaries[0]);
    }

    @:test
    public static function interactionBoundariesWithTextRange():Void {
        currentTrace().section("interactionBoundariesWithTextRange");
        final boundaries:Array<Int> = SourceInteractionBoundaries.interactionBoundaries("abc", new TextRange(1, 2));
        TracedAssertions.assertEqualsRendered("[1, 2]", renderInts(boundaries));
    }

    @:test
    public static function getSelectionOffsetForPositionReturnsStartOfFirstCluster():Void {
        currentTrace().section("getSelectionOffsetForPositionReturnsStartOfFirstCluster");
        final value:LayoutResult = interactionResult("abc");
        TracedAssertions.assertEqualsInt(0, LayoutQueries.getSelectionOffsetForPosition(value, 0.0, 10.0));
        TracedAssertions.assertEqualsInt(1, LayoutQueries.getSelectionOffsetForPosition(value, 10.0, 10.0));
        TracedAssertions.assertEqualsInt(2, LayoutQueries.getSelectionOffsetForPosition(value, 20.0, 10.0));
    }

    @:test
    public static function getSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClusters():Void {
        currentTrace().section("getSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClusters");
        final input:LayoutInput = new LayoutInput(
            new TiqianTextContent("", [], [], [], []),
            new LayoutConstraints(100.0, Math.POSITIVE_INFINITY, 2147483647),
            new TextStyle(16.0, "zh-Hans", 400, false, 0.0, InlineAttachment.None, []), new ParagraphStyle(LastLineAlignment.Start, WritingMode.HorizontalTb, null, null, Ic.Zero, new MeasureAdaptiveFirstLineIndent(14.0, 1.0, 2.0), new LineLengthGrid(true, null), RubyLineHeightMode.PerLine, ParagraphStyle.DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM, ParagraphStyle.DEFAULT_EMPHASIS_DOT_GAP_EM), BuiltInLayoutProfiles.ClreqHorizontal, [], [], [], []
        );
        final line:LineBox = new LineBox(
            new TextRange(0, 0), new IntRange(0, -1), 15.0, 0.0, 20.0,
            0.0, 0.0, 0.0, 0.0, 0.0, LineEndReason.ParagraphEnd, 0.0, [], new LineDebugInfo(null, [])
        );
        final value:LayoutResult = new LayoutResult(input, new Size(0.0, 20.0), [], [], [line], new LayoutDebugInfo(null, [], [], [], [], []));
        TracedAssertions.assertEqualsInt(0, LayoutQueries.getSelectionOffsetForPosition(value, 5.0, 10.0));
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

    public static function flushTestTrace():Void {
        currentTrace().flush();
    }
}
