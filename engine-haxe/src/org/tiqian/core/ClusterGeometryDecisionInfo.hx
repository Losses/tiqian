package org.tiqian.core;

class ClusterGeometryDecisionInfo {
    public final range:TextRange;
    public final sourceText:String;
    public final displayText:String;
    public final baseAdvance:Float;
    public final bodyWidth:Float;
    public final leadingGlueNatural:Float;
    public final leadingGlueConsumed:Float;
    public final trailingGlueNatural:Float;
    public final trailingGlueConsumed:Float;
    public final justificationDelta:Float;
    public final rubySpread:Float;
    public final glyphInlineShift:Float;
    public final glyphPlacementReason:Null<String>;
    public final resolvedAdvance:Float;
    public final source:String;
    public final reason:String;

    public function new(
        range:TextRange,
        sourceText:String,
        displayText:String,
        baseAdvance:Float,
        bodyWidth:Float,
        leadingGlueNatural:Float,
        leadingGlueConsumed:Float,
        trailingGlueNatural:Float,
        trailingGlueConsumed:Float,
        justificationDelta:Float,
        resolvedAdvance:Float,
        source:String,
        reason:String,
        rubySpread:Float = 0.0,
        glyphInlineShift:Float = 0.0,
        glyphPlacementReason:Null<String>
    ) {
        this.range = range;
        this.sourceText = sourceText;
        this.displayText = displayText;
        this.baseAdvance = baseAdvance;
        this.bodyWidth = bodyWidth;
        this.leadingGlueNatural = leadingGlueNatural;
        this.leadingGlueConsumed = leadingGlueConsumed;
        this.trailingGlueNatural = trailingGlueNatural;
        this.trailingGlueConsumed = trailingGlueConsumed;
        this.justificationDelta = justificationDelta;
        this.rubySpread = rubySpread;
        this.glyphInlineShift = glyphInlineShift;
        this.glyphPlacementReason = glyphPlacementReason;
        this.resolvedAdvance = resolvedAdvance;
        this.source = source;
        this.reason = reason;
    }
}
