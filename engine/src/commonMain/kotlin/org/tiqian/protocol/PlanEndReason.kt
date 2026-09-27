package org.tiqian.protocol

enum class PlanEndReason {
    AutoWrap,
    MandatoryBreak,
    ParagraphEnd
}
fun comparePlanEndReason(a: PlanEndReason, b: PlanEndReason): Int = a.ordinal - b.ordinal
