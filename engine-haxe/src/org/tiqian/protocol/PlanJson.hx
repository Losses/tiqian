package org.tiqian.protocol;

import std.StringBuf;
import org.tiqian.protocol.Plan;

/**
 * Plan JSON serialization (Stage1-P3). Serialises a built Plan into the wire
 * JSON the Kotlin producer emitted by hand (PreparedParagraph.kt
 * toPreparedParagraphJson) and the Rust reader parsed by hand (plan.rs).
 * Field order and omission rules mirror PreparedParagraph.kt:73-154 exactly.
 *
 * Two boring-emitter constraints shape the code: struct array indexing
 * returns Null<T>, so loops iterate for-each with an explicit first flag
 * instead of an index; and Null<T> fields are bound to a local before use
 * because the emitters do not narrow a struct field to non-null.
 */
class PlanJson {
    public static function encode(plan:Plan):String {
        final out = new StringBuf();
        out.add("{");
        out.add("\"schema\":");
        out.add(Std.string(PlanSchema.PLAN_SCHEMA));
        out.add(",\"layoutRevision\":\"");
        out.add(PlanSchema.PLAN_LAYOUT_REVISION);
        out.add("\",\"width\":");
        out.add(PlanJsonNumber.ecmaJsonNumber(plan.width));
        out.add(",\"height\":");
        out.add(PlanJsonNumber.ecmaJsonNumber(plan.height));
        out.add(",\"lines\":[");
        var firstLine = true;
        for (line in plan.lines) {
            if (!firstLine)
                out.add(",");
            firstLine = false;
            appendLine(out, line);
        }
        out.add("]");
        appendParagraphEvidence(out, plan);
        out.add("}");
        return out.toString();
    }

    static function appendLine(out:StringBuf, line:Plan.PlanLine):Void {
        out.add("{\"rangeStart\":");
        out.add(Std.string(line.rangeStart));
        out.add(",\"rangeEnd\":");
        out.add(Std.string(line.rangeEnd));
        out.add(",\"top\":");
        out.add(PlanJsonNumber.ecmaJsonNumber(line.top));
        out.add(",\"bottom\":");
        out.add(PlanJsonNumber.ecmaJsonNumber(line.bottom));
        out.add(",\"baseline\":");
        out.add(PlanJsonNumber.ecmaJsonNumber(line.baseline));
        out.add(",\"indent\":");
        out.add(PlanJsonNumber.ecmaJsonNumber(line.indent));
        out.add(",\"visualWidth\":");
        out.add(PlanJsonNumber.ecmaJsonNumber(line.visualWidth));
        out.add(",\"hyphenAdvance\":");
        out.add(PlanJsonNumber.ecmaJsonNumber(line.hyphenAdvance));
        out.add(",\"endReason\":");
        PlanJsonNumber.appendJsonString(out, endReasonName(line.endReason));
        out.add(",\"cells\":[");
        var firstCell = true;
        for (cell in line.cells) {
            if (!firstCell)
                out.add(",");
            firstCell = false;
            appendCell(out, cell);
        }
        out.add("]}");
    }

    static function appendCell(out:StringBuf, cell:Plan.PlanCell):Void {
        out.add("{\"rangeStart\":");
        out.add(Std.string(cell.rangeStart));
        out.add(",\"rangeEnd\":");
        out.add(Std.string(cell.rangeEnd));
        out.add(",\"source\":");
        PlanJsonNumber.appendJsonString(out, cell.source);
        out.add(",\"display\":");
        PlanJsonNumber.appendJsonString(out, cell.display);
        out.add(",\"drawX\":");
        out.add(PlanJsonNumber.ecmaJsonNumber(cell.drawX));
        out.add(",\"naturalWidth\":");
        out.add(PlanJsonNumber.ecmaJsonNumber(cell.naturalWidth));
        out.add(",\"leadingLayoutAdvance\":");
        out.add(PlanJsonNumber.ecmaJsonNumber(cell.leadingLayoutAdvance));
        if (cell.shapingBoundary)
            out.add(",\"shapingBoundary\":true");
        if (cell.openTypeFeatures.length > 0) {
            out.add(",\"openTypeFeatures\":[");
            var firstFeature = true;
            for (feature in cell.openTypeFeatures) {
                if (!firstFeature)
                    out.add(",");
                firstFeature = false;
                PlanJsonNumber.appendJsonString(out, feature);
            }
            out.add("]");
        }
        appendCellEvidence(out, cell);
        out.add("}");
    }

