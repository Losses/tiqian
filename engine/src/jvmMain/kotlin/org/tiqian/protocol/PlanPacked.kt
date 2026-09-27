package org.tiqian.protocol

import org.tiqian.boring.runtime.BytesBuffer
import org.tiqian.boring.runtime.FPHelper

object PlanPacked {
    private fun f32(v: Float): Float {
        return v
    }

    fun encode(plan: Plan): ByteArray {
        val pool = StringPool()
        val writer = PlanPackedWriter()
        val cellRangeStart = mutableListOf<Int>()
        val cellRangeEnd = mutableListOf<Int>()
        val cellSourceRef = mutableListOf<Int>()
        val cellDisplayRef = mutableListOf<Int>()
        val cellDrawX = mutableListOf<Float>()
        val cellNaturalWidth = mutableListOf<Float>()
        val cellLeadingAdvance = mutableListOf<Float>()
        val cellShapingBoundary = mutableListOf<Int>()
        val cellLatin = mutableListOf<Int>()
        val cellRenderFamilyRef = mutableListOf<Int>()
        val cellDashRef = mutableListOf<Int>()
        val cellLanguageRef = mutableListOf<Int>()
        val cellResolvedFaceRef = mutableListOf<Int>()
        val cellGlyphIdsRef = mutableListOf<Int>()
        val cellEvidenceRef = mutableListOf<Int>()
        val cellInkFloor = mutableListOf<Float>()
        val cellBodyWidth = mutableListOf<Float>()
        val cellAdvance = mutableListOf<Float>()
        val cellInlineObject = mutableListOf<Float>()
        val cellStyleFontSize = mutableListOf<Float>()
        val cellStyleFontWeight = mutableListOf<Float>()
        val cellStyleItalic = mutableListOf<Int>()
        val cellFeatureOffset = mutableListOf<Int>()
        val cellFeatureCount = mutableListOf<Int>()
        val featurePool = mutableListOf<Int>()
        var cellCount = 0
        run {
            val _g1 = plan.lines
            for (line in _g1) {
                cellCount += line.cells.size
            }
        }
        run {
            val _g1_2 = plan.lines
            for (line_2 in _g1_2) {
                run {
                    var _g = 0
                    val _g1_3 = line_2.cells
                    while ((_g < _g1_3.size)) {
                        val cell = _g1_3[_g]
                        ++_g
                        cellRangeStart.add(cell.rangeStart)
                        cellRangeEnd.add(cell.rangeEnd)
                        cellSourceRef.add(pool.intern(cell.source))
                        cellDisplayRef.add(pool.intern(cell.display))
                        cellDrawX.add(cell.drawX)
                        cellNaturalWidth.add(cell.naturalWidth)
                        cellLeadingAdvance.add(cell.leadingLayoutAdvance)
                        cellShapingBoundary.add((if ((cell.shapingBoundary)) 1 else 0))
                        cellLatin.add((if ((cell.latin)) 1 else 0))
                        cellRenderFamilyRef.add((if ((cell.renderFontFamily != null)) pool.intern(cell.renderFontFamily!!) else -1))
                        cellDashRef.add((if ((cell.dashStrategy != null)) pool.intern(cell.dashStrategy!!) else -1))
                        cellLanguageRef.add((if ((cell.shapingLanguage != null)) pool.intern(cell.shapingLanguage!!) else -1))
                        cellResolvedFaceRef.add((if ((cell.resolvedFace != null)) pool.intern(cell.resolvedFace!!) else -1))
                        cellGlyphIdsRef.add((if ((cell.glyphIds != null)) pool.intern(cell.glyphIds!!) else -1))
                        cellEvidenceRef.add((if ((cell.shapingEvidence != null)) pool.intern(cell.shapingEvidence!!) else -1))
                        val ink = cell.punctuationInkFloor
                        cellInkFloor.add((if ((ink != null)) ink else Float.NaN))
                        val bw = cell.punctuationBodyWidth
                        cellBodyWidth.add((if ((bw != null)) bw else Float.NaN))
                        val adv = cell.advance
                        cellAdvance.add((if ((adv != null)) adv else Float.NaN))
                        val inl = cell.inlineObject
                        cellInlineObject.add((if ((inl != null)) inl else Float.NaN))
                        val style = cell.styleDelta
                        var styleFontSize = Float.NaN
                        var styleFontWeight = Float.NaN
                        var styleItalic = 2
                        if ((style != null)) {
                            val fs = style.fontSize
                            if ((fs != null)) {
                                styleFontSize = fs
                            }
                            val fw = style.fontWeight
                            if ((fw != null)) {
                                styleFontWeight = PlanPacked.f32((fw).toFloat())
                            }
                            val it = style.italic
                            if ((it != null)) {
                                styleItalic = (if ((it)) 1 else 0)
                            }
                        }
                        cellStyleFontSize.add(styleFontSize)
                        cellStyleFontWeight.add(styleFontWeight)
                        cellStyleItalic.add(styleItalic)
                        val feats = cell.openTypeFeatures
                        run {
                            var _g_2 = 0
                            while ((_g_2 < feats.size)) {
                                val f = feats[_g_2]
                                ++_g_2
                                pool.intern(f)
                            }
                        }
                        cellFeatureOffset.add(featurePool.size)
                        cellFeatureCount.add(feats.size)
                        run {
                            var _g_3 = 0
                            while ((_g_3 < feats.size)) {
                                val f_2 = feats[_g_3]
                                ++_g_3
                                featurePool.add(pool.indexOf(f_2))
                            }
                        }
                    }
                }
            }
        }
        val rubyBaseStart = mutableListOf<Int>()
        val rubyBaseEnd = mutableListOf<Int>()
        val rubyTextRef = mutableListOf<Int>()
        val rubyCenterX = mutableListOf<Float>()
        val rubyBaselineY = mutableListOf<Float>()
        val rubyFontSize = mutableListOf<Float>()
        val rubyFontWeight = mutableListOf<Int>()
        val rubyFamilyOffset = mutableListOf<Int>()
        val rubyFamilyCount = mutableListOf<Int>()
        val rubyAscent = mutableListOf<Float>()
        val rubyFamilyPool = mutableListOf<Int>()
        run {
            val _g1_4 = plan.rubyDecisions
            for (ruby in _g1_4) {
                run {
                    var _g_4 = 0
                    val _g1_5 = ruby.fontFamilies
                    while ((_g_4 < _g1_5.size)) {
                        val fam = _g1_5[_g_4]
                        ++_g_4
                        pool.intern(fam)
                    }
                }
                pool.intern(ruby.text)
            }
        }
        run {
            val _g1_6 = plan.rubyDecisions
            for (ruby_2 in _g1_6) {
                rubyBaseStart.add(ruby_2.baseRangeStart)
                rubyBaseEnd.add(ruby_2.baseRangeEnd)
                rubyTextRef.add(pool.indexOf(ruby_2.text))
                rubyCenterX.add(ruby_2.centerX)
                rubyBaselineY.add(ruby_2.baselineY)
                rubyFontSize.add(ruby_2.fontSize)
                rubyFontWeight.add(ruby_2.fontWeight)
                rubyFamilyOffset.add(rubyFamilyPool.size)
                rubyFamilyCount.add(ruby_2.fontFamilies.size)
                val ascent = ruby_2.ascent
                rubyAscent.add((if ((ascent != null)) ascent else Float.NaN))
                run {
                    var _g_5 = 0
                    val _g1_7 = ruby_2.fontFamilies
                    while ((_g_5 < _g1_7.size)) {
                        val fam_2 = _g1_7[_g_5]
                        ++_g_5
                        rubyFamilyPool.add(pool.indexOf(fam_2))
                    }
                }
            }
        }
        val bopomofoBaseStart = mutableListOf<Int>()
        val bopomofoBaseEnd = mutableListOf<Int>()
        val bopomofoTextRef = mutableListOf<Int>()
        val bopomofoFontWeight = mutableListOf<Int>()
        val bopomofoFamilyOffset = mutableListOf<Int>()
        val bopomofoFamilyCount = mutableListOf<Int>()
        val bopomofoPlaceOffset = mutableListOf<Int>()
        val bopomofoPlaceCount = mutableListOf<Int>()
        val bopomofoFamilyPool = mutableListOf<Int>()
        val bopomofoPlaceTextRef = mutableListOf<Int>()
        val bopomofoPlaceRoleRef = mutableListOf<Int>()
        val bopomofoPlaceLeft = mutableListOf<Float>()
        val bopomofoPlaceTop = mutableListOf<Float>()
        val bopomofoPlaceWidth = mutableListOf<Float>()
        val bopomofoPlaceHeight = mutableListOf<Float>()
        run {
            val _g1_8 = plan.bopomofoDecisions
            for (bopomofo in _g1_8) {
                run {
                    var _g_6 = 0
                    val _g1_9 = bopomofo.fontFamilies
                    while ((_g_6 < _g1_9.size)) {
                        val fam_3 = _g1_9[_g_6]
                        ++_g_6
                        pool.intern(fam_3)
                    }
                }
                pool.intern(bopomofo.text)
                run {
                    var _g_7 = 0
                    val _g1_10 = bopomofo.placements
                    while ((_g_7 < _g1_10.size)) {
                        val pl = _g1_10[_g_7]
                        ++_g_7
                        pool.intern(pl.text)
                        pool.intern(pl.role)
                    }
                }
            }
        }
        run {
            val _g1_11 = plan.bopomofoDecisions
            for (bopomofo_2 in _g1_11) {
                bopomofoBaseStart.add(bopomofo_2.baseRangeStart)
                bopomofoBaseEnd.add(bopomofo_2.baseRangeEnd)
                bopomofoTextRef.add(pool.indexOf(bopomofo_2.text))
                bopomofoFontWeight.add(bopomofo_2.fontWeight)
                bopomofoFamilyOffset.add(bopomofoFamilyPool.size)
                bopomofoFamilyCount.add(bopomofo_2.fontFamilies.size)
                bopomofoPlaceOffset.add(bopomofoPlaceTextRef.size)
                bopomofoPlaceCount.add(bopomofo_2.placements.size)
                run {
                    var _g_8 = 0
                    val _g1_12 = bopomofo_2.fontFamilies
                    while ((_g_8 < _g1_12.size)) {
                        val fam_4 = _g1_12[_g_8]
                        ++_g_8
                        bopomofoFamilyPool.add(pool.indexOf(fam_4))
                    }
                }
                run {
                    var _g_9 = 0
                    val _g1_13 = bopomofo_2.placements
                    while ((_g_9 < _g1_13.size)) {
                        val pl_2 = _g1_13[_g_9]
                        ++_g_9
                        bopomofoPlaceTextRef.add(pool.indexOf(pl_2.text))
                        bopomofoPlaceRoleRef.add(pool.indexOf(pl_2.role))
                        bopomofoPlaceLeft.add(pl_2.left)
                        bopomofoPlaceTop.add(pl_2.top)
                        bopomofoPlaceWidth.add(pl_2.width)
                        bopomofoPlaceHeight.add(pl_2.height)
                    }
                }
            }
        }
        val decorationKindRef = mutableListOf<Int>()
        val decorationLeft = mutableListOf<Float>()
        val decorationTop = mutableListOf<Float>()
        val decorationRight = mutableListOf<Float>()
        run {
            val _g1_14 = plan.decorationSegments
            for (seg in _g1_14) {
                pool.intern(seg.kind)
            }
        }
        run {
            val _g1_15 = plan.decorationSegments
            for (seg_2 in _g1_15) {
                decorationKindRef.add(pool.indexOf(seg_2.kind))
                decorationLeft.add(seg_2.left)
                decorationTop.add(seg_2.top)
                decorationRight.add(seg_2.right)
            }
        }
        val emphasisRangeCount = plan.emphasisRanges.size
        val inlineEdgeCount = plan.inlineEdges.size
        val rubyCount = plan.rubyDecisions.size
        val bopomofoCount = plan.bopomofoDecisions.size
        val bopomofoPlacementTotal = bopomofoPlaceTextRef.size
        val decorationSegmentCount = plan.decorationSegments.size
        val emphasisDotCount = plan.emphasisDots.size
        val stringCount = pool.ordered.size
        val featureTotal = featurePool.size
        val rubyFamilyTotal = rubyFamilyPool.size
        val bopomofoFamilyTotal = bopomofoFamilyPool.size
        writer.u32(1414615120)
        writer.u32(1)
        writer.f64(plan.width)
        writer.f64(plan.height)
        val fontSize = plan.fontSize
        writer.f64((if ((fontSize != null)) fontSize else Float.NaN))
        val overlayWidth = plan.overlayWidth
        writer.f64((if ((overlayWidth != null)) overlayWidth else Float.NaN))
        writer.u32(plan.lines.size)
        writer.u32(cellCount)
        writer.u32(emphasisRangeCount)
        writer.u32(inlineEdgeCount)
        writer.u32(rubyCount)
        writer.u32(bopomofoCount)
        writer.u32(bopomofoPlacementTotal)
        writer.u32(decorationSegmentCount)
        writer.u32(emphasisDotCount)
        writer.u32(stringCount)
        writer.u32(featureTotal)
        writer.u32(rubyFamilyTotal)
        writer.u32(bopomofoFamilyTotal)
        run {
            val _g1_16 = pool.ordered
            for (s in _g1_16) {
                val encoded = s.toByteArray(Charsets.UTF_8)
                writer.u32(encoded.size)
            }
        }
        run {
            val _g1_17 = pool.ordered
            for (s_2 in _g1_17) {
                writer.raw(s_2.toByteArray(Charsets.UTF_8))
            }
        }
        run {
            val _g1_18 = plan.lines
            for (line_3 in _g1_18) {
                writer.u32(line_3.rangeStart)
            }
        }
        run {
            val _g1_19 = plan.lines
            for (line_4 in _g1_19) {
                writer.u32(line_4.rangeEnd)
            }
        }
        run {
            val _g1_20 = plan.lines
            for (line_5 in _g1_20) {
                writer.f64(line_5.top)
            }
        }
        run {
            val _g1_21 = plan.lines
            for (line_6 in _g1_21) {
                writer.f64(line_6.bottom)
            }
        }
        run {
            val _g1_22 = plan.lines
            for (line_7 in _g1_22) {
                writer.f64(line_7.baseline)
            }
        }
        run {
            val _g1_23 = plan.lines
            for (line_8 in _g1_23) {
                writer.f64(line_8.indent)
            }
        }
        run {
            val _g1_24 = plan.lines
            for (line_9 in _g1_24) {
                writer.f64(line_9.visualWidth)
            }
        }
        run {
            val _g1_25 = plan.lines
            for (line_10 in _g1_25) {
                writer.f64(line_10.hyphenAdvance)
            }
        }
        run {
            val _g1_26 = plan.lines
            for (line_11 in _g1_26) {
                writer.u8(PlanPacked.endReasonCode(line_11.endReason))
            }
        }
        run {
            val _g1_27 = plan.lines
            for (line_12 in _g1_27) {
                writer.u32(line_12.cells.size)
            }
        }
        for (v in cellRangeStart) {
            writer.u32(v)
        }
        for (v_2 in cellRangeEnd) {
            writer.u32(v_2)
        }
        for (v_3 in cellSourceRef) {
            writer.u32(v_3)
        }
        for (v_4 in cellDisplayRef) {
            writer.u32(v_4)
        }
        for (v_5 in cellDrawX) {
            writer.f64(v_5)
        }
        for (v_6 in cellNaturalWidth) {
            writer.f64(v_6)
        }
        for (v_7 in cellLeadingAdvance) {
            writer.f64(v_7)
        }
        for (v_8 in cellShapingBoundary) {
            writer.u8(v_8)
        }
        for (v_9 in cellLatin) {
            writer.u8(v_9)
        }
        for (v_10 in cellRenderFamilyRef) {
            writer.u32(v_10)
        }
        for (v_11 in cellDashRef) {
            writer.u32(v_11)
        }
        for (v_12 in cellLanguageRef) {
            writer.u32(v_12)
        }
        for (v_13 in cellResolvedFaceRef) {
            writer.u32(v_13)
        }
        for (v_14 in cellGlyphIdsRef) {
            writer.u32(v_14)
        }
        for (v_15 in cellEvidenceRef) {
            writer.u32(v_15)
        }
        for (v_16 in cellInkFloor) {
            writer.f64(v_16)
        }
        for (v_17 in cellBodyWidth) {
            writer.f64(v_17)
        }
        for (v_18 in cellAdvance) {
            writer.f64(v_18)
        }
        for (v_19 in cellInlineObject) {
            writer.f64(v_19)
        }
        for (v_20 in cellStyleFontSize) {
            writer.f64(v_20)
        }
        for (v_21 in cellStyleFontWeight) {
            writer.f64(v_21)
        }
        for (v_22 in cellStyleItalic) {
            writer.u8(v_22)
        }
        for (v_23 in cellFeatureOffset) {
            writer.u32(v_23)
        }
        for (v_24 in cellFeatureCount) {
            writer.u32(v_24)
        }
        for (v_25 in featurePool) {
            writer.u32(v_25)
        }
        run {
            val _g1_28 = plan.emphasisRanges
            for (range in _g1_28) {
                writer.f64((range.start).toFloat())
            }
        }
        run {
            val _g1_29 = plan.emphasisRanges
            for (range_2 in _g1_29) {
                writer.f64((range_2.end).toFloat())
            }
        }
        run {
            val _g1_30 = plan.inlineEdges
            for (edge in _g1_30) {
                writer.f64((edge.offset).toFloat())
            }
        }
        run {
            val _g1_31 = plan.inlineEdges
            for (edge_2 in _g1_31) {
                val s_3 = edge_2.inlineStart
                writer.f64((if ((s_3 != null)) s_3 else Float.NaN))
            }
        }
        run {
            val _g1_32 = plan.inlineEdges
            for (edge_3 in _g1_32) {
                val e = edge_3.inlineEnd
                writer.f64((if ((e != null)) e else Float.NaN))
            }
        }
        for (v_26 in rubyBaseStart) {
            writer.u32(v_26)
        }
        for (v_27 in rubyBaseEnd) {
            writer.u32(v_27)
        }
        for (v_28 in rubyTextRef) {
            writer.u32(v_28)
        }
        for (v_29 in rubyCenterX) {
            writer.f64(v_29)
        }
        for (v_30 in rubyBaselineY) {
            writer.f64(v_30)
        }
        for (v_31 in rubyFontSize) {
            writer.f64(v_31)
        }
        for (v_32 in rubyFontWeight) {
            writer.u32(v_32)
        }
        for (v_33 in rubyFamilyOffset) {
            writer.u32(v_33)
        }
        for (v_34 in rubyFamilyCount) {
            writer.u32(v_34)
        }
        for (v_35 in rubyAscent) {
            writer.f64(v_35)
        }
        for (v_36 in rubyFamilyPool) {
            writer.u32(v_36)
        }
        for (v_37 in bopomofoBaseStart) {
            writer.u32(v_37)
        }
        for (v_38 in bopomofoBaseEnd) {
            writer.u32(v_38)
        }
        for (v_39 in bopomofoTextRef) {
            writer.u32(v_39)
        }
        for (v_40 in bopomofoFontWeight) {
            writer.u32(v_40)
        }
        for (v_41 in bopomofoFamilyOffset) {
            writer.u32(v_41)
        }
        for (v_42 in bopomofoFamilyCount) {
            writer.u32(v_42)
        }
        for (v_43 in bopomofoPlaceOffset) {
            writer.u32(v_43)
        }
        for (v_44 in bopomofoPlaceCount) {
            writer.u32(v_44)
        }
        for (v_45 in bopomofoFamilyPool) {
            writer.u32(v_45)
        }
        for (v_46 in bopomofoPlaceTextRef) {
            writer.u32(v_46)
        }
        for (v_47 in bopomofoPlaceRoleRef) {
            writer.u32(v_47)
        }
        for (v_48 in bopomofoPlaceLeft) {
            writer.f64(v_48)
        }
        for (v_49 in bopomofoPlaceTop) {
            writer.f64(v_49)
        }
        for (v_50 in bopomofoPlaceWidth) {
            writer.f64(v_50)
        }
        for (v_51 in bopomofoPlaceHeight) {
            writer.f64(v_51)
        }
        for (v_52 in decorationKindRef) {
            writer.u32(v_52)
        }
        for (v_53 in decorationLeft) {
            writer.f64(v_53)
        }
        for (v_54 in decorationTop) {
            writer.f64(v_54)
        }
        for (v_55 in decorationRight) {
            writer.f64(v_55)
        }
        run {
            val _g1_33 = plan.emphasisDots
            for (dot in _g1_33) {
                val cs = dot.clusterRangeStart
                writer.f64((if ((cs != null)) cs else Float.NaN))
            }
        }
        run {
            val _g1_34 = plan.emphasisDots
            for (dot_2 in _g1_34) {
                writer.f64(dot_2.anchorX)
            }
        }
        run {
            val _g1_35 = plan.emphasisDots
            for (dot_3 in _g1_35) {
                writer.f64(dot_3.anchorY)
            }
        }
        run {
            val _g1_36 = plan.emphasisDots
            for (dot_4 in _g1_36) {
                writer.f64(dot_4.dotDiameter)
            }
        }
        return writer.finish()
    }

