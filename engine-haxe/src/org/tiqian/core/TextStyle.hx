package org.tiqian.core;

import std.ReadOnlyArray;

// Slant axis: italic/oblique typeface when the family offers one (ADR 0030 B 档).
// Per-span text color (ARGB) over a SOURCE range — rich-text 颜色 (ADR 0030 A 档).
class TextStyle {
    public final fontFamilies:ReadOnlyArray<String>;
    public final fontSize:Float;
    public final locale:String;
    public final fontWeight:Int;
    public final italic:Bool;
    public final baselineShift:Float;
    public final inlineAttachment:InlineAttachment;

    public function new(
        fontSize:Float = 16.0,
        locale:String = "zh-Hans",
        fontWeight:Int = 400,
        italic:Bool = false,
        baselineShift:Float = 0.0,
        inlineAttachment:InlineAttachment = InlineAttachment.None,
        ?fontFamilies:Array<String>
    ) {
        this.fontFamilies = fontFamilies == null ? [] : fontFamilies;
        this.fontSize = fontSize;
        this.locale = locale;
        this.fontWeight = fontWeight;
        this.italic = italic;
        this.baselineShift = baselineShift;
        this.inlineAttachment = inlineAttachment;
    }

    public static function withFontFamilies(fontFamilies:Array<String>, fontSize:Float, locale:String, fontWeight:Int, italic:Bool, baselineShift:Float, inlineAttachment:InlineAttachment):TextStyle {
        return new TextStyle(fontSize, locale, fontWeight, italic, baselineShift, inlineAttachment, fontFamilies);
    }

    public function toString():String {
        return "TextStyle(fontFamilies=" + Std.string(fontFamilies)
            + ", fontSize=" + fontSize
            + ", locale=" + locale
            + ", fontWeight=" + fontWeight
            + ", italic=" + italic
            + ", baselineShift=" + baselineShift
            + ", inlineAttachment=" + Std.string(inlineAttachment) + ")";
    }
}
