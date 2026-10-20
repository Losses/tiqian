package org.tiqian.shared.html;

/**
 * Test fixtures for the linkedom-backed DOM query lane. The query classes
 * take the parsed document from their consumer, so the production lane has
 * no module binding; these fixtures build the document with linkedom, the
 * backend the prepare pipeline mounts, and every assertion runs against it.
 */
typedef DomWindow = {
    final document:DomNode;
}

@:jsRequire("linkedom")
extern class LinkedDom {
    public static function parseHTML(markup:String):DomWindow;
}

class HtmlDomTestSupport {
    public static function parseDocument(markup:String):DomParser {
        final window = LinkedDom.parseHTML(markup);
        return new DomParser(window.document);
    }

    public static function tagNames(parser:DomParser, nodes:Array<DomNode>):Array<String> {
        final names:Array<String> = [];
        for (node in nodes) {
            names.push(parser.tagName(node));
        }
        return names;
    }

    public static function attributePairs(node:DomNode):Array<String> {
        final pairs:Array<String> = [];
        for (attribute in node.attributes) {
            pairs.push(attribute.name + "=" + attributeOrEmpty(attribute.value));
        }
        return pairs;
    }

    public static function attributeOrEmpty(value:Null<String>):String {
        return value == null ? "" : value;
    }

    public static function issueName(error:HtmlError):String {
        return switch (error) {
            case HtmlOffsetConversion: "HtmlOffsetConversion";
            case InvalidHtmlAttributeInsertionOffset: "InvalidHtmlAttributeInsertionOffset";
            case UnsupportedHtmlAncestorSelector: "UnsupportedHtmlAncestorSelector";
        }
    }

    public static function selectorIssueOf(selector:String):String {
        try {
            CompoundSelector.parseCompoundSelectorList(selector);
        } catch (error:HtmlParseException) {
            return issueName(error.error);
        }
        return "";
    }
}
