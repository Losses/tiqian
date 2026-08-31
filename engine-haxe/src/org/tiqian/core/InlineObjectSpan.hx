package org.tiqian.core;

class InlineObjectSpan {
    public static final INLINE_OBJECT_REPLACEMENT_CHAR:String = "\uFFFC";

    public final range:TextRange;
    public final advance:Float;
    public final ascent:Float;
    public final descent:Float;
    public final leadingBoundary:InlineObjectBoundaryAdjustment;
    public final trailingBoundary:InlineObjectBoundaryAdjustment;

    public function new(
        range:TextRange,
        advance:Float,
        ascent:Float,
        descent:Float,
        // Kotlin declares both boundaries = InlineObjectBoundaryAdjustment.Fixed,
        // a companion-field default. The port exposes Fixed as the fixed()
        // static call, which the boring gap 4 extension grammar accepts; the
        // parameters stay mandatory until that lowering lands.
        leadingBoundary:InlineObjectBoundaryAdjustment,
        trailingBoundary:InlineObjectBoundaryAdjustment
    ) {
        this.range = range;
        this.advance = advance;
        this.ascent = ascent;
        this.descent = descent;
        this.leadingBoundary = leadingBoundary;
        this.trailingBoundary = trailingBoundary;
    }

    public function toString():String {
        return "InlineObjectSpan(range=" + range.toString()
            + ", advance=" + advance
            + ", ascent=" + ascent
            + ", descent=" + descent
            + ", leadingBoundary=" + leadingBoundary.toString()
            + ", trailingBoundary=" + trailingBoundary.toString() + ")";
    }
}
