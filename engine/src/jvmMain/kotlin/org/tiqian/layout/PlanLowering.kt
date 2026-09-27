package org.tiqian.layout

import org.tiqian.boring.runtime.SortedTable
import org.tiqian.core.BopomofoGlyphRole
import org.tiqian.core.DecorationKind
import org.tiqian.core.LayoutQueries
import org.tiqian.core.LayoutResult
import org.tiqian.core.LineEndReason
import org.tiqian.core.PunctuationDecisionInfo
import org.tiqian.core.ShapingDecisionInfo
import org.tiqian.core.TextRange
import org.tiqian.core.TextStyle
import org.tiqian.font.FontRole
import org.tiqian.protocol.Plan
import org.tiqian.protocol.PlanBopomofo
import org.tiqian.protocol.PlanBopomofoPlacement
import org.tiqian.protocol.PlanCell
import org.tiqian.protocol.PlanDecorationSegment
import org.tiqian.protocol.PlanEmphasisDot
import org.tiqian.protocol.PlanEmphasisRange
import org.tiqian.protocol.PlanEndReason
import org.tiqian.protocol.PlanInlineEdge
import org.tiqian.protocol.PlanLine
import org.tiqian.protocol.PlanRuby
import org.tiqian.protocol.PlanStyleDelta

object PlanLowering {
    private fun f32(v: Float): Float {
        return v
    }

    private fun rangeKey(r: TextRange): String {
        return (r.start).toString() + ":" + r.end
    }

    private fun styleAt(result: LayoutResult, offset: Int): TextStyle {
        val spanSrc = result.input.content.spans
        for (span in spanSrc) {
            if ((offset >= span.range.start && offset < span.range.end)) {
                return span.style
            }
        }
        return result.input.textStyle
    }

