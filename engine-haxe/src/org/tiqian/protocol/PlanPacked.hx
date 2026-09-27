package org.tiqian.protocol;

import haxe.io.Bytes;
import haxe.io.BytesBuffer;
import haxe.io.FPHelper;
import org.tiqian.protocol.Plan;

/**
 * Packed plan encoding and decoding (Stage1-P3).
 *
 * The column-major layout mirrors PlanPackedWriter.kt (the Kotlin producer)
 * and plan_packed.rs (the Rust reader) one-to-one: u32 magic + version
 * header, a string pool with u32 deltas, per-column f64 geometry and u32
 * string refs, and NaN sentinels for absent scalars.
 *
 * P2's unified Reader/Writer (TableWriter / TableReader, stdlib/02:15-217)
 * provides the byte-level primitives: u8/u16/u32/f64 little-endian,
 * BytesBuffer for growing, Bytes.get for reading. This module reuses them
 * for the plan-specific column schema.
 */

private class PlanPackedWriter {
    final buf:BytesBuffer;

    public function new() { buf = new BytesBuffer(); }

    public function u8(v:Int):Void { buf.addByte(v & 0xFF); }
    public function u16(v:Int):Void {
        buf.addByte(v & 0xFF); buf.addByte((v >>> 8) & 0xFF);
    }
    public function u32(v:Int):Void {
        buf.addByte(v & 0xFF); buf.addByte((v >>> 8) & 0xFF);
        buf.addByte((v >>> 16) & 0xFF); buf.addByte((v >>> 24) & 0xFF);
    }
    public function f64(v:Float):Void {
        final bits = FPHelper.doubleToI64(v);
        u32(bits.low); u32(bits.high);
    }
    public function raw(bytes:Bytes):Void {
        for (i in 0...bytes.length) buf.addByte(bytes.get(i));
    }
    public function finish():Bytes { return buf.getBytes(); }
}

private class StringPool {
    public final ordered:Array<String> = [];

    public function new() {
        ordered = [];
    }

    public function intern(value:String):Int {
        for (i in 0...ordered.length) {
            if (ordered[i] == value) return i;
        }
        final idx = ordered.length;
        ordered.push(value);
        return idx;
    }

    public function indexOf(value:String):Int {
        for (i in 0...ordered.length) {
            if (ordered[i] == value) return i;
        }
        return -1;
    }
}

class PlanPacked {
    static inline var PLAN_MAGIC:Int = 0x54515050;
    static inline var PLAN_PROTOCOL_REVISION:Int = 1;
    static inline var PLAN_STRING_ABSENT:Int = 0xFFFFFFFF;

