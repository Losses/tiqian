package org.tiqian.protocol

data class InlineBoxInput(
    var start: Int,
    var end: Int,
    var inlineStart: Double,
    var inlineEnd: Double,
    var outerSpacing: String
)
