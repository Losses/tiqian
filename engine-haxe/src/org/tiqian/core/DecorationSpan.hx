package org.tiqian.core;

class DecorationSpan {
    public final range:TextRange;
    public final kind:DecorationKind;

    public function new(range:TextRange, kind:DecorationKind) {
        this.range = range;
        this.kind = kind;
    }

    public function toString():String {
        return "DecorationSpan(range=" + range + ", kind=" + Std.string(kind) + ")";
    }
}
