package org.tiqian.protocol

data class DecorationInput(
    var start: Int,
    var end: Int,
    var kind: String
)

fun compare(a: DecorationInput, b: DecorationInput): Int {
    if (a === b) return 0
    var cmp = 0
    cmp = a.start.compareTo(b.start)
    if (cmp != 0) return cmp
    cmp = a.end.compareTo(b.end)
    if (cmp != 0) return cmp
    cmp = a.kind.compareTo(b.kind)
    if (cmp != 0) return cmp
    return 0
}
