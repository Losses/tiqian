package org.tiqian.core;

@:dataClass
class RichTextSpan {
    public final range:TextRange;
    public final role:RichTextRole;
    public final paint:RichTextPaint;

    public function new(range:TextRange, role:RichTextRole, paint:RichTextPaint) {
        this.range = range;
        this.role = role;
        this.paint = paint;
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
            case Background: b == RichTextRole.Background;
            case Underline: b == RichTextRole.Underline;
            case LineThrough: b == RichTextRole.LineThrough;
            case Link(target): sameLinkTarget(target, b);
            case TechnicalInline: b == RichTextRole.TechnicalInline;
            case InlineCode: b == RichTextRole.InlineCode;
        };
    }

    private static function sameLinkTarget(target:String, b:RichTextRole):Bool {
        return switch (b) {
            case Background: false;
            case Underline: false;
            case LineThrough: false;
            case Link(otherTarget): target == otherTarget;
            case TechnicalInline: false;
            case InlineCode: false;
        };
    }
}
