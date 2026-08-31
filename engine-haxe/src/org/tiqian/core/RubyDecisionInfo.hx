package org.tiqian.core;

import std.ReadOnlyArray;

class RubyDecisionInfo {
    public final baseRange:TextRange;
    public final text:String;
    public final lineIndex:Int;
    public final centerX:Float;
    public final baselineY:Float;
    public final fontSize:Float;
    public final overhang:Float;
    public final ascent:Float;
    public final descent:Float;
    public final width:Float;
    public final fontFamilies:ReadOnlyArray<String>;
    public final fontWeight:Int;
    public final locale:String;
    public final glyphs:ReadOnlyArray<Glyph>;

    public function new(baseRange:TextRange, text:String, lineIndex:Int, centerX:Float, baselineY:Float, fontSize:Float, overhang:Float, ascent:Float = 0.0, descent:Float = 0.0, width:Float = 0.0, ?fontFamilies:Array<String>, fontWeight:Int = 400, locale:String = "zh-Hans", ?glyphs:Array<Glyph>) {
        this.baseRange = baseRange;
        this.text = text;
        this.lineIndex = lineIndex;
        this.centerX = centerX;
        this.baselineY = baselineY;
        this.fontSize = fontSize;
        this.overhang = overhang;
        this.ascent = ascent;
        this.descent = descent;
        this.width = width;
        this.fontFamilies = fontFamilies == null ? [] : fontFamilies;
        this.fontWeight = fontWeight;
        this.locale = locale;
        this.glyphs = glyphs == null ? [] : glyphs;
    }
}
