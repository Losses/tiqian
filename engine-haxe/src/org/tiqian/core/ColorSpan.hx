package org.tiqian.core;

class ColorSpan {
    public final start:Int;
    public final end:Int;
    public final argb:Int;

    public function new(start:Int, end:Int, argb:Int) {
        this.start = start;
        this.end = end;
        this.argb = argb;
    }

    public function toString():String {
        return "ColorSpan(start=" + start + ", end=" + end + ", argb=" + argb + ")";
    }
}
