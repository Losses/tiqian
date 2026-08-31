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
        // Kotlin declares textStyle = TextStyle() and paragraphStyle =
        // ParagraphStyle() (constructor-call defaults, outside the sanctioned
        // grammar) and profileId = BuiltInLayoutProfiles.ClreqHorizontal (a
        // static-field default, boring gap 4). All three parameters stay
        // mandatory until gap 4 lands for profileId; the constructor-call
        // pair stays mandatory permanently.
        textStyle:TextStyle,
        paragraphStyle:ParagraphStyle,
        constraints:LayoutConstraints,
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

}
