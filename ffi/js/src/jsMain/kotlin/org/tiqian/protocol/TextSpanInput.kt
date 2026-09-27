package org.tiqian.protocol

data class TextSpanInput(
    var start: Int,
    var end: Int,
    var families: MutableList<String>,
    var fontSizePx: Double,
    var fontWeight: Int,
    var italic: Boolean,
    var baselineShift: Double
)
