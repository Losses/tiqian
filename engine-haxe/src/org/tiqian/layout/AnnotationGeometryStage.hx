package org.tiqian.layout;

import org.tiqian.core.Glyph;
import org.tiqian.core.InlineObjectDecisionInfo;
import org.tiqian.core.DecorationDecisionInfo;
import org.tiqian.core.DecorationSegmentInfo;
import org.tiqian.core.RubyDecisionInfo;
import org.tiqian.core.BopomofoDecisionInfo;

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

@:dataClass class AnnotationGeometryStageResult {
    public final inlineObjectDecisions:Array<InlineObjectDecisionInfo>;
    public final decorationDecisions:Array<DecorationDecisionInfo>;
    public final decorationSegments:Array<DecorationSegmentInfo>;
    public final rubyDecisions:Array<RubyDecisionInfo>;
    public final bopomofoDecisions:Array<BopomofoDecisionInfo>;

    public function new(inlineObjectDecisions:Array<InlineObjectDecisionInfo>, decorationDecisions:Array<DecorationDecisionInfo>,
            decorationSegments:Array<DecorationSegmentInfo>, rubyDecisions:Array<RubyDecisionInfo>, bopomofoDecisions:Array<BopomofoDecisionInfo>) {
        this.inlineObjectDecisions = inlineObjectDecisions;
        this.decorationDecisions = decorationDecisions;
        this.decorationSegments = decorationSegments;
        this.rubyDecisions = rubyDecisions;
        this.bopomofoDecisions = bopomofoDecisions;
    }
}

class AnnotationGeometryStage {
    public static inline final EMPHASIS_DOT_DIAMETER_EM:Float = 0.19;
    public static inline final BOPOMOFO_ANNOTATION_FONT_EM:Float = 0.3;
    public static inline final BOPOMOFO_SYMBOL_BASELINE_FACTOR:Float = 0.88;
    public static inline final MOURNING_FRAME_FACE_ASCENT_EM:Float = 0.88;
    public static inline final MOURNING_FRAME_FACE_DESCENT_EM:Float = 0.12;
    public static inline final INTERLINEAR_LINE_Y_EM:Float = 0.18;
    public static inline final BOOK_TITLE_WAVE_LINE_Y_EM:Float = 0.24;
    public static inline final ADJACENT_LINE_SHORTEN_EM:Float = 0.0625;
    public static inline final ADJACENT_LINE_EPSILON:Float = 0.01;
}
