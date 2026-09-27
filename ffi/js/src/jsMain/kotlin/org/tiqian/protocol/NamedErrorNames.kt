package org.tiqian.protocol

object NamedErrorNames {
    fun variants(): MutableList<NamedError> {
        return mutableListOf<NamedError>(NamedError.EmptyParagraph, NamedError.InvalidMaximumMeasure, NamedError.InvalidFontSize, NamedError.InvalidLineHeight, NamedError.InvalidFirstLineIndent, NamedError.InvalidFontWeight, NamedError.InvalidEmphasisDotGapEm, NamedError.MissingExplicitFontFamilies, NamedError.InvalidTextSpanRange, NamedError.MissingTextSpanFontFamilies, NamedError.InvalidTextSpanFontSize, NamedError.InvalidTextSpanFontWeight, NamedError.InvalidTextSpanBaselineShift, NamedError.InvalidSourceBoundary, NamedError.InvalidLineBreakSpanRange, NamedError.InvalidInlineBoxRange, NamedError.InvalidInlineBoxGeometry, NamedError.InvalidInlineObjectRange, NamedError.InvalidInlineObjectAdvance, NamedError.InvalidInlineObjectVerticalGeometry, NamedError.InvalidDecorationRange)
    }

    fun describe(error: NamedError): String {
        return when (error) {
            NamedError.EmptyParagraph -> "EmptyParagraph"
            NamedError.InvalidMaximumMeasure -> "InvalidMaximumMeasure"
            NamedError.InvalidFontSize -> "InvalidFontSize"
            NamedError.InvalidLineHeight -> "InvalidLineHeight"
            NamedError.InvalidFirstLineIndent -> "InvalidFirstLineIndent"
            NamedError.InvalidFontWeight -> "InvalidFontWeight"
            NamedError.InvalidEmphasisDotGapEm -> "InvalidEmphasisDotGapEm"
            NamedError.MissingExplicitFontFamilies -> "MissingExplicitFontFamilies"
            NamedError.InvalidTextSpanRange -> "InvalidTextSpanRange"
            NamedError.MissingTextSpanFontFamilies -> "MissingTextSpanFontFamilies"
            NamedError.InvalidTextSpanFontSize -> "InvalidTextSpanFontSize"
            NamedError.InvalidTextSpanFontWeight -> "InvalidTextSpanFontWeight"
            NamedError.InvalidTextSpanBaselineShift -> "InvalidTextSpanBaselineShift"
            NamedError.InvalidSourceBoundary -> "InvalidSourceBoundary"
            NamedError.InvalidLineBreakSpanRange -> "InvalidLineBreakSpanRange"
            NamedError.InvalidInlineBoxRange -> "InvalidInlineBoxRange"
            NamedError.InvalidInlineBoxGeometry -> "InvalidInlineBoxGeometry"
            NamedError.InvalidInlineObjectRange -> "InvalidInlineObjectRange"
            NamedError.InvalidInlineObjectAdvance -> "InvalidInlineObjectAdvance"
            NamedError.InvalidInlineObjectVerticalGeometry -> "InvalidInlineObjectVerticalGeometry"
            NamedError.InvalidDecorationRange -> "InvalidDecorationRange"
        }
    }
}
