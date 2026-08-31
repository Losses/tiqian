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

    // Kotlin's List.toString joins with ", " while Std.string over the array
    // joins with ","; generic synthesis cannot reproduce the Kotlin text, so
    // this explicit member stays.
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

    public function copy():BopomofoReading {
        final copiedSymbols:Array<String> = [];
        var index:Int = 0;
        while (index < symbols.length) {
            copiedSymbols.push(symbols[index]);
            index += 1;
        }
        return new BopomofoReading(copiedSymbols, tone);
    }

    public function hashCode():Int {
        var hash:Int = 17;
        hash = hash * 31 + toneIndex();
        var index:Int = 0;
        while (index < symbols.length) {
            final symbol = symbols[index];
            var unitIndex:Int = 0;
            while (unitIndex < symbol.length) {
                hash = hash * 31 + symbol.charCodeAt(unitIndex);
                unitIndex += 1;
            }
            index += 1;
        }
        return hash;
    }

    private function toneIndex():Int {
        return switch (tone) {
            case BopomofoTone.Yinping:
                0;
            case BopomofoTone.Yangping:
                1;
            case BopomofoTone.Shang:
                2;
            case BopomofoTone.Qu:
                3;
            case BopomofoTone.Neutral:
                4;
            case BopomofoTone.Ru:
                5;
        };
    }
}
