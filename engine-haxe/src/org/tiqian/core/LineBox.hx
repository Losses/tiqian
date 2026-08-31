package org.tiqian.core;

import std.ReadOnlyArray;

class LineBox {
    public final range:TextRange;
    public final clusterRange:IntRange;
    public final baseline:Float;
    public final top:Float;
    public final bottom:Float;
    public final naturalWidth:Float;
    public final adjustedWidth:Float;
    public final visualWidth:Float;
    public final hangingPunctuationAdvance:Float;
    public final indent:Float;
    public final endReason:LineEndReason;
    public final hyphenAdvance:Float;
    public final hyphenGlyphs:ReadOnlyArray<Glyph>;
    public final debug:LineDebugInfo;

    public function new(
        range:TextRange,
        clusterRange:IntRange,
        baseline:Float,
        top:Float,
        bottom:Float,
        naturalWidth:Float,
        adjustedWidth:Float,
        visualWidth:Float,
        hangingPunctuationAdvance:Float = 0.0,
        indent:Float = 0.0,
        endReason:LineEndReason = LineEndReason.ParagraphEnd,
        hyphenAdvance:Float = 0.0,
        ?hyphenGlyphs:Array<Glyph>,
        debug:LineDebugInfo
    ) {
        this.range = range;
        this.clusterRange = clusterRange;
        this.baseline = baseline;
        this.top = top;
        this.bottom = bottom;
        this.naturalWidth = naturalWidth;
        this.adjustedWidth = adjustedWidth;
        this.visualWidth = visualWidth;
        this.hangingPunctuationAdvance = hangingPunctuationAdvance;
        this.indent = indent;
        this.endReason = endReason;
        this.hyphenAdvance = hyphenAdvance;
        this.hyphenGlyphs = hyphenGlyphs == null ? [] : hyphenGlyphs;
        this.debug = debug;
    }
}
