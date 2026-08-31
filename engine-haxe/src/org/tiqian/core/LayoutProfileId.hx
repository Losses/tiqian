package org.tiqian.core;

class LayoutProfileId {
    public final value:String;

    public function new(value:String) {
        this.value = value;
    }

    public function toString():String {
        return "LayoutProfileId(value=" + value + ")";
    }
}
