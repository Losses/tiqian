package org.tiqian.core;

import std.SortedSet;

/**
 * Projects render-only range edges into source cluster boundaries required by
 * geometry queries. Frontends must use this shared projection instead of
 * deriving their own boundary set: color, link and decoration ranges do not
 * change line layout, but their exact source edges must survive shaping so hit
 * testing and paint geometry do not slice a larger cluster proportionally.
 */
class RichTextLayoutProjection {
    @:topLevel
    public static function sourceBoundariesForPresentation(textLength:Int, ?decorations:Array<DecorationSpan>, ?colorSpans:Array<ColorSpan>,
            ?richTextSpans:Array<RichTextSpan>, ?textSpans:Array<TextSpan>, ?rubySpans:Array<RubySpan>):SortedSet<Int> {
        final builder = SortedSet.builder();
        function addBoundary(offset:Int):Void {
            if (offset > 0 && offset < textLength)
                builder.put(offset);
        }
        function addRange(start:Int, end:Int):Void {
            addBoundary(start);
            addBoundary(end);
        }
        if (decorations != null)
            for (span in decorations)
                addRange(span.range.start, span.range.end);
        if (colorSpans != null)
            for (span in colorSpans)
                addRange(span.start, span.end);
        if (richTextSpans != null)
            for (span in richTextSpans)
                addRange(span.range.start, span.range.end);
        if (textSpans != null)
            for (span in textSpans)
                addRange(span.range.start, span.range.end);
        if (rubySpans != null)
            for (span in rubySpans)
                addRange(span.baseRange.start, span.baseRange.end);
        return builder.build();
    }
}
