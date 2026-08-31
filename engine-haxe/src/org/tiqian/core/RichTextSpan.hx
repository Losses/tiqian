package org.tiqian.core;

class RichTextSpan {
    public final range:TextRange;
    public final role:RichTextRole;
    public final paint:RichTextPaint;

    public function new(range:TextRange, role:RichTextRole, paint:RichTextPaint) {
        this.range = range;
        this.role = role;
        this.paint = paint;
    }

    public function toString():String {
        return "RichTextSpan(range=" + range + ", role=" + roleToString(role) + ", paint=" + paint + ")";
    }

    private static function roleToString(role:RichTextRole):String {
        return switch (role) {
            case Background: "Background";
            case Underline: "Underline";
            case LineThrough: "LineThrough";
            case Link(target): "Link(target=" + target + ")";
            case TechnicalInline: "TechnicalInline";
            case InlineCode: "InlineCode";
        };
    }

    @:allow(org.tiqian.core.LayoutQueries)
    private static function sameRole(a:RichTextRole, b:RichTextRole):Bool {
        return switch (a) {
            case Background: switch (b) { case Background: true; case _: false; };
            case Underline: switch (b) { case Underline: true; case _: false; };
            case LineThrough: switch (b) { case LineThrough: true; case _: false; };
            case Link(target): switch (b) { case Link(otherTarget): target == otherTarget; case _: false; };
            case TechnicalInline: switch (b) { case TechnicalInline: true; case _: false; };
            case InlineCode: switch (b) { case InlineCode: true; case _: false; };
        };
    }
}
