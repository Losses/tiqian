package org.tiqian.core;

import org.tiqian.test.TestHelpers;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;
import org.tiqian.test.trace.TestTraceRender;
import std.StringBuf;

class LayoutQueriesResidualCoverageTest {
    private static var testTrace:Null<TestTraceRecorder> = null;

    private static function currentTrace():TestTraceRecorder {
        if (testTrace == null) {
            testTrace = new TestTraceRecorder("LayoutQueriesResidualCoverageTest");
        }
        return testTrace;
    }

    private static function cluster(range:TextRange, text:String, advance:Float):Cluster {
        return new Cluster(range, text, "test", advance, (text), 0.0, 0.0, 0.0);
    }

    private static function line(range:TextRange, clusterStart:Int, clusterEnd:Int, top:Float, bottom:Float, baseline:Float, indent:Float, width:Float):LineBox {
        return new LineBox(
            range, new IntRange(clusterStart, clusterEnd), baseline, top, bottom,
            width, width, width, 0.0, indent, LineEndReason.ParagraphEnd, 0.0, [], new LineDebugInfo(null, [])
        );
    }

    private static function style(fontSize:Float):TextStyle {
        return new TextStyle(fontSize, "zh-Hans", 400, false, 0.0, InlineAttachment.None, []);
    }

    private static function emptyDebug():LayoutDebugInfo {
        return new LayoutDebugInfo(null, [], [], [], [], []);
    }

    private static function result(
        text:String,
        clusters:Array<Cluster>,
        lines:Array<LineBox>,
        glyphRuns:Array<GlyphRun>,
        spans:Array<TextSpan>,
        inlineObjects:Array<InlineObjectSpan>,
        debug:LayoutDebugInfo,
        textStyle:TextStyle
    ):LayoutResult {
        final content:TiqianTextContent = new TiqianTextContent(text, spans, [], [], []);
        final input:LayoutInput = new LayoutInput(
            content,
            new LayoutConstraints(100.0, Math.POSITIVE_INFINITY, 2147483647),
            textStyle,
            new ParagraphStyle(LastLineAlignment.Start, WritingMode.HorizontalTb, null, null, Ic.Zero, new MeasureAdaptiveFirstLineIndent(14.0, 1.0, 2.0), new LineLengthGrid(true, null), RubyLineHeightMode.PerLine, ParagraphStyle.DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM, ParagraphStyle.DEFAULT_EMPHASIS_DOT_GAP_EM), BuiltInLayoutProfiles.ClreqHorizontal, [], [], [], inlineObjects
        );
        return new LayoutResult(input, new Size(30.0, 40.0), clusters, glyphRuns, lines, debug);
    }

    private static function segment(
        range:TextRange,
        role:RichTextRole,
        paint:RichTextPaint,
        lineIndex:Int,
        spanRange:TextRange,
        left:Float,
        top:Float,
        right:Float,
        bottom:Float,
        baseline:Float
    ):RichTextLineSegment {
        return new RichTextLineSegment(
            new RichTextSpan(spanRange, role, paint), lineIndex, range,
            left, top, right, bottom, baseline
        );
    }

    private static function plainSegment(range:TextRange):RichTextLineSegment {
        return segment(range, RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, range, 0.0, 0.0, 20.0, 20.0, 15.0);
    }

    private static function renderSegments(values:Array<RichTextLineSegment>):String {
        final output = new StringBuf();
        output.add("[");
        var index:Int = 0;
        while (index < values.length) {
            if (index > 0) {
                output.add(", ");
            }
            output.add(TestTraceRender.cap(values[index].toString()));
            index += 1;
        }
        output.add("]");
        return output.toString();
    }

    private static function renderRects(values:Array<Rect>):String {
        final output = new StringBuf();
        output.add("[");
        var index:Int = 0;
        while (index < values.length) {
            if (index > 0) {
                output.add(", ");
            }
            output.add(values[index].toString());
            index += 1;
        }
        output.add("]");
        return output.toString();
    }

    private static function nan():Float {
        return 0.0 / 0.0;
    }

    private static function metric(range:TextRange, metricBox:String, ascent:Float, descent:Float, baselineClass:String):MetricDecisionInfo {
        return new MetricDecisionInfo(
            range, "ab", "body", "test", 8.0, 2.0, 0.0, "stub",
            ascent, descent, baselineClass, metricBox, "normalized", "test"
        );
    }

    private static function geometry(range:TextRange, sourceText:String, leading:Float, leadingConsumed:Float, trailing:Float, trailingConsumed:Float):ClusterGeometryDecisionInfo {
        return new ClusterGeometryDecisionInfo(
            range, sourceText, sourceText, 10.0,
            10.0 - leading - trailing,
            leading, leadingConsumed, trailing, trailingConsumed,
            0.0, 10.0, "test", "test", 0.0, 0.0, null
        );
    }

    private static function metricBounds(decisionRange:TextRange):Array<Float> {
        final decision:MetricDecisionInfo = metric(decisionRange, "IdeographicEmBox", 7.0, 3.0, "ideographic");
        final content:LayoutResult = result(
            "ab", [cluster(new TextRange(0, 2), "ab", 20.0)],
            [line(new TextRange(0, 2), 0, 0, 0.0, 20.0, 15.0, 0.0, 20.0)],
            [], [], [], new LayoutDebugInfo(null, [decision], [], [], [], []), style(10.0)
        );
        final box:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(
            content,
            [segment(new TextRange(0, 2), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(0, 2), 0.0, 0.0, 20.0, 20.0, 15.0)]
        );
        return [box[0].top, box[0].bottom];
    }

