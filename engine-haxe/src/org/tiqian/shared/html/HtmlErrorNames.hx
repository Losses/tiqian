package org.tiqian.shared.html;

/**
 * The single-source name accessors of the HtmlError family. The published
 * issue name of a variant is the variant's own name, so describe is the
 * identity of the variant names and the ordered list carries the declaration
 * order. Mirrors NamedErrorNames
 * (engine-haxe/src/org/tiqian/protocol/NamedErrorNames.hx).
 */
class HtmlErrorNames {
    public static function variants():Array<HtmlError> {
        return [
            HtmlOffsetConversion,
            InvalidHtmlAttributeInsertionOffset,
            UnsupportedHtmlAncestorSelector,
        ];
    }

    public static function describe(error:HtmlError):String {
        return switch (error) {
            case HtmlOffsetConversion: "HtmlOffsetConversion";
            case InvalidHtmlAttributeInsertionOffset: "InvalidHtmlAttributeInsertionOffset";
            case UnsupportedHtmlAncestorSelector: "UnsupportedHtmlAncestorSelector";
        };
    }
}
