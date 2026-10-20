package org.tiqian.shared.html;

/**
 * The compound selector parser's tests, ported from
 * platforms/web/server/precompute/engine/src/html_parse.rs L739-759 (the
 * unsupported-form list) plus the accepted forms that fixture covers. The
 * published issue name is the contract the prepare pipeline rethrows as a
 * NamedError, so the name itself is asserted.
 */
class CompoundSelectorTest {
    @:test
    public static function parsesTheAcceptedCompoundForms():Void {
        final recorder = new org.tiqian.test.trace.TestTraceRecorder("CompoundSelectorTest");
        recorder.section("parsesTheAcceptedCompoundForms");
        final compounds = CompoundSelector.parseCompoundSelectorList(
            "section, #px, p[q], p[data-x=\"1\"], *,  p ");
        org.tiqian.test.trace.TracedAssertions.assertEquals(6, compounds.length,
            "every accepted compound parses, including the bare p");
        final parser = HtmlDomTestSupport.parseDocument(
            "<main><section id=\"px\" data-x=\"1\"><p>t</p></section></main>");
        final mains = parser.descendantsByTag(parser.documentNode(), ["main"]);
        final paragraph = parser.descendantsByTag(mains[0], ["p"])[0];
        org.tiqian.test.trace.TracedAssertions.assertTrue(parser.closest(paragraph, compounds),
            "the section ancestor matches the accepted compound list");
        final any = CompoundSelector.parseCompoundSelectorList("*");
        org.tiqian.test.trace.TracedAssertions.assertTrue(parser.closest(paragraph, any),
            "the star compound matches every element");
        final attribute = CompoundSelector.parseCompoundSelectorList("section[data-x=\"1\"]");
        org.tiqian.test.trace.TracedAssertions.assertTrue(parser.closest(paragraph, attribute),
            "the quoted attribute value matches");
        final wrongValue = CompoundSelector.parseCompoundSelectorList("section[data-x=\"2\"]");
        org.tiqian.test.trace.TracedAssertions.assertFalse(parser.closest(paragraph, wrongValue),
            "a different attribute value does not match");
    }

    @:test
    public static function unsupportedSelectorFormsAreRejectedWithThePublishedName():Void {
        final recorder = new org.tiqian.test.trace.TestTraceRecorder("CompoundSelectorTest");
        recorder.section("unsupportedSelectorFormsAreRejectedWithThePublishedName");
        final forms = [
            "",
            " ",
            "a b",
            "a>b",
            "li:hover",
            "li:nth-child(2)",
            "[data-x^=\"y\"]",
            "[data-x~=y]",
            "a..b",
            "*p",
            "[unclosed",
        ];
        for (form in forms) {
            org.tiqian.test.trace.TracedAssertions.assertEqualsString(
                "UnsupportedHtmlAncestorSelector", HtmlDomTestSupport.selectorIssueOf(form),
                "the form " + form + " is rejected instead of guessed");
        }
        org.tiqian.test.trace.TracedAssertions.assertEqualsString("UnsupportedHtmlAncestorSelector",
            HtmlParseException.describe(HtmlError.UnsupportedHtmlAncestorSelector),
            "the exception message is the published issue name");
    }
}
