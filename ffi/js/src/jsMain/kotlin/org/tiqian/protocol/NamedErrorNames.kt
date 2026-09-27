package org.tiqian.protocol

object NamedErrorNames {
    fun variants(): MutableList<ParagraphRequestException> {
        return mutableListOf<ParagraphRequestException>(ParagraphRequestException.EmptyParagraph, ParagraphRequestException.InvalidMaximumMeasure, ParagraphRequestException.InvalidFontSize, ParagraphRequestException.InvalidLineHeight, ParagraphRequestException.InvalidFirstLineIndent, ParagraphRequestException.InvalidFontWeight, ParagraphRequestException.InvalidEmphasisDotGapEm, ParagraphRequestException.MissingExplicitFontFamilies, ParagraphRequestException.InvalidTextSpanRange, ParagraphRequestException.MissingTextSpanFontFamilies, ParagraphRequestException.InvalidTextSpanFontSize, ParagraphRequestException.InvalidTextSpanFontWeight, ParagraphRequestException.InvalidTextSpanBaselineShift, ParagraphRequestException.InvalidSourceBoundary, ParagraphRequestException.InvalidLineBreakSpanRange, ParagraphRequestException.InvalidInlineBoxRange, ParagraphRequestException.InvalidInlineBoxGeometry, ParagraphRequestException.InvalidInlineObjectRange, ParagraphRequestException.InvalidInlineObjectAdvance, ParagraphRequestException.InvalidInlineObjectVerticalGeometry, ParagraphRequestException.InvalidDecorationRange)
    }

    fun describe(error: ParagraphRequestException): String {
        return when (error) {
            ParagraphRequestException.EmptyParagraph -> "EmptyParagraph"
            ParagraphRequestException.InvalidMaximumMeasure -> "InvalidMaximumMeasure"
            ParagraphRequestException.InvalidFontSize -> "InvalidFontSize"
            ParagraphRequestException.InvalidLineHeight -> "InvalidLineHeight"
            ParagraphRequestException.InvalidFirstLineIndent -> "InvalidFirstLineIndent"
            ParagraphRequestException.InvalidFontWeight -> "InvalidFontWeight"
            ParagraphRequestException.InvalidEmphasisDotGapEm -> "InvalidEmphasisDotGapEm"
            ParagraphRequestException.MissingExplicitFontFamilies -> "MissingExplicitFontFamilies"
            ParagraphRequestException.InvalidTextSpanRange -> "InvalidTextSpanRange"
            ParagraphRequestException.MissingTextSpanFontFamilies -> "MissingTextSpanFontFamilies"
            ParagraphRequestException.InvalidTextSpanFontSize -> "InvalidTextSpanFontSize"
            ParagraphRequestException.InvalidTextSpanFontWeight -> "InvalidTextSpanFontWeight"
            ParagraphRequestException.InvalidTextSpanBaselineShift -> "InvalidTextSpanBaselineShift"
            ParagraphRequestException.InvalidSourceBoundary -> "InvalidSourceBoundary"
            ParagraphRequestException.InvalidLineBreakSpanRange -> "InvalidLineBreakSpanRange"
            ParagraphRequestException.InvalidInlineBoxRange -> "InvalidInlineBoxRange"
            ParagraphRequestException.InvalidInlineBoxGeometry -> "InvalidInlineBoxGeometry"
            ParagraphRequestException.InvalidInlineObjectRange -> "InvalidInlineObjectRange"
            ParagraphRequestException.InvalidInlineObjectAdvance -> "InvalidInlineObjectAdvance"
            ParagraphRequestException.InvalidInlineObjectVerticalGeometry -> "InvalidInlineObjectVerticalGeometry"
            ParagraphRequestException.InvalidDecorationRange -> "InvalidDecorationRange"
        }
    }
}
