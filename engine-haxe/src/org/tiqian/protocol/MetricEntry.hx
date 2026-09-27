package org.tiqian.protocol;

class MetricEntry {
    public var familiesRef:Int;
    public var weight:Float;
    public var italic:Int;
    public var roleRef:Int;
    public var faceSelectionRef:Int;
    public var valuePoolRef:Int;
    public var stored:Bool;

    public function new(familiesRef:Int, weight:Float, italic:Int, roleRef:Int, faceSelectionRef:Int, valuePoolRef:Int) {
        this.familiesRef = familiesRef;
        this.weight = weight;
        this.italic = italic;
        this.roleRef = roleRef;
        this.faceSelectionRef = faceSelectionRef;
        this.valuePoolRef = valuePoolRef;
        this.stored = false;
    }
}

/** One deduplicated metric value quintuple; null marks the absent slot. */
