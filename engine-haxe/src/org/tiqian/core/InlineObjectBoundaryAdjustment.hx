package org.tiqian.core;

class InlineObjectBoundaryAdjustment {
    public final participatesInUniformStretch:Bool;
    public final preferredStretch:Null<InlineObjectPreferredStretch>;
    public final shrinkCapacity:Float;
    public final lineEndDiscardableAdvance:Float;
    public final preventsLineBreak:Bool;

    public function new(
        participatesInUniformStretch:Bool = false,
        preferredStretch:Null<InlineObjectPreferredStretch>,
        shrinkCapacity:Null<Float>,
        lineEndDiscardableAdvance:Null<Float>,
        preventsLineBreak:Bool = false
    ) {
        final resolvedShrink:Float = shrinkCapacity == null ? 0.0 : shrinkCapacity;
        final resolvedDiscard:Float = lineEndDiscardableAdvance == null ? 0.0 : lineEndDiscardableAdvance;
        if (!isFinite(resolvedShrink) || resolvedShrink < 0.0) {
            throw new TiqianIllegalArgumentException(Message("Inline-object boundary shrink capacity must be finite and non-negative"));
        }
        if (!isFinite(resolvedDiscard) || resolvedDiscard < 0.0) {
            throw new TiqianIllegalArgumentException(Message("Inline-object line-end discardable advance must be finite and non-negative"));
        }
        this.participatesInUniformStretch = participatesInUniformStretch;
        this.preferredStretch = preferredStretch;
        this.shrinkCapacity = resolvedShrink;
        this.lineEndDiscardableAdvance = resolvedDiscard;
        this.preventsLineBreak = preventsLineBreak;
    }

    public static function fixed():InlineObjectBoundaryAdjustment {
        return new InlineObjectBoundaryAdjustment(false, null, null, null, false);
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
