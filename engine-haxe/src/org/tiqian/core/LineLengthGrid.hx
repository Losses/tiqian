package org.tiqian.core;

class LineLengthGrid {
    public final enabled:Bool;
    public final bodyAlignment:Null<LastLineAlignment>;

    public function new(enabled:Bool = true, bodyAlignment:Null<LastLineAlignment>) {
        this.enabled = enabled;
        this.bodyAlignment = bodyAlignment;
    }

    public function toString():String {
        return "LineLengthGrid(enabled=" + enabled
            + ", bodyAlignment=" + (bodyAlignment == null ? "null" : Std.string(bodyAlignment)) + ")";
    }
}
