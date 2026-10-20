package org.tiqian.layout;

import std.StringBuf;
import org.tiqian.core.LayoutResult;
import org.tiqian.core.*;
import org.tiqian.protocol.PlanJson;
import org.tiqian.protocol.PlanJsonNumber;
import org.tiqian.protocol.PlanPacked;
import haxe.io.Bytes;

/** Prepared paragraph JSON serialization functions. */
class PreparedParagraphFns {
    public static function toPreparedParagraphJson(result:LayoutResult, renderEvidence:Bool = false):String {
        final plan = PlanLowering.toPlan(result, renderEvidence);
        return PlanJson.encode(plan);
    }

    public static function toPlanWithDiagnosticsJson(result:LayoutResult, renderEvidence:Bool, zeroAdvanceEpsilonPx:Float):String {
        final out = new StringBuf();
        out.add("{\"plan\":");
        PlanJsonNumber.appendJsonString(out, toPreparedParagraphJson(result, renderEvidence));
        out.add(",\"diagnostics\":{\"capabilityIssues\":[");
        var first = true;
        final shapingDiagSrc = result.debug.shapingDecisions;
        for (di in 0...shapingDiagSrc.length) {
            final d = shapingDiagSrc[di];
            if (d.capabilityIssue != null) {
                if (!first)
                    out.add(",");
                first = false;
                out.add("{\"name\":");
                PlanJsonNumber.appendJsonString(out, d.capabilityIssue);
                out.add(",\"reason\":");
                PlanJsonNumber.appendJsonString(out, d.reason);
                out.add(",\"rangeStart\":");
                out.add(Std.string(d.range.start));
                out.add(",\"rangeEnd\":");
                out.add(Std.string(d.range.end));
                out.add("}");
            }
        }
        out.add("],\"advanceSuspects\":[");
        first = true;
        for (di in 0...shapingDiagSrc.length) {
            final d = shapingDiagSrc[di];
            if (!(Math.isFinite(d.advance) && d.advance > zeroAdvanceEpsilonPx)) {
                if (!first)
                    out.add(",");
                first = false;
                out.add("{\"displayText\":");
                PlanJsonNumber.appendJsonString(out, d.displayText);
                out.add(",\"advance\":\"");
                out.add(Math.isFinite(d.advance) ? PlanJsonNumber.ecmaJsonNumber(d.advance) : Std.string(d.advance));
                out.add("\",\"reason\":");
                PlanJsonNumber.appendJsonString(out, d.reason);
                out.add(",\"rangeStart\":");
                out.add(Std.string(d.range.start));
                out.add(",\"rangeEnd\":");
                out.add(Std.string(d.range.end));
                out.add("}");
            }
        }
        out.add("]}}");
        return out.toString();
    }

    public static function toPackedPlanBytes(result:LayoutResult):Bytes {
        final plan = PlanLowering.toPlan(result, false);
        return PlanPacked.encode(plan);
    }
}