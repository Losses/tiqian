package org.tiqian.clreq;

import std.ReadOnlyArray;
import std.StringBuf;

@:dataClass
class BopomofoReading {
    public final symbols:ReadOnlyArray<String>;
    public final tone:BopomofoTone;

    public function new(symbols:Array<String>, tone:BopomofoTone) {
        this.symbols = symbols;
        this.tone = tone;
    }

    public function toString():String {
        final output = new StringBuf();
        output.add("BopomofoReading(symbols=[");
        var index:Int = 0;
        while (index < symbols.length) {
            if (index > 0) {
                output.add(", ");
            }
            output.add(symbols[index]);
            index += 1;
        }
        output.add("], tone=");
        output.add(Std.string(tone));
        output.add(")");
        return output.toString();
    }
}