    fun toPlan(result: LayoutResult, renderEvidence: Boolean = false): Plan {
        val naturalB = SortedTable.mapBuilder<String, Float>(SortedTable::compareStrings)
        val featuresB = SortedTable.mapBuilder<String, MutableList<String>>(SortedTable::compareStrings)
        val fontsB = SortedTable.mapBuilder<String, String>(SortedTable::compareStrings)
        val glyphIdsB = SortedTable.mapBuilder<String, MutableList<String>>(SortedTable::compareStrings)
        for (run in result.glyphRuns) {
            for (glyph in run.glyphs) {
                val k = PlanLowering.rangeKey(glyph.clusterRange)
                val priorNatural = naturalB.get(k)
                val priorN = (if ((priorNatural == null)) 0.0f else priorNatural)
                naturalB.put(k, priorN + glyph.advance)
                if ((run.openTypeFeatures.size > 0)) {
                    var fs = featuresB.get(k)
                    if ((fs == null)) {
                        fs = mutableListOf<String>()
                    }
                    for (f in run.openTypeFeatures) {
                        if ((fs!!.indexOf(f) < 0)) {
                            fs.add(f)
                        }
                    }
                    featuresB.put(k, fs!!)
                }
                val rfk = glyph.renderFontKey
                if ((rfk != null)) {
                    fontsB.put(k, rfk)
                }
                var ids = glyphIdsB.get(k)
                if ((ids == null)) {
                    ids = mutableListOf<String>()
                }
                ids.add((glyph.id).toString())
                glyphIdsB.put(k, ids!!)
            }
        }
        val zeroB = SortedTable.mapBuilder<String, Boolean>(SortedTable::compareStrings)
        val zeroSrc = result.debug.zeroWidthBreakDecisions
        for (zi in 0 until zeroSrc.size) {
            zeroB.put(PlanLowering.rangeKey(zeroSrc[zi].range), true)
        }
        val shapingB = SortedTable.mapBuilder<String, ShapingDecisionInfo>(SortedTable::compareStrings)
        val shapingSrc = result.debug.shapingDecisions
        for (zi_2 in 0 until shapingSrc.size) {
            shapingB.put(PlanLowering.rangeKey(shapingSrc[zi_2].range), shapingSrc[zi_2])
        }
        val punctB = SortedTable.mapBuilder<String, PunctuationDecisionInfo>(SortedTable::compareStrings)
        val punctSrc = result.debug.punctuationDecisions
        for (zi_3 in 0 until punctSrc.size) {
            punctB.put(PlanLowering.rangeKey(punctSrc[zi_3].range), punctSrc[zi_3])
        }
        val inlineAdvanceB = SortedTable.mapBuilder<String, Float>(SortedTable::compareStrings)
        val inlineObjSrc = result.input.inlineObjects
        for (zi_4 in 0 until inlineObjSrc.size) {
            inlineAdvanceB.put(PlanLowering.rangeKey(inlineObjSrc[zi_4].range), inlineObjSrc[zi_4].advance)
        }
        val edgeStartB = SortedTable.mapBuilder<String, Float>(SortedTable::compareStrings)
        val edgeEndB = SortedTable.mapBuilder<String, Float>(SortedTable::compareStrings)
        val boxSrc = result.input.inlineBoxes
        for (b in boxSrc) {
            if ((b.inlineStart != (0).toFloat())) {
                val sk = (b.range.start).toString()
                val priorStart = edgeStartB.get(sk)
                val pStart = (if ((priorStart == null)) 0.0f else priorStart)
                edgeStartB.put(sk, pStart + b.inlineStart)
            }
            if ((b.inlineEnd != (0).toFloat())) {
                val ek = (b.range.end).toString()
                val priorEnd = edgeEndB.get(ek)
                val pEnd = (if ((priorEnd == null)) 0.0f else priorEnd)
                edgeEndB.put(ek, pEnd + b.inlineEnd)
            }
        }
        val natural = naturalB.build()
        val features = featuresB.build()
        val fonts = fontsB.build()
        val glyphIds = glyphIdsB.build()
        val zero = zeroB.build()
        val shaping = shapingB.build()
        val punct = punctB.build()
        val inlineAdvance = inlineAdvanceB.build()
        val edgeStart = edgeStartB.build()
        val edgeEnd = edgeEndB.build()
        val lines = mutableListOf<PlanLine>()
        for (line in result.lines) {
            val cells = mutableListOf<PlanCell>()
            run {
                val _g1 = LayoutQueries.positionedClustersForLine(result, line)
                for (p in _g1) {
                    val c = result.clusters[p.clusterIndex]
                    val ck = PlanLowering.rangeKey(c.range)
                    if ((!(c.displayText.length > 0 || zero.has(ck) || renderEvidence && inlineAdvance.has(ck)))) {
                        continue
                    }
                    val natW = (if ((natural.has(ck))) natural.get(ck) else c.advance)!!
                    var inlineObj: Float? = null
                    var advOverride: Float? = null
                    var renderFam: String? = null
                    var dash: String? = null
                    var lang: String? = null
                    var face: String? = null
                    var gIds: String? = null
                    var ev: String? = null
                    var inkFloor: Float? = null
                    var bodyW: Float? = null
                    var latin = false
                    var styleDelta: PlanStyleDelta? = null
                    if ((renderEvidence)) {
                        inlineObj = (if ((inlineAdvance.has(ck))) inlineAdvance.get(ck) else null)
                        val glyphW = (if ((inlineObj == null)) natW else inlineObj)
                        advOverride = (if ((c.advance != glyphW)) c.advance else null)
                        renderFam = (if ((fonts.has(ck))) fonts.get(ck) else null)
                        val sd = shaping.get(ck)
                        dash = (if ((sd != null)) sd.strategy else null)
                        lang = (if ((sd != null)) sd.language else null)
                        face = (if ((sd != null)) sd.resolvedFace else null)
                        gIds = (if ((glyphIds.has(ck) && glyphIds.get(ck)?.size!! > 0)) glyphIds.get(ck)?.joinToString(",") else null)
                        ev = (if ((sd != null)) sd.reason else null)
                        val pd = punct.get(ck)
                        if ((pd != null && pd.inkContainmentApplied && pd.inkContainmentBodyFloor != null)) {
                            inkFloor = pd.inkContainmentBodyFloor
                            bodyW = pd.bodyWidth
                        }
                        val fdSrc = result.debug.fontDecisions
                        for (fd in fdSrc) {
                            if ((c.range.start >= fd.range.start && c.range.end <= fd.range.end && fd.role == FontRole.LatinText.name)) {
                                latin = true
                            }
                        }
                        val cs = PlanLowering.styleAt(result, c.range.start)
                        styleDelta = (if ((cs != result.input.textStyle)) PlanStyleDelta(fontSize = (if ((cs.fontSize != result.input.textStyle.fontSize)) cs.fontSize else null), fontWeight = (if ((cs.fontWeight != result.input.textStyle.fontWeight)) cs.fontWeight else null), italic = (if ((cs.italic != result.input.textStyle.italic)) cs.italic else null)) else null)
                    }
                    val feats = features.get(ck)
                    cells.add(PlanCell(rangeStart = c.range.start, rangeEnd = c.range.end, source = c.text, display = c.displayText, drawX = p.drawX, naturalWidth = natW, leadingLayoutAdvance = c.leadingLayoutAdvance, shapingBoundary = c.range.end - c.range.start > 1, openTypeFeatures = (if ((feats != null)) feats else mutableListOf<String>()), renderFontFamily = renderFam, dashStrategy = dash, shapingLanguage = lang, resolvedFace = face, glyphIds = gIds, shapingEvidence = ev, punctuationInkFloor = inkFloor, punctuationBodyWidth = bodyW, latin = latin, advance = advOverride, inlineObject = inlineObj, styleDelta = styleDelta))
                }
            }
            lines.add(PlanLine(rangeStart = line.range.start, rangeEnd = line.range.end, top = line.top, bottom = line.bottom, baseline = line.baseline, indent = line.indent, visualWidth = line.visualWidth, hyphenAdvance = line.hyphenAdvance, endReason = PlanLowering.endReasonFrom(line.endReason), cells = cells))
        }
        val emphasisRanges = mutableListOf<PlanEmphasisRange>()
        val inlineEdges = mutableListOf<PlanInlineEdge>()
        val rubyDecisions = mutableListOf<PlanRuby>()
        val bopomofoDecisions = mutableListOf<PlanBopomofo>()
        val decorationSegments = mutableListOf<PlanDecorationSegment>()
        val emphasisDots = mutableListOf<PlanEmphasisDot>()
        var fontSize: Float? = null
        var overlayWidth: Float? = null
        if ((renderEvidence)) {
            fontSize = result.input.textStyle.fontSize
            overlayWidth = result.size.width
            val emphSrc = result.input.decorations
            for (d in emphSrc) {
                if ((d.kind == DecorationKind.Emphasis)) {
                    emphasisRanges.add(PlanEmphasisRange(start = d.range.start, end = d.range.end))
                }
            }
            val offsets = mutableListOf<Int>()
            for (oi in 0 until edgeStart.size()) {
                offsets.add((run { val s = edgeStart.keyAt(oi); val t = s.trim(' ', '\t', '\n', '\r', '\u000B', '\u000C'); val neg = t.startsWith("-"); val sign = if (neg || t.startsWith("+")) 1 else 0; val hex = t.startsWith("0x", sign) || t.startsWith("0X", sign); val d = if (hex) t.substring(sign + 2) else t.substring(sign); if (!hex) { var i = sign; val start = i; while (i < t.length && t[i] in '0'..'9') i++; if (i != t.length || i == start) null else t.toIntOrNull() } else { var i = 0; while (i < d.length && d[i] in '0'..'9' || i < d.length && d[i] in 'a'..'f' || i < d.length && d[i] in 'A'..'F') i++; if (i != d.length || d.isEmpty()) null else { val n = d.toLongOrNull(16); if (n == null) null else { val v = if (neg) -n else n; if (v >= -2147483648L && v <= 2147483647L) v.toInt() else null } } } }) ?: throw IllegalArgumentException("argument is null"))
            }
            for (ei in 0 until edgeEnd.size()) {
                val ev_2 = run { val s = edgeEnd.keyAt(ei); val t = s.trim(' ', '\t', '\n', '\r', '\u000B', '\u000C'); val neg = t.startsWith("-"); val sign = if (neg || t.startsWith("+")) 1 else 0; val hex = t.startsWith("0x", sign) || t.startsWith("0X", sign); val d = if (hex) t.substring(sign + 2) else t.substring(sign); if (!hex) { var i = sign; val start = i; while (i < t.length && t[i] in '0'..'9') i++; if (i != t.length || i == start) null else t.toIntOrNull() } else { var i = 0; while (i < d.length && d[i] in '0'..'9' || i < d.length && d[i] in 'a'..'f' || i < d.length && d[i] in 'A'..'F') i++; if (i != d.length || d.isEmpty()) null else { val n = d.toLongOrNull(16); if (n == null) null else { val v = if (neg) -n else n; if (v >= -2147483648L && v <= 2147483647L) v.toInt() else null } } } }
                if ((offsets.indexOf(ev_2!!) < 0)) {
                    offsets.add(ev_2)
                }
            }
            var oi_2 = 1
            while ((oi_2 < offsets.size)) {
                val ov = offsets[oi_2]
                var oj = oi_2 - 1
                while ((oj >= 0 && offsets[oj] > ov)) {
                    val _growIndex1 = oj + 1
                    while (offsets.size <= _growIndex1) { offsets.add(0) }
                    offsets[_growIndex1] = offsets[oj]
                    oj--
                }
                val _growIndex2 = oj + 1
                while (offsets.size <= _growIndex2) { offsets.add(0) }
                offsets[_growIndex2] = ov
                oi_2++
            }
            for (offset in offsets) {
                val sk_2 = (offset).toString()
                inlineEdges.add(PlanInlineEdge(offset = offset, inlineStart = (if ((edgeStart.has(sk_2))) edgeStart.get(sk_2) else null), inlineEnd = (if ((edgeEnd.has(sk_2))) edgeEnd.get(sk_2) else null)))
            }
            val rubySrc = result.debug.rubyDecisions
            for (r in rubySrc) {
                rubyDecisions.add(PlanRuby(baseRangeStart = r.baseRange.start, baseRangeEnd = r.baseRange.end, text = r.text, centerX = r.centerX, baselineY = r.baselineY, fontSize = r.fontSize, fontWeight = r.fontWeight, fontFamilies = PlanLowering.toArray(r.fontFamilies), ascent = r.ascent))
            }
            val bopoSrc = result.debug.bopomofoDecisions
            for (z in bopoSrc) {
                val placements = mutableListOf<PlanBopomofoPlacement>()
                for (pl in z.placements) {
                    placements.add(PlanBopomofoPlacement(text = pl.text, role = pl.role.name, left = pl.left, top = pl.top, width = pl.width, height = pl.height))
                }
                bopomofoDecisions.add(PlanBopomofo(baseRangeStart = z.baseRange.start, baseRangeEnd = z.baseRange.end, text = z.text, fontWeight = z.fontWeight, fontFamilies = PlanLowering.toArray(z.fontFamilies), placements = placements))
            }
            val segSrc = result.debug.decorationSegments
            for (s in segSrc) {
                if ((s.kind == DecorationKind.ProperNoun.name || s.kind == DecorationKind.BookTitle.name)) {
                    decorationSegments.add(PlanDecorationSegment(kind = s.kind, left = s.left, top = s.top, right = s.right, sourceRangeStart = s.sourceRange.start, sourceRangeEnd = s.sourceRange.end))
                }
            }
            val dotSrc = result.debug.decorationDecisions
            for (d_2 in dotSrc) {
                if ((d_2.applied && d_2.kind == DecorationKind.Emphasis.name && d_2.dotDiameter > (0).toFloat())) {
                    emphasisDots.add(PlanEmphasisDot(clusterRangeStart = PlanLowering.f32((d_2.clusterRange.start).toFloat()), anchorX = d_2.anchorX, anchorY = d_2.anchorY, dotDiameter = d_2.dotDiameter))
                }
            }
        }
        return Plan(width = result.input.constraints.maxWidth, height = result.size.height, lines = lines, emphasisRanges = emphasisRanges, inlineEdges = inlineEdges, rubyDecisions = rubyDecisions, bopomofoDecisions = bopomofoDecisions, fontSize = fontSize, overlayWidth = overlayWidth, decorationSegments = decorationSegments, emphasisDots = emphasisDots)
    }

    private fun endReasonFrom(reason: LineEndReason): PlanEndReason {
        return when (reason) {
            LineEndReason.AutoWrap -> PlanEndReason.AutoWrap
            LineEndReason.MandatoryBreak -> PlanEndReason.MandatoryBreak
            LineEndReason.ParagraphEnd -> PlanEndReason.ParagraphEnd
        }
    }

    private fun toArray(src: List<String>): MutableList<String> {
        val out = mutableListOf<String>()
        for (i in 0 until src.size) {
            out.add(src[i])
        }
        return out
    }
}
