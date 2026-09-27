package org.tiqian.protocol

data class LineBreakSpanInput(
    var start: Int,
    var end: Int,
    var policy: String
)

fun compare(a: LineBreakSpanInput, b: LineBreakSpanInput): Int {
    if (a === b) return 0
    var cmp = 0
    cmp = a.start.compareTo(b.start)
    if (cmp != 0) return cmp
    cmp = a.end.compareTo(b.end)
    if (cmp != 0) return cmp
    cmp = a.policy.compareTo(b.policy)
    if (cmp != 0) return cmp
    return 0
}
