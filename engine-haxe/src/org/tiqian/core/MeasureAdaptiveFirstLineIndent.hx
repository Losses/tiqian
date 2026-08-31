package org.tiqian.core;

class MeasureAdaptiveFirstLineIndent {
    public final shortBelowEm:Float;
    public final shortEm:Float;
    public final longEm:Float;

    public function new(shortBelowEm:Float = 14.0, shortEm:Float = 1.0, longEm:Float = 2.0) {
        this.shortBelowEm = shortBelowEm;
        this.shortEm = shortEm;
        this.longEm = longEm;
    }

    public function resolveEm(measureEm:Float):Float {
        return measureEm < shortBelowEm ? shortEm : longEm;
    }

    public function toString():String {
        return "MeasureAdaptiveFirstLineIndent(shortBelowEm=" + shortBelowEm
            + ", shortEm=" + shortEm
            + ", longEm=" + longEm + ")";
    }
}