    private static function inkWithBounds(bounds:Rect):Null<Rect> {
        final clusters:Array<Cluster> = [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0)];
        final content:LayoutResult = result(
            "ab", clusters,
            [line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0)],
            [new GlyphRun(new TextRange(0, 2), "test", [new Glyph(1, new TextRange(0, 1), 10.0, 0.0, 0.0, null, bounds, null, null)], 20.0, [])],
            [], [], emptyDebug(), style(10.0)
        );
        return LayoutQueries.glyphInkBounds(content);
    }

    @:test
    public static function cornerRadiiPredicatesCoverEveryComparison():Void {
        currentTrace().section("cornerRadiiPredicatesCoverEveryComparison");
        TracedAssertions.assertTrue(new RichTextCornerRadii(0.0, 0.0, 0.0, 0.0).isSquare);
        TracedAssertions.assertTrue(!new RichTextCornerRadii(1.0, 0.0, 0.0, 0.0).isSquare);
        TracedAssertions.assertTrue(!new RichTextCornerRadii(0.0, 1.0, 0.0, 0.0).isSquare);
        TracedAssertions.assertTrue(!new RichTextCornerRadii(0.0, 0.0, 1.0, 0.0).isSquare);
        TracedAssertions.assertTrue(!new RichTextCornerRadii(0.0, 0.0, 0.0, 1.0).isSquare);
        TracedAssertions.assertTrue(new RichTextCornerRadii(2.0, 2.0, 2.0, 2.0).isUniform);
        TracedAssertions.assertTrue(!new RichTextCornerRadii(1.0, 2.0, 2.0, 2.0).isUniform);
        TracedAssertions.assertTrue(!new RichTextCornerRadii(2.0, 1.0, 2.0, 2.0).isUniform);
        TracedAssertions.assertTrue(!new RichTextCornerRadii(2.0, 2.0, 1.0, 2.0).isUniform);
        TracedAssertions.assertTrue(!new RichTextCornerRadii(2.0, 2.0, 2.0, 1.0).isUniform);
    }

    @:test
    public static function resolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuations():Void {
        currentTrace().section("resolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuations");
        final continuing:RichTextLineSegment = segment(
            new TextRange(1, 2), RichTextRole.Background,
            new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 6.0, 2.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0),
            0, new TextRange(0, 3), 0.0, 0.0, 30.0, 10.0, 15.0
        );
        TracedAssertions.assertFailsWith(null, function():Void {
            LayoutQueries.resolvedBackgroundCornerRadii(continuing, -1.0);
        });
        TracedAssertions.assertFailsWith(null, function():Void {
            LayoutQueries.resolvedBackgroundCornerRadii(continuing, nan());
        });
        final resolved:RichTextCornerRadii = LayoutQueries.resolvedBackgroundCornerRadii(continuing, 0.0);
        TracedAssertions.assertEqualsFloat(2.0, resolved.topLeft);
        TracedAssertions.assertEqualsFloat(2.0, resolved.topRight);
        TracedAssertions.assertEqualsFloat(2.0, resolved.bottomRight);
        TracedAssertions.assertEqualsFloat(2.0, resolved.bottomLeft);
    }

    @:test
    public static function copyProjectionAppendsFullySelectedAnnotationsOnly():Void {
        currentTrace().section("copyProjectionAppendsFullySelectedAnnotationsOnly");
        final debug:LayoutDebugInfo = new LayoutDebugInfo(
            null, [], [], [],
            [new RubyDecisionInfo(new TextRange(0, 2), "zhù", 0, 10.0, 12.0, 6.0, 0.0, 0.0, 0.0, 12.0, [], 400, "zh-Hans", [])],
            [new BopomofoDecisionInfo(new TextRange(2, 4), "ㄋㄧˇ", 0, [], [], 400, "zh-Hans")]
        );
        final content:LayoutResult = result("abcd", [], [], [], [], [], debug, style(10.0));
        TracedAssertions.assertEqualsString("", LayoutQueries.getTextForCopy(content, new TextRange(1, 1)));
        TracedAssertions.assertEqualsString("ab（zhù）c", LayoutQueries.getTextForCopy(content, new TextRange(0, 3)));
        TracedAssertions.assertEqualsString("ab（zhù）cd（ㄋㄧˇ）", LayoutQueries.getTextForCopy(content, new TextRange(0, 4)));
        TracedAssertions.assertEqualsString("d", LayoutQueries.getTextForCopy(content, new TextRange(3, 4)));
    }

    @:test
    public static function positionedClustersByLineRejectsForeignLines():Void {
        currentTrace().section("positionedClustersByLineRejectsForeignLines");
        final owned:LineBox = line(new TextRange(0, 2), 0, 0, 0.0, 20.0, 15.0, 0.0, 10.0);
        final foreign:LineBox = line(new TextRange(0, 2), 0, 0, 99.0, 119.0, 114.0, 0.0, 10.0);
        final content:LayoutResult = result("ab", [cluster(new TextRange(0, 2), "ab", 20.0)], [owned], [], [], [], emptyDebug(), style(10.0));
        TracedAssertions.assertEqualsInt(1, LayoutQueries.positionedClustersForLine(content, owned).length);
        final error:TiqianIllegalArgumentException = TracedAssertions.assertFailsWith(null, function():Void {
            LayoutQueries.positionedClustersForLine(content, foreign);
        });
        TracedAssertions.assertTrue(error.message.indexOf("must belong") >= 0, error.message);
    }

    @:test
    public static function glyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInk():Void {
        currentTrace().section("glyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInk");
        final clusters:Array<Cluster> = [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0)];
        final runs:Array<GlyphRun> = [new GlyphRun(
            new TextRange(0, 2), "test",
            [
                new Glyph(1, new TextRange(0, 1), 10.0, 0.0, 0.0, null, new Rect(0.0, 2.0, 8.0, 12.0), null, null),
                new Glyph(2, new TextRange(0, 1), 10.0, 0.0, 0.0, null, null, null, null),
                new Glyph(3, new TextRange(5, 6), 10.0, 0.0, 0.0, null, new Rect(0.0, 0.0, 1.0, 1.0), null, null)
            ], 20.0, []
        )];
        final content:LayoutResult = result("ab", clusters, [line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0)], runs, [], [], emptyDebug(), style(10.0));
        TracedAssertions.assertEqualsRendered(new Rect(0.0, 17.0, 8.0, 27.0).toString(), LayoutQueries.glyphInkBounds(content).toString());
        final noInk:LayoutResult = result("ab", clusters, [line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0)], [], [], [], emptyDebug(), style(10.0));
        final noInkBounds = LayoutQueries.glyphInkBounds(noInk);
        TracedAssertions.assertNullRendered(noInkBounds == null, noInkBounds == null ? "-" : noInkBounds.toString());
    }

    @:test
    public static function emptyLineResultsShortCircuitEveryQuery():Void {
        currentTrace().section("emptyLineResultsShortCircuitEveryQuery");
        final content:LayoutResult = result("ab", [], [], [], [], [], emptyDebug(), style(10.0));
        TracedAssertions.assertEqualsInt(-1, LayoutQueries.getLineForOffset(content, 0));
        TracedAssertions.assertEqualsRendered(new Rect(0.0, 0.0, 0.0, 0.0).toString(), LayoutQueries.getBoundingBox(content, 0).toString());
        TracedAssertions.assertEqualsRendered(new Rect(0.0, 0.0, 0.0, 0.0).toString(), LayoutQueries.getCursorRect(content, 0).toString());
        TracedAssertions.assertEqualsInt(0, LayoutQueries.getOffsetForPosition(content, 5.0, 5.0));
        TracedAssertions.assertEqualsInt(0, LayoutQueries.getSelectionOffsetForPosition(content, 5.0, 5.0));
        TracedAssertions.assertEqualsRendered("[]", renderRects(LayoutQueries.getBoundingBoxes(content, new TextRange(0, 2))));
        final noWordBoundary = LayoutQueries.getSelectionWordBoundaryForPosition(content, 5.0, 5.0);
        TracedAssertions.assertNullRendered(noWordBoundary == null, noWordBoundary == null ? "-" : noWordBoundary.toString());
        TracedAssertions.assertEqualsRendered("[]", renderSegments(LayoutQueries.positionedRichTextSegments(content, [new RichTextSpan(new TextRange(0, 1), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0))])));
        TracedAssertions.assertEqualsRendered("[]", renderSegments(LayoutQueries.trimmedRichTextDecorationSegments(content, [])));
        TracedAssertions.assertEqualsRendered("[]", renderSegments(LayoutQueries.richTextBackgroundSegments(content, [])));
    }

    @:test
    public static function boundingBoxFallsBackToTheCursorRectAtClusterGaps():Void {
        currentTrace().section("boundingBoxFallsBackToTheCursorRectAtClusterGaps");
        final content:LayoutResult = result(
            "abc",
            [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(2, 3), "c", 10.0)],
            [line(new TextRange(0, 3), 0, 1, 0.0, 20.0, 15.0, 0.0, 10.0)],
            [], [], [], emptyDebug(), style(10.0)
        );
        TracedAssertions.assertEqualsRendered(new Rect(10.0, 0.0, 11.0, 20.0).toString(), LayoutQueries.getBoundingBox(content, 1).toString());
        TracedAssertions.assertEqualsRendered(new Rect(20.0, 0.0, 21.0, 20.0).toString(), LayoutQueries.getBoundingBox(content, 3).toString());
        TracedAssertions.assertEqualsRendered("[]", renderRects(LayoutQueries.getBoundingBoxes(content, new TextRange(3, 5))));
    }

    @:test
    public static function richTextSegmentsSplitOnLineBreaksAndClusterGaps():Void {
        currentTrace().section("richTextSegmentsSplitOnLineBreaksAndClusterGaps");
        final content:LayoutResult = result(
            "abcd",
            [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(2, 3), "c", 10.0), cluster(new TextRange(3, 4), "d", 10.0)],
            [
                line(new TextRange(0, 3), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0),
                line(new TextRange(3, 4), 2, 2, 20.0, 40.0, 35.0, 0.0, 10.0)
            ], [], [], [], emptyDebug(), style(10.0)
        );
        final span:RichTextSpan = new RichTextSpan(new TextRange(0, 4), RichTextRole.Underline, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0));
        final split:Array<RichTextLineSegment> = LayoutQueries.positionedRichTextSegments(content, [span]);
        TracedAssertions.assertEqualsInt(3, split.length);
        TracedAssertions.assertEqualsRendered(new TextRange(0, 1).toString(), split[0].range.toString());
        TracedAssertions.assertEqualsRendered(new TextRange(2, 3).toString(), split[1].range.toString());
        TracedAssertions.assertEqualsRendered(new TextRange(3, 4).toString(), split[2].range.toString());
        TracedAssertions.assertEqualsInt(0, split[0].lineIndex);
        TracedAssertions.assertEqualsInt(0, split[1].lineIndex);
        TracedAssertions.assertEqualsInt(1, split[2].lineIndex);
        TracedAssertions.assertTrue(LayoutQueries.positionedRichTextSegments(content, [new RichTextSpan(new TextRange(5, 8), RichTextRole.Underline, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0))]).length == 0);
    }

    @:test
    public static function richTextSegmentsSkipZeroLengthClustersBetweenSlices():Void {
        currentTrace().section("richTextSegmentsSkipZeroLengthClustersBetweenSlices");
        final content:LayoutResult = result(
            "ab",
            [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 1), "", 0.0), cluster(new TextRange(1, 2), "b", 10.0)],
            [line(new TextRange(0, 2), 0, 2, 0.0, 20.0, 15.0, 0.0, 20.0)],
            [], [], [], emptyDebug(), style(10.0)
        );
        final segments:Array<RichTextLineSegment> = LayoutQueries.positionedRichTextSegments(content, [new RichTextSpan(new TextRange(0, 2), RichTextRole.Underline, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0))]);
        TracedAssertions.assertEqualsInt(1, segments.length);
        TracedAssertions.assertEqualsRendered(new TextRange(0, 2).toString(), segments[0].range.toString());
        TracedAssertions.assertEqualsFloat(0.0, segments[0].left);
        TracedAssertions.assertEqualsFloat(20.0, segments[0].right);
    }

    @:test
    public static function trimmedDecorationSegmentsKeepOnlyDecorationRoles():Void {
        currentTrace().section("trimmedDecorationSegmentsKeepOnlyDecorationRoles");
        final content:LayoutResult = result("ab", [], [], [], [], [], emptyDebug(), style(10.0));
        final decoration:RichTextLineSegment = segment(new TextRange(0, 2), RichTextRole.Underline, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(0, 2), 0.0, 0.0, 20.0, 20.0, 15.0);
        TracedAssertions.assertEqualsRendered(renderSegments([decoration]), renderSegments(LayoutQueries.trimmedRichTextDecorationSegments(content, [decoration])));
        TracedAssertions.assertTrue(LayoutQueries.trimmedRichTextDecorationSegments(content, [plainSegment(new TextRange(0, 2))]).length == 0);
    }

    @:test
    public static function backgroundSegmentsPassThroughUnmatchableSegments():Void {
        currentTrace().section("backgroundSegmentsPassThroughUnmatchableSegments");
        final content:LayoutResult = result("ab", [cluster(new TextRange(0, 1), "a", 10.0)], [line(new TextRange(0, 2), 0, 0, 0.0, 20.0, 15.0, 0.0, 10.0)], [], [], [], emptyDebug(), style(10.0));
        final far:RichTextLineSegment = plainSegment(new TextRange(10, 12));
        TracedAssertions.assertEqualsRendered(renderSegments([far]), renderSegments(LayoutQueries.richTextBackgroundSegments(content, [far])));
        final orphan:RichTextLineSegment = segment(new TextRange(0, 1), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 5, new TextRange(0, 1), 0.0, 0.0, 20.0, 20.0, 15.0);
        TracedAssertions.assertEqualsRendered(renderSegments([orphan]), renderSegments(LayoutQueries.richTextBackgroundSegments(content, [orphan])));
        final underline:RichTextLineSegment = segment(new TextRange(0, 1), RichTextRole.Underline, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(0, 1), 0.0, 0.0, 20.0, 20.0, 15.0);
        TracedAssertions.assertTrue(LayoutQueries.richTextBackgroundSegments(content, [underline]).length == 0);
    }

    @:test
    public static function backgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvances():Void {
        currentTrace().section("backgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvances");
        final clusters:Array<Cluster> = [cluster(new TextRange(0, 1), "，", 10.0), cluster(new TextRange(1, 2), "字", 10.0)];
        final glue:ClusterGeometryDecisionInfo = new ClusterGeometryDecisionInfo(
            new TextRange(0, 1), "，", "，", 10.0, 5.0, 4.0, 1.0, 4.0, 1.0, 0.0, 10.0, "test", "test", 0.0, 0.0, null
        );
        final runs:Array<GlyphRun> = [new GlyphRun(new TextRange(1, 2), "test", [new Glyph(9, new TextRange(1, 2), 9.0, 1.0, 0.0, null, null, null, null)], 10.0, [])];
        final content:LayoutResult = result("，字", clusters, [line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0)], runs, [], [], new LayoutDebugInfo(null, [], [glue], [], [], []), style(10.0));
        final full:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [segment(new TextRange(0, 2), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(0, 2), 0.0, 0.0, 20.0, 20.0, 15.0)]);
        TracedAssertions.assertEqualsFloat(3.0, full[0].left);
        TracedAssertions.assertEqualsFloat(20.0, full[0].right);
        TracedAssertions.assertEqualsFloat(15.0 - 10.0 * 0.88, full[0].top);
        TracedAssertions.assertEqualsFloat(15.0 + 10.0 * 0.12, full[0].bottom);
        final headPaint:RichTextPaint = new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(5.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0);
        final head:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [segment(new TextRange(0, 2), RichTextRole.Background, headPaint, 0, new TextRange(0, 3), 0.0, 0.0, 20.0, 20.0, 15.0)]);
        TracedAssertions.assertEqualsFloat(3.0, head[0].left);
        TracedAssertions.assertEqualsFloat(20.0, head[0].right);
        final continuation:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [segment(new TextRange(1, 2), RichTextRole.Background, headPaint, 0, new TextRange(0, 3), 10.0, 0.0, 20.0, 20.0, 15.0)]);
        TracedAssertions.assertEqualsFloat(10.0, continuation[0].left);
        TracedAssertions.assertEqualsFloat(20.0, continuation[0].right);
    }

    @:test
    public static function markedFacesUseMetricDecisionsWhenTheyCoverTheCluster():Void {
        currentTrace().section("markedFacesUseMetricDecisionsWhenTheyCoverTheCluster");
        final decision:MetricDecisionInfo = metric(new TextRange(0, 2), "IdeographicEmBox", 7.0, 3.0, "ideographic");
        final content:LayoutResult = result("ab", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0)], [line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0)], [], [], [], new LayoutDebugInfo(null, [decision], [], [], [], []), style(10.0));
        final box:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [segment(new TextRange(0, 2), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(0, 2), 0.0, 0.0, 20.0, 20.0, 15.0)]);
        TracedAssertions.assertEqualsFloat(8.0, box[0].top);
        TracedAssertions.assertEqualsFloat(18.0, box[0].bottom);
    }

    @:test
    public static function uniformTextStyleFallsBackWhenEveryMetricFieldDiffers():Void {
        currentTrace().section("uniformTextStyleFallsBackWhenEveryMetricFieldDiffers");
        final base:TextStyle = style(10.0);
        final variants:Array<TextStyle> = [
            new TextStyle(10.0, "zh-Hans", 400, false, 0.0, InlineAttachment.None, ["other"]),
            new TextStyle(11.0, "zh-Hans", 400, false, 0.0, InlineAttachment.None, []),
            new TextStyle(10.0, "ja-JP", 400, false, 0.0, InlineAttachment.None, []),
            new TextStyle(10.0, "zh-Hans", 700, false, 0.0, InlineAttachment.None, []),
            new TextStyle(10.0, "zh-Hans", 400, true, 0.0, InlineAttachment.None, []),
            new TextStyle(10.0, "zh-Hans", 400, false, 2.0, InlineAttachment.None, [])
        ];
        final clusters:Array<Cluster> = [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0)];
        final decision:MetricDecisionInfo = metric(new TextRange(1, 2), "LatinBox", 9.0, 1.0, "latin");
        final paint:RichTextPaint = new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.UniformTextStyle, RichTextBackgroundDrawStyle.Fill), 0.0);
        var index:Int = 0;
        while (index < variants.length) {
            final variant:TextStyle = variants[index];
            final content:LayoutResult = result("ab", clusters, [line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0)], [], [new TextSpan(new TextRange(0, 1), variant)], [], new LayoutDebugInfo(null, [decision], [], [], [], []), base);
            final box:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [segment(new TextRange(0, 2), RichTextRole.Background, paint, 0, new TextRange(0, 2), 0.0, 0.0, 20.0, 20.0, 15.0)]);
            final message:String = "variant=" + variant.toString();
            TracedAssertions.assertEqualsFloat(15.0 - variant.fontSize * 0.88, box[0].top, message);
            TracedAssertions.assertEqualsFloat(15.0 + variant.fontSize * 0.12, box[0].bottom, message);
            index += 1;
        }
    }

    @:test
    public static function uniformTextStylePrefersIdeographicMetricsThenAnyMatchingFace():Void {
        currentTrace().section("uniformTextStylePrefersIdeographicMetricsThenAnyMatchingFace");
        final paint:RichTextPaint = new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.UniformTextStyle, RichTextBackgroundDrawStyle.Fill), 0.0);
        final clusters:Array<Cluster> = [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0)];
        final lineValue:LineBox = line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0);
        final latin:LayoutResult = result("ab", clusters, [lineValue], [], [], [], new LayoutDebugInfo(null, [metric(new TextRange(0, 2), "LatinBox", 9.0, 1.0, "latin")], [], [], [], []), style(10.0));
        final latinBox:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(latin, [segment(new TextRange(0, 2), RichTextRole.Background, paint, 0, new TextRange(0, 2), 0.0, 0.0, 20.0, 20.0, 15.0)]);
        TracedAssertions.assertEqualsFloat(6.0, latinBox[0].top);
        TracedAssertions.assertEqualsFloat(16.0, latinBox[0].bottom);
        final bothMetrics:Array<MetricDecisionInfo> = [
            metric(new TextRange(0, 1), "LatinBox", 9.0, 1.0, "latin"),
            metric(new TextRange(0, 2), "IdeographicEmBox", 8.0, 2.0, "ideographic")
        ];
        final both:LayoutResult = result("ab", clusters, [lineValue], [], [], [], new LayoutDebugInfo(null, bothMetrics, [], [], [], []), style(10.0));
        final ideographicBox:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(both, [segment(new TextRange(0, 2), RichTextRole.Background, paint, 0, new TextRange(0, 2), 0.0, 0.0, 20.0, 20.0, 15.0)]);
        TracedAssertions.assertEqualsFloat(7.0, ideographicBox[0].top);
        TracedAssertions.assertEqualsFloat(17.0, ideographicBox[0].bottom);
    }

    @:test
    public static function adjacentSameStyleSegmentsShareClearance():Void {
        currentTrace().section("adjacentSameStyleSegmentsShareClearance");
        final content:LayoutResult = result("ab", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0)], [line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0)], [], [], [], emptyDebug(), style(10.0));
        final paint:RichTextPaint = new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 4.0);
        final first:RichTextLineSegment = segment(new TextRange(0, 1), RichTextRole.Background, paint, 0, new TextRange(0, 1), 0.0, 0.0, 10.0, 20.0, 15.0);
        final second:RichTextLineSegment = segment(new TextRange(1, 2), RichTextRole.Background, paint, 0, new TextRange(1, 2), 10.0, 0.0, 20.0, 20.0, 15.0);
        final cleared:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [first, second]);
        TracedAssertions.assertEqualsInt(2, cleared.length);
        TracedAssertions.assertEqualsFloat(8.0, cleared[0].right);
        TracedAssertions.assertEqualsFloat(12.0, cleared[1].left);
        final other:RichTextLineSegment = segment(new TextRange(1, 2), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 3.0, 3.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill),  0.0), 0, new TextRange(1, 2), 10.0, 0.0, 20.0, 20.0, 15.0);
        final untouched:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [first, other]);
        TracedAssertions.assertEqualsInt(2, untouched.length);
        TracedAssertions.assertEqualsFloat(10.0, untouched[0].right);
        TracedAssertions.assertEqualsFloat(10.0, untouched[1].left);
    }

    @:test
    public static function decorationLineYRequiresValidStrokeAndDecorationRoles():Void {
        currentTrace().section("decorationLineYRequiresValidStrokeAndDecorationRoles");
        final content:LayoutResult = result("ab", [], [line(new TextRange(0, 2), 0, 0, 0.0, 20.0, 15.0, 0.0, 10.0)], [], [], [], emptyDebug(), style(10.0));
        final underline:RichTextLineSegment = segment(new TextRange(0, 1), RichTextRole.Underline, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(0, 1), 0.0, 0.0, 20.0, 20.0, 15.0);
        TracedAssertions.assertFailsWith(null, function():Void {
            LayoutQueries.richTextDecorationLineY(content, underline, -1.0);
        });
        TracedAssertions.assertFailsWith(null, function():Void {
            LayoutQueries.richTextDecorationLineY(content, underline, nan());
        });
        final error:TiqianIllegalArgumentException = TracedAssertions.assertFailsWith(null, function():Void {
            LayoutQueries.richTextDecorationLineY(content, plainSegment(new TextRange(0, 1)), 1.0);
        });
        TracedAssertions.assertTrue(error.message.indexOf("underline and line-through") >= 0, error.message);
        final withSpanStyle:LayoutResult = result(
            "ab", [cluster(new TextRange(0, 1), "a", 10.0)],
            [line(new TextRange(0, 2), 0, 0, 0.0, 20.0, 15.0, 0.0, 10.0)], [],
            [new TextSpan(new TextRange(0, 1), style(10.0))], [], emptyDebug(), style(10.0)
        );
        final y:Float = LayoutQueries.richTextDecorationLineY(withSpanStyle, underline, 1.0);
        TracedAssertions.assertTrue(y >= underline.top && y <= underline.bottom, Std.string(y));
        final lineThrough:RichTextLineSegment = segment(new TextRange(0, 1), RichTextRole.LineThrough, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(0, 1), 0.0, 0.0, 20.0, 20.0, 15.0);
        final strike:Float = LayoutQueries.richTextDecorationLineY(withSpanStyle, lineThrough, 1.0);
        TracedAssertions.assertEqualsFloatTolerance(11.2, strike, 0.001);
    }

    @:test
    public static function cursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClusters():Void {
        currentTrace().section("cursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClusters");
        final emptyClusterLine:LineBox = line(new TextRange(0, 0), 0, -1, 0.0, 20.0, 15.0, 6.0, 10.0);
        final withEmptyLine:LayoutResult = result("a", [], [emptyClusterLine], [], [], [], emptyDebug(), style(10.0));
        TracedAssertions.assertEqualsRendered(new Rect(6.0, 0.0, 7.0, 20.0).toString(), LayoutQueries.getCursorRect(withEmptyLine, 0).toString());
        final linear:LayoutResult = result("ab", [cluster(new TextRange(0, 2), "ab", 20.0)], [line(new TextRange(0, 2), 0, 0, 0.0, 20.0, 15.0, 0.0, 20.0)], [], [], [], emptyDebug(), style(10.0));
        TracedAssertions.assertEqualsFloat(10.0, LayoutQueries.getCursorRect(linear, 1).left);
        final stops:LayoutResult = result(
            "ab", [cluster(new TextRange(0, 2), "ab", 20.0)],
            [line(new TextRange(0, 2), 0, 0, 0.0, 20.0, 15.0, 0.0, 20.0)],
            [new GlyphRun(new TextRange(0, 2), "test", [
                new Glyph(1, new TextRange(0, 2), 10.0, 0.0, 0.0, null, null, null, null),
                new Glyph(2, new TextRange(0, 2), 10.0, 12.0, 0.0, null, null, null, null)
            ], 20.0, [])], [], [], emptyDebug(), style(10.0)
        );
        TracedAssertions.assertEqualsFloat(12.0, LayoutQueries.getCursorRect(stops, 1).left);
    }

    @:test
    public static function offsetForPositionCoversVerticalDistancesAndNaNPoints():Void {
        currentTrace().section("offsetForPositionCoversVerticalDistancesAndNaNPoints");
        final content:LayoutResult = result(
            "ab", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0)],
            [
                line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0),
                line(new TextRange(2, 2), 2, -1, 20.0, 40.0, 35.0, 0.0, 0.0)
            ], [], [], [], emptyDebug(), style(10.0)
        );
        TracedAssertions.assertEqualsInt(0, LayoutQueries.getOffsetForPosition(content, 2.0, -50.0));
        TracedAssertions.assertEqualsInt(2, LayoutQueries.getOffsetForPosition(content, 5.0, 90.0));
        TracedAssertions.assertEqualsInt(2, LayoutQueries.getOffsetForPosition(content, 5.0, 30.0));
        TracedAssertions.assertEqualsInt(0, LayoutQueries.getSelectionOffsetForPosition(content, 2.0, -50.0));
        TracedAssertions.assertEqualsInt(2, LayoutQueries.getSelectionOffsetForPosition(content, 5.0, 90.0));
        final withStops:LayoutResult = result(
            "ab", [cluster(new TextRange(0, 2), "ab", 20.0)],
            [line(new TextRange(0, 2), 0, 0, 0.0, 20.0, 15.0, 0.0, 20.0)],
            [new GlyphRun(new TextRange(0, 2), "test", [
                new Glyph(1, new TextRange(0, 2), 10.0, 0.0, 0.0, null, null, null, null),
                new Glyph(2, new TextRange(0, 2), 10.0, 10.0, 0.0, null, null, null, null)
            ], 20.0, [])], [], [], emptyDebug(), style(10.0)
        );
        TracedAssertions.assertEqualsInt(0, LayoutQueries.getOffsetForPosition(withStops, nan(), 5.0));
        TracedAssertions.assertEqualsInt(0, LayoutQueries.getSelectionOffsetForPosition(withStops, nan(), 5.0));
    }

    @:test
    public static function selectionSnapPrefersTheCloserInlineObjectBoundary():Void {
        currentTrace().section("selectionSnapPrefersTheCloserInlineObjectBoundary");
        final object:InlineObjectSpan = new InlineObjectSpan(new TextRange(1, 3), 8.0, 4.0, 4.0, InlineObjectBoundaryAdjustment.fixed(), InlineObjectBoundaryAdjustment.fixed());
        final content:LayoutResult = result("abb", [cluster(new TextRange(0, 3), "abb", 30.0)], [line(new TextRange(0, 3), 0, 0, 0.0, 20.0, 15.0, 0.0, 30.0)], [], [], [object], emptyDebug(), style(10.0));
        TracedAssertions.assertEqualsInt(1, LayoutQueries.getSelectionOffsetForPosition(content, 15.0, 5.0));
        TracedAssertions.assertEqualsInt(3, LayoutQueries.getSelectionOffsetForPosition(content, 21.0, 5.0));
    }

    @:test
    public static function selectionWordBoundaryForPositionRejectsDegenerateContent():Void {
        currentTrace().section("selectionWordBoundaryForPositionRejectsDegenerateContent");
        final emptyText:LayoutResult = result("", [], [line(new TextRange(0, 0), 0, -1, 0.0, 20.0, 15.0, 0.0, 0.0)], [], [], [], emptyDebug(), style(10.0));
        final emptyTextBoundary = LayoutQueries.getSelectionWordBoundaryForPosition(emptyText, 0.0, 0.0);
        TracedAssertions.assertNullRendered(emptyTextBoundary == null, emptyTextBoundary == null ? "-" : emptyTextBoundary.toString());
        final emptyLine:LayoutResult = result(
            "a", [cluster(new TextRange(0, 1), "a", 10.0)],
            [line(new TextRange(0, 1), 0, 0, 0.0, 20.0, 15.0, 0.0, 10.0), line(new TextRange(1, 1), 1, -1, 20.0, 40.0, 35.0, 0.0, 0.0)],
            [], [], [], emptyDebug(), style(10.0)
        );
        final emptyLineBoundary = LayoutQueries.getSelectionWordBoundaryForPosition(emptyLine, 5.0, 30.0);
        TracedAssertions.assertNullRendered(emptyLineBoundary == null, emptyLineBoundary == null ? "-" : emptyLineBoundary.toString());
        final leadingEmpty:LayoutResult = result(
            "a", [cluster(new TextRange(0, 0), "", 0.0), cluster(new TextRange(0, 1), "a", 10.0)],
            [line(new TextRange(0, 1), 0, 1, 0.0, 20.0, 15.0, 0.0, 10.0)],
            [], [], [], emptyDebug(), style(10.0)
        );
        final leadingEmptyBoundary = LayoutQueries.getSelectionWordBoundaryForPosition(leadingEmpty, 0.0, 5.0);
        TracedAssertions.assertNullRendered(leadingEmptyBoundary == null, leadingEmptyBoundary == null ? "-" : leadingEmptyBoundary.toString());
        TracedAssertions.assertEqualsRendered(new TextRange(0, 1).toString(), LayoutQueries.getSelectionWordBoundaryForPosition(leadingEmpty, 5.0, 5.0).toString());
    }

    @:test
    public static function zeroWidthClustersReturnTheirStartInHitTests():Void {
        currentTrace().section("zeroWidthClustersReturnTheirStartInHitTests");
        final emptyRange:LayoutResult = result("", [cluster(new TextRange(0, 0), "", 5.0)], [line(new TextRange(0, 0), 0, 0, 0.0, 20.0, 15.0, 0.0, 5.0)], [], [], [], emptyDebug(), style(10.0));
        TracedAssertions.assertEqualsInt(0, LayoutQueries.getOffsetForPosition(emptyRange, 2.0, 5.0));
        final zeroAdvance:LayoutResult = result(
            "ab", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 0.0)],
            [line(new TextRange(0, 1), 0, 0, 0.0, 20.0, 15.0, 0.0, 10.0), line(new TextRange(1, 2), 1, 1, 20.0, 40.0, 35.0, 0.0, 0.0)],
            [], [], [], emptyDebug(), style(10.0)
        );
        TracedAssertions.assertEqualsRendered(new TextRange(0, 2).toString(), LayoutQueries.getSelectionWordBoundaryForPosition(zeroAdvance, 0.0, 30.0).toString());
    }

    @:test
    public static function coerceSelectionOffsetHonoursInlineObjectBoundaries():Void {
        currentTrace().section("coerceSelectionOffsetHonoursInlineObjectBoundaries");
        final object:InlineObjectSpan = new InlineObjectSpan(new TextRange(1, 3), 8.0, 4.0, 4.0, InlineObjectBoundaryAdjustment.fixed(), InlineObjectBoundaryAdjustment.fixed());
        final content:LayoutResult = result("abb", [], [line(new TextRange(0, 3), 0, 0, 0.0, 20.0, 15.0, 0.0, 0.0)], [], [], [object], emptyDebug(), style(10.0));
        TracedAssertions.assertEqualsInt(1, LayoutQueries.coerceSelectionOffset(content, 2, SourceBoundaryBias.Backward));
        TracedAssertions.assertEqualsInt(3, LayoutQueries.coerceSelectionOffset(content, 2, SourceBoundaryBias.Forward));
        TracedAssertions.assertEqualsInt(3, LayoutQueries.coerceSelectionOffset(content, 2, SourceBoundaryBias.Nearest));
        TracedAssertions.assertEqualsInt(1, LayoutQueries.coerceSelectionOffset(content, 1, SourceBoundaryBias.Nearest));
        TracedAssertions.assertEqualsInt(3, LayoutQueries.coerceSelectionOffset(content, 3, SourceBoundaryBias.Nearest));
    }

    @:test
    public static function selectionWordBoundaryExpandsWordsAndHonoursInlineObjects():Void {
        currentTrace().section("selectionWordBoundaryExpandsWordsAndHonoursInlineObjects");
        final content:LayoutResult = result("hello", [], [line(new TextRange(0, 5), 0, 0, 0.0, 20.0, 15.0, 0.0, 0.0)], [], [], [], emptyDebug(), style(10.0));
        TracedAssertions.assertEqualsRendered(new TextRange(0, 5).toString(), LayoutQueries.getSelectionWordBoundary(content, 2).toString());
        TracedAssertions.assertEqualsRendered(new TextRange(0, 5).toString(), LayoutQueries.getSelectionWordBoundary(content, 5).toString());
        final emojiText:String = TestHelpers.surrogateText([0xD83D, 0xDE00]);
        final emoji:LayoutResult = result(emojiText, [], [line(new TextRange(0, 2), 0, 0, 0.0, 20.0, 15.0, 0.0, 0.0)], [], [], [], emptyDebug(), style(10.0));
        TracedAssertions.assertEqualsRendered(new TextRange(0, 2).toString(), LayoutQueries.getSelectionWordBoundary(emoji, 1).toString());
        final object:InlineObjectSpan = new InlineObjectSpan(new TextRange(1, 3), 8.0, 4.0, 4.0, InlineObjectBoundaryAdjustment.fixed(), InlineObjectBoundaryAdjustment.fixed());
        final withObject:LayoutResult = result("abb", [], [line(new TextRange(0, 3), 0, 0, 0.0, 20.0, 15.0, 0.0, 0.0)], [], [], [object], emptyDebug(), style(10.0));
        TracedAssertions.assertEqualsRendered(new TextRange(1, 3).toString(), LayoutQueries.getSelectionWordBoundary(withObject, 2).toString());
        final mandatory:LayoutResult = result("a\nb", [], [line(new TextRange(0, 3), 0, 0, 0.0, 20.0, 15.0, 0.0, 0.0)], [], [], [], emptyDebug(), style(10.0));
        TracedAssertions.assertEqualsRendered(new TextRange(1, 2).toString(), LayoutQueries.getSelectionWordBoundary(mandatory, 1).toString());
        final connectors:LayoutResult = result("a_b", [], [line(new TextRange(0, 3), 0, 0, 0.0, 20.0, 15.0, 0.0, 0.0)], [], [], [], emptyDebug(), style(10.0));
        TracedAssertions.assertEqualsRendered(new TextRange(0, 3).toString(), LayoutQueries.getSelectionWordBoundary(connectors, 1).toString());
        final empty:LayoutResult = result("", [], [line(new TextRange(0, 0), 0, -1, 0.0, 20.0, 15.0, 0.0, 0.0)], [], [], [], emptyDebug(), style(10.0));
        TracedAssertions.assertEqualsRendered(new TextRange(0, 0).toString(), LayoutQueries.getSelectionWordBoundary(empty, 0).toString());
    }

    @:test
    public static function selectionWordKindCoversEveryHanBlock():Void {
        currentTrace().section("selectionWordKindCoversEveryHanBlock");
        final supplementary:String = TestHelpers.surrogateText([0xD840, 0xDC00]);
        final values:Array<String> = ["㐀", "一", "豈", supplementary];
        var index:Int = 0;
        while (index < values.length) {
            final text:String = values[index];
            final content:LayoutResult = result(text, [], [line(new TextRange(0, text.length), 0, 0, 0.0, 20.0, 15.0, 0.0, 0.0)], [], [], [], emptyDebug(), style(10.0));
            TracedAssertions.assertEqualsRendered(new TextRange(0, text.length).toString(), LayoutQueries.getSelectionWordBoundary(content, 0).toString(), "text=" + text);
            index += 1;
        }
    }

    @:test
    public static function nearestLineFallsBackToTheOnlyLineAtItsEndOffset():Void {
        currentTrace().section("nearestLineFallsBackToTheOnlyLineAtItsEndOffset");
        final content:LayoutResult = result("abc", [cluster(new TextRange(0, 2), "ab", 20.0)], [line(new TextRange(0, 2), 0, 0, 0.0, 20.0, 15.0, 0.0, 20.0)], [], [], [], emptyDebug(), style(10.0));
        TracedAssertions.assertEqualsInt(0, LayoutQueries.getLineForOffset(content, 2));
    }

    @:test
    public static function rubyGeometryRedistributesSelectionBoxesAndDropsSourceStops():Void {
        currentTrace().section("rubyGeometryRedistributesSelectionBoxesAndDropsSourceStops");
        final clusters:Array<Cluster> = [cluster(new TextRange(0, 2), "ab", 20.0), cluster(new TextRange(2, 3), "c", 10.0)];
        final runs:Array<GlyphRun> = [new GlyphRun(new TextRange(0, 2), "test", [
            new Glyph(1, new TextRange(0, 2), 10.0, 0.0, 0.0, null, null, null, null),
            new Glyph(2, new TextRange(0, 2), 10.0, 10.0, 0.0, null, null, null, null)
        ], 20.0, [])];
        final matching:RubyDecisionInfo = new RubyDecisionInfo(new TextRange(0, 3), "zhù", 0, 15.0, 4.0, 6.0, 0.0, 0.0, 0.0, 30.0, [], 400, "zh-Hans", []);
        final stray:RubyDecisionInfo = new RubyDecisionInfo(new TextRange(5, 6), "x", 0, 0.0, 4.0, 6.0, 0.0, 0.0, 0.0, 6.0, [], 400, "zh-Hans", []);
        final content:LayoutResult = result("abc", clusters, [line(new TextRange(0, 3), 0, 1, 0.0, 20.0, 15.0, 0.0, 30.0)], runs, [], [], new LayoutDebugInfo(null, [], [], [], [matching, stray], []), style(10.0));
        final positioned:Array<PositionedCluster> = LayoutQueries.positionedClusters(content);
        TracedAssertions.assertEqualsInt(2, positioned.length);
        final firstStops = positioned[0].sourceStops;
        TracedAssertions.assertNullRendered(firstStops == null, firstStops == null ? "-" : Std.string(firstStops));
        final secondStops = positioned[1].sourceStops;
        TracedAssertions.assertNullRendered(secondStops == null, secondStops == null ? "-" : Std.string(secondStops));
        TracedAssertions.assertEqualsFloat(0.0, positioned[0].left);
        TracedAssertions.assertEqualsFloat(17.5, positioned[0].right);
        TracedAssertions.assertEqualsFloat(17.5, positioned[1].left);
        TracedAssertions.assertEqualsFloat(30.0, positioned[1].right);
    }

    @:test
    public static function boundingBoxesSliceZeroWidthAndEmptyClusters():Void {
        currentTrace().section("boundingBoxesSliceZeroWidthAndEmptyClusters");
        final content:LayoutResult = result("ab", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 0.0)], [line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 10.0)], [], [], [], emptyDebug(), style(10.0));
        final boxes:Array<Rect> = LayoutQueries.getBoundingBoxes(content, new TextRange(0, 2));
        TracedAssertions.assertEqualsInt(2, boxes.length);
        TracedAssertions.assertEqualsFloat(10.0, boxes[1].left);
        TracedAssertions.assertEqualsFloat(10.0, boxes[1].right);
        final tail:Array<Rect> = LayoutQueries.getBoundingBoxes(content, new TextRange(1, 2));
        TracedAssertions.assertEqualsInt(1, tail.length);
        TracedAssertions.assertEqualsFloat(10.0, tail[0].left);
    }

    @:test
    public static function positionedClustersAndSegmentsReturnEmptyWithoutLines():Void {
        currentTrace().section("positionedClustersAndSegmentsReturnEmptyWithoutLines");
        final noLines:LayoutResult = result("ab", [cluster(new TextRange(0, 1), "a", 10.0)], [], [], [], [], emptyDebug(), style(10.0));
        TracedAssertions.assertTrue(LayoutQueries.positionedClusters(noLines).length == 0);
        TracedAssertions.assertTrue(LayoutQueries.positionedRichTextSegments(noLines, [new RichTextSpan(new TextRange(0, 2), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0))]).length == 0);
        final noSpans:LayoutResult = result("ab", [cluster(new TextRange(0, 1), "a", 10.0)], [line(new TextRange(0, 1), 0, 0, 0.0, 20.0, 15.0, 0.0, 10.0)], [], [], [], emptyDebug(), style(10.0));
        TracedAssertions.assertTrue(LayoutQueries.positionedRichTextSegments(noSpans, []).length == 0);
    }

    @:test
    public static function sameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegment():Void {
        currentTrace().section("sameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegment");
        final inputContent:TiqianTextContent = new TiqianTextContent("ab", [], [1], [], []);
        final input:LayoutInput = new LayoutInput(inputContent, new LayoutConstraints(100.0, Math.POSITIVE_INFINITY, 2147483647), style(10.0), new ParagraphStyle(LastLineAlignment.Start, WritingMode.HorizontalTb, null, null, Ic.Zero, new MeasureAdaptiveFirstLineIndent(14.0, 1.0, 2.0), new LineLengthGrid(true, null), RubyLineHeightMode.PerLine, ParagraphStyle.DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM, ParagraphStyle.DEFAULT_EMPHASIS_DOT_GAP_EM), BuiltInLayoutProfiles.ClreqHorizontal, [], [], [], []);
        final content:LayoutResult = new LayoutResult(
            input, new Size(20.0, 20.0),
            [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0)], [],
            [line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0)], emptyDebug()
        );
        final segments:Array<RichTextLineSegment> = LayoutQueries.positionedRichTextSegments(content, [new RichTextSpan(new TextRange(0, 2), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0))]);
        TracedAssertions.assertEqualsInt(1, segments.length);
        TracedAssertions.assertEqualsRendered(new TextRange(0, 2).toString(), segments[0].range.toString());
        TracedAssertions.assertEqualsFloat(0.0, segments[0].left);
        TracedAssertions.assertEqualsFloat(20.0, segments[0].right);
    }

    @:test
    public static function glyphInkBoundsSkipsUnusableGlyphsAndReportsNull():Void {
        currentTrace().section("glyphInkBoundsSkipsUnusableGlyphsAndReportsNull");
        final clusters:Array<Cluster> = [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0)];
        final lines:Array<LineBox> = [line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0)];
        final noBounds:LayoutResult = result("ab", clusters, lines, [new GlyphRun(new TextRange(0, 2), "test", [new Glyph(1, new TextRange(0, 1), 10.0, 0.0, 0.0, null, null, null, null)], 20.0, [])], [], [], emptyDebug(), style(10.0));
        final absentBounds = LayoutQueries.glyphInkBounds(noBounds);
        TracedAssertions.assertNullRendered(absentBounds == null, absentBounds == null ? "-" : absentBounds.toString());
        final nanPlaced:LayoutResult = result("ab", clusters, lines, [new GlyphRun(new TextRange(1, 2), "test", [new Glyph(9, new TextRange(1, 2), 9.0, nan(), 0.0, null, new Rect(1.0, 2.0, 8.0, 4.0), null, null)], 10.0, [])], [], [], emptyDebug(), style(10.0));
        final nanPlacedBounds = LayoutQueries.glyphInkBounds(nanPlaced);
        TracedAssertions.assertNullRendered(nanPlacedBounds == null, nanPlacedBounds == null ? "-" : nanPlacedBounds.toString());
        final usable:LayoutResult = result(
            "ab", clusters, lines,
            [new GlyphRun(new TextRange(0, 2), "test", [
                new Glyph(1, new TextRange(0, 1), 10.0, 2.0, 1.0, null, new Rect(1.0, 2.0, 8.0, 4.0), null, null),
                new Glyph(2, new TextRange(1, 2), 10.0, 1.0, 0.0, null, new Rect(0.0, 1.0, 9.0, 3.0), null, null)
            ], 20.0, [])], [], [], emptyDebug(), style(10.0)
        );
        final ink:Rect = LayoutQueries.glyphInkBounds(usable);
        TracedAssertions.assertEqualsFloat(3.0, ink.left);
        TracedAssertions.assertEqualsFloat(20.0, ink.right);
        TracedAssertions.assertEqualsFloat(16.0, ink.top);
        TracedAssertions.assertEqualsFloat(20.0, ink.bottom);
    }

    @:test
    public static function backgroundTrailingEdgeUsesGlyphAdvancesWhenAvailable():Void {
        currentTrace().section("backgroundTrailingEdgeUsesGlyphAdvancesWhenAvailable");
        final clusters:Array<Cluster> = [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0)];
        final lines:Array<LineBox> = [line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0)];
        final shortGlyph:LayoutResult = result("ab", clusters, lines, [new GlyphRun(new TextRange(1, 2), "test", [new Glyph(2, new TextRange(1, 2), 5.0, 0.0, 0.0, null, null, null, null)], 10.0, [])], [], [], emptyDebug(), style(10.0));
        final shortSegments:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(shortGlyph, [segment(new TextRange(0, 2), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(0, 2), 0.0, 0.0, 20.0, 20.0, 15.0)]);
        TracedAssertions.assertEqualsFloat(15.0, shortSegments[0].right);
        final emptyGlyphRun:LayoutResult = result("ab", clusters, lines, [new GlyphRun(new TextRange(1, 2), "test", [], 10.0, [])], [], [], emptyDebug(), style(10.0));
        final emptySegments:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(emptyGlyphRun, [segment(new TextRange(0, 2), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(0, 2), 0.0, 0.0, 20.0, 20.0, 15.0)]);
        TracedAssertions.assertEqualsFloat(20.0, emptySegments[0].right);
    }

    @:test
    public static function clearanceNeedsSameRoleAndUsesTheSmallerSide():Void {
        currentTrace().section("clearanceNeedsSameRoleAndUsesTheSmallerSide");
        final content:LayoutResult = result("ab", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0)], [line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0)], [], [], [], emptyDebug(), style(10.0));
        final background:RichTextLineSegment = segment(new TextRange(0, 1), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 4.0), 0, new TextRange(0, 1), 0.0, 0.0, 10.0, 20.0, 15.0);
        final inlineCode:RichTextLineSegment = segment(new TextRange(1, 2), RichTextRole.InlineCode, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 4.0), 0, new TextRange(1, 2), 10.0, 0.0, 20.0, 20.0, 15.0);
        final byRole:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [background, inlineCode]);
        TracedAssertions.assertEqualsFloat(10.0, byRole[0].right);
        TracedAssertions.assertEqualsFloat(10.0, byRole[1].left);
        final weak:RichTextLineSegment = segment(new TextRange(0, 1), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 2.0), 0, new TextRange(0, 1), 0.0, 0.0, 10.0, 20.0, 15.0);
        final strong:RichTextLineSegment = segment(new TextRange(1, 2), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 6.0), 0, new TextRange(1, 2), 10.0, 0.0, 20.0, 20.0, 15.0);
        final cleared:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [weak, strong]);
        TracedAssertions.assertEqualsFloat(9.0, cleared[0].right);
        TracedAssertions.assertEqualsFloat(11.0, cleared[1].left);
    }

    @:test
    public static function metricDecisionsMustFullyContainTheCluster():Void {
        currentTrace().section("metricDecisionsMustFullyContainTheCluster");
        final first:Array<Float> = metricBounds(new TextRange(1, 2));
        TracedAssertions.assertEqualsFloat(15.0 - 10.0 * 0.88, first[0]);
        TracedAssertions.assertEqualsFloat(15.0 + 10.0 * 0.12, first[1]);
        final second:Array<Float> = metricBounds(new TextRange(0, 1));
        TracedAssertions.assertEqualsFloat(15.0 - 10.0 * 0.88, second[0]);
    }

    @:test
    public static function decorationStyleResolvesInsideSpansAndAtTheirEdges():Void {
        currentTrace().section("decorationStyleResolvesInsideSpansAndAtTheirEdges");
        final content:LayoutResult = result(
            "abc",
            [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0), cluster(new TextRange(2, 3), "c", 10.0)],
            [line(new TextRange(0, 3), 0, 2, 0.0, 20.0, 15.0, 0.0, 30.0)], [],
            [new TextSpan(new TextRange(0, 1), style(10.0)), new TextSpan(new TextRange(2, 3), style(20.0))],
            [], emptyDebug(), style(10.0)
        );
        final between:Float = LayoutQueries.richTextDecorationLineY(content, segment(new TextRange(1, 2), RichTextRole.Underline, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(1, 2), 10.0, 0.0, 20.0, 20.0, 15.0), 1.0);
        final inside:Float = LayoutQueries.richTextDecorationLineY(content, segment(new TextRange(2, 3), RichTextRole.Underline, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(2, 3), 20.0, 0.0, 30.0, 20.0, 15.0), 1.0);
        TracedAssertions.assertEqualsFloat(15.0 + 10.0 * 0.18, between);
        TracedAssertions.assertEqualsFloat(15.0 + 20.0 * 0.18, inside);
    }

    @:test
    public static function glueTrimSkipsInteriorSegmentEdges():Void {
        currentTrace().section("glueTrimSkipsInteriorSegmentEdges");
        final glue:ClusterGeometryDecisionInfo = new ClusterGeometryDecisionInfo(
            new TextRange(0, 2), "ab", "ab", 20.0, 10.0, 4.0, 1.0, 4.0, 1.0, 0.0, 20.0, "test", "test", 0.0, 0.0, null
        );
        final content:LayoutResult = result("ab", [cluster(new TextRange(0, 2), "ab", 20.0)], [line(new TextRange(0, 2), 0, 0, 0.0, 20.0, 15.0, 0.0, 20.0)], [], [], [], new LayoutDebugInfo(null, [], [glue], [], [], []), style(10.0));
        final interiorStart:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [segment(new TextRange(1, 2), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(1, 2), 10.0, 0.0, 20.0, 20.0, 15.0)]);
        TracedAssertions.assertEqualsFloat(10.0, interiorStart[0].left);
        final interiorEnd:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [segment(new TextRange(0, 1), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(0, 1), 0.0, 0.0, 10.0, 20.0, 15.0)]);
        TracedAssertions.assertEqualsFloat(10.0, interiorEnd[0].right);
    }

    @:test
    public static function backgroundSegmentOutsideEverySpanUsesTheParagraphStyle():Void {
        currentTrace().section("backgroundSegmentOutsideEverySpanUsesTheParagraphStyle");
        final content:LayoutResult = result(
            "abc",
            [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0), cluster(new TextRange(2, 3), "c", 10.0)],
            [line(new TextRange(0, 3), 0, 2, 0.0, 20.0, 15.0, 0.0, 30.0)], [],
            [new TextSpan(new TextRange(1, 2), style(40.0))], [], emptyDebug(), style(10.0)
        );
        final before:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [segment(new TextRange(0, 1), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(0, 1), 0.0, 0.0, 10.0, 20.0, 15.0)]);
        TracedAssertions.assertEqualsFloat(15.0 - 10.0 * 0.88, before[0].top);
        final atEnd:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [segment(new TextRange(2, 3), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(2, 3), 20.0, 0.0, 30.0, 20.0, 15.0)]);
        TracedAssertions.assertEqualsFloat(15.0 - 10.0 * 0.88, atEnd[0].top);
    }

    @:test
    public static function cursorRectFindsLaterClustersAndRejectsGappedRanges():Void {
        currentTrace().section("cursorRectFindsLaterClustersAndRejectsGappedRanges");
        final content:LayoutResult = result(
            "abc", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0), cluster(new TextRange(2, 3), "c", 10.0)],
            [line(new TextRange(0, 3), 0, 2, 0.0, 20.0, 15.0, 0.0, 30.0)], [], [], [], emptyDebug(), style(10.0)
        );
        TracedAssertions.assertEqualsFloat(20.0, LayoutQueries.getCursorRect(content, 2).left);
        final gapped:LayoutResult = result(
            "abcde", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(4, 5), "e", 10.0)],
            [line(new TextRange(0, 5), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0)], [], [], [], emptyDebug(), style(10.0)
        );
        TracedAssertions.assertFailsWithNoSuchElement(null, function():Void {
            LayoutQueries.getCursorRect(gapped, 2);
        });
    }

    @:test
    public static function emptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRects():Void {
        currentTrace().section("emptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRects");
        final content:LayoutResult = result(
            "abc", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(2, 2), "", 0.0), cluster(new TextRange(2, 3), "c", 10.0)],
            [line(new TextRange(0, 3), 0, 2, 0.0, 20.0, 15.0, 0.0, 20.0)], [], [], [], emptyDebug(), style(10.0)
        );
        TracedAssertions.assertEqualsFloat(10.0, LayoutQueries.getCursorRect(content, 2).left);
        final withEmpty:LayoutResult = result(
            "ab", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 1), "", 0.0), cluster(new TextRange(1, 2), "b", 10.0)],
            [line(new TextRange(0, 2), 0, 2, 0.0, 20.0, 15.0, 0.0, 20.0)], [], [], [], emptyDebug(), style(10.0)
        );
        final boxes:Array<Rect> = LayoutQueries.getBoundingBoxes(withEmpty, new TextRange(0, 2));
        TracedAssertions.assertEqualsInt(2, boxes.length);
        TracedAssertions.assertEqualsFloat(0.0, boxes[0].left);
        TracedAssertions.assertEqualsFloat(10.0, boxes[0].right);
        TracedAssertions.assertEqualsFloat(10.0, boxes[1].left);
        TracedAssertions.assertEqualsFloat(20.0, boxes[1].right);
        final zeroAdvance:LayoutResult = result(
            "abc", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 0.0), cluster(new TextRange(2, 3), "c", 10.0)],
            [line(new TextRange(0, 3), 0, 2, 0.0, 20.0, 15.0, 0.0, 20.0)], [], [], [], emptyDebug(), style(10.0)
        );
        final degenerate:Array<Rect> = LayoutQueries.getBoundingBoxes(zeroAdvance, new TextRange(0, 3));
        TracedAssertions.assertEqualsInt(3, degenerate.length);
        TracedAssertions.assertEqualsFloat(10.0, degenerate[1].left);
        TracedAssertions.assertEqualsFloat(10.0, degenerate[1].right);
    }

    @:test
    public static function selectionWordBoundarySkipsInlineObjectsItDoesNotContain():Void {
        currentTrace().section("selectionWordBoundarySkipsInlineObjectsItDoesNotContain");
        final objects:Array<InlineObjectSpan> = [
            new InlineObjectSpan(new TextRange(1, 3), 8.0, 4.0, 4.0, InlineObjectBoundaryAdjustment.fixed(), InlineObjectBoundaryAdjustment.fixed()),
            new InlineObjectSpan(new TextRange(5, 7), 8.0, 4.0, 4.0, InlineObjectBoundaryAdjustment.fixed(), InlineObjectBoundaryAdjustment.fixed())
        ];
        final content:LayoutResult = result("abcdefg", [cluster(new TextRange(0, 7), "abcdefg", 70.0)], [line(new TextRange(0, 7), 0, 0, 0.0, 20.0, 15.0, 0.0, 70.0)], [], [], objects, emptyDebug(), style(10.0));
        TracedAssertions.assertEqualsRendered(new TextRange(0, 7).toString(), LayoutQueries.getSelectionWordBoundary(content, 4).toString());
        TracedAssertions.assertEqualsRendered(new TextRange(1, 3).toString(), LayoutQueries.getSelectionWordBoundary(content, 2).toString());
    }

    @:test
    public static function selectionWordBoundaryForPositionCoversDistancesAndFallbacks():Void {
        currentTrace().section("selectionWordBoundaryForPositionCoversDistancesAndFallbacks");
        final content:LayoutResult = result(
            "甲乙", [cluster(new TextRange(0, 1), "甲", 10.0), cluster(new TextRange(1, 2), "乙", 10.0)],
            [line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0)], [], [], [], emptyDebug(), style(10.0)
        );
        TracedAssertions.assertEqualsRendered(new TextRange(0, 1).toString(), LayoutQueries.getSelectionWordBoundaryForPosition(content, 5.0, 10.0).toString());
        TracedAssertions.assertEqualsRendered(new TextRange(0, 1).toString(), LayoutQueries.getSelectionWordBoundaryForPosition(content, 5.0, -10.0).toString());
        TracedAssertions.assertEqualsRendered(new TextRange(0, 1).toString(), LayoutQueries.getSelectionWordBoundaryForPosition(content, 5.0, 60.0).toString());
        TracedAssertions.assertEqualsRendered(new TextRange(0, 1).toString(), LayoutQueries.getSelectionWordBoundaryForPosition(content, -50.0, 10.0).toString());
        TracedAssertions.assertEqualsRendered(new TextRange(1, 2).toString(), LayoutQueries.getSelectionWordBoundaryForPosition(content, 500.0, 10.0).toString());
    }

    @:test
    public static function lineForOffsetInsideARangeTakesTheZeroDistanceArm():Void {
        currentTrace().section("lineForOffsetInsideARangeTakesTheZeroDistanceArm");
        final content:LayoutResult = result(
            "abcde", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(4, 5), "e", 10.0)],
            [line(new TextRange(0, 2), 0, 0, 0.0, 20.0, 15.0, 0.0, 10.0), line(new TextRange(4, 5), 1, 1, 20.0, 40.0, 35.0, 0.0, 10.0)], [], [], [], emptyDebug(), style(10.0)
        );
        TracedAssertions.assertEqualsInt(0, LayoutQueries.getLineForOffset(content, 1));
    }

    @:test
    public static function compatibilityIdeographsFormIndividualWordUnits():Void {
        currentTrace().section("compatibilityIdeographsFormIndividualWordUnits");
        final text:String = TestHelpers.surrogateText([0xD840, 0xDC00]) + "\uF900";
        final content:LayoutResult = result(
            text, [cluster(new TextRange(0, 2), TestHelpers.surrogateText([0xD840, 0xDC00]), 10.0), cluster(new TextRange(2, 3), "\uF900", 10.0)],
            [line(new TextRange(0, 3), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0)], [], [], [], emptyDebug(), style(10.0)
        );
        TracedAssertions.assertEqualsRendered(new TextRange(0, 2).toString(), LayoutQueries.getSelectionWordBoundary(content, 0).toString());
        TracedAssertions.assertEqualsRendered(new TextRange(2, 3).toString(), LayoutQueries.getSelectionWordBoundary(content, 2).toString());
    }

    @:test
    public static function rubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnored():Void {
        currentTrace().section("rubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnored");
        final clusters:Array<Cluster> = [cluster(new TextRange(0, 2), "ab", 20.0), cluster(new TextRange(2, 3), "c", 10.0)];
        final geometries:Array<ClusterGeometryDecisionInfo> = [geometry(new TextRange(0, 2), "ab", 0.0, 0.0, 0.0, 0.0), geometry(new TextRange(2, 3), "c", 0.0, 0.0, 0.0, 0.0)];
        final firstGeometry:ClusterGeometryDecisionInfo = new ClusterGeometryDecisionInfo(new TextRange(0, 2), "ab", "ab", 20.0, 10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 20.0, "test", "test", 5.0, 0.0, null);
        final secondGeometry:ClusterGeometryDecisionInfo = new ClusterGeometryDecisionInfo(new TextRange(2, 3), "c", "c", 10.0, 10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 10.0, "test", "test", 2.0, 0.0, null);
        geometries[0] = firstGeometry;
        geometries[1] = secondGeometry;
        final rubies:Array<RubyDecisionInfo> = [
            new RubyDecisionInfo(new TextRange(0, 3), "zhù", 0, 15.0, 4.0, 6.0, 0.0, 0.0, 0.0, 30.0, [], 400, "zh-Hans", []),
            new RubyDecisionInfo(new TextRange(2, 3), "x", 0, 25.0, 4.0, 6.0, 0.0, 0.0, 0.0, 0.0, [], 400, "zh-Hans", []),
            new RubyDecisionInfo(new TextRange(5, 6), "y", 0, 25.0, 4.0, 6.0, 0.0, 0.0, 0.0, 6.0, [], 400, "zh-Hans", [])
        ];
        final content:LayoutResult = result("abc", clusters, [line(new TextRange(0, 3), 0, 1, 0.0, 20.0, 15.0, 0.0, 30.0)], [], [], [], new LayoutDebugInfo(null, [], geometries, [], rubies, []), style(10.0));
        final positioned:Array<PositionedCluster> = LayoutQueries.positionedClusters(content);
        TracedAssertions.assertEqualsFloat(0.0, positioned[0].left);
        TracedAssertions.assertEqualsFloat(15.75, positioned[0].right);
        TracedAssertions.assertEqualsFloat(15.75, positioned[1].left);
        TracedAssertions.assertEqualsFloat(30.0, positioned[1].right);
        final glyphResult:LayoutResult = result(
            "abc", clusters, [line(new TextRange(0, 3), 0, 1, 0.0, 20.0, 15.0, 0.0, 30.0)],
            [new GlyphRun(new TextRange(0, 3), "test", [new Glyph(1, new TextRange(0, 2), 16.0, 0.0, 0.0, null, null, null, null), new Glyph(2, new TextRange(2, 3), 8.0, 0.0, 0.0, null, null, null, null)], 30.0, [])],
            [], [], new LayoutDebugInfo(null, [], geometries, [], rubies, []), style(10.0)
        );
        final glyphPositioned:Array<PositionedCluster> = LayoutQueries.positionedClusters(glyphResult);
        TracedAssertions.assertEqualsFloat(0.0, glyphPositioned[0].left);
        TracedAssertions.assertEqualsFloat(16.0, glyphPositioned[0].right);
        TracedAssertions.assertEqualsFloat(16.0, glyphPositioned[1].left);
        TracedAssertions.assertEqualsFloat(30.0, glyphPositioned[1].right);
    }

    @:test
    public static function noArgPositionedClustersWalksEveryLine():Void {
        currentTrace().section("noArgPositionedClustersWalksEveryLine");
        final content:LayoutResult = result(
            "abcd",
            [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0), cluster(new TextRange(2, 3), "c", 10.0), cluster(new TextRange(3, 4), "d", 10.0)],
            [
                line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0),
                line(new TextRange(2, 4), 2, 3, 20.0, 40.0, 35.0, 0.0, 20.0),
                line(new TextRange(4, 4), 2, 1, 40.0, 60.0, 55.0, 0.0, 0.0)
            ], [], [], [], emptyDebug(), style(10.0)
        );
        final positioned:Array<PositionedCluster> = LayoutQueries.positionedClusters(content);
        TracedAssertions.assertEqualsInt(4, positioned.length);
        TracedAssertions.assertEqualsInt(0, positioned[0].lineIndex);
        TracedAssertions.assertEqualsInt(1, positioned[2].lineIndex);
        TracedAssertions.assertEqualsFloat(20.0, positioned[3].right);
    }

    @:test
    public static function glyphInkBoundsRejectsEachNonFiniteEdgeIndependently():Void {
        currentTrace().section("glyphInkBoundsRejectsEachNonFiniteEdgeIndependently");
        final nonFiniteLeft = inkWithBounds(new Rect(nan(), 2.0, 8.0, 4.0));
        TracedAssertions.assertNullRendered(nonFiniteLeft == null, nonFiniteLeft == null ? "-" : nonFiniteLeft.toString());
        final nonFiniteTop = inkWithBounds(new Rect(1.0, nan(), 8.0, 4.0));
        TracedAssertions.assertNullRendered(nonFiniteTop == null, nonFiniteTop == null ? "-" : nonFiniteTop.toString());
        final nonFiniteRight = inkWithBounds(new Rect(1.0, 2.0, nan(), 4.0));
        TracedAssertions.assertNullRendered(nonFiniteRight == null, nonFiniteRight == null ? "-" : nonFiniteRight.toString());
        final nonFiniteBottom = inkWithBounds(new Rect(1.0, 2.0, 8.0, nan()));
        TracedAssertions.assertNullRendered(nonFiniteBottom == null, nonFiniteBottom == null ? "-" : nonFiniteBottom.toString());
    }

    @:test
    public static function clearanceTakesTheSmallerSideWhicheverSegmentOwnsIt():Void {
        currentTrace().section("clearanceTakesTheSmallerSideWhicheverSegmentOwnsIt");
        final content:LayoutResult = result("ab", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0)], [line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0)], [], [], [], emptyDebug(), style(10.0));
        final weakFirst:RichTextLineSegment = segment(new TextRange(0, 1), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 6.0), 0, new TextRange(0, 1), 0.0, 0.0, 10.0, 20.0, 15.0);
        final strongSecond:RichTextLineSegment = segment(new TextRange(1, 2), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 2.0), 0, new TextRange(1, 2), 10.0, 0.0, 20.0, 20.0, 15.0);
        final cleared:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [weakFirst, strongSecond]);
        TracedAssertions.assertEqualsFloat(9.0, cleared[0].right);
        TracedAssertions.assertEqualsFloat(11.0, cleared[1].left);
        final styledA:RichTextLineSegment = segment(new TextRange(1, 2), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 4.0), 0, new TextRange(1, 2), 10.0, 0.0, 20.0, 20.0, 15.0);
        final scanPast:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [
            segment(new TextRange(1, 2), RichTextRole.InlineCode, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 4.0), 0, new TextRange(1, 2), 10.0, 0.0, 20.0, 20.0, 15.0),
            segment(new TextRange(0, 1), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 4.0), 0, new TextRange(0, 1), 0.0, 0.0, 10.0, 20.0, 15.0),
            styledA
        ]);
        TracedAssertions.assertEqualsInt(3, scanPast.length);
        TracedAssertions.assertEqualsFloat(8.0, scanPast[1].right);
        TracedAssertions.assertEqualsFloat(12.0, scanPast[2].left);
    }

    @:test
    public static function uniformTextStylePolicyResolvesSpanStyleOrParagraphStyle():Void {
        currentTrace().section("uniformTextStylePolicyResolvesSpanStyleOrParagraphStyle");
        final uniform:RichTextPaint = new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.UniformTextStyle, RichTextBackgroundDrawStyle.Fill), 0.0);
        final content:LayoutResult = result(
            "abc", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0), cluster(new TextRange(2, 3), "c", 10.0)],
            [line(new TextRange(0, 3), 0, 2, 0.0, 20.0, 15.0, 0.0, 30.0)], [], [new TextSpan(new TextRange(1, 2), style(40.0))], [], emptyDebug(), style(10.0)
        );
        final outside:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [segment(new TextRange(0, 1), RichTextRole.Background, uniform, 0, new TextRange(0, 1), 0.0, 0.0, 10.0, 20.0, 15.0)]);
        TracedAssertions.assertEqualsFloat(15.0 - 10.0 * 0.88, outside[0].top);
        final inside:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [segment(new TextRange(1, 2), RichTextRole.Background, uniform, 0, new TextRange(1, 2), 10.0, 0.0, 20.0, 20.0, 15.0)]);
        TracedAssertions.assertEqualsFloat(0.0, inside[0].top);
    }

    @:test
    public static function trailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEnd():Void {
        currentTrace().section("trailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEnd");
        final content:LayoutResult = result("ab", [cluster(new TextRange(1, 2), "b", 10.0)], [line(new TextRange(0, 2), 0, 0, 0.0, 20.0, 15.0, 0.0, 10.0)], [], [], [], emptyDebug(), style(10.0));
        final out:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [segment(new TextRange(0, 1), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(0, 1), 0.0, 0.0, 10.0, 20.0, 15.0)]);
        TracedAssertions.assertEqualsFloat(10.0, out[0].right);
    }

    @:test
    public static function decorationLineYWithoutSpansUsesTheParagraphStyle():Void {
        currentTrace().section("decorationLineYWithoutSpansUsesTheParagraphStyle");
        final content:LayoutResult = result("ab", [cluster(new TextRange(0, 2), "ab", 20.0)], [line(new TextRange(0, 2), 0, 0, 0.0, 20.0, 15.0, 0.0, 20.0)], [], [], [], emptyDebug(), style(10.0));
        final value:Float = LayoutQueries.richTextDecorationLineY(content, segment(new TextRange(0, 2), RichTextRole.Underline, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(0, 2), 0.0, 0.0, 20.0, 20.0, 15.0), 1.0);
        TracedAssertions.assertEqualsFloat(15.0 + 10.0 * 0.18, value);
    }

    @:test
    public static function wordBoundaryForPositionHandlesANonFiniteY():Void {
        currentTrace().section("wordBoundaryForPositionHandlesANonFiniteY");
        final content:LayoutResult = result("甲乙", [cluster(new TextRange(0, 1), "甲", 10.0), cluster(new TextRange(1, 2), "乙", 10.0)], [line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0)], [], [], [], emptyDebug(), style(10.0));
        TracedAssertions.assertEqualsRendered(new TextRange(0, 1).toString(), LayoutQueries.getSelectionWordBoundaryForPosition(content, 5.0, nan()).toString());
    }

    @:test
    public static function supplementaryIdeographBeyondTheHanRangesIsItsOwnUnit():Void {
        currentTrace().section("supplementaryIdeographBeyondTheHanRangesIsItsOwnUnit");
        final text:String = TestHelpers.surrogateText([0xD880, 0xDC00]);
        final content:LayoutResult = result(text, [cluster(new TextRange(0, 2), text, 10.0)], [line(new TextRange(0, 2), 0, 0, 0.0, 20.0, 15.0, 0.0, 10.0)], [], [], [], emptyDebug(), style(10.0));
        TracedAssertions.assertEqualsRendered(new TextRange(0, 2).toString(), LayoutQueries.getSelectionWordBoundary(content, 0).toString());
    }

    @:test
    public static function planeFourCodepointAboveTheHanBandsIsItsOwnUnit():Void {
        currentTrace().section("planeFourCodepointAboveTheHanBandsIsItsOwnUnit");
        final text:String = TestHelpers.surrogateText([0xD900, 0xDC00]);
        final content:LayoutResult = result(text, [cluster(new TextRange(0, 2), text, 10.0)], [line(new TextRange(0, 2), 0, 0, 0.0, 20.0, 15.0, 0.0, 10.0)], [], [], [], emptyDebug(), style(10.0));
        TracedAssertions.assertEqualsRendered(new TextRange(0, 2).toString(), LayoutQueries.getSelectionWordBoundary(content, 0).toString());
    }

    @:test
    public static function nearestLineSearchCoversAllThreeDistanceArms():Void {
        currentTrace().section("nearestLineSearchCoversAllThreeDistanceArms");
        final content:LayoutResult = result(
            "abcde", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(4, 5), "e", 10.0)],
            [line(new TextRange(0, 2), 0, 0, 0.0, 20.0, 15.0, 0.0, 10.0), line(new TextRange(4, 5), 1, 1, 20.0, 40.0, 35.0, 0.0, 10.0)], [], [], [], emptyDebug(), style(10.0)
        );
        TracedAssertions.assertEqualsFloat(10.0, LayoutQueries.getCursorRect(content, 2).left);
        TracedAssertions.assertEqualsFloat(10.0, LayoutQueries.getCursorRect(content, 3).left);
    }

    @:test
    public static function rubiesOnOtherLinesDoNotAffectThisLineGeometry():Void {
        currentTrace().section("rubiesOnOtherLinesDoNotAffectThisLineGeometry");
        final content:LayoutResult = result(
            "ab", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0)],
            [line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0)], [], [], [],
            new LayoutDebugInfo(null, [], [], [], [new RubyDecisionInfo(new TextRange(0, 2), "zhù", 1, 10.0, 4.0, 6.0, 0.0, 0.0, 0.0, 30.0, [], 400, "zh-Hans", [])], []), style(10.0)
        );
        final positioned:Array<PositionedCluster> = LayoutQueries.positionedClusters(content);
        TracedAssertions.assertEqualsFloat(0.0, positioned[0].left);
        TracedAssertions.assertEqualsFloat(10.0, positioned[0].right);
        TracedAssertions.assertEqualsFloat(10.0, positioned[1].left);
        TracedAssertions.assertEqualsFloat(20.0, positioned[1].right);
    }

    @:test
    public static function backgroundTrailingEdgePicksTheLargestGlyphAdvance():Void {
        currentTrace().section("backgroundTrailingEdgePicksTheLargestGlyphAdvance");
        final clusters:Array<Cluster> = [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0)];
        final runs:Array<GlyphRun> = [new GlyphRun(new TextRange(1, 2), "test", [new Glyph(1, new TextRange(1, 2), 5.0, 0.0, 0.0, null, null, null, null), new Glyph(2, new TextRange(1, 2), 6.0, 0.0, 0.0, null, null, null, null)], 10.0, [])];
        final content:LayoutResult = result("ab", clusters, [line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0)], runs, [], [], emptyDebug(), style(10.0));
        final output:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [segment(new TextRange(0, 2), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(0, 2), 0.0, 0.0, 20.0, 20.0, 15.0)]);
        TracedAssertions.assertEqualsFloat(16.0, output[0].right);
    }

    @:test
    public static function backgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargest():Void {
        currentTrace().section("backgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargest");
        final clusters:Array<Cluster> = [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0)];
        final runs:Array<GlyphRun> = [new GlyphRun(new TextRange(1, 2), "test", [new Glyph(1, new TextRange(1, 2), 6.0, 0.0, 0.0, null, null, null, null), new Glyph(2, new TextRange(1, 2), 5.0, 0.0, 0.0, null, null, null, null)], 10.0, [])];
        final content:LayoutResult = result("ab", clusters, [line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0)], runs, [], [], emptyDebug(), style(10.0));
        final output:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [segment(new TextRange(0, 2), RichTextRole.Background, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(0, 2), 0.0, 0.0, 20.0, 20.0, 15.0)]);
        TracedAssertions.assertEqualsFloat(16.0, output[0].right);
    }

    @:test
    public static function selectionWordBoundaryForPositionPrefersTheCloserLaterLine():Void {
        currentTrace().section("selectionWordBoundaryForPositionPrefersTheCloserLaterLine");
        final content:LayoutResult = result(
            "甲乙丙丁",
            [cluster(new TextRange(0, 1), "甲", 10.0), cluster(new TextRange(1, 2), "乙", 10.0), cluster(new TextRange(2, 3), "丙", 10.0), cluster(new TextRange(3, 4), "丁", 10.0)],
            [line(new TextRange(0, 2), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0), line(new TextRange(2, 4), 2, 3, 40.0, 60.0, 55.0, 0.0, 20.0)], [], [], [], emptyDebug(), style(10.0)
        );
        TracedAssertions.assertEqualsRendered(new TextRange(2, 3).toString(), LayoutQueries.getSelectionWordBoundaryForPosition(content, 5.0, 50.0).toString());
        TracedAssertions.assertEqualsRendered(new TextRange(0, 1).toString(), LayoutQueries.getSelectionWordBoundaryForPosition(content, 5.0, 30.0).toString());
        TracedAssertions.assertEqualsRendered(new TextRange(0, 1).toString(), LayoutQueries.getSelectionWordBoundaryForPosition(content, 5.0, -10.0).toString());
        TracedAssertions.assertEqualsRendered(new TextRange(0, 1).toString(), LayoutQueries.getSelectionWordBoundaryForPosition(content, 5.0, 10.0).toString());
        TracedAssertions.assertEqualsRendered(new TextRange(2, 3).toString(), LayoutQueries.getSelectionWordBoundaryForPosition(content, 5.0, 100.0).toString());
    }

    @:test
    public static function nearestLineSearchUpdatesToAStrictlyCloserLaterLine():Void {
        currentTrace().section("nearestLineSearchUpdatesToAStrictlyCloserLaterLine");
        final content:LayoutResult = result(
            "abcde", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(5, 6), "e", 10.0)],
            [line(new TextRange(0, 2), 0, 0, 0.0, 20.0, 15.0, 0.0, 10.0), line(new TextRange(5, 7), 1, 1, 20.0, 40.0, 35.0, 10.0, 10.0)], [], [], [], emptyDebug(), style(10.0)
        );
        TracedAssertions.assertEqualsFloat(10.0, LayoutQueries.getCursorRect(content, 4).left);
    }

    @:test
    public static function nearestLineSearchCoversBothLambdaCopiesOfEachArm():Void {
        currentTrace().section("nearestLineSearchCoversBothLambdaCopiesOfEachArm");
        final content:LayoutResult = result(
            "abcdefghij",
            [cluster(new TextRange(2, 3), "c", 10.0), cluster(new TextRange(3, 4), "d", 10.0), cluster(new TextRange(6, 7), "g", 10.0), cluster(new TextRange(7, 8), "h", 10.0)],
            [line(new TextRange(2, 4), 0, 1, 0.0, 20.0, 15.0, 0.0, 20.0), line(new TextRange(6, 8), 2, 3, 20.0, 40.0, 35.0, 0.0, 20.0)], [], [], [], emptyDebug(), style(10.0)
        );
        TracedAssertions.assertEqualsFloat(0.0, LayoutQueries.getCursorRect(content, 1).left);
        TracedAssertions.assertEqualsFloat(20.0, LayoutQueries.getCursorRect(content, 8).left);
        TracedAssertions.assertEqualsFloat(20.0, LayoutQueries.getCursorRect(content, 9).left);
    }

    @:test
    public static function uniformTextStylePolicyPicksTheLastMatchingSpan():Void {
        currentTrace().section("uniformTextStylePolicyPicksTheLastMatchingSpan");
        final uniform:RichTextPaint = new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.UniformTextStyle, RichTextBackgroundDrawStyle.Fill), 0.0);
        final content:LayoutResult = result(
            "abc", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0), cluster(new TextRange(2, 3), "c", 10.0)],
            [line(new TextRange(0, 3), 0, 2, 0.0, 20.0, 15.0, 0.0, 30.0)], [],
            [new TextSpan(new TextRange(0, 2), style(10.0)), new TextSpan(new TextRange(1, 3), style(40.0))], [], emptyDebug(), style(10.0)
        );
        final inside:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [segment(new TextRange(2, 3), RichTextRole.Background, uniform, 0, new TextRange(2, 3), 20.0, 0.0, 30.0, 20.0, 15.0)]);
        TracedAssertions.assertEqualsFloat(0.0, inside[0].top);
    }

    @:test
    public static function decorationLineYPicksTheLastMatchingSpan():Void {
        currentTrace().section("decorationLineYPicksTheLastMatchingSpan");
        final content:LayoutResult = result(
            "abc", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0), cluster(new TextRange(2, 3), "c", 10.0)],
            [line(new TextRange(0, 3), 0, 2, 0.0, 20.0, 15.0, 0.0, 30.0)], [],
            [new TextSpan(new TextRange(0, 2), style(10.0)), new TextSpan(new TextRange(1, 3), style(20.0))], [], emptyDebug(), style(10.0)
        );
        final value:Float = LayoutQueries.richTextDecorationLineY(content, segment(new TextRange(2, 3), RichTextRole.Underline, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(2, 3), 20.0, 0.0, 30.0, 20.0, 15.0), 1.0);
        TracedAssertions.assertEqualsFloat(15.0 + 20.0 * 0.18, value);
    }

    @:test
    public static function uniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMisses():Void {
        currentTrace().section("uniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMisses");
        final uniform:RichTextPaint = new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.UniformTextStyle, RichTextBackgroundDrawStyle.Fill), 0.0);
        final content:LayoutResult = result(
            "abc", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0), cluster(new TextRange(2, 3), "c", 10.0)],
            [line(new TextRange(0, 3), 0, 2, 0.0, 20.0, 15.0, 0.0, 30.0)], [],
            [new TextSpan(new TextRange(0, 3), style(40.0)), new TextSpan(new TextRange(1, 2), style(10.0))], [], emptyDebug(), style(10.0)
        );
        final output:Array<RichTextLineSegment> = LayoutQueries.richTextBackgroundSegments(content, [segment(new TextRange(0, 1), RichTextRole.Background, uniform, 0, new TextRange(0, 1), 0.0, 0.0, 10.0, 20.0, 15.0)]);
        TracedAssertions.assertEqualsFloat(0.0, output[0].top);
    }

    @:test
    public static function decorationLineYKeepsTheEarlierSpanWhenALaterOneMisses():Void {
        currentTrace().section("decorationLineYKeepsTheEarlierSpanWhenALaterOneMisses");
        final content:LayoutResult = result(
            "abc", [cluster(new TextRange(0, 1), "a", 10.0), cluster(new TextRange(1, 2), "b", 10.0), cluster(new TextRange(2, 3), "c", 10.0)],
            [line(new TextRange(0, 3), 0, 2, 0.0, 20.0, 15.0, 0.0, 30.0)], [],
            [new TextSpan(new TextRange(0, 3), style(20.0)), new TextSpan(new TextRange(1, 2), style(10.0))], [], emptyDebug(), style(10.0)
        );
        final value:Float = LayoutQueries.richTextDecorationLineY(content, segment(new TextRange(0, 1), RichTextRole.Underline, new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0), 0, new TextRange(0, 1), 0.0, 0.0, 10.0, 20.0, 15.0), 1.0);
        TracedAssertions.assertEqualsFloat(15.0 + 20.0 * 0.18, value);
    }

    public static function flushTestTrace():Void {
        currentTrace().flush();
    }
}
