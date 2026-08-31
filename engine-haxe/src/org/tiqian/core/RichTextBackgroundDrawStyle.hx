package org.tiqian.core;

class RichTextBackgroundDrawStyle {
    public final strokeWidth:Float;
    private final kind:String;

    private function new(kind:String, strokeWidth:Float) {
        this.kind = kind;
        this.strokeWidth = strokeWidth;
    }

    public static final Fill:RichTextBackgroundDrawStyle = new RichTextBackgroundDrawStyle("Fill", 0.0);

    public static function Border(strokeWidth:Float):RichTextBackgroundDrawStyle {
        if (!isFinite(strokeWidth) || strokeWidth <= 0.0) {
            throw new TiqianIllegalArgumentException(Message("Failed requirement."));
        }
        return new RichTextBackgroundDrawStyle("Border", strokeWidth);
    }

    public function toString():String {
        return kind == "Fill" ? "Fill" : "Border(strokeWidth=" + strokeWidth + ")";
    }

    private static function isFinite(value:Float):Bool {
        return value == value && value != Math.POSITIVE_INFINITY && value != Math.NEGATIVE_INFINITY;
    }
}
