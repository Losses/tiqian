package org.tiqian.protocol;

/** The exception class carries the enum instance (features/06:44-51); the
 * message function lives in the class, as the VectorException sample
 * (.haxelib/boring/git/samples/boring/VectorException.hx:18) rules. The
 * message is the published issue name, so no consumer reads it for
 * identity (features/06:397). */
class ParagraphRequestException extends haxe.Exception {
    public final error:ParagraphRequestError;

    public function new(error:ParagraphRequestError) {
        this.error = error;
        super(ParagraphRequestException.describe(error));
    }

    public static function describe(error:ParagraphRequestError):String {
        return switch (error) {
            case EmptyParagraph: "EmptyParagraph";
            case InvalidMaximumMeasure: "InvalidMaximumMeasure";
            case InvalidFontSize: "InvalidFontSize";
            case InvalidLineHeight: "InvalidLineHeight";
            case InvalidFirstLineIndent: "InvalidFirstLineIndent";
            case InvalidFontWeight: "InvalidFontWeight";
            case InvalidEmphasisDotGapEm: "InvalidEmphasisDotGapEm";
            case MissingExplicitFontFamilies: "MissingExplicitFontFamilies";
            case InvalidTextSpanRange: "InvalidTextSpanRange";
            case MissingTextSpanFontFamilies: "MissingTextSpanFontFamilies";
            case InvalidTextSpanFontSize: "InvalidTextSpanFontSize";
            case InvalidTextSpanFontWeight: "InvalidTextSpanFontWeight";
            case InvalidTextSpanBaselineShift: "InvalidTextSpanBaselineShift";
            case InvalidSourceBoundary: "InvalidSourceBoundary";
            case InvalidLineBreakSpanRange: "InvalidLineBreakSpanRange";
            case InvalidInlineBoxRange: "InvalidInlineBoxRange";
            case InvalidInlineBoxGeometry: "InvalidInlineBoxGeometry";
            case InvalidInlineObjectRange: "InvalidInlineObjectRange";
            case InvalidInlineObjectAdvance: "InvalidInlineObjectAdvance";
            case InvalidInlineObjectVerticalGeometry: "InvalidInlineObjectVerticalGeometry";
            case InvalidDecorationRange: "InvalidDecorationRange";
        };
    }
}
