package org.tiqian.core;

import std.ReadOnlyArray;

class LayoutInput {
    public final content:TiqianTextContent;
    public final textStyle:TextStyle;
    public final paragraphStyle:ParagraphStyle;
    public final constraints:LayoutConstraints;
    public final profileId:LayoutProfileId;
    public final decorations:ReadOnlyArray<DecorationSpan>;
    public final rubySpans:ReadOnlyArray<RubySpan>;
    public final inlineBoxes:ReadOnlyArray<InlineBoxSpan>;
    public final inlineObjects:ReadOnlyArray<InlineObjectSpan>;

    public function new(
        content:TiqianTextContent,
        constraints:LayoutConstraints,
        textStyle:TextStyle,
        paragraphStyle:ParagraphStyle,
        profileId:LayoutProfileId,
        ?decorations:Array<DecorationSpan>,
        ?rubySpans:Array<RubySpan>,
        ?inlineBoxes:Array<InlineBoxSpan>,
        ?inlineObjects:Array<InlineObjectSpan>
    ) {
        this.content = content;
        this.textStyle = textStyle;
        this.paragraphStyle = paragraphStyle;
        this.constraints = constraints;
        this.profileId = profileId;
        this.decorations = decorations == null ? [] : decorations;
        this.rubySpans = rubySpans == null ? [] : rubySpans;
        this.inlineBoxes = inlineBoxes == null ? [] : inlineBoxes;
        this.inlineObjects = inlineObjects == null ? [] : inlineObjects;
    }

    public function toString():String {
        return "LayoutInput(content=" + content
            + ", textStyle=" + textStyle
            + ", paragraphStyle=" + paragraphStyle
            + ", constraints=" + constraints
            + ", profileId=" + profileId
            + ", decorations=" + renderArray(decorations)
            + ", rubySpans=" + renderArray(rubySpans)
            + ", inlineBoxes=" + renderArray(inlineBoxes)
            + ", inlineObjects=" + renderArray(inlineObjects) + ")";
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
