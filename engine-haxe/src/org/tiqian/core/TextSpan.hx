package org.tiqian.core;

class TextSpan {
    public final range:TextRange;
    public final style:TextStyle;

    public function new(range:TextRange, style:TextStyle) {
        this.range = range;
        this.style = style;
    }

    public function toString():String {
        return "TextSpan(range=" + range.toString() + ", style=" + style.toString() + ")";
    }
}
