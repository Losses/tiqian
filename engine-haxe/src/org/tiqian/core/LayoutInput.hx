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
        constraints:LayoutConstraints,
        // Kotlin declares textStyle = TextStyle() and paragraphStyle =
        // ParagraphStyle() (constructor-call defaults, outside the sanctioned
        // grammar) and profileId = BuiltInLayoutProfiles.ClreqHorizontal (a
        // static-field default, boring gap 4). All three parameters stay
        // mandatory until gap 4 lands for profileId; the constructor-call
        // pair stays mandatory permanently.
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

    // The constructor parameter order diverges from the Kotlin primary
    // constructor (constraints moved ahead of the gap-4-mandatory trio), and
    // synthesis prints parameter order, so this explicit member stays.
    public function toString():String {
        return "LayoutInput(content=" + content.toString()
            + ", textStyle=" + textStyle.toString()
            + ", paragraphStyle=" + paragraphStyle.toString()
            + ", constraints=" + constraints.toString()
            + ", profileId=" + profileId.toString()
            + ", decorations=" + renderDecorations(decorations)
            + ", rubySpans=" + renderRubySpans(rubySpans)
            + ", inlineBoxes=" + renderInlineBoxes(inlineBoxes)
            + ", inlineObjects=" + renderInlineObjects(inlineObjects) + ")";
    }

    private static function renderDecorations(values:ReadOnlyArray<DecorationSpan>):String {
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

    private static function renderRubySpans(values:ReadOnlyArray<RubySpan>):String {
        var output:String = "[";
        var index:Int = 0;
        while (index < values.length) {
            if (index > 0) output += ", ";
            output += values[index].toString();
            index += 1;
        }
        return output + "]";
    }

    private static function renderInlineBoxes(values:ReadOnlyArray<InlineBoxSpan>):String {
        var output:String = "[";
        var index:Int = 0;
        while (index < values.length) {
            if (index > 0) output += ", ";
            output += values[index].toString();
            index += 1;
        }
        return output + "]";
    }

    private static function renderInlineObjects(values:ReadOnlyArray<InlineObjectSpan>):String {
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
