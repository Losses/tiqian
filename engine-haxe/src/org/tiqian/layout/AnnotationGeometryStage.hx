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
import org.tiqian.font.FontRole;
import org.tiqian.layout.LineGeometryStage.ClusterMetricDecision;
import org.tiqian.layout.LineBreakPlanningStage;
import std.SortedMap;
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
}
