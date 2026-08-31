package org.tiqian.core;

class RichTextPaint {
    public final argb:Null<Int>;
    public final linePattern:RichTextLinePattern;
    public final background:RichTextBackgroundPaint;
    public final adjacentSameStyleClearance:Float;

    public function new(
        argb:Null<Int>,
        linePattern:RichTextLinePattern,
        background:RichTextBackgroundPaint,
        adjacentSameStyleClearance:Float = 0.0
    ) {
        this.argb = argb;
        this.linePattern = linePattern;
        this.background = background;
        this.adjacentSameStyleClearance = adjacentSameStyleClearance;
        if (!isFinite(this.adjacentSameStyleClearance) || this.adjacentSameStyleClearance < 0.0) {
            throw new TiqianIllegalArgumentException(Message("Failed requirement."));
        }
    }

    public static function withBackground(background:RichTextBackgroundPaint):RichTextPaint {
        return new RichTextPaint(null, RichTextLinePattern.Solid, background, 0.0);
    }

    public static function withClearance(clearance:Float):RichTextPaint {
        return new RichTextPaint(null, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), clearance);
    }

    public static function withArgb(value:Int):RichTextPaint {
        return new RichTextPaint(value, RichTextLinePattern.Solid, new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill), 0.0);
    }

    public function sameVisibleStyle(other:RichTextPaint):Bool {
        return argb == other.argb
            && Std.string(linePattern) == Std.string(other.linePattern)
            && Std.string(background) == Std.string(other.background);
    }

    public function toString():String {
        return "RichTextPaint(argb=" + (argb == null ? "null" : Std.string(argb))
            + ", linePattern=" + Std.string(linePattern)
            + ", background=" + Std.string(background)
            + ", adjacentSameStyleClearance=" + adjacentSameStyleClearance + ")";
    }

    private static function isFinite(value:Float):Bool {
        return value == value && value != Math.POSITIVE_INFINITY && value != Math.NEGATIVE_INFINITY;
    }
}
