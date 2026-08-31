package org.tiqian.clreq;

import std.ReadOnlyArray;

class AutoSpacePolicy {
    public final cjkLatin:AutoSpaceMode;
    public final cjkDigit:AutoSpaceMode;
    public final gapEm:Float;
    public final stretchMaxEm:Float;

    public function new(
        ?cjkLatin:Null<AutoSpaceMode>,
        ?cjkDigit:Null<AutoSpaceMode>,
        ?gapEm:Null<Float>,
        // Kotlin writes 1f / 3f, a binary-operator default (boring gap 4).
        // The parameter stays mandatory until that lowering lands; callers
        // pass 0.33333334, the shortest decimal that reads back as the same
        // f32 as 1f / 3f.
        stretchMaxEm:Float
    ) {
        this.cjkLatin = cjkLatin == null ? AutoSpaceMode.Insert : cjkLatin;
        this.cjkDigit = cjkDigit == null ? AutoSpaceMode.Insert : cjkDigit;
        this.gapEm = gapEm == null ? 0.125 : gapEm;
        this.stretchMaxEm = stretchMaxEm;
    }

    public function toString():String {
        return "AutoSpacePolicy(cjkLatin=" + cjkLatin
            + ", cjkDigit=" + cjkDigit
            + ", gapEm=" + gapEm
            + ", stretchMaxEm=" + stretchMaxEm + ")";
    }

    public static function samePolicy(a:AutoSpacePolicy, b:AutoSpacePolicy):Bool {
        return a.cjkLatin == b.cjkLatin
            && a.cjkDigit == b.cjkDigit
            && a.gapEm == b.gapEm
            && a.stretchMaxEm == b.stretchMaxEm;
    }

    /** Practice-converged preset: 1/8 base, 1/3 ceiling. The default. */
    public static final Default:AutoSpacePolicy = new AutoSpacePolicy(null, null, null, 0.33333334);

    /** CLREQ literal: 1/4 base, 1/2 ceiling. */
    public static final Clreq:AutoSpacePolicy = new AutoSpacePolicy(null, null, 0.25, 0.5);

    public static final Disabled:AutoSpacePolicy = new AutoSpacePolicy(AutoSpaceMode.Disabled, AutoSpaceMode.Disabled, null, 0.33333334);
}
