package org.tiqian.core;

import std.ReadOnlyArray;

@:dataClass
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
        textStyle:TextStyle,
        paragraphStyle:ParagraphStyle,
        constraints:LayoutConstraints,
        ?profileId:Null<LayoutProfileId>,
        ?decorations:Array<DecorationSpan>,
        ?rubySpans:Array<RubySpan>,
        ?inlineBoxes:Array<InlineBoxSpan>,
        ?inlineObjects:Array<InlineObjectSpan>
    ) {
        this.content = content;
        this.textStyle = textStyle;
        this.paragraphStyle = paragraphStyle;
        this.constraints = constraints;
        this.profileId = profileId == null ? BuiltInLayoutProfiles.ClreqHorizontal : profileId;
        this.decorations = decorations == null ? [] : decorations;
        this.rubySpans = rubySpans == null ? [] : rubySpans;
        this.inlineBoxes = inlineBoxes == null ? [] : inlineBoxes;
        this.inlineObjects = inlineObjects == null ? [] : inlineObjects;
    }

}
