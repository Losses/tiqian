package org.tiqian.layout;

import org.tiqian.core.Glyph;
import org.tiqian.core.InlineObjectDecisionInfo;
import org.tiqian.core.DecorationDecisionInfo;
import org.tiqian.core.DecorationSegmentInfo;
import org.tiqian.core.RubyDecisionInfo;
import org.tiqian.core.BopomofoDecisionInfo;
import org.tiqian.core.DecorationSpan;
import org.tiqian.core.DecorationKind;
import org.tiqian.core.IntRange;
import org.tiqian.core.LineBox;
import org.tiqian.core.Cluster;
import org.tiqian.core.TextRange;
import org.tiqian.core.ClusterGeometryDecisionInfo;
import org.tiqian.font.FontRole;
import org.tiqian.layout.LineGeometryStage.ClusterMetricDecision;
import org.tiqian.layout.LineBreakPlanningStage;
import std.SortedMap;
import std.SortedSet;
import std.ReadOnlyArray;

/*
 * Partial port: only RubyFontGeometry (Kotlin AnnotationGeometryStage.kt:790) is
 * translated so far because LineGeometryStage.kt:254/:273 consumes it. The rest
 * of AnnotationGeometryStage.kt lands in a later lane and must extend this file.
 */
@:dataClass class RubyFontGeometry {
    public final width:Float;
    public final ascent:Float;
    public final descent:Float;
    public final requiredExtent:Float;
    public final glyphs:Array<Glyph>;

    public function new(width:Float, ascent:Float, descent:Float, requiredExtent:Float, glyphs:Array<Glyph>) {
        this.width = width;
        this.ascent = ascent;
        this.descent = descent;
        this.requiredExtent = requiredExtent;
        this.glyphs = glyphs;
    }
}

@:dataClass class AnnotationGeometryStageResult {
    public final inlineObjectDecisions:Array<InlineObjectDecisionInfo>;
    public final decorationDecisions:Array<DecorationDecisionInfo>;
    public final decorationSegments:Array<DecorationSegmentInfo>;
    public final rubyDecisions:Array<RubyDecisionInfo>;
    public final bopomofoDecisions:Array<BopomofoDecisionInfo>;

    public function new(inlineObjectDecisions:Array<InlineObjectDecisionInfo>, decorationDecisions:Array<DecorationDecisionInfo>,
            decorationSegments:Array<DecorationSegmentInfo>, rubyDecisions:Array<RubyDecisionInfo>, bopomofoDecisions:Array<BopomofoDecisionInfo>) {
        this.inlineObjectDecisions = inlineObjectDecisions;
        this.decorationDecisions = decorationDecisions;
        this.decorationSegments = decorationSegments;
        this.rubyDecisions = rubyDecisions;
        this.bopomofoDecisions = bopomofoDecisions;
    }
}

class AnnotationGeometryStage {
    public static inline final EMPHASIS_DOT_DIAMETER_EM:Float = 0.19;
    public static inline final BOPOMOFO_ANNOTATION_FONT_EM:Float = 0.3;
    public static inline final BOPOMOFO_SYMBOL_BASELINE_FACTOR:Float = 0.88;
    public static inline final MOURNING_FRAME_FACE_ASCENT_EM:Float = 0.88;
    public static inline final MOURNING_FRAME_FACE_DESCENT_EM:Float = 0.12;
    public static inline final INTERLINEAR_LINE_Y_EM:Float = 0.18;
    public static inline final BOOK_TITLE_WAVE_LINE_Y_EM:Float = 0.24;
    public static inline final ADJACENT_LINE_SHORTEN_EM:Float = 0.0625;
    public static inline final ADJACENT_LINE_EPSILON:Float = 0.01;

