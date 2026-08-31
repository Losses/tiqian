package org.tiqian.core;

class LineLengthGrid {
    public final enabled:Bool;
    public final bodyAlignment:Null<LastLineAlignment>;

    public function new(?enabled:Null<Bool>, ?bodyAlignment:Null<LastLineAlignment>) {
        this.enabled = enabled == null ? true : enabled;
        this.bodyAlignment = bodyAlignment == null ? null : bodyAlignment;
    }

    public function toString():String {
        final alignment = bodyAlignment;
        return "LineLengthGrid(enabled=" + enabled
            + ", bodyAlignment=" + (alignment == null ? "null" : Std.string(alignment)) + ")";
    }
}
