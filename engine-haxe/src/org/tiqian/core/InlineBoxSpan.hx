package org.tiqian.core;

class InlineBoxSpan {
    public final range:TextRange;
    public final inlineStart:Float;
    public final inlineEnd:Float;
    public final outerSpacing:InlineBoxOuterSpacing;

    public function new(
        range:TextRange,
        inlineStart:Float = 0.0,
        inlineEnd:Float = 0.0,
        outerSpacing:InlineBoxOuterSpacing = InlineBoxOuterSpacing.Narrow
    ) {
        this.range = range;
        this.inlineStart = inlineStart;
        this.inlineEnd = inlineEnd;
        this.outerSpacing = outerSpacing;
    }

    public function toString():String {
        return "InlineBoxSpan(range=" + range
            + ", inlineStart=" + inlineStart
            + ", inlineEnd=" + inlineEnd
            + ", outerSpacing=" + Std.string(outerSpacing) + ")";
    }
}