    private fun endReasonCode(reason: PlanEndReason): Int {
        return when (reason) {
            PlanEndReason.AutoWrap -> 0
            PlanEndReason.MandatoryBreak -> 1
            PlanEndReason.ParagraphEnd -> 2
        }
    }

    fun decode(bytes: ByteArray): Plan? {
        val r = PlanPackedReader(bytes)
        if ((r.u32() != 1414615120)) {
            return null
        }
        if ((r.u32() != 1)) {
            return null
        }
        val width = r.f64()
        val height = r.f64()
        val fontSz = r.f64()
        val overlayW = r.f64()
        val lc = r.u32()
        val cc = r.u32()
        val erc = r.u32()
        val iec = r.u32()
        val rc = r.u32()
        val bc = r.u32()
        val bpt = r.u32()
        val dsc = r.u32()
        val edc = r.u32()
        val sc = r.u32()
        val ft = r.u32()
        val rft = r.u32()
        val bft = r.u32()
        if ((r.failed)) {
            return null
        }
        val deltas = mutableListOf<Int>()
        for (i in 0 until sc) {
            deltas.add(r.u32())
        }
        val pool = mutableListOf<String>()
        for (d in deltas) {
            pool.add(r.string(d))
        }
        if ((r.failed)) {
            return null
        }
        val sref = fun(ref: Int): String? {
    return (if ((ref != -1)) pool[ref] else null)
}
        val lrs = mutableListOf<Int>()
        for (i_2 in 0 until lc) {
            lrs.add(r.u32())
        }
        val lre = mutableListOf<Int>()
        for (i_3 in 0 until lc) {
            lre.add(r.u32())
        }
        val lt = mutableListOf<Float>()
        for (i_4 in 0 until lc) {
            lt.add(r.f64())
        }
        val lb = mutableListOf<Float>()
        for (i_5 in 0 until lc) {
            lb.add(r.f64())
        }
        val lbl = mutableListOf<Float>()
        for (i_6 in 0 until lc) {
            lbl.add(r.f64())
        }
        val lin = mutableListOf<Float>()
        for (i_7 in 0 until lc) {
            lin.add(r.f64())
        }
        val lvw = mutableListOf<Float>()
        for (i_8 in 0 until lc) {
            lvw.add(r.f64())
        }
        val lha = mutableListOf<Float>()
        for (i_9 in 0 until lc) {
            lha.add(r.f64())
        }
        val ler = mutableListOf<Int>()
        for (i_10 in 0 until lc) {
            ler.add(r.u8())
        }
        val lcc = mutableListOf<Int>()
        for (i_11 in 0 until lc) {
            lcc.add(r.u32())
        }
        if ((r.failed)) {
            return null
        }
        val crs = mutableListOf<Int>()
        for (i_12 in 0 until cc) {
            crs.add(r.u32())
        }
        val cre = mutableListOf<Int>()
        for (i_13 in 0 until cc) {
            cre.add(r.u32())
        }
        val csr = mutableListOf<Int>()
        for (i_14 in 0 until cc) {
            csr.add(r.u32())
        }
        val cdr = mutableListOf<Int>()
        for (i_15 in 0 until cc) {
            cdr.add(r.u32())
        }
        val cdx = mutableListOf<Float>()
        for (i_16 in 0 until cc) {
            cdx.add(r.f64())
        }
        val cnw = mutableListOf<Float>()
        for (i_17 in 0 until cc) {
            cnw.add(r.f64())
        }
        val cla = mutableListOf<Float>()
        for (i_18 in 0 until cc) {
            cla.add(r.f64())
        }
        val csb = mutableListOf<Int>()
        for (i_19 in 0 until cc) {
            csb.add(r.u8())
        }
        val cla2 = mutableListOf<Int>()
        for (i_20 in 0 until cc) {
            cla2.add(r.u8())
        }
        val crf = mutableListOf<Int>()
        for (i_21 in 0 until cc) {
            crf.add(r.u32())
        }
        val cd2 = mutableListOf<Int>()
        for (i_22 in 0 until cc) {
            cd2.add(r.u32())
        }
        val cl2 = mutableListOf<Int>()
        for (i_23 in 0 until cc) {
            cl2.add(r.u32())
        }
        val crf2 = mutableListOf<Int>()
        for (i_24 in 0 until cc) {
            crf2.add(r.u32())
        }
        val cgi = mutableListOf<Int>()
        for (i_25 in 0 until cc) {
            cgi.add(r.u32())
        }
        val cev = mutableListOf<Int>()
        for (i_26 in 0 until cc) {
            cev.add(r.u32())
        }
        val cif = mutableListOf<Float>()
        for (i_27 in 0 until cc) {
            cif.add(r.f64())
        }
        val cbw = mutableListOf<Float>()
        for (i_28 in 0 until cc) {
            cbw.add(r.f64())
        }
        val cad = mutableListOf<Float>()
        for (i_29 in 0 until cc) {
            cad.add(r.f64())
        }
        val cio = mutableListOf<Float>()
        for (i_30 in 0 until cc) {
            cio.add(r.f64())
        }
        val csf = mutableListOf<Float>()
        for (i_31 in 0 until cc) {
            csf.add(r.f64())
        }
        val csw = mutableListOf<Float>()
        for (i_32 in 0 until cc) {
            csw.add(r.f64())
        }
        val csi = mutableListOf<Int>()
        for (i_33 in 0 until cc) {
            csi.add(r.u8())
        }
        val cfo = mutableListOf<Int>()
        for (i_34 in 0 until cc) {
            cfo.add(r.u32())
        }
        val cfc = mutableListOf<Int>()
        for (i_35 in 0 until cc) {
            cfc.add(r.u32())
        }
        if ((r.failed)) {
            return null
        }
        val fp = mutableListOf<Int>()
        for (i_36 in 0 until ft) {
            fp.add(r.u32())
        }
        var cursor = 0
        val lines = mutableListOf<PlanLine>()
        for (li in 0 until lc) {
            val cellN = lcc[li]
            val cells = mutableListOf<PlanCell>()
            for (ci in 0 until cellN) {
                val i_37 = cursor
                cursor += 1
                val fo = cfo[i_37]
                val fn = cfc[i_37]
                val feats = mutableListOf<String>()
                for (k in 0 until fn) {
                    feats.add(pool[fp[fo + k]])
                }
                val sf = csf[i_37]
                val sw = csw[i_37]
                val si = csi[i_37]
                val style = (if (((sf).isNaN() && (sw).isNaN() && si == 2)) null else PlanStyleDelta(fontSize = (if (((sf).isNaN())) null else sf), fontWeight = (if (((sw).isNaN())) null else (sw).toInt()), italic = (if ((si == 2)) null else si != 0)))
                cells.add(PlanCell(rangeStart = crs[i_37], rangeEnd = cre[i_37], source = pool[csr[i_37]], display = pool[cdr[i_37]], drawX = cdx[i_37], naturalWidth = cnw[i_37], leadingLayoutAdvance = cla[i_37], shapingBoundary = csb[i_37] != 0, openTypeFeatures = feats, renderFontFamily = sref(crf[i_37]), dashStrategy = sref(cd2[i_37]), shapingLanguage = sref(cl2[i_37]), resolvedFace = sref(crf2[i_37]), glyphIds = sref(cgi[i_37]), shapingEvidence = sref(cev[i_37]), punctuationInkFloor = (if (((cif[i_37]).isNaN())) null else cif[i_37]), punctuationBodyWidth = (if (((cbw[i_37]).isNaN())) null else cbw[i_37]), latin = cla2[i_37] != 0, advance = (if (((cad[i_37]).isNaN())) null else cad[i_37]), inlineObject = (if (((cio[i_37]).isNaN())) null else cio[i_37]), styleDelta = style))
            }
            val er = ler[li]
            val endR = (if ((er == 1)) PlanEndReason.MandatoryBreak else (if ((er == 2)) PlanEndReason.ParagraphEnd else PlanEndReason.AutoWrap))
            lines.add(PlanLine(rangeStart = lrs[li], rangeEnd = lre[li], top = lt[li], bottom = lb[li], baseline = lbl[li], indent = lin[li], visualWidth = lvw[li], hyphenAdvance = lha[li], endReason = endR, cells = cells))
        }
        val emphasis = mutableListOf<PlanEmphasisRange>()
        for (i_38 in 0 until erc) {
            val s = r.f64()
            val e = r.f64()
            emphasis.add(PlanEmphasisRange(start = (s).toInt(), end = (e).toInt()))
        }
        val edges = mutableListOf<PlanInlineEdge>()
        for (i_39 in 0 until iec) {
            val off = r.f64()
            val s_2 = r.f64()
            val e_2 = r.f64()
            edges.add(PlanInlineEdge(offset = (off).toInt(), inlineStart = (if (((s_2).isNaN())) null else s_2), inlineEnd = (if (((e_2).isNaN())) null else e_2)))
        }
        val rbS = mutableListOf<Int>()
        for (i_40 in 0 until rc) {
            rbS.add(r.u32())
        }
        val rbE = mutableListOf<Int>()
        for (i_41 in 0 until rc) {
            rbE.add(r.u32())
        }
        val rbT = mutableListOf<Int>()
        for (i_42 in 0 until rc) {
            rbT.add(r.u32())
        }
        val rbCx = mutableListOf<Float>()
        for (i_43 in 0 until rc) {
            rbCx.add(r.f64())
        }
        val rbBy = mutableListOf<Float>()
        for (i_44 in 0 until rc) {
            rbBy.add(r.f64())
        }
        val rbFs = mutableListOf<Float>()
        for (i_45 in 0 until rc) {
            rbFs.add(r.f64())
        }
        val rbFw = mutableListOf<Int>()
        for (i_46 in 0 until rc) {
            rbFw.add(r.u32())
        }
        val rbFo = mutableListOf<Int>()
        for (i_47 in 0 until rc) {
            rbFo.add(r.u32())
        }
        val rbFc = mutableListOf<Int>()
        for (i_48 in 0 until rc) {
            rbFc.add(r.u32())
        }
        val rbAs = mutableListOf<Float>()
        for (i_49 in 0 until rc) {
            rbAs.add(r.f64())
        }
        val rubyFam = mutableListOf<Int>()
        for (i_50 in 0 until rft) {
            rubyFam.add(r.u32())
        }
        val rubys = mutableListOf<PlanRuby>()
        for (i_51 in 0 until rc) {
            val fams = mutableListOf<String>()
            for (k_2 in 0 until rbFc[i_51]) {
                fams.add(pool[rubyFam[rbFo[i_51] + k_2]])
            }
            rubys.add(PlanRuby(baseRangeStart = rbS[i_51], baseRangeEnd = rbE[i_51], text = pool[rbT[i_51]], centerX = rbCx[i_51], baselineY = rbBy[i_51], fontSize = rbFs[i_51], fontWeight = rbFw[i_51], fontFamilies = fams, ascent = (if (((rbAs[i_51]).isNaN())) null else rbAs[i_51])))
        }
        val bbS = mutableListOf<Int>()
        for (i_52 in 0 until bc) {
            bbS.add(r.u32())
        }
        val bbE = mutableListOf<Int>()
        for (i_53 in 0 until bc) {
            bbE.add(r.u32())
        }
        val bbT = mutableListOf<Int>()
        for (i_54 in 0 until bc) {
            bbT.add(r.u32())
        }
        val bbFw = mutableListOf<Int>()
        for (i_55 in 0 until bc) {
            bbFw.add(r.u32())
        }
        val bbFo = mutableListOf<Int>()
        for (i_56 in 0 until bc) {
            bbFo.add(r.u32())
        }
        val bbFc = mutableListOf<Int>()
        for (i_57 in 0 until bc) {
            bbFc.add(r.u32())
        }
        val bbPo = mutableListOf<Int>()
        for (i_58 in 0 until bc) {
            bbPo.add(r.u32())
        }
        val bbPc = mutableListOf<Int>()
        for (i_59 in 0 until bc) {
            bbPc.add(r.u32())
        }
        val bopoFam = mutableListOf<Int>()
        for (i_60 in 0 until bft) {
            bopoFam.add(r.u32())
        }
        val bpT = mutableListOf<Int>()
        for (i_61 in 0 until bpt) {
            bpT.add(r.u32())
        }
        val bpR = mutableListOf<Int>()
        for (i_62 in 0 until bpt) {
            bpR.add(r.u32())
        }
        val bpL = mutableListOf<Float>()
        for (i_63 in 0 until bpt) {
            bpL.add(r.f64())
        }
        val bpT2 = mutableListOf<Float>()
        for (i_64 in 0 until bpt) {
            bpT2.add(r.f64())
        }
        val bpW = mutableListOf<Float>()
        for (i_65 in 0 until bpt) {
            bpW.add(r.f64())
        }
        val bpH = mutableListOf<Float>()
        for (i_66 in 0 until bpt) {
            bpH.add(r.f64())
        }
        val bopos = mutableListOf<PlanBopomofo>()
        for (i_67 in 0 until bc) {
            val fams_2 = mutableListOf<String>()
            for (k_3 in 0 until bbFc[i_67]) {
                fams_2.add(pool[bopoFam[bbFo[i_67] + k_3]])
            }
            val placements = mutableListOf<PlanBopomofoPlacement>()
            for (k_4 in 0 until bbPc[i_67]) {
                val pidx = bbPo[i_67] + k_4
                placements.add(PlanBopomofoPlacement(text = pool[bpT[pidx]], role = pool[bpR[pidx]], left = bpL[pidx], top = bpT2[pidx], width = bpW[pidx], height = bpH[pidx]))
            }
            bopos.add(PlanBopomofo(baseRangeStart = bbS[i_67], baseRangeEnd = bbE[i_67], text = pool[bbT[i_67]], fontWeight = bbFw[i_67], fontFamilies = fams_2, placements = placements))
        }
        val decos = mutableListOf<PlanDecorationSegment>()
        for (i_68 in 0 until dsc) {
            val kr = r.u32()
            decos.add(PlanDecorationSegment(kind = pool[kr], left = r.f64(), top = r.f64(), right = r.f64(), sourceRangeStart = 0, sourceRangeEnd = 0))
        }
        val dots = mutableListOf<PlanEmphasisDot>()
        for (i_69 in 0 until edc) {
            val ds = r.f64()
            dots.add(PlanEmphasisDot(clusterRangeStart = (if (((ds).isNaN())) null else ds), anchorX = r.f64(), anchorY = r.f64(), dotDiameter = r.f64()))
        }
        if ((r.failed)) {
            return null
        }
        return Plan(width = width, height = height, lines = lines, emphasisRanges = emphasis, inlineEdges = edges, rubyDecisions = rubys, bopomofoDecisions = bopos, fontSize = (if (((fontSz).isNaN())) null else fontSz), overlayWidth = (if (((overlayW).isNaN())) null else overlayW), decorationSegments = decos, emphasisDots = dots)
    }
    private class PlanPackedWriter {
        val buf: BytesBuffer
        init {
        this.buf = BytesBuffer()
        }
    
