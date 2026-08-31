package org.tiqian.core;

class EastAsianSpacingEdges {
    public final leading:EastAsianSpacingValue;
    public final trailing:EastAsianSpacingValue;
    public final containsWide:Bool;

    public function new(leading:EastAsianSpacingValue, trailing:EastAsianSpacingValue, containsWide:Bool) {
        this.leading = leading;
        this.trailing = trailing;
        this.containsWide = containsWide;
    }

    public function toString():String {
        return "EastAsianSpacingEdges(leading=" + Std.string(leading)
            + ", trailing=" + Std.string(trailing)
            + ", containsWide=" + Std.string(containsWide) + ")";
    }
}
