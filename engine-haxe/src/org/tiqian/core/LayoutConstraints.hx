package org.tiqian.core;

class LayoutConstraints {
    public final maxWidth:Float;
    public final maxHeight:Float;
    public final maxLines:Int;

    public function new(maxWidth:Float, maxHeight:Float, maxLines:Int) {
        final resolvedMaxHeight:Float = maxHeight;
        if (!(maxWidth > 0.0)) {
            throw new TiqianIllegalArgumentException(Message("maxWidth must be positive."));
        }
        if (!(resolvedMaxHeight > 0.0)) {
            throw new TiqianIllegalArgumentException(Message("maxHeight must be positive."));
        }
        if (maxLines <= 0) {
            throw new TiqianIllegalArgumentException(Message("maxLines must be positive."));
        }
        this.maxWidth = maxWidth;
        this.maxHeight = resolvedMaxHeight;
        this.maxLines = maxLines;
    }

    public function toString():String {
        return "LayoutConstraints(maxWidth=" + maxWidth
            + ", maxHeight=" + maxHeight
            + ", maxLines=" + maxLines + ")";
    }
}
