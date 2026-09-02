package org.tiqian.test.trace;

import std.ReadOnlyArray;
import std.StringBuf;

class TestTraceRender {
    private static final MAX_OPERAND_CHARS:Int = 240;
    private static final HEX:String = "0123456789abcdef";

    public static function escapeOperand(value:String):String {
        return cap(escape(value));
    }

    public static function renderString(value:String):String {
        return cap("'" + escape(value) + "'");
    }

    public static function renderInt(value:Int):String {
        return cap(TraceFormat.i(value));
    }

    public static function renderLong(value:Int):String {
        return cap(TraceFormat.valueLong(value));
    }

    public static function renderFloat(value:Float):String {
        return cap(TraceFormat.fd(value, 6));
    }

    public static function renderBool(value:Bool):String {
        return cap(TraceFormat.valueBool(value));
    }

    public static function renderNull():String {
        return "-";
    }

    public static function renderIntArray(values:Array<Int>):String {
        final output = new StringBuf();
        output.add("[");
        var index = 0;
        while (index < values.length) {
            if (index > 0) {
                output.add(", ");
            }
            output.add(renderInt(values[index]));
            index += 1;
        }
        output.add("]");
        return cap(output.toString());
    }

    public static function renderStringArray(values:ReadOnlyArray<String>):String {
        final output = new StringBuf();
        output.add("[");
        var index = 0;
        while (index < values.length) {
            if (index > 0) {
                output.add(", ");
            }
            output.add(renderString(values[index]));
            index += 1;
        }
        output.add("]");
        return cap(output.toString());
    }

    public static function canonicalNumbers(value:String):String {
        return stripWholeFraction(expandScientific(value));
    }

    public static function cap(value:String):String {
        final canonical = canonicalNumbers(value);
        if (canonical.length <= MAX_OPERAND_CHARS) {
            return canonical;
        }
        return canonical.substring(0, MAX_OPERAND_CHARS) + "~" + canonical.length + "#" + fnv1a(canonical);
    }

    private static function escape(value:String):String {
        final output = new StringBuf();
        var index = 0;
        while (index < value.length) {
            final codeUnit = value.charCodeAt(index);
            if (codeUnit == 0) {
                output.add("\\u0000");
            } else {
                output.add(TraceFormat.escapeText(value.substring(index, index + 1)));
            }
            index += 1;
        }
        return output.toString();
    }

    private static function expandScientific(value:String):String {
        final output = new StringBuf();
        var copied = 0;
        var index = 0;
        while (index < value.length) {
            final codeUnit = value.charCodeAt(index);
            if (codeUnit != 69 && codeUnit != 101) {
                index += 1;
                continue;
            }

            final match = scientificMatch(value, index, copied);
            if (match == null) {
                index += 1;
                continue;
            }

            output.add(value.substring(copied, match.start));
            output.add(expandMantissa(value.substring(match.start, match.mantissaEnd), match.exponent));
            copied = match.end;
            index = match.end;
        }
        output.add(value.substring(copied, value.length));
        return output.toString();
    }

    private static function stripWholeFraction(value:String):String {
        final output = new StringBuf();
        var index = 0;
        while (index < value.length) {
            if (value.charCodeAt(index) == 46 && index > 0 && isDigit(value.charCodeAt(index - 1))) {
                var cursor = index + 1;
                while (cursor < value.length && value.charCodeAt(cursor) == 48) {
                    cursor += 1;
                }
                if (cursor > index + 1
                    && (cursor == value.length || (!isDigit(value.charCodeAt(cursor)) && value.charCodeAt(cursor) != 46))) {
                    index = cursor;
                    continue;
                }
            }
            output.add(value.substring(index, index + 1));
            index += 1;
        }
        return output.toString();
    }

    private static function scientificMatch(s:String, eIndex:Int, floor:Int):Null<ScientificMatch> {
        var i = eIndex + 1;
        if (i < s.length && (s.charCodeAt(i) == 43 || s.charCodeAt(i) == 45)) {
            i += 1;
        }
        final digitsStart = i;
        while (i < s.length && isDigit(s.charCodeAt(i))) {
            i += 1;
        }
        if (i == digitsStart) {
            return null;
        }
        final end = i;
        final exponent = parseExponent(s, eIndex + 1, end);

        final last = eIndex - 1;
        if (last < floor || !isDigit(s.charCodeAt(last))) {
            return null;
        }
        var runStart = last;
        while (runStart > floor && isDigit(s.charCodeAt(runStart - 1))) {
            runStart -= 1;
        }
        var lead = -1;
        if (runStart > floor + 1 && s.charCodeAt(runStart - 1) == 46 && isDigit(s.charCodeAt(runStart - 2))) {
            lead = runStart - 2;
        } else {
            if (runStart != last) {
                return null;
            }
            lead = last;
        }

        if (lead > 0 && s.charCodeAt(lead - 1) == 45 && lead - 1 >= floor) {
            final beforeMinus = lead >= 2 && (isDigit(s.charCodeAt(lead - 2)) || s.charCodeAt(lead - 2) == 46);
            if (!beforeMinus) {
                return new ScientificMatch(lead - 1, eIndex, end, exponent);
            }
            return new ScientificMatch(lead, eIndex, end, exponent);
        }
        final beforeDigit = lead > 0 && (isDigit(s.charCodeAt(lead - 1)) || s.charCodeAt(lead - 1) == 46);
        if (!beforeDigit) {
            return new ScientificMatch(lead, eIndex, end, exponent);
        }
        return null;
    }