        fun u8(v: Int) {
            this.buf.addByte(((v) and (255)))
        }
    
        fun u16(v: Int) {
            this.buf.addByte(((v) and (255)))
            this.buf.addByte(((((v) ushr (8))) and (255)))
        }
    
        fun u32(v: Int) {
            this.buf.addByte(((v) and (255)))
            this.buf.addByte(((((v) ushr (8))) and (255)))
            this.buf.addByte(((((v) ushr (16))) and (255)))
            this.buf.addByte(((((v) ushr (24))) and (255)))
        }
    
        fun f64(v: Float) {
            val bits = FPHelper.f32ToI64(v)
            this.u32(bits.low)
            this.u32(bits.high)
        }
    
        fun raw(bytes: ByteArray) {
            for (i in 0 until bytes.size) {
                this.buf.addByte((( bytes[i].toInt() and 0xFF )))
            }
        }
    
        fun finish(): ByteArray {
            return this.buf.getBytes()
        }
    }
    private class PlanPackedReader(private val bytes: ByteArray) {
        var pos: Int
        var failed: Boolean
        init {
        this.pos = 0
        this.failed = false
        }
    
        fun u8(): Int {
            if ((!this.need(1))) {
                return 0
            }
            val v = (( this.bytes[this.pos].toInt() and 0xFF ))
            this.pos += 1
            return v
        }
    
