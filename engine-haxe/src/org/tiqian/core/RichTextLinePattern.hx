package org.tiqian.core;

@:dataClass
class RichTextLinePattern {
    public final strokeWidth:Float;
    public final dashLength:Float;
    public final gapLength:Float;
    public final dotDiameter:Float;
    private final kind:String;

    private function new(kind:String, strokeWidth:Float, dashLength:Float, gapLength:Float, dotDiameter:Float) {
        this.kind = kind;
        this.strokeWidth = strokeWidth;
        this.dashLength = dashLength;
        this.gapLength = gapLength;
        this.dotDiameter = dotDiameter;
    }

    public static final Solid:RichTextLinePattern = new RichTextLinePattern("Solid", 0.0, 0.0, 0.0, 0.0);

    public static function Dashed(strokeWidth:Float, dashLength:Float, gapLength:Float):RichTextLinePattern {
        if (!isFinite(strokeWidth) || strokeWidth <= 0.0
            || !isFinite(dashLength) || dashLength <= 0.0
            || !isFinite(gapLength) || gapLength <= 0.0) {
            throw new TiqianIllegalArgumentException(Message("Failed requirement."));
        }
        return new RichTextLinePattern("Dashed", strokeWidth, dashLength, gapLength, 0.0);
    }

    public static function Dotted(dotDiameter:Float, gapLength:Float):RichTextLinePattern {
        if (!isFinite(dotDiameter) || dotDiameter <= 0.0
            || !isFinite(gapLength) || gapLength <= 0.0) {
            throw new TiqianIllegalArgumentException(Message("Failed requirement."));
        }
        return new RichTextLinePattern("Dotted", 0.0, 0.0, gapLength, dotDiameter);
    }


    @:allow(org.tiqian.core.RichTextPaint)
    private static function sameValues(a:RichTextLinePattern, b:RichTextLinePattern):Bool {
        if (a.kind != b.kind) return false;
        if (a.kind == "Dashed") {
            return a.strokeWidth == b.strokeWidth && a.dashLength == b.dashLength && a.gapLength == b.gapLength;
        }
        if (a.kind == "Dotted") return a.dotDiameter == b.dotDiameter && a.gapLength == b.gapLength;
        return true;
    }

    private static function isFinite(value:Float):Bool {
        return value == value && value != Math.POSITIVE_INFINITY && value != Math.NEGATIVE_INFINITY;
    }
}
