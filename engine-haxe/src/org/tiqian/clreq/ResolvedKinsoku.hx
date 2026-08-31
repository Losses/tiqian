package org.tiqian.clreq;

/** Resolved kinsoku level + hanging style for a given measure. */
class ResolvedKinsoku {
    public final level:KinsokuLevel;
    public final hanging:HangingPunctuationStyle;
    public final reason:String;

    public function new(level:KinsokuLevel, hanging:HangingPunctuationStyle, reason:String) {
        this.level = level;
        this.hanging = hanging;
        this.reason = reason;
    }

    public function toString():String {
        return "ResolvedKinsoku(level=" + level
            + ", hanging=" + hanging
            + ", reason=" + reason + ")";
    }
}
