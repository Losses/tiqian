package org.tiqian.protocol;

/**
 * The closed domain failure set (features/06-errors-and-results.md:34, the
 * failure identity is a Haxe enum carried by the exception class). One
 * variant per published issue name, declared in check order; the names are
 * the ones both replaced lanes publish (paragraph.rs:72-131,
 * ParagraphWireCodec.kt:279-297).
 */
enum ParagraphRequestError {
    EmptyParagraph;
    InvalidMaximumMeasure;
    InvalidFontSize;
    InvalidLineHeight;
    InvalidFirstLineIndent;
    InvalidFontWeight;
    InvalidEmphasisDotGapEm;
    MissingExplicitFontFamilies;
    InvalidTextSpanRange;
    MissingTextSpanFontFamilies;
    InvalidTextSpanFontSize;
    InvalidTextSpanFontWeight;
    InvalidTextSpanBaselineShift;
    InvalidSourceBoundary;
    InvalidLineBreakSpanRange;
    InvalidInlineBoxRange;
    InvalidInlineBoxGeometry;
    InvalidInlineObjectRange;
    InvalidInlineObjectAdvance;
    InvalidInlineObjectVerticalGeometry;
    InvalidDecorationRange;
}
