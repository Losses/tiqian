package org.tiqian.protocol

data class PlanEmphasisRange(
    var start: Int,
    var end: Int
)

fun compare(a: PlanEmphasisRange, b: PlanEmphasisRange): Int {
    if (a === b) return 0
    var cmp = 0
    cmp = a.start.compareTo(b.start)
    if (cmp != 0) return cmp
    cmp = a.end.compareTo(b.end)
    if (cmp != 0) return cmp
    return 0
}

data class PlanInlineEdge(
    var offset: Int,
    var inlineStart: Double?,
    var inlineEnd: Double?
)

data class PlanRuby(
    var baseRangeStart: Int,
    var baseRangeEnd: Int,
    var text: String,
    var centerX: Double,
    var baselineY: Double,
    var fontSize: Double,
    var fontWeight: Int,
    var fontFamilies: MutableList<String>,
    var ascent: Double?
)

data class PlanBopomofo(
    var baseRangeStart: Int,
    var baseRangeEnd: Int,
    var text: String,
    var fontWeight: Int,
    var fontFamilies: MutableList<String>,
    var placements: MutableList<PlanBopomofoPlacement>
)

data class PlanBopomofoPlacement(
    var text: String,
    var role: String,
    var left: Double,
    var top: Double,
    var width: Double,
    var height: Double
)

data class PlanDecorationSegment(
    var kind: String,
    var left: Double,
    var top: Double,
    var right: Double,
    var sourceRangeStart: Int,
    var sourceRangeEnd: Int
)

data class PlanEmphasisDot(
    var clusterRangeStart: Double?,
    var anchorX: Double,
    var anchorY: Double,
    var dotDiameter: Double
)

data class PlanLine(
    var rangeStart: Int,
    var rangeEnd: Int,
    var top: Double,
    var bottom: Double,
    var baseline: Double,
    var indent: Double,
    var visualWidth: Double,
    var hyphenAdvance: Double,
    var endReason: PlanEndReason,
    var cells: MutableList<PlanCell>
)

data class PlanCell(
    var rangeStart: Int,
    var rangeEnd: Int,
    var source: String,
    var display: String,
    var drawX: Double,
    var naturalWidth: Double,
    var leadingLayoutAdvance: Double,
    var shapingBoundary: Boolean,
    var openTypeFeatures: MutableList<String>,
    var renderFontFamily: String?,
    var dashStrategy: String?,
    var shapingLanguage: String?,
    var resolvedFace: String?,
    var glyphIds: String?,
    var shapingEvidence: String?,
    var punctuationInkFloor: Double?,
    var punctuationBodyWidth: Double?,
    var latin: Boolean,
    var advance: Double?,
    var inlineObject: Double?,
    var styleDelta: PlanStyleDelta?
)

data class Plan(
    var width: Double,
    var height: Double,
    var lines: MutableList<PlanLine>,
    var emphasisRanges: MutableList<PlanEmphasisRange>,
    var inlineEdges: MutableList<PlanInlineEdge>,
    var rubyDecisions: MutableList<PlanRuby>,
    var bopomofoDecisions: MutableList<PlanBopomofo>,
    var fontSize: Double?,
    var overlayWidth: Double?,
    var decorationSegments: MutableList<PlanDecorationSegment>,
    var emphasisDots: MutableList<PlanEmphasisDot>
)
