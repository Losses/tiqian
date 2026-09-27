package org.tiqian.protocol

data class ParagraphRequest(
    var fontSessionId: String,
    var text: String,
    var maxWidthPx: Double,
    var fontFamilies: MutableList<String>,
    var fontSizePx: Double,
    var lineHeightPx: Double,
    var locale: String,
    var fontWeight: Int,
    var italic: Boolean,
    var firstLineIndentIc: Double,
    var lineLengthGridEnabled: Boolean,
    var emphasisDotGapEm: Double?,
    var sourceBoundaries: MutableList<Int>,
    var textSpans: MutableList<TextSpanInput>,
    var lineBreakSpans: MutableList<LineBreakSpanInput>,
    var inlineBoxes: MutableList<InlineBoxInput>,
    var inlineObjects: MutableList<InlineObjectInput>,
    var decorations: MutableList<DecorationInput>
)