        fun u32(): Int {
            if ((!this.need(4))) {
                return 0
            }
            val v = ((((((( this.bytes[this.pos].toInt() and 0xFF ))) or ((((( this.bytes[this.pos + 1].toInt() and 0xFF ))) shl (8))))) or ((((( this.bytes[this.pos + 2].toInt() and 0xFF ))) shl (16))))) + (( this.bytes[this.pos + 3].toInt() and 0xFF )) * 16777216
            this.pos += 4
            return v
        }
    
        fun f64(): Float {
            val low = this.u32()
            val high = this.u32()
            if ((this.failed)) {
                return 0.0f
            }
            return FPHelper.i64ToF32(low, high)
        }
    
        fun string(len: Int): String {
            if ((!this.need(len))) {
                return ""
            }
            val s = String(this.bytes, this.pos, len, Charsets.UTF_8)
            this.pos += len
            return s
        }
    
        private fun need(length: Int): Boolean {
            if ((this.failed)) {
                return false
            }
            if ((length < 0 || this.pos + length > this.bytes.size)) {
                this.failed = true
                return false
            }
            return true
        }
    }
    private class StringPool {
        val ordered: MutableList<String>
        init {
        this.ordered = mutableListOf<String>()
        }
    
        fun intern(value: String): Int {
            for (i in 0 until this.ordered.size) {
                if ((this.ordered[i] == value)) {
                    return i
                }
            }
            val idx = this.ordered.size
            this.ordered.add(value)
            return idx
        }
    
        fun indexOf(value: String): Int {
            for (i in 0 until this.ordered.size) {
                if ((this.ordered[i] == value)) {
                    return i
                }
            }
            return -1
        }
    }
}
