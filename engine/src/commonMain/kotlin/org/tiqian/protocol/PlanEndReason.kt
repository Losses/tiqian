package org.tiqian.protocol

@Deprecated("Legacy engine, migrate to engine-haxe")
enum class PlanEndReason {
    AutoWrap,
    MandatoryBreak,
    ParagraphEnd
}
@Deprecated("Legacy engine, migrate to engine-haxe")
fun comparePlanEndReason(a: PlanEndReason, b: PlanEndReason): Int = a.ordinal - b.ordinal
