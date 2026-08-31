package org.tiqian.clreq;

class PunctuationPolicy {
    public final punctuationClass:PunctuationClass;
    public final allowAtLineStart:Bool;
    public final allowAtLineEnd:Bool;
    public final defaultBodyEm:Float;
    public final defaultAdvanceEm:Float;

    public function new(
        punctuationClass:PunctuationClass,
        allowAtLineStart:Bool,
        allowAtLineEnd:Bool,
        defaultBodyEm:Float,
        ?defaultAdvanceEm:Null<Float>
    ) {
        this.punctuationClass = punctuationClass;
        this.allowAtLineStart = allowAtLineStart;
        this.allowAtLineEnd = allowAtLineEnd;
        this.defaultBodyEm = defaultBodyEm;
        this.defaultAdvanceEm = defaultAdvanceEm == null ? 1.0 : defaultAdvanceEm;
    }

    public function toString():String {
        return "PunctuationPolicy(punctuationClass=" + punctuationClass
            + ", allowAtLineStart=" + allowAtLineStart
            + ", allowAtLineEnd=" + allowAtLineEnd
            + ", defaultBodyEm=" + defaultBodyEm
            + ", defaultAdvanceEm=" + defaultAdvanceEm + ")";
    }
}
