package org.tiqian.shared.html;

/**
 * The linkedom-backed DOM query lane's equivalence tests, ported from the
 * html5ever suite (platforms/web/server/precompute/engine/src/html_parse.rs
 * tests at L662-737). The fixtures and expected readings are the Rust
 * tests' own, so a green run pins the generated lane to the same document
 * tree, attribute order and closest outcomes.
 *
 * to_dom_node (html_parse.rs L689-693) has no counterpart here: the
 * projection that consumes it belongs to Html-B2, which walks the linkedom
 * DOM directly.
 */
class DomParserTest {
    @:test
    public static function parseFollowsTheHtmlTreeConstruction():Void {
        final recorder = new org.tiqian.test.trace.TestTraceRecorder("DomParserTest");
        recorder.section("parseFollowsTheHtmlTreeConstruction");
        final parser = HtmlDomTestSupport.parseDocument(
            "<main data-tq-html-prepare-root><p>a &amp; b<p>two<ul><li>x</UL><P>upper</main>");
        final mains = parser.descendantsByTag(parser.documentNode(), ["main"]);
        org.tiqian.test.trace.TracedAssertions.assertEquals(1, mains.length,
            "exactly one main element is reachable from the document root");
        final matched = parser.descendantsByTag(mains[0], ["p", "li"]);
        org.tiqian.test.trace.TracedAssertions.assertEqualsStringArray(["p", "p", "li", "p"],
            HtmlDomTestSupport.tagNames(parser, matched),
            "the fixture's tree order matches the html5ever reading");
        org.tiqian.test.trace.TracedAssertions.assertEqualsStringArray(["p", "p", "li", "p"],
            HtmlDomTestSupport.tagNames(parser, parser.descendantsByTag(mains[0], ["P", "LI"])),
            "tag names match ASCII case insensitively");
        org.tiqian.test.trace.TracedAssertions.assertTrue(
            parser.attributeValue(mains[0], "data-tq-html-prepare-root") != null,
            "the prepare root attribute is present");
        org.tiqian.test.trace.TracedAssertions.assertEqualsString("",
            HtmlDomTestSupport.attributeOrEmpty(parser.attributeValue(mains[0], "data-tq-html-prepare-root")),
            "the valueless prepare root attribute reads as the empty string");
    }

    @:test
    public static function templateContentsStayOutsideTheTree():Void {
        final recorder = new org.tiqian.test.trace.TestTraceRecorder("DomParserTest");
        recorder.section("templateContentsStayOutsideTheTree");
        final parser = HtmlDomTestSupport.parseDocument(
            "<main><template><p>inert</p></template><p>live</p></main>");
        final mains = parser.descendantsByTag(parser.documentNode(), ["main"]);
        final matched = parser.descendantsByTag(mains[0], ["p"]);
        org.tiqian.test.trace.TracedAssertions.assertEquals(1, matched.length,
            "the inert template paragraph stays outside the document tree");
        final templates = parser.descendantsByTag(mains[0], ["template"]);
        org.tiqian.test.trace.TracedAssertions.assertEquals(1, templates.length,
            "the template element itself stays in the tree");
        org.tiqian.test.trace.TracedAssertions.assertEquals(0, parser.elementChildren(templates[0]).length,
            "element children of template stay empty like html5ever's template_contents");
    }

    @:test
    public static function noscriptChildrenAreRealElements():Void {
        final recorder = new org.tiqian.test.trace.TestTraceRecorder("DomParserTest");
        recorder.section("noscriptChildrenAreRealElements");
        final parser = HtmlDomTestSupport.parseDocument("<main><noscript><p>a</p></noscript></main>");
        final mains = parser.descendantsByTag(parser.documentNode(), ["main"]);
        final matched = parser.descendantsByTag(mains[0], ["p"]);
        org.tiqian.test.trace.TracedAssertions.assertEquals(1, matched.length,
            "linkedom parses noscript children as elements, as the Rust lane does");
    }

    @:test
    public static function attributesKeepSourceOrderAndFirstDuplicateWins():Void {
        final recorder = new org.tiqian.test.trace.TestTraceRecorder("DomParserTest");
        recorder.section("attributesKeepSourceOrderAndFirstDuplicateWins");
        final parser = HtmlDomTestSupport.parseDocument(
            "<main><p title='a&quot;b' data-x=1 class=\"z w\" class=\"q\">t</p></main>");
        final mains = parser.descendantsByTag(parser.documentNode(), ["main"]);
        final paragraph = parser.descendantsByTag(mains[0], ["p"])[0];
        org.tiqian.test.trace.TracedAssertions.assertEqualsStringArray(
            ["title=a\"b", "data-x=1", "class=z w"],
            HtmlDomTestSupport.attributePairs(paragraph),
            "attribute order and the first duplicate class value match the Rust lane");
        org.tiqian.test.trace.TracedAssertions.assertEqualsString("a\"b",
            HtmlDomTestSupport.attributeOrEmpty(parser.attributeValue(paragraph, "TITLE")),
            "attribute lookup folds ASCII case");
        org.tiqian.test.trace.TracedAssertions.assertTrue(parser.attributeValue(paragraph, "id") == null,
            "a missing attribute reports null");
    }

    @:test
    public static function closestMatchesSelfAncestorsAndTheDefaultKinds():Void {
        final recorder = new org.tiqian.test.trace.TestTraceRecorder("DomParserTest");
        recorder.section("closestMatchesSelfAncestorsAndTheDefaultKinds");
        final parser = HtmlDomTestSupport.parseDocument(
            "<main><div class=\"not-prose katex\"><p data-tqian-skip=\"1\" id=\"px\">t</p></div></main>");
        final mains = parser.descendantsByTag(parser.documentNode(), ["main"]);
        final paragraph = parser.descendantsByTag(mains[0], ["p"])[0];
        final skip = CompoundSelector.parseCompoundSelectorList(
            "[data-tiqian-skip], pre, table, .not-prose, .katex, .katex-display, .expressive-code");
        org.tiqian.test.trace.TracedAssertions.assertTrue(parser.closest(paragraph, skip),
            "the default skipped-ancestor selector matches the paragraph");
        final host = CompoundSelector.parseCompoundSelectorList("section, #px, p[q], p[data-x=\"1\"]");
        org.tiqian.test.trace.TracedAssertions.assertTrue(parser.closest(paragraph, host),
            "the host selector matches through the paragraph's id");
        final none = CompoundSelector.parseCompoundSelectorList("pre, table");
        org.tiqian.test.trace.TracedAssertions.assertFalse(parser.closest(paragraph, none),
            "unrelated compounds do not match");
        org.tiqian.test.trace.TracedAssertions.assertEqualsString("div",
            parser.tagName(parser.parent(paragraph)),
            "the ancestor walk reads parentElement");
    }
}
