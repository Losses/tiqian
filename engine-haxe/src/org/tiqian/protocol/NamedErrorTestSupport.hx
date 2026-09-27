package org.tiqian.protocol;

/**
 * The NamedError family's golden name list, kept out of the test class so
 * the test carries test members only (the interceptor rules shared logic to
 * an ordinary class). The goldens are the names the three lanes publish
 * today: the Rust port's check order (platforms/web/server/precompute/
 * engine/src/paragraph.rs:74-131) with the Kotlin lane's own names at their
 * check positions (ffi/js/src/jsMain/kotlin/org/tiqian/ffi/js/
 * ParagraphWireCodec.kt:279-297 and :48-144). The npm lane asserts the same
 * names (platforms/web/server/core/test/fonts.test.ts:105-108) and the Rust
 * unit tests pin them (paragraph.rs:261-280): this list is the three-lane
 * agreement point.
 */
class NamedErrorTestSupport {
    /** The published names in check order (authorities in the class header). */
    public static function golden():Array<String> {
        return [
            "EmptyParagraph",
            "InvalidMaximumMeasure",
            "InvalidFontSize",
            "InvalidLineHeight",
            "InvalidFirstLineIndent",
            "InvalidFontWeight",
            "InvalidEmphasisDotGapEm",
            "MissingExplicitFontFamilies",
            "InvalidTextSpanRange",
            "MissingTextSpanFontFamilies",
            "InvalidTextSpanFontSize",
            "InvalidTextSpanFontWeight",
            "InvalidTextSpanBaselineShift",
            "InvalidSourceBoundary",
            "InvalidLineBreakSpanRange",
            "InvalidInlineBoxRange",
            "InvalidInlineBoxGeometry",
            "InvalidInlineObjectRange",
            "InvalidInlineObjectAdvance",
            "InvalidInlineObjectVerticalGeometry",
            "InvalidDecorationRange",
        ];
    }
}
