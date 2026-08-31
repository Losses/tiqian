package org.tiqian.core;

class MaxLinesDecisionInfo {
    public final laidOutLines:Int;
    public final visibleLines:Int;
    public final reason:String;

    public function new(laidOutLines:Int, visibleLines:Int, reason:String) {
        this.laidOutLines = laidOutLines;
        this.visibleLines = visibleLines;
        this.reason = reason;
    }

    public function toString():String {
        return "MaxLinesDecisionInfo(laidOutLines=" + laidOutLines
            + ", visibleLines=" + visibleLines + ", reason=" + reason + ")";
    }
}