    /**
     * Encodes one Plan into its column-major packed bytes. Every string
     * (source, display, font families, feature tags, evidence refs, …) goes
     * through a global pool; absent scalar fields ride NaN (f64) or
     * PLAN_STRING_ABSENT (u32). The column order mirrors
     * PlanPackedWriter.kt:170-355 exactly.
     */
    public static function encode(plan:Plan):Bytes {
        final pool = new StringPool();
        final writer = new PlanPackedWriter();

        // Gather cell writes (column vectors), flat feature pool, families.
        final cellRangeStart:Array<Int> = [];
        final cellRangeEnd:Array<Int> = [];
        final cellSourceRef:Array<Int> = [];
        final cellDisplayRef:Array<Int> = [];
        final cellDrawX:Array<Float> = [];
        final cellNaturalWidth:Array<Float> = [];
        final cellLeadingAdvance:Array<Float> = [];
        final cellShapingBoundary:Array<Int> = [];
        final cellLatin:Array<Int> = [];
        final cellRenderFamilyRef:Array<Int> = [];
        final cellDashRef:Array<Int> = [];
        final cellLanguageRef:Array<Int> = [];
        final cellResolvedFaceRef:Array<Int> = [];
        final cellGlyphIdsRef:Array<Int> = [];
        final cellEvidenceRef:Array<Int> = [];
        final cellInkFloor:Array<Float> = [];
        final cellBodyWidth:Array<Float> = [];
        final cellAdvance:Array<Float> = [];
        final cellInlineObject:Array<Float> = [];
        final cellStyleFontSize:Array<Float> = [];
        final cellStyleFontWeight:Array<Float> = [];
        final cellStyleItalic:Array<Int> = [];
        final cellFeatureOffset:Array<Int> = [];
        final cellFeatureCount:Array<Int> = [];
        final featurePool:Array<Int> = [];

        var cellCount = 0;
        for (line in plan.lines) {
            cellCount += line.cells.length;
        }

        for (line in plan.lines) {
            for (cell in line.cells) {
                cellRangeStart.push(cell.rangeStart);
                cellRangeEnd.push(cell.rangeEnd);
                cellSourceRef.push(pool.intern(cell.source));
                cellDisplayRef.push(pool.intern(cell.display));
                cellDrawX.push(cell.drawX);
                cellNaturalWidth.push(cell.naturalWidth);
                cellLeadingAdvance.push(cell.leadingLayoutAdvance);
                cellShapingBoundary.push(cell.shapingBoundary ? 1 : 0);
                cellLatin.push(cell.latin ? 1 : 0);
                cellRenderFamilyRef.push(cell.renderFontFamily != null ? pool.intern(cell.renderFontFamily) : PLAN_STRING_ABSENT);
                cellDashRef.push(cell.dashStrategy != null ? pool.intern(cell.dashStrategy) : PLAN_STRING_ABSENT);
                cellLanguageRef.push(cell.shapingLanguage != null ? pool.intern(cell.shapingLanguage) : PLAN_STRING_ABSENT);
                cellResolvedFaceRef.push(cell.resolvedFace != null ? pool.intern(cell.resolvedFace) : PLAN_STRING_ABSENT);
                cellGlyphIdsRef.push(cell.glyphIds != null ? pool.intern(cell.glyphIds) : PLAN_STRING_ABSENT);
                cellEvidenceRef.push(cell.shapingEvidence != null ? pool.intern(cell.shapingEvidence) : PLAN_STRING_ABSENT);
                cellInkFloor.push(cell.punctuationInkFloor != null ? cell.punctuationInkFloor : Math.NaN);
                cellBodyWidth.push(cell.punctuationBodyWidth != null ? cell.punctuationBodyWidth : Math.NaN);
                cellAdvance.push(cell.advance != null ? cell.advance : Math.NaN);
                cellInlineObject.push(cell.inlineObject != null ? cell.inlineObject : Math.NaN);
                final style = cell.styleDelta;
                var styleFontSize = Math.NaN;
                var styleFontWeight = Math.NaN;
                var styleItalic = 2;
                if (style != null) {
                    final fs = style.fontSize;
                    if (fs != null) styleFontSize = (fs : Float);
                    final fw = style.fontWeight;
                    if (fw != null) styleFontWeight = (fw : Float);
                    final it = style.italic;
                    if (it != null) styleItalic = it ? 1 : 0;
                }
                cellStyleFontSize.push(styleFontSize);
                cellStyleFontWeight.push(styleFontWeight);
                cellStyleItalic.push(styleItalic);
                final feats = cell.openTypeFeatures;
                for (f in feats) pool.intern(f);
                cellFeatureOffset.push(featurePool.length);
                cellFeatureCount.push(feats.length);
                for (f in feats) featurePool.push(pool.indexOf(f));
            }
        }

        // Ruby: pool families and texts, collect columns.
        final rubyBaseStart:Array<Int> = [];
        final rubyBaseEnd:Array<Int> = [];
        final rubyTextRef:Array<Int> = [];
        final rubyCenterX:Array<Float> = [];
        final rubyBaselineY:Array<Float> = [];
        final rubyFontSize:Array<Float> = [];
        final rubyFontWeight:Array<Int> = [];
        final rubyFamilyOffset:Array<Int> = [];
        final rubyFamilyCount:Array<Int> = [];
        final rubyAscent:Array<Float> = [];
        final rubyFamilyPool:Array<Int> = [];
        for (ruby in plan.rubyDecisions) {
            for (fam in ruby.fontFamilies) pool.intern(fam);
            pool.intern(ruby.text);
        }
        for (ruby in plan.rubyDecisions) {
            rubyBaseStart.push(ruby.baseRangeStart);
            rubyBaseEnd.push(ruby.baseRangeEnd);
            rubyTextRef.push(pool.indexOf(ruby.text));
            rubyCenterX.push(ruby.centerX);
            rubyBaselineY.push(ruby.baselineY);
            rubyFontSize.push(ruby.fontSize);
            rubyFontWeight.push(ruby.fontWeight);
            rubyFamilyOffset.push(rubyFamilyPool.length);
            rubyFamilyCount.push(ruby.fontFamilies.length);
            rubyAscent.push(ruby.ascent != null ? (ruby.ascent : Float) : Math.NaN);
            for (fam in ruby.fontFamilies) rubyFamilyPool.push(pool.indexOf(fam));
        }

        // Bopomofo: pool families, texts, roles, placements.
        final bopomofoBaseStart:Array<Int> = [];
        final bopomofoBaseEnd:Array<Int> = [];
        final bopomofoTextRef:Array<Int> = [];
        final bopomofoFontWeight:Array<Int> = [];
        final bopomofoFamilyOffset:Array<Int> = [];
        final bopomofoFamilyCount:Array<Int> = [];
        final bopomofoPlaceOffset:Array<Int> = [];
        final bopomofoPlaceCount:Array<Int> = [];
        final bopomofoFamilyPool:Array<Int> = [];
        final bopomofoPlaceTextRef:Array<Int> = [];
        final bopomofoPlaceRoleRef:Array<Int> = [];
        final bopomofoPlaceLeft:Array<Float> = [];
        final bopomofoPlaceTop:Array<Float> = [];
        final bopomofoPlaceWidth:Array<Float> = [];
        final bopomofoPlaceHeight:Array<Float> = [];
        for (bopomofo in plan.bopomofoDecisions) {
            for (fam in bopomofo.fontFamilies) pool.intern(fam);
            pool.intern(bopomofo.text);
            for (pl in bopomofo.placements) {
                pool.intern(pl.text);
                pool.intern(pl.role);
            }
        }
        for (bopomofo in plan.bopomofoDecisions) {
            bopomofoBaseStart.push(bopomofo.baseRangeStart);
            bopomofoBaseEnd.push(bopomofo.baseRangeEnd);
            bopomofoTextRef.push(pool.indexOf(bopomofo.text));
            bopomofoFontWeight.push(bopomofo.fontWeight);
            bopomofoFamilyOffset.push(bopomofoFamilyPool.length);
            bopomofoFamilyCount.push(bopomofo.fontFamilies.length);
            bopomofoPlaceOffset.push(bopomofoPlaceTextRef.length);
            bopomofoPlaceCount.push(bopomofo.placements.length);
            for (fam in bopomofo.fontFamilies) bopomofoFamilyPool.push(pool.indexOf(fam));
            for (pl in bopomofo.placements) {
                bopomofoPlaceTextRef.push(pool.indexOf(pl.text));
                bopomofoPlaceRoleRef.push(pool.indexOf(pl.role));
                bopomofoPlaceLeft.push(pl.left);
                bopomofoPlaceTop.push(pl.top);
                bopomofoPlaceWidth.push(pl.width);
                bopomofoPlaceHeight.push(pl.height);
            }
        }

        // Decoration segments: pool kind strings.
        final decorationKindRef:Array<Int> = [];
        final decorationLeft:Array<Float> = [];
        final decorationTop:Array<Float> = [];
        final decorationRight:Array<Float> = [];
        for (seg in plan.decorationSegments) pool.intern(seg.kind);
        for (seg in plan.decorationSegments) {
            decorationKindRef.push(pool.indexOf(seg.kind));
            decorationLeft.push(seg.left);
            decorationTop.push(seg.top);
            decorationRight.push(seg.right);
        }

        // Compute totals.
        final emphasisRangeCount = plan.emphasisRanges.length;
        final inlineEdgeCount = plan.inlineEdges.length;
        final rubyCount = plan.rubyDecisions.length;
        final bopomofoCount = plan.bopomofoDecisions.length;
        final bopomofoPlacementTotal = bopomofoPlaceTextRef.length;
        final decorationSegmentCount = plan.decorationSegments.length;
        final emphasisDotCount = plan.emphasisDots.length;
        final stringCount = pool.ordered.length;
        final featureTotal = featurePool.length;
        final rubyFamilyTotal = rubyFamilyPool.length;
        final bopomofoFamilyTotal = bopomofoFamilyPool.length;

        // === WRITE HEADER ===
        writer.u32(PLAN_MAGIC);
        writer.u32(PLAN_PROTOCOL_REVISION);
        writer.f64(plan.width);
        writer.f64(plan.height);
        final fontSize = plan.fontSize;
        writer.f64(fontSize != null ? (fontSize : Float) : Math.NaN);
        final overlayWidth = plan.overlayWidth;
        writer.f64(overlayWidth != null ? (overlayWidth : Float) : Math.NaN);
        writer.u32(plan.lines.length);
        writer.u32(cellCount);
        writer.u32(emphasisRangeCount);
        writer.u32(inlineEdgeCount);
        writer.u32(rubyCount);
        writer.u32(bopomofoCount);
        writer.u32(bopomofoPlacementTotal);
        writer.u32(decorationSegmentCount);
        writer.u32(emphasisDotCount);
        writer.u32(stringCount);
        writer.u32(featureTotal);
        writer.u32(rubyFamilyTotal);
        writer.u32(bopomofoFamilyTotal);

        // === STRING POOL ===
        for (s in pool.ordered) {
            final encoded = Bytes.ofString(s);
            writer.u32(encoded.length);
        }
        for (s in pool.ordered) {
            writer.raw(Bytes.ofString(s));
        }

        // === LINE COLUMNS ===
        for (line in plan.lines) writer.u32(line.rangeStart);
        for (line in plan.lines) writer.u32(line.rangeEnd);
        for (line in plan.lines) writer.f64(line.top);
        for (line in plan.lines) writer.f64(line.bottom);
        for (line in plan.lines) writer.f64(line.baseline);
        for (line in plan.lines) writer.f64(line.indent);
        for (line in plan.lines) writer.f64(line.visualWidth);
        for (line in plan.lines) writer.f64(line.hyphenAdvance);
        for (line in plan.lines) {
            writer.u8(endReasonCode(line.endReason));
        }
        for (line in plan.lines) writer.u32(line.cells.length);

        // === CELL COLUMNS (column-major) ===
        for (v in cellRangeStart) writer.u32(v);
        for (v in cellRangeEnd) writer.u32(v);
        for (v in cellSourceRef) writer.u32(v);
        for (v in cellDisplayRef) writer.u32(v);
        for (v in cellDrawX) writer.f64(v);
        for (v in cellNaturalWidth) writer.f64(v);
        for (v in cellLeadingAdvance) writer.f64(v);
        for (v in cellShapingBoundary) writer.u8(v);
        for (v in cellLatin) writer.u8(v);
        for (v in cellRenderFamilyRef) writer.u32(v);
        for (v in cellDashRef) writer.u32(v);
        for (v in cellLanguageRef) writer.u32(v);
        for (v in cellResolvedFaceRef) writer.u32(v);
        for (v in cellGlyphIdsRef) writer.u32(v);
        for (v in cellEvidenceRef) writer.u32(v);
        for (v in cellInkFloor) writer.f64(v);
        for (v in cellBodyWidth) writer.f64(v);
        for (v in cellAdvance) writer.f64(v);
        for (v in cellInlineObject) writer.f64(v);
        for (v in cellStyleFontSize) writer.f64(v);
        for (v in cellStyleFontWeight) writer.f64(v);
        for (v in cellStyleItalic) writer.u8(v);
        for (v in cellFeatureOffset) writer.u32(v);
        for (v in cellFeatureCount) writer.u32(v);

        // === FEATURE POOL ===
        for (v in featurePool) writer.u32(v);

        // === EMPHASIS RANGES ===
        for (range in plan.emphasisRanges) writer.f64(range.start);
        for (range in plan.emphasisRanges) writer.f64(range.end);

        // === INLINE EDGES ===
        for (edge in plan.inlineEdges) writer.f64(edge.offset);
        for (edge in plan.inlineEdges) writer.f64(edge.inlineStart != null ? (edge.inlineStart : Float) : Math.NaN);
        for (edge in plan.inlineEdges) writer.f64(edge.inlineEnd != null ? (edge.inlineEnd : Float) : Math.NaN);

        // === RUBY ===
        for (v in rubyBaseStart) writer.u32(v);
        for (v in rubyBaseEnd) writer.u32(v);
        for (v in rubyTextRef) writer.u32(v);
        for (v in rubyCenterX) writer.f64(v);
        for (v in rubyBaselineY) writer.f64(v);
        for (v in rubyFontSize) writer.f64(v);
        for (v in rubyFontWeight) writer.u32(v);
        for (v in rubyFamilyOffset) writer.u32(v);
        for (v in rubyFamilyCount) writer.u32(v);
        for (v in rubyAscent) writer.f64(v);
        for (v in rubyFamilyPool) writer.u32(v);

        // === BOPOMOFO ===
        for (v in bopomofoBaseStart) writer.u32(v);
        for (v in bopomofoBaseEnd) writer.u32(v);
        for (v in bopomofoTextRef) writer.u32(v);
        for (v in bopomofoFontWeight) writer.u32(v);
        for (v in bopomofoFamilyOffset) writer.u32(v);
        for (v in bopomofoFamilyCount) writer.u32(v);
        for (v in bopomofoPlaceOffset) writer.u32(v);
        for (v in bopomofoPlaceCount) writer.u32(v);
        for (v in bopomofoFamilyPool) writer.u32(v);
        for (v in bopomofoPlaceTextRef) writer.u32(v);
        for (v in bopomofoPlaceRoleRef) writer.u32(v);
        for (v in bopomofoPlaceLeft) writer.f64(v);
        for (v in bopomofoPlaceTop) writer.f64(v);
        for (v in bopomofoPlaceWidth) writer.f64(v);
        for (v in bopomofoPlaceHeight) writer.f64(v);

        // === DECORATION SEGMENTS ===
        for (v in decorationKindRef) writer.u32(v);
        for (v in decorationLeft) writer.f64(v);
        for (v in decorationTop) writer.f64(v);
        for (v in decorationRight) writer.f64(v);

        // === EMPHASIS DOTS ===
        for (dot in plan.emphasisDots) {
            final cs = dot.clusterRangeStart;
            writer.f64(cs != null ? (cs : Float) : Math.NaN);
        }
        for (dot in plan.emphasisDots) writer.f64(dot.anchorX);
        for (dot in plan.emphasisDots) writer.f64(dot.anchorY);
        for (dot in plan.emphasisDots) writer.f64(dot.dotDiameter);

        return writer.finish();
    }

    static function endReasonCode(reason:PlanEndReason):Int {
        return switch (reason) {
            case AutoWrap: 0;
            case MandatoryBreak: 1;
            case ParagraphEnd: 2;
        };
    }
}