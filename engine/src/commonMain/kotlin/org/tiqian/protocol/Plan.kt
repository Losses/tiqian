package org.tiqian.protocol

@Deprecated("Legacy engine, migrate to engine-haxe")
data class PlanEmphasisRange(
    var start: Int,
    var end: Int
)

@Deprecated("Legacy engine, migrate to engine-haxe")
fun compare(a: PlanEmphasisRange, b: PlanEmphasisRange): Int {
    if (a === b) return 0
    var cmp = 0
    cmp = a.start.compareTo(b.start)
    if (cmp != 0) return cmp
    cmp = a.end.compareTo(b.end)
    if (cmp != 0) return cmp
    return 0
}

@Deprecated("Legacy engine, migrate to engine-haxe")
data class PlanInlineEdge(
    var offset: Int,
    var inlineStart: Float?,
    var inlineEnd: Float?
)

@Deprecated("Legacy engine, migrate to engine-haxe")
data class PlanRuby(
    var baseRangeStart: Int,
    var baseRangeEnd: Int,
    var text: String,
    var centerX: Float,
    var baselineY: Float,
    var fontSize: Float,
    var fontWeight: Int,
    var fontFamilies: MutableList<String>,
    var ascent: Float?
)

@Deprecated("Legacy engine, migrate to engine-haxe")
data class PlanBopomofo(
    var baseRangeStart: Int,
    var baseRangeEnd: Int,
    var text: String,
    var fontWeight: Int,
    var fontFamilies: MutableList<String>,
    var placements: MutableList<PlanBopomofoPlacement>
)

@Deprecated("Legacy engine, migrate to engine-haxe")
data class PlanBopomofoPlacement(
    var text: String,
    var role: String,
    var left: Float,
    var top: Float,
    var width: Float,
    var height: Float
)

@Deprecated("Legacy engine, migrate to engine-haxe")
data class PlanDecorationSegment(
    var kind: String,
    var left: Float,
    var top: Float,
    var right: Float,
    var sourceRangeStart: Int,
    var sourceRangeEnd: Int
)

@Deprecated("Legacy engine, migrate to engine-haxe")
data class PlanEmphasisDot(
    var clusterRangeStart: Float?,
    var anchorX: Float,
    var anchorY: Float,
    var dotDiameter: Float
)

@Deprecated("Legacy engine, migrate to engine-haxe")
data class PlanLine(
    var rangeStart: Int,
    var rangeEnd: Int,
    var top: Float,
    var bottom: Float,
    var baseline: Float,
    var indent: Float,
    var visualWidth: Float,
    var hyphenAdvance: Float,
    var endReason: PlanEndReason,
    var cells: MutableList<PlanCell>
)

@Deprecated("Legacy engine, migrate to engine-haxe")
data class PlanCell(
    var rangeStart: Int,
    var rangeEnd: Int,
    var source: String,
    var display: String,
    var drawX: Float,
    var naturalWidth: Float,
    var leadingLayoutAdvance: Float,
    var shapingBoundary: Boolean,
    var openTypeFeatures: MutableList<String>,
    var renderFontFamily: String?,
    var dashStrategy: String?,
    var shapingLanguage: String?,
    var resolvedFace: String?,
    var glyphIds: String?,
    var shapingEvidence: String?,
    var punctuationInkFloor: Float?,
    var punctuationBodyWidth: Float?,
    var latin: Boolean,
    var advance: Float?,
    var inlineObject: Float?,
    var styleDelta: PlanStyleDelta?
)

@Deprecated("Legacy engine, migrate to engine-haxe")
data class Plan(
    var width: Float,
    var height: Float,
    var lines: MutableList<PlanLine>,
    var emphasisRanges: MutableList<PlanEmphasisRange>,
    var inlineEdges: MutableList<PlanInlineEdge>,
    var rubyDecisions: MutableList<PlanRuby>,
    var bopomofoDecisions: MutableList<PlanBopomofo>,
    var fontSize: Float?,
    var overlayWidth: Float?,
    var decorationSegments: MutableList<PlanDecorationSegment>,
    var emphasisDots: MutableList<PlanEmphasisDot>
)
