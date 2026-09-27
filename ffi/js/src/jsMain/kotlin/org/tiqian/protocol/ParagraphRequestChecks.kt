package org.tiqian.protocol

object ParagraphRequestChecks {
    fun validate(request: ParagraphRequest) {
        if ((request.text.length == 0 || ParagraphRequestChecks.isBlankText(request.text))) {
            throw ParagraphRequestException.EmptyParagraph
        }
        if ((!(request.maxWidthPx).isFinite() || request.maxWidthPx <= 0.0)) {
            throw ParagraphRequestException.InvalidMaximumMeasure
        }
        if ((!(request.fontSizePx).isFinite() || request.fontSizePx <= 0.0)) {
            throw ParagraphRequestException.InvalidFontSize
        }
        if ((!(request.lineHeightPx).isFinite() || request.lineHeightPx <= 0.0)) {
            throw ParagraphRequestException.InvalidLineHeight
        }
        if ((!(request.firstLineIndentIc).isFinite())) {
            throw ParagraphRequestException.InvalidFirstLineIndent
        }
        if ((request.fontWeight < 1 || request.fontWeight > 1000)) {
            throw ParagraphRequestException.InvalidFontWeight
        }
        val rawGapEm = request.emphasisDotGapEm
        val gapEm = (if ((rawGapEm == null)) 0.1 else rawGapEm)
        if ((!(gapEm).isFinite() || gapEm < 0.0)) {
            throw ParagraphRequestException.InvalidEmphasisDotGapEm
        }
        if ((!ParagraphRequestChecks.hasNonBlankFamily(request.fontFamilies))) {
            throw ParagraphRequestException.MissingExplicitFontFamilies
        }
        val textLength = request.text.length
        var spanIndexIdx = 0
        while ((spanIndexIdx < request.textSpans.size)) {
            val span = request.textSpans[spanIndexIdx]
            if ((!ParagraphRequestChecks.validRange(span.start, span.end, textLength))) {
                throw ParagraphRequestException.InvalidTextSpanRange
            }
            if ((!ParagraphRequestChecks.hasNonBlankFamily(span.families))) {
                throw ParagraphRequestException.MissingTextSpanFontFamilies
            }
            if ((!(span.fontSizePx).isFinite() || span.fontSizePx <= 0.0)) {
                throw ParagraphRequestException.InvalidTextSpanFontSize
            }
            if ((span.fontWeight < 1 || span.fontWeight > 1000)) {
                throw ParagraphRequestException.InvalidTextSpanFontWeight
            }
            if ((!(span.baselineShift).isFinite())) {
                throw ParagraphRequestException.InvalidTextSpanBaselineShift
            }
            spanIndexIdx++
        }
        var boundaryIndexIdx = 0
        while ((boundaryIndexIdx < request.sourceBoundaries.size)) {
            val boundary = request.sourceBoundaries[boundaryIndexIdx]
            if ((boundary < 0 || boundary > textLength)) {
                throw ParagraphRequestException.InvalidSourceBoundary
            }
            boundaryIndexIdx++
        }
        var breakIndexIdx = 0
        while ((breakIndexIdx < request.lineBreakSpans.size)) {
            val span_2 = request.lineBreakSpans[breakIndexIdx]
            if ((!ParagraphRequestChecks.validRange(span_2.start, span_2.end, textLength))) {
                throw ParagraphRequestException.InvalidLineBreakSpanRange
            }
            breakIndexIdx++
        }
        var boxIndexIdx = 0
        while ((boxIndexIdx < request.inlineBoxes.size)) {
            val box = request.inlineBoxes[boxIndexIdx]
            if ((!ParagraphRequestChecks.validRange(box.start, box.end, textLength))) {
                throw ParagraphRequestException.InvalidInlineBoxRange
            }
            if ((!(box.inlineStart).isFinite() || !(box.inlineEnd).isFinite())) {
                throw ParagraphRequestException.InvalidInlineBoxGeometry
            }
            boxIndexIdx++
        }
        var objectIndexIdx = 0
        while ((objectIndexIdx < request.inlineObjects.size)) {
            val `object` = request.inlineObjects[objectIndexIdx]
            if ((!ParagraphRequestChecks.validRange(`object`.start, `object`.end, textLength))) {
                throw ParagraphRequestException.InvalidInlineObjectRange
            }
            if ((!(`object`.advance).isFinite() || `object`.advance < 0.0)) {
                throw ParagraphRequestException.InvalidInlineObjectAdvance
            }
            if ((!(`object`.ascent).isFinite() || !(`object`.descent).isFinite())) {
                throw ParagraphRequestException.InvalidInlineObjectVerticalGeometry
            }
            objectIndexIdx++
        }
        var decorationIndexIdx = 0
        while ((decorationIndexIdx < request.decorations.size)) {
            val decoration = request.decorations[decorationIndexIdx]
            if ((!ParagraphRequestChecks.validRange(decoration.start, decoration.end, textLength))) {
                throw ParagraphRequestException.InvalidDecorationRange
            }
            decorationIndexIdx++
        }
    }

    fun validRange(start: Int, end: Int, textLength: Int): Boolean {
        return start >= 0 && start < end && end <= textLength
    }

    fun isKotlinWhitespace(code: Int): Boolean {
        if ((code >= 9 && code <= 13)) {
            return true
        }
        if ((code >= 28 && code <= 31)) {
            return true
        }
        if ((code == 32 || code == 133 || code == 5760)) {
            return true
        }
        if ((code >= 8192 && code <= 8202)) {
            return true
        }
        return code == 8232 || code == 8233 || code == 8287 || code == 12288
    }

    fun isBlankText(text: String): Boolean {
        var indexIdx = 0
        while ((indexIdx < text.length)) {
            val code = run { val _s = text; val _i = indexIdx; if (_i >= 0 && _i < _s.length) _s[_i].code else null }
            if ((code!! >= 55296 && code <= 57343)) {
                return false
            }
            if ((!ParagraphRequestChecks.isKotlinWhitespace(code))) {
                return false
            }
            indexIdx++
        }
        return true
    }

    fun hasNonBlankFamily(families: MutableList<String>): Boolean {
        var indexIdx = 0
        while ((indexIdx < families.size)) {
            if ((!ParagraphRequestChecks.isBlankText(families[indexIdx]))) {
                return true
            }
            indexIdx++
        }
        return false
    }
}