    /**
     * Named heuristic: `EmphasisDotOnHanText` (ADR 0018, CLREQ 着重号).
     *
     * Resolves decoration spans into per-cluster dot anchors AFTER all
     * geometry is final — decorations never feed back into metrics, breaks
     * or justification. Per CLREQ, only Han text carries a dot: punctuation
     * inside the span is skipped (`clreq-no-dot-on-punctuation`), and
     * non-Han clusters are skipped (`no-dot-on-non-han`; western emphasis is
     * italics instead — `BilingualEmphasisWesternItalic`, applied at shaping).
     *
     * Anchor = the point the dot INK CENTRE must land on: x is the glyph
     * centre (final position minus the trailing justification delta); y starts
     * at the annotated cluster's real ideographic-face bottom, then adds
     * `ParagraphStyle.emphasisDotGapEm·clusterEm + dotRadius`. This
     * `ExplicitEmphasisDotGap` is independent of line height and stays correct
     * for mixed font sizes and explicit baseline shifts. [dotDiameter] is final
     * paint geometry: renderers draw it exactly and apply no hidden scaling.
     */
    public static function computeDecorationDecisions(
        decorations:ReadOnlyArray<DecorationSpan>,
        lineRanges:Array<IntRange>,
        lineBoxes:Array<LineBox>,
        finalClusters:Array<Cluster>,
        clusterRoles:Array<FontRole>,
        justifyDeltaByCluster:SortedMap<Int, Float>,
        rubySpreadByCluster:SortedMap<Int, Float>,
        metricDecisions:Array<ClusterMetricDecision>,
        fontSize:Float,
        emphasisDotGapEm:Float
    ):Array<DecorationDecisionInfo> {
        if (decorations.length == 0) return [];

        final decisions = new Array<DecorationDecisionInfo>();
        for (si in 0...decorations.length) {
            final span = decorations[si];
            if (span.kind != DecorationKind.Emphasis) continue;
            for (lineIndex in 0...lineRanges.length) {
                final clusterRange = lineRanges[lineIndex];
                var x = lineBoxes[lineIndex].indent;
                for (idx in clusterRange.start...clusterRange.end + 1) {
                    final cluster = finalClusters[idx];
                    final coveredBySpan = cluster.range.start >= span.range.start &&
                        cluster.range.end <= span.range.end;
                    if (coveredBySpan) {
                        final role = clusterRoles[idx];
                        final applied = role == FontRole.CjkText;
                        // Centre on the base BODY: drop the trailing justify stretch AND
                        // the 注音 column reservation (着重号 belongs under 基文, not 基文+注音).
                        final justifyDelta = justifyDeltaByCluster.has(idx) ? justifyDeltaByCluster.get(idx) : 0.0;
                        final rubySpread = rubySpreadByCluster.has(idx) ? rubySpreadByCluster.get(idx) : 0.0;
                        final glyphAdvance = cluster.advance - justifyDelta - rubySpread;
                        var metric:Null<ClusterMetricDecision> = null;
                        for (mi in 0...metricDecisions.length) {
                            final m = metricDecisions[mi];
                            if (cluster.range.start >= m.range.start && cluster.range.end <= m.range.end) {
                                metric = m;
                                break;
                            }
                        }
                        final clusterEm = (metric != null && metric.request != null) ? metric.request.fontSize : fontSize;
                        final faceDescent = (metric != null && metric.layoutMetrics != null) ? metric.layoutMetrics.descent
                            : clusterEm * LineBreakPlanningStage.CJK_FACE_DESCENT_FALLBACK_EM;
                        final candidateDotDiameter = clusterEm * EMPHASIS_DOT_DIAMETER_EM;
                        final dotDiameter = applied ? candidateDotDiameter : 0.0;
                        final reason = if (applied) {
                            "EmphasisDotOnHanText";
                        } else if (role == FontRole.CjkPunctuation) {
                            "clreq-no-dot-on-punctuation";
                        } else {
                            "no-dot-on-non-han";
                        };
                        decisions.push(new DecorationDecisionInfo(
                            cluster.range,
                            cluster.text,
                            Std.string(span.kind),
                            applied,
                            reason,
                            x + glyphAdvance / 2.0,
                            lineBoxes[lineIndex].baseline + cluster.baselineShift +
                                faceDescent + clusterEm * emphasisDotGapEm + candidateDotDiameter / 2.0,
                            dotDiameter
                        ));
                    }
                    x += cluster.advance;
                }
            }
        }
        return decisions;
    }

    public static function shortenAdjacentInterlinearLines(
        segments:Array<DecorationSegmentInfo>,
        fontSize:Float
    ):Array<DecorationSegmentInfo> {
        return segments;
    }

    /**
     * 示亡号 frame geometry (ADR 0018). One rectangle per line the span
     * touches. Vertical bounds are the conventional CJK CHARACTER FACE
     * (字面): `baseline - 0.88em .. baseline + 0.12em`, hugging the face
     * with NO margin. Neither layout em box (artificial 0.5/0.5 split that
     * real ink overflows), nor raw line metrics (include inter-line air),
     * nor per-glyph ink (varies with glyph shape — `一` would collapse the
     * frame and break uniformity across a name list) describe the face;
     * the 0.88/0.12 split encodes the standard CJK design box. Replacing
     * it with font-reported ideographic metrics (BASE table) is follow-up.
     * `openStart`/`openEnd` mark continuation edges when the span had to
     * split across lines (only when wider than the measure —
     * `MourningSpanKeptUnbroken` otherwise prevents the split at break
     * time).
     */
    public static function computeDecorationSegments(
        decorations:ReadOnlyArray<DecorationSpan>,
        lineRanges:Array<IntRange>,
        lineBoxes:Array<LineBox>,
        finalClusters:Array<Cluster>,
        justifyDeltaByCluster:SortedMap<Int, Float>,
        geometryByRange:SortedMap<TextRange, ClusterGeometryDecisionInfo>,
        leadingGapRanges:SortedSet<TextRange>,
        trailingGapRanges:SortedSet<TextRange>,
        autoSpaceGapPx:Float,
        fontSize:Float
    ):Array<DecorationSegmentInfo> {
        // Remaining edge blank to strip off a covered cluster so 行间线 hugs the ink/body
        // (CLREQ 避两侧空白): the autospace gap + the punctuation glue still present
        // (开/闭标点 half-width), mirroring how the renderer positions the glyph.
        final leadingBlank = function(range:TextRange, atLineStart:Bool):Float {
            final g = geometryByRange.has(range) ? geometryByRange.get(range) : null;
            final glue = g != null ? (g.leadingGlueNatural - g.leadingGlueConsumed) : 0.0;
            final auto = (leadingGapRanges.has(range) && !atLineStart) ? autoSpaceGapPx : 0.0;
            return glue + auto;
        };
        final trailingBlank = function(range:TextRange, atLineEnd:Bool):Float {
            final g = geometryByRange.has(range) ? geometryByRange.get(range) : null;
            final glue = g != null ? (g.trailingGlueNatural - g.trailingGlueConsumed) : 0.0;
            final auto = (trailingGapRanges.has(range) && !atLineEnd) ? autoSpaceGapPx : 0.0;
            return glue + auto;
        };
        final boxSpans = new Array<DecorationSpan>();
        for (si in 0...decorations.length) {
            final s = decorations[si];
            if (s.kind == DecorationKind.Mourning || s.kind == DecorationKind.ProperNoun || s.kind == DecorationKind.BookTitle) {
                boxSpans.push(s);
            }
        }
        if (boxSpans.length == 0) return [];

        final segments = new Array<DecorationSegmentInfo>();
        for (bi in 0...boxSpans.length) {
            final span = boxSpans[bi];
            final spanSegments = new Array<DecorationSegmentInfo>();
            for (lineIndex in 0...lineRanges.length) {
                final clusterRange = lineRanges[lineIndex];
                var x = lineBoxes[lineIndex].indent;
                var left:Null<Float> = null;
                var right:Float = 0.0;
                var segStart:Int = -1;
                var segEnd:Int = -1;
                for (idx in clusterRange.start...clusterRange.end + 1) {
                    final cluster = finalClusters[idx];
                    final covered = cluster.range.start >= span.range.start &&
                        cluster.range.end <= span.range.end;
                    if (covered) {
                        if (left == null) {
                            // Start at the first covered cluster's ink/body left: skip the
                            // leading blank (autospace + 开标点 glue), CLREQ 避两侧空白.
                            left = x + leadingBlank(cluster.range, idx == clusterRange.start);
                            segStart = cluster.range.start;
                        }
                        // End at the last covered cluster's ink/body right: drop the
                        // trailing justify stretch AND the trailing blank (autospace +
                        // 闭标点 glue) — 长度与文字外框一致, both sides.
                        final justifyDelta = justifyDeltaByCluster.has(idx) ? justifyDeltaByCluster.get(idx) : 0.0;
                        right = x + cluster.advance - justifyDelta - trailingBlank(cluster.range, idx == clusterRange.end);
                        segEnd = cluster.range.end;
                    }
                    x += cluster.advance;
                }
                if (left == null) continue;
                final leftEdge = left;
                final baseline = lineBoxes[lineIndex].baseline;
                final isLine = span.kind != DecorationKind.Mourning;
                // 行间线贴字：face bottom (+0.12em) plus a hairline of air.
                // At the default 0.1em emphasis gap, dot ink starts at +0.22em,
                // so the +0.18em line remains first.
                // The straight line's centre and the wavy line's upper envelope keep the same
                // visual clearance from the face. A shared centre line made the wave crest rise
                // 0.06em into that clearance and touch the glyphs.
                final lineYEm = (span.kind == DecorationKind.BookTitle) ? BOOK_TITLE_WAVE_LINE_Y_EM : INTERLINEAR_LINE_Y_EM;
                final lineY = baseline + fontSize * lineYEm;
                spanSegments.push(new DecorationSegmentInfo(
                    new TextRange(segStart, segEnd),
                    Std.string(span.kind),
                    lineIndex,
                    leftEdge,
                    isLine ? lineY : baseline - fontSize * MOURNING_FRAME_FACE_ASCENT_EM,
                    right,
                    isLine ? lineY : baseline + fontSize * MOURNING_FRAME_FACE_DESCENT_EM,
                    segStart > span.range.start,
                    segEnd < span.range.end,
                    ""
                ));
            }
            final reason = if (span.kind == DecorationKind.Mourning && spanSegments.length <= 1) {
                "MourningSpanKeptUnbroken";
            } else if (span.kind == DecorationKind.Mourning) {
                "mourning-span-split-across-lines";
            } else {
                "InterlinearLinePerAnnotatedItem";
            };
            for (ssi in 0...spanSegments.length) {
                final seg = spanSegments[ssi];
                segments.push(new DecorationSegmentInfo(
                    seg.sourceRange,
                    seg.kind,
                    seg.lineIndex,
                    seg.left,
                    seg.top,
                    seg.right,
                    seg.bottom,
                    seg.openStart,
                    seg.openEnd,
                    reason
                ));
            }
        }
        return shortenAdjacentInterlinearLines(segments, fontSize);
    }
}
