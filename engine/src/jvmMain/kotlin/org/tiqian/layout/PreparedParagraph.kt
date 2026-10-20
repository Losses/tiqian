package org.tiqian.layout

import org.tiqian.boring.runtime.FPHelper
import org.tiqian.core.LayoutResult
import org.tiqian.protocol.PlanJson
import org.tiqian.protocol.PlanJsonNumber
import org.tiqian.protocol.PlanPacked
import std.UStringException

object PreparedParagraphFns {
    fun toPreparedParagraphJson(result: LayoutResult, renderEvidence: Boolean = false): String {
        val plan = PlanLowering.toPlan(result, renderEvidence)
        return PlanJson.encode(plan)
    }

    fun toPlanWithDiagnosticsJson(result: LayoutResult, renderEvidence: Boolean, zeroAdvanceEpsilonPx: Float): String {
        val out = StringBuilder()
        val tail = out.lastOrNull()?.code ?: -1
        if (tail >= 55296 && tail <= 56319 && "{\"plan\":".length > 0 && !("{\"plan\":"[0].code >= 56320 && "{\"plan\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail)
        }
        out.append("{\"plan\":")
        PlanJsonNumber.appendJsonString(out, PreparedParagraphFns.toPreparedParagraphJson(result, renderEvidence))
        val tail2 = out.lastOrNull()?.code ?: -1
        if (tail2 >= 55296 && tail2 <= 56319 && ",\"diagnostics\":{\"capabilityIssues\":[".length > 0 && !(",\"diagnostics\":{\"capabilityIssues\":["[0].code >= 56320 && ",\"diagnostics\":{\"capabilityIssues\":["[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail2)
        }
        out.append(",\"diagnostics\":{\"capabilityIssues\":[")
        var first = true
        val shapingDiagSrc = result.debug.shapingDecisions
        for (d in shapingDiagSrc) {
            if ((d.capabilityIssue != null)) {
                if ((!first)) {
                    val tail3 = out.lastOrNull()?.code ?: -1
                    if (tail3 >= 55296 && tail3 <= 56319 && ",".length > 0 && !(","[0].code >= 56320 && ","[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail3)
                    }
                    out.append(",")
                }
                first = false
                val tail4 = out.lastOrNull()?.code ?: -1
                if (tail4 >= 55296 && tail4 <= 56319 && "{\"name\":".length > 0 && !("{\"name\":"[0].code >= 56320 && "{\"name\":"[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail4)
                }
                out.append("{\"name\":")
                PlanJsonNumber.appendJsonString(out, d.capabilityIssue)
                val tail5 = out.lastOrNull()?.code ?: -1
                if (tail5 >= 55296 && tail5 <= 56319 && ",\"reason\":".length > 0 && !(",\"reason\":"[0].code >= 56320 && ",\"reason\":"[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail5)
                }
                out.append(",\"reason\":")
                PlanJsonNumber.appendJsonString(out, d.reason)
                val tail6 = out.lastOrNull()?.code ?: -1
                if (tail6 >= 55296 && tail6 <= 56319 && ",\"rangeStart\":".length > 0 && !(",\"rangeStart\":"[0].code >= 56320 && ",\"rangeStart\":"[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail6)
                }
                out.append(",\"rangeStart\":")
                val tail7 = out.lastOrNull()?.code ?: -1
                if (tail7 >= 55296 && tail7 <= 56319 && ((d.range.start).toString()).length > 0 && !(((d.range.start).toString())[0].code >= 56320 && ((d.range.start).toString())[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail7)
                }
                out.append(((d.range.start).toString()))
                val tail8 = out.lastOrNull()?.code ?: -1
                if (tail8 >= 55296 && tail8 <= 56319 && ",\"rangeEnd\":".length > 0 && !(",\"rangeEnd\":"[0].code >= 56320 && ",\"rangeEnd\":"[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail8)
                }
                out.append(",\"rangeEnd\":")
                val tail9 = out.lastOrNull()?.code ?: -1
                if (tail9 >= 55296 && tail9 <= 56319 && ((d.range.end).toString()).length > 0 && !(((d.range.end).toString())[0].code >= 56320 && ((d.range.end).toString())[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail9)
                }
                out.append(((d.range.end).toString()))
                val tail10 = out.lastOrNull()?.code ?: -1
                if (tail10 >= 55296 && tail10 <= 56319 && "}".length > 0 && !("}"[0].code >= 56320 && "}"[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail10)
                }
                out.append("}")
            }
        }
        val tail11 = out.lastOrNull()?.code ?: -1
        if (tail11 >= 55296 && tail11 <= 56319 && "],\"advanceSuspects\":[".length > 0 && !("],\"advanceSuspects\":["[0].code >= 56320 && "],\"advanceSuspects\":["[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail11)
        }
        out.append("],\"advanceSuspects\":[")
        first = true
        for (d_2 in shapingDiagSrc) {
            if ((!((d_2.advance).isFinite() && d_2.advance > zeroAdvanceEpsilonPx))) {
                if ((!first)) {
                    val tail12 = out.lastOrNull()?.code ?: -1
                    if (tail12 >= 55296 && tail12 <= 56319 && ",".length > 0 && !(","[0].code >= 56320 && ","[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail12)
                    }
                    out.append(",")
                }
                first = false
                val tail13 = out.lastOrNull()?.code ?: -1
                if (tail13 >= 55296 && tail13 <= 56319 && "{\"displayText\":".length > 0 && !("{\"displayText\":"[0].code >= 56320 && "{\"displayText\":"[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail13)
                }
                out.append("{\"displayText\":")
                PlanJsonNumber.appendJsonString(out, d_2.displayText)
                val tail14 = out.lastOrNull()?.code ?: -1
                if (tail14 >= 55296 && tail14 <= 56319 && ",\"advance\":\"".length > 0 && !(",\"advance\":\""[0].code >= 56320 && ",\"advance\":\""[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail14)
                }
                out.append(",\"advance\":\"")
                val tail15 = out.lastOrNull()?.code ?: -1
                if (tail15 >= 55296 && tail15 <= 56319 && ((if (((d_2.advance).isFinite())) PlanJsonNumber.ecmaJsonNumber(d_2.advance) else org.tiqian.boring.runtime.FPHelper.formatFloat(d_2.advance))).length > 0 && !(((if (((d_2.advance).isFinite())) PlanJsonNumber.ecmaJsonNumber(d_2.advance) else org.tiqian.boring.runtime.FPHelper.formatFloat(d_2.advance)))[0].code >= 56320 && ((if (((d_2.advance).isFinite())) PlanJsonNumber.ecmaJsonNumber(d_2.advance) else org.tiqian.boring.runtime.FPHelper.formatFloat(d_2.advance)))[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail15)
                }
                out.append(((if (((d_2.advance).isFinite())) PlanJsonNumber.ecmaJsonNumber(d_2.advance) else org.tiqian.boring.runtime.FPHelper.formatFloat(d_2.advance))))
                val tail16 = out.lastOrNull()?.code ?: -1
                if (tail16 >= 55296 && tail16 <= 56319 && "\",\"reason\":".length > 0 && !("\",\"reason\":"[0].code >= 56320 && "\",\"reason\":"[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail16)
                }
                out.append("\",\"reason\":")
                PlanJsonNumber.appendJsonString(out, d_2.reason)
                val tail17 = out.lastOrNull()?.code ?: -1
                if (tail17 >= 55296 && tail17 <= 56319 && ",\"rangeStart\":".length > 0 && !(",\"rangeStart\":"[0].code >= 56320 && ",\"rangeStart\":"[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail17)
                }
                out.append(",\"rangeStart\":")
                val tail18 = out.lastOrNull()?.code ?: -1
                if (tail18 >= 55296 && tail18 <= 56319 && ((d_2.range.start).toString()).length > 0 && !(((d_2.range.start).toString())[0].code >= 56320 && ((d_2.range.start).toString())[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail18)
                }
                out.append(((d_2.range.start).toString()))
                val tail19 = out.lastOrNull()?.code ?: -1
                if (tail19 >= 55296 && tail19 <= 56319 && ",\"rangeEnd\":".length > 0 && !(",\"rangeEnd\":"[0].code >= 56320 && ",\"rangeEnd\":"[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail19)
                }
                out.append(",\"rangeEnd\":")
                val tail20 = out.lastOrNull()?.code ?: -1
                if (tail20 >= 55296 && tail20 <= 56319 && ((d_2.range.end).toString()).length > 0 && !(((d_2.range.end).toString())[0].code >= 56320 && ((d_2.range.end).toString())[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail20)
                }
                out.append(((d_2.range.end).toString()))
                val tail21 = out.lastOrNull()?.code ?: -1
                if (tail21 >= 55296 && tail21 <= 56319 && "}".length > 0 && !("}"[0].code >= 56320 && "}"[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail21)
                }
                out.append("}")
            }
        }
        val tail22 = out.lastOrNull()?.code ?: -1
        if (tail22 >= 55296 && tail22 <= 56319 && "]}}".length > 0 && !("]}}"[0].code >= 56320 && "]}}"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail22)
        }
        out.append("]}}")
        val tail23 = out.lastOrNull()?.code ?: -1
        if (tail23 >= 55296 && tail23 <= 56319) {
            throw UStringException.UnpairedSurrogate(tail23)
        }
        return out.toString()
    }

    fun toPackedPlanBytes(result: LayoutResult): ByteArray {
        val plan = PlanLowering.toPlan(result, false)
        return PlanPacked.encode(plan)
    }
}