    static function appendCellEvidence(out:StringBuf, cell:Plan.PlanCell):Void {
        final inlineObject = cell.inlineObject;
        if (inlineObject != null) {
            out.add(",\"inlineObject\":");
            out.add(PlanJsonNumber.ecmaJsonNumber((inlineObject : Float)));
        }
        final advance = cell.advance;
        if (advance != null) {
            out.add(",\"advance\":");
            out.add(PlanJsonNumber.ecmaJsonNumber((advance : Float)));
        }
        final renderFontFamily = cell.renderFontFamily;
        if (renderFontFamily != null) {
            out.add(",\"renderFontFamily\":");
            PlanJsonNumber.appendJsonString(out, (renderFontFamily : String));
        }
        final dashStrategy = cell.dashStrategy;
        if (dashStrategy != null) {
            out.add(",\"dashStrategy\":");
            PlanJsonNumber.appendJsonString(out, (dashStrategy : String));
            final shapingLanguage = cell.shapingLanguage;
            if (shapingLanguage != null) {
                out.add(",\"shapingLanguage\":");
                PlanJsonNumber.appendJsonString(out, (shapingLanguage : String));
            }
            final resolvedFace = cell.resolvedFace;
            if (resolvedFace != null) {
                out.add(",\"resolvedFace\":");
                PlanJsonNumber.appendJsonString(out, (resolvedFace : String));
            }
            final glyphIds = cell.glyphIds;
            if (glyphIds != null) {
                out.add(",\"glyphIds\":");
                PlanJsonNumber.appendJsonString(out, (glyphIds : String));
            }
            final shapingEvidence = cell.shapingEvidence;
            if (shapingEvidence != null) {
                out.add(",\"shapingEvidence\":");
                PlanJsonNumber.appendJsonString(out, (shapingEvidence : String));
            }
        }
        final punctuationInkFloor = cell.punctuationInkFloor;
        if (punctuationInkFloor != null) {
            out.add(",\"punctuationInkFloor\":");
            out.add(PlanJsonNumber.ecmaJsonNumber((punctuationInkFloor : Float)));
            final punctuationBodyWidth = cell.punctuationBodyWidth;
            if (punctuationBodyWidth != null) {
                out.add(",\"punctuationBodyWidth\":");
                out.add(PlanJsonNumber.ecmaJsonNumber((punctuationBodyWidth : Float)));
            }
        }
        if (cell.latin)
            out.add(",\"latin\":true");
        final styleDelta = cell.styleDelta;
        if (styleDelta != null)
            appendStyleDelta(out, styleDelta);
    }

    static function appendStyleDelta(out:StringBuf, style:PlanStyleDelta):Void {
        out.add(",\"style\":{");
        var fieldCount = 0;
        final fontSize = style.fontSize;
        if (fontSize != null) {
            out.add("\"fontSize\":");
            out.add(PlanJsonNumber.ecmaJsonNumber((fontSize : Float)));
            fieldCount++;
        }
        final fontWeight = style.fontWeight;
        if (fontWeight != null) {
            if (fieldCount > 0)
                out.add(",");
            out.add("\"fontWeight\":");
            out.add(Std.string((fontWeight : Int)));
            fieldCount++;
        }
        final italic = style.italic;
        if (italic != null) {
            if (fieldCount > 0)
                out.add(",");
            out.add("\"italic\":");
            out.add(italic ? "true" : "false");
        }
        out.add("}");
    }

