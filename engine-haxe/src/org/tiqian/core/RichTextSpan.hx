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
}
