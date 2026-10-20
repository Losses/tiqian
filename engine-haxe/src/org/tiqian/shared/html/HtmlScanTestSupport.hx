package org.tiqian.shared.html;

/**
 * Test fixtures for the shared HTML scan and injection lane: the byte-identity
 * cases from the native-lane suite
 * (platforms/web/server/core/test/precompute-html.test.ts: tests 1 and 2)
 * plus the issue-name probe that returns the thrown variant's published name.
 */
class HtmlScanTestSupport {
    public static function issueNameOf(error:HtmlError):String {
        return switch (error) {
            case HtmlOffsetConversion: "HtmlOffsetConversion";
            case InvalidHtmlAttributeInsertionOffset: "InvalidHtmlAttributeInsertionOffset";
            case UnsupportedHtmlAncestorSelector: "UnsupportedHtmlAncestorSelector";
        };
    }

    public static function issueOf(html:String, insertions:Array<HtmlAttributeInsertion>):String {
        var name = "";
        try {
            HtmlScan.injectHtmlAttributes(html, insertions);
        } catch (error:HtmlParseException) {
            name = issueNameOf(error.error);
        }
        return name;
    }
}
