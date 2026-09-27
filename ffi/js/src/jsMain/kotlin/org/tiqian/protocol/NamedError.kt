package org.tiqian.protocol

enum class NamedError {
    EmptyParagraph,
    InvalidMaximumMeasure,
    InvalidFontSize,
    InvalidLineHeight,
    InvalidFirstLineIndent,
    InvalidFontWeight,
    InvalidEmphasisDotGapEm,
    MissingExplicitFontFamilies,
    InvalidTextSpanRange,
    MissingTextSpanFontFamilies,
    InvalidTextSpanFontSize,
    InvalidTextSpanFontWeight,
    InvalidTextSpanBaselineShift,
    InvalidSourceBoundary,
    InvalidLineBreakSpanRange,
    InvalidInlineBoxRange,
    InvalidInlineBoxGeometry,
    InvalidInlineObjectRange,
    InvalidInlineObjectAdvance,
    InvalidInlineObjectVerticalGeometry,
    InvalidDecorationRange
}
fun compareNamedError(a: NamedError, b: NamedError): Int = a.ordinal - b.ordinal