    static function parseExponent(s:String, from:Int, to:Int):Int {
        var index = from;
        var negative = false;
        if (index < to && (s.charCodeAt(index) == 43 || s.charCodeAt(index) == 45)) {
            negative = s.charCodeAt(index) == 45;
            index += 1;
        }
        var value = 0;
        while (index < to) {
            value = value * 10 + s.charCodeAt(index) - 48;
            index += 1;
        }
        return negative ? -value : value;
    }

    private static function expandMantissa(mantissa:String, exponent:Int):String {
        var sign = "";
        var value = mantissa;
        if (value.length > 0 && value.charCodeAt(0) == 45) {
            sign = "-";
            value = value.substring(1);
        }

        var dotIndex = -1;
        var index = 0;
        while (index < value.length) {
            if (value.charCodeAt(index) == 46) {
                dotIndex = index;
                break;
            }
            index += 1;
        }
        var digits = dotIndex < 0 ? value : value.substring(0, dotIndex) + value.substring(dotIndex + 1);
        while (digits.length > 1 && digits.charCodeAt(digits.length - 1) == 48) {
            digits = digits.substring(0, digits.length - 1);
        }
        final decimalPosition = (dotIndex < 0 ? value.length : dotIndex) + exponent;
        if (decimalPosition <= 0) {
            return sign + "0." + zeroes(-decimalPosition) + digits;
        }
        if (decimalPosition >= digits.length) {
            return sign + digits + zeroes(decimalPosition - digits.length);
        }
        return sign + digits.substring(0, decimalPosition) + "." + digits.substring(decimalPosition);
    }

    private static function zeroes(count:Int):String {
        final output = new StringBuf();
        var index = 0;
        while (index < count) {
            output.add("0");
            index += 1;
        }
        return output.toString();
    }

    private static function isDigit(codeUnit:Int):Bool {
        return codeUnit >= 48 && codeUnit <= 57;
    }

    private static function fnv1a(value:String):String {
        var hash:Int = -2128831035;
        var index = 0;
        while (index < value.length) {
            hash = (hash ^ value.charCodeAt(index));
            hash = multiplyFNVPrime(hash);
            index += 1;
        }

        final first = (hash >>> 24) & 255;
        final second = (hash >>> 16) & 255;
        final third = (hash >>> 8) & 255;
        final fourth = hash & 255;
        var output = hexByte(first) + hexByte(second) + hexByte(third) + hexByte(fourth);
        while (output.length > 1 && output.charCodeAt(0) == 48) {
            output = output.substring(1);
        }
        return output;
    }

    private static function multiplyFNVPrime(value:Int):Int {
        // Split the 32-bit product into sixteen-bit limbs. JavaScript's ordinary
        // multiplication rounds the full product above 2^53, while this form
        // preserves the modulo-2^32 FNV-1a result on every target.
        final low:Int = value & 0xFFFF;
        final high:Int = (value >>> 16) & 0xFFFF;
        final lowProduct:Int = low * 0x0193;
        final highProduct:Int = high * 0x0193 + low * 0x0100 + Std.int(lowProduct / 0x10000);
        return ((highProduct & 0xFFFF) << 16) | (lowProduct & 0xFFFF);
    }

    private static function hexByte(value:Int):String {
        return HEX.substring((value >>> 4) & 15, ((value >>> 4) & 15) + 1) + HEX.substring(value & 15, (value & 15) + 1);
    }
}

private class ScientificMatch {
    public final start:Int;
    public final mantissaEnd:Int;
    public final end:Int;
    public final exponent:Int;

    public function new(start:Int, mantissaEnd:Int, end:Int, exponent:Int) {
        this.start = start;
        this.mantissaEnd = mantissaEnd;
        this.end = end;
        this.exponent = exponent;
    }
}
