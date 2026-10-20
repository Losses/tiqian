package org.tiqian.shared.html;

/**
 * The shared HTML lane's byte-identity tests, ported from the native-lane
 * suite (platforms/web/server/core/test/precompute-html.test.ts: tests 1 and
 * 2) plus the injection gate's issue-name rejection. The two golden outputs
 * are the byte-identical strings the addon lane already produces, so the
 * generated lane is pinned to the same bytes.
 */
class HtmlScanTest {
    @:test
    public static function theOpeningTagScanSkipsCommentsRawTextAndTemplates():Void {
        final recorder = new org.tiqian.test.trace.TestTraceRecorder("HtmlScanTest");
        recorder.section("theOpeningTagScanSkipsCommentsRawTextAndTemplates");
        final tags = HtmlScan.findHtmlOpeningTags(
            '<!-- <p>comment</p> --><p>正文</p><script>"<p>script</p>"</script>'
            + '<template><p>inert</p></template><li>条目</li>',
            ["p", "li"]);
        org.tiqian.test.trace.TracedAssertions.assertEquals(2, tags.length,
            "two selected opening tags remain after comment/raw-text/template skip");
        org.tiqian.test.trace.TracedAssertions.assertEqualsString("p", tags[0].tagName);
        org.tiqian.test.trace.TracedAssertions.assertEqualsString("<p>", tags[0].source);
        org.tiqian.test.trace.TracedAssertions.assertEqualsString("li", tags[1].tagName);
        org.tiqian.test.trace.TracedAssertions.assertEqualsString("<li>", tags[1].source);
    }

    @:test
    public static function attributeInjectionAppliesInsertionsAtTheGivenOffsets():Void {
        final recorder = new org.tiqian.test.trace.TestTraceRecorder("HtmlScanTest");
        recorder.section("attributeInjectionAppliesInsertionsAtTheGivenOffsets");
        final result = HtmlScan.injectHtmlAttributes("<p>甲</p><p>乙</p>", [
            { offset: 10.0, attribute: ' data-b="2"' },
            { offset: 2.0, attribute: ' data-a="1"' },
        ]);
        org.tiqian.test.trace.TracedAssertions.assertEqualsString(
            '<p data-a="1">甲</p><p data-b="2">乙</p>', result);
    }

    @:test
    public static function theInjectionGateRejectsAnInvalidOffset():Void {
        final recorder = new org.tiqian.test.trace.TestTraceRecorder("HtmlScanTest");
        recorder.section("theInjectionGateRejectsAnInvalidOffset");
        org.tiqian.test.trace.TracedAssertions.assertEqualsString(
            "InvalidHtmlAttributeInsertionOffset",
            HtmlScanTestSupport.issueOf("<p>甲</p>", [{ offset: 99.0, attribute: ' data-a="1"' }]));
    }

    @:test
    public static function thePublishedErrorNamesMatchTheVariantList():Void {
        final recorder = new org.tiqian.test.trace.TestTraceRecorder("HtmlScanTest");
        recorder.section("thePublishedErrorNamesMatchTheVariantList");
        final variants = HtmlErrorNames.variants();
        org.tiqian.test.trace.TracedAssertions.assertEquals(3, variants.length,
            "the family has the 3 published names");
        org.tiqian.test.trace.TracedAssertions.assertEqualsString("HtmlOffsetConversion",
            HtmlErrorNames.describe(variants[0]));
        org.tiqian.test.trace.TracedAssertions.assertEqualsString("InvalidHtmlAttributeInsertionOffset",
            HtmlErrorNames.describe(variants[1]));
        org.tiqian.test.trace.TracedAssertions.assertEqualsString("UnsupportedHtmlAncestorSelector",
            HtmlErrorNames.describe(variants[2]));
    }
}
