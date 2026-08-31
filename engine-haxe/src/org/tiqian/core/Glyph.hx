package org.tiqian.core;

class Glyph {
    public final id:Int;
    public final clusterRange:TextRange;
    public final advance:Float;
    public final x:Float;
    public final y:Float;
    public final renderFontKey:Null<String>;
    public final bounds:Null<Rect>;
    public final haltAdvance:Null<Float>;
    public final haltPlacementX:Null<Float>;

    public function new(
        id:Int,
        clusterRange:TextRange,
        advance:Float,
        x:Float = 0.0,
        y:Float = 0.0,
        renderFontKey:Null<String>,
        bounds:Null<Rect>,
        haltAdvance:Null<Float>,
        haltPlacementX:Null<Float>
    ) {
        this.id = id;
        this.clusterRange = clusterRange;
        this.advance = advance;
        this.x = x;
        this.y = y;
        this.renderFontKey = renderFontKey;
        this.bounds = bounds;
        this.haltAdvance = haltAdvance;
        this.haltPlacementX = haltPlacementX;
    }
}
