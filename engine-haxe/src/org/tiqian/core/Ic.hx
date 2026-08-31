package org.tiqian.core;

/** Unit value used for a count of CJK 字身框 cells. */
class Ic {
    public final count:Float;

    public function new(count:Float) {
        this.count = count;
    }

    public function toPx(emPx:Float):Float {
        return count * emPx;
    }

    public static function plus(left:Ic, right:Ic):Ic {
        return new Ic(left.count + right.count);
    }

    public static function unaryMinus(value:Ic):Ic {
        return new Ic(-value.count);
    }

    public static final Zero:Ic = new Ic(0.0);

    public function toString():String {
        return "Ic(count=" + Std.string(count) + ")";
    }

}
