package org.tiqian.core;

class RichTextPaint {
    public final argb:Null<Int>;
    public final linePattern:RichTextLinePattern;
    public final background:RichTextBackgroundPaint;
    public final adjacentSameStyleClearance:Float;

    public function new(
        ?argb:Null<Int>,
        // Kotlin declares linePattern = RichTextLinePattern.Solid (a static
        // field of a class, boring gap 4) and background =
        // RichTextBackgroundPaint() (a constructor call, outside the
        // sanctioned grammar). Both parameters stay mandatory.
        linePattern:RichTextLinePattern,
        background:RichTextBackgroundPaint,
        ?adjacentSameStyleClearance:Null<Float>
    ) {
        this.argb = argb == null ? null : argb;
        this.linePattern = linePattern;
        this.background = background;
        this.adjacentSameStyleClearance = adjacentSameStyleClearance == null ? 0.0 : adjacentSameStyleClearance;
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
        final a = argb;
        final b = other.argb;
        if (a != b) return false;
        if (!RichTextLinePattern.sameValues(linePattern, other.linePattern)) return false;
        return RichTextBackgroundPaint.sameValues(background, other.background);
    }

    public function toString():String {
        final value = argb;
        return "RichTextPaint(argb=" + (value == null ? "null" : "" + value)
            + ", linePattern=" + linePattern.toString()
            + ", background=" + background.toString()
            + ", adjacentSameStyleClearance=" + adjacentSameStyleClearance + ")";
    }

    private static function isFinite(value:Float):Bool {
        return value == value && value != Math.POSITIVE_INFINITY && value != Math.NEGATIVE_INFINITY;
    }
}
