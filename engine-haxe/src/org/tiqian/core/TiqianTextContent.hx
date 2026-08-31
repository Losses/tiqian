package org.tiqian.core;

import std.ReadOnlyArray;

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

    public function toString():String {
        return "TiqianTextContent(text=" + text
            + ", spans=" + renderArray(spans)
            + ", sourceBoundaries=" + renderArray(sourceBoundaries)
            + ", lineBreakSpans=" + renderArray(lineBreakSpans)
            + ", autoSpaceSuppressedRanges=" + renderArray(autoSpaceSuppressedRanges) + ")";
    }

    private static function renderArray<T>(values:ReadOnlyArray<T>):String {
        var output:String = "[";
        var index:Int = 0;
        while (index < values.length) {
            if (index > 0) {
                output += ", ";
            }
            output += Std.string(values[index]);
            index += 1;
        }
        return output + "]";
    }
}
