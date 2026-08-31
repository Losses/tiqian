package org.tiqian.core;

class LineBreakSpan {
    public final range:TextRange;
    public final policy:LineBreakPolicy;

    public function new(range:TextRange, policy:LineBreakPolicy) {
        this.range = range;
        this.policy = policy;
    }

    public function toString():String {
        return "LineBreakSpan(range=" + range + ", policy=" + Std.string(policy) + ")";
    }
}
