package org.tiqian.core;

class InlineObjectBoundaryAdjustment {
    public final participatesInUniformStretch:Bool;
    public final preferredStretch:Null<InlineObjectPreferredStretch>;
    public final shrinkCapacity:Float;
    public final lineEndDiscardableAdvance:Float;
    public final preventsLineBreak:Bool;

    public function new(
        participatesInUniformStretch:Bool = false,
        preferredStretch:Null<InlineObjectPreferredStretch> = null,
        shrinkCapacity:Float = 0.0,
        lineEndDiscardableAdvance:Float = 0.0,
        preventsLineBreak:Bool = false
    ) {
        if (!isFinite(shrinkCapacity) || shrinkCapacity < 0.0) {
            throw new TiqianIllegalArgumentException(Message("Inline-object boundary shrink capacity must be finite and non-negative"));
        }
        if (!isFinite(lineEndDiscardableAdvance) || lineEndDiscardableAdvance < 0.0) {
            throw new TiqianIllegalArgumentException(Message("Inline-object line-end discardable advance must be finite and non-negative"));
        }
        this.participatesInUniformStretch = participatesInUniformStretch;
        this.preferredStretch = preferredStretch;
        this.shrinkCapacity = shrinkCapacity;
        this.lineEndDiscardableAdvance = lineEndDiscardableAdvance;
        this.preventsLineBreak = preventsLineBreak;
    }

    public static function fixed():InlineObjectBoundaryAdjustment {
        return new InlineObjectBoundaryAdjustment();
    }

    public function toString():String {
        return "InlineObjectBoundaryAdjustment(participatesInUniformStretch=" + participatesInUniformStretch
            + ", preferredStretch=" + (preferredStretch == null ? "null" : preferredStretch.toString())
            + ", shrinkCapacity=" + shrinkCapacity
            + ", lineEndDiscardableAdvance=" + lineEndDiscardableAdvance
            + ", preventsLineBreak=" + preventsLineBreak + ")";
    }

    private static function isFinite(value:Float):Bool {
        return value == value && value != Math.POSITIVE_INFINITY && value != Math.NEGATIVE_INFINITY;
    }
}
