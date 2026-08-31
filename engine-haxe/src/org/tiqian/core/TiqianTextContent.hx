package org.tiqian.core;

import std.ReadOnlyArray;

@:dataClass
class TiqianTextContent {
    public final text:String;
    public final spans:ReadOnlyArray<TextSpan>;
    public final sourceBoundaries:ReadOnlyArray<Int>;
    public final lineBreakSpans:ReadOnlyArray<LineBreakSpan>;
    public final autoSpaceSuppressedRanges:ReadOnlyArray<TextRange>;

    public function new(
        text:String,
        ?spans:Array<TextSpan>,
        ?sourceBoundaries:Array<Int>,
        ?lineBreakSpans:Array<LineBreakSpan>,
        ?autoSpaceSuppressedRanges:Array<TextRange>
    ) {
        this.text = text;
        this.spans = spans == null ? [] : spans;
        this.sourceBoundaries = sourceBoundaries == null ? [] : sourceBoundaries;
        this.lineBreakSpans = lineBreakSpans == null ? [] : lineBreakSpans;
        this.autoSpaceSuppressedRanges = autoSpaceSuppressedRanges == null ? [] : autoSpaceSuppressedRanges;
    }



    private static function renderSpans(values:ReadOnlyArray<TextSpan>):String {
        var output:String = "[";
        var index:Int = 0;
        while (index < values.length) {
            if (index > 0) {
                output += ", ";
            }
            output += values[index].toString();
            index += 1;
        }
        return output + "]";
    }

    private static function renderSourceBoundaries(values:ReadOnlyArray<Int>):String {
        var output:String = "[";
        var index:Int = 0;
        while (index < values.length) {
            if (index > 0) output += ", ";
            output += "" + values[index];
            index += 1;
        }
        return output + "]";
    }

    private static function renderLineBreakSpans(values:ReadOnlyArray<LineBreakSpan>):String {
        var output:String = "[";
        var index:Int = 0;
        while (index < values.length) {
            if (index > 0) output += ", ";
            output += values[index].toString();
            index += 1;
        }
        return output + "]";
    }

    private static function renderRanges(values:ReadOnlyArray<TextRange>):String {
        var output:String = "[";
        var index:Int = 0;
        while (index < values.length) {
            if (index > 0) output += ", ";
            output += values[index].toString();
            index += 1;
        }
        return output + "]";
    }
}
