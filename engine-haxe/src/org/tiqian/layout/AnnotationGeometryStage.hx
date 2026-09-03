package org.tiqian.layout;

import org.tiqian.core.Glyph;

/*
 * Partial port: only RubyFontGeometry (Kotlin AnnotationGeometryStage.kt:790) is
 * translated so far because LineGeometryStage.kt:254/:273 consumes it. The rest
 * of AnnotationGeometryStage.kt lands in a later lane and must extend this file.
 */
@:dataClass class RubyFontGeometry {
    public final width:Float;
    public final ascent:Float;
    public final descent:Float;
    public final requiredExtent:Float;
    public final glyphs:Array<Glyph>;

    public function new(width:Float, ascent:Float, descent:Float, requiredExtent:Float, glyphs:Array<Glyph>) {
        this.width = width;
        this.ascent = ascent;
        this.descent = descent;
        this.requiredExtent = requiredExtent;
        this.glyphs = glyphs;
    }
}