    static function appendParagraphEvidence(out:StringBuf, plan:Plan):Void {
        final fontSize = plan.fontSize;
        if (fontSize != null) {
            out.add(",\"fontSize\":");
            out.add(PlanJsonNumber.ecmaJsonNumber((fontSize : Float)));
        }
        final overlayWidth = plan.overlayWidth;
        if (overlayWidth != null) {
            out.add(",\"overlayWidth\":");
            out.add(PlanJsonNumber.ecmaJsonNumber((overlayWidth : Float)));
        }
        if (plan.emphasisRanges.length > 0) {
            out.add(",\"emphasisRanges\":[");
            var firstRange = true;
            for (range in plan.emphasisRanges) {
                if (!firstRange)
                    out.add(",");
                firstRange = false;
                out.add("[");
                out.add(Std.string(range.start));
                out.add(",");
                out.add(Std.string(range.end));
                out.add("]");
            }
            out.add("]");
        }
        if (plan.inlineEdges.length > 0) {
            out.add(",\"inlineEdges\":[");
            var firstEdge = true;
            for (edge in plan.inlineEdges) {
                if (!firstEdge)
                    out.add(",");
                firstEdge = false;
                out.add("{\"offset\":");
                out.add(Std.string(edge.offset));
                final inlineStart = edge.inlineStart;
                if (inlineStart != null) {
                    out.add(",\"inlineStart\":");
                    out.add(PlanJsonNumber.ecmaJsonNumber((inlineStart : Float)));
                }
                final inlineEnd = edge.inlineEnd;
                if (inlineEnd != null) {
                    out.add(",\"inlineEnd\":");
                    out.add(PlanJsonNumber.ecmaJsonNumber((inlineEnd : Float)));
                }
                out.add("}");
            }
            out.add("]");
        }
        if (plan.rubyDecisions.length > 0) {
            out.add(",\"rubyDecisions\":[");
            var firstRuby = true;
            for (ruby in plan.rubyDecisions) {
                if (!firstRuby)
                    out.add(",");
                firstRuby = false;
                out.add("{\"baseRangeStart\":");
                out.add(Std.string(ruby.baseRangeStart));
                out.add(",\"baseRangeEnd\":");
                out.add(Std.string(ruby.baseRangeEnd));
                out.add(",\"text\":");
                PlanJsonNumber.appendJsonString(out, ruby.text);
                out.add(",\"centerX\":");
                out.add(PlanJsonNumber.ecmaJsonNumber(ruby.centerX));
                out.add(",\"baselineY\":");
                out.add(PlanJsonNumber.ecmaJsonNumber(ruby.baselineY));
                out.add(",\"fontSize\":");
                out.add(PlanJsonNumber.ecmaJsonNumber(ruby.fontSize));
                final ascent = ruby.ascent;
                if (ascent != null) {
                    out.add(",\"ascent\":");
                    out.add(PlanJsonNumber.ecmaJsonNumber((ascent : Float)));
                }
                out.add(",\"fontWeight\":");
                out.add(Std.string(ruby.fontWeight));
                if (ruby.fontFamilies.length > 0) {
                    out.add(",\"fontFamilies\":[");
                    var firstFamily = true;
                    for (family in ruby.fontFamilies) {
                        if (!firstFamily)
                            out.add(",");
                        firstFamily = false;
                        PlanJsonNumber.appendJsonString(out, family);
                    }
                    out.add("]");
                }
                out.add("}");
            }
            out.add("]");
        }
        if (plan.bopomofoDecisions.length > 0) {
            out.add(",\"bopomofoDecisions\":[");
            var firstBopomofo = true;
            for (bopomofo in plan.bopomofoDecisions) {
                if (!firstBopomofo)
                    out.add(",");
                firstBopomofo = false;
                out.add("{\"baseRangeStart\":");
                out.add(Std.string(bopomofo.baseRangeStart));
                out.add(",\"baseRangeEnd\":");
                out.add(Std.string(bopomofo.baseRangeEnd));
                out.add(",\"text\":");
                PlanJsonNumber.appendJsonString(out, bopomofo.text);
                out.add(",\"fontWeight\":");
                out.add(Std.string(bopomofo.fontWeight));
                if (bopomofo.fontFamilies.length > 0) {
                    out.add(",\"fontFamilies\":[");
                    var firstFamily = true;
                    for (family in bopomofo.fontFamilies) {
                        if (!firstFamily)
                            out.add(",");
                        firstFamily = false;
                        PlanJsonNumber.appendJsonString(out, family);
                    }
                    out.add("]");
                }
                out.add(",\"placements\":[");
                var firstPlacement = true;
                for (placement in bopomofo.placements) {
                    if (!firstPlacement)
                        out.add(",");
                    firstPlacement = false;
                    out.add("{\"text\":");
                    PlanJsonNumber.appendJsonString(out, placement.text);
                    out.add(",\"left\":");
                    out.add(PlanJsonNumber.ecmaJsonNumber(placement.left));
                    out.add(",\"top\":");
                    out.add(PlanJsonNumber.ecmaJsonNumber(placement.top));
                    out.add(",\"width\":");
                    out.add(PlanJsonNumber.ecmaJsonNumber(placement.width));
                    out.add(",\"height\":");
                    out.add(PlanJsonNumber.ecmaJsonNumber(placement.height));
                    out.add(",\"role\":");
                    PlanJsonNumber.appendJsonString(out, placement.role);
                    out.add("}");
                }
                out.add("]}");
            }
            out.add("]");
        }
        if (plan.decorationSegments.length > 0) {
            out.add(",\"decorationSegments\":[");
            var firstSegment = true;
            for (seg in plan.decorationSegments) {
                if (!firstSegment)
                    out.add(",");
                firstSegment = false;
                out.add("{\"kind\":");
                PlanJsonNumber.appendJsonString(out, seg.kind);
                out.add(",\"left\":");
                out.add(PlanJsonNumber.ecmaJsonNumber(seg.left));
                out.add(",\"top\":");
                out.add(PlanJsonNumber.ecmaJsonNumber(seg.top));
                out.add(",\"right\":");
                out.add(PlanJsonNumber.ecmaJsonNumber(seg.right));
                out.add(",\"sourceRangeStart\":");
                out.add(Std.string(seg.sourceRangeStart));
                out.add(",\"sourceRangeEnd\":");
                out.add(Std.string(seg.sourceRangeEnd));
                out.add("}");
            }
            out.add("]");
        }
        if (plan.emphasisDots.length > 0) {
            out.add(",\"emphasisDots\":[");
            var firstDot = true;
            for (dot in plan.emphasisDots) {
                if (!firstDot)
                    out.add(",");
                firstDot = false;
                out.add("{\"clusterRangeStart\":");
                final clusterRangeStart = dot.clusterRangeStart;
                if (clusterRangeStart != null) {
                    out.add(PlanJsonNumber.ecmaJsonNumber((clusterRangeStart : Float)));
                } else {
                    out.add("null");
                }
                out.add(",\"anchorX\":");
                out.add(PlanJsonNumber.ecmaJsonNumber(dot.anchorX));
                out.add(",\"anchorY\":");
                out.add(PlanJsonNumber.ecmaJsonNumber(dot.anchorY));
                out.add(",\"dotDiameter\":");
                out.add(PlanJsonNumber.ecmaJsonNumber(dot.dotDiameter));
                out.add("}");
            }
            out.add("]");
        }
    }

    static function endReasonName(reason:PlanEndReason):String {
        return switch (reason) {
            case AutoWrap: "AutoWrap";
            case MandatoryBreak: "MandatoryBreak";
            case ParagraphEnd: "ParagraphEnd";
        };
    }
}