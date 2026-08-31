package org.tiqian.core;

import std.ReadOnlyArray;

class Cluster {
    public final range:TextRange;
    public final text:String;
    public final displayText:String;
    public final fontKey:String;
    public final advance:Float;
    public final baselineShift:Float;
    public final leadingLayoutAdvance:Float;
    public final glyphInlineShift:Float;

    public function new(
        range:TextRange,
        text:String,
        fontKey:String,
        advance:Float,
        displayText:String,
        baselineShift:Float = 0.0,
        leadingLayoutAdvance:Float = 0.0,
        glyphInlineShift:Float = 0.0
    ) {
        this.range = range;
        this.text = text;
        this.displayText = displayText;
        this.fontKey = fontKey;
        this.advance = advance;
        this.baselineShift = baselineShift;
        this.leadingLayoutAdvance = leadingLayoutAdvance;
        this.glyphInlineShift = glyphInlineShift;
    }
}
