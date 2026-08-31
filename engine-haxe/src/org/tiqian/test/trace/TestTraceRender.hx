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
        var index = 0;
        while (index < value.length) {
            final codeUnit = value.charCodeAt(index);
            if (codeUnit != 69 && codeUnit != 101) {
                output.add(value.substring(index, index + 1));
                index += 1;
                continue;
            }

            final match = scientificMatch(value, index);
            if (match == null) {
                output.add(value.substring(index, index + 1));
                index += 1;
                continue;
            }

            output.add(expandMantissa(match.mantissa, match.exponent));
            index = match.end;
        }
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
                if (cursor > index + 1 && (cursor == value.length || (!isDigit(value.charCodeAt(cursor)) && value.charCodeAt(cursor) != 46))) {
                    index = cursor;
                    continue;
                }
            }
            output.add(value.substring(index, index + 1));
            index += 1;
        }
        return output.toString();
    }

    private static function scientificMatch(value:String, exponentMarker:Int):Null<ScientificMatch> {
        var exponentIndex = exponentMarker + 1;
        var exponentNegative = false;
        if (exponentIndex < value.length && (value.charCodeAt(exponentIndex) == 43 || value.charCodeAt(exponentIndex) == 45)) {
            exponentNegative = value.charCodeAt(exponentIndex) == 45;
            exponentIndex += 1;
        }
        final exponentStart = exponentIndex;
        var exponent = 0;
        while (exponentIndex < value.length && isDigit(value.charCodeAt(exponentIndex))) {
            exponent = exponent * 10 + value.charCodeAt(exponentIndex) - 48;
            exponentIndex += 1;
        }
        if (exponentIndex == exponentStart) {
            return null;
        }
        if (exponentNegative) {
            exponent = -exponent;
        }

        var cursor = exponentMarker - 1;
        while (cursor >= 0 && isDigit(value.charCodeAt(cursor))) {
            cursor -= 1;
        }
        final fractionEnd = exponentMarker;
        var dotIndex = -1;
        if (cursor >= 0 && value.charCodeAt(cursor) == 46) {
            dotIndex = cursor;
            cursor -= 1;
            final integerEnd = cursor;
            while (cursor >= 0 && isDigit(value.charCodeAt(cursor))) {
                cursor -= 1;
            }
            if (integerEnd == cursor) {
                return null;
            }
            if (integerEnd - cursor != 1) {
                return null;
            }
        } else if (fractionEnd - (cursor + 1) != 1) {
            return null;
        }

        var mantissaStart = cursor + 1;
        if (mantissaStart > 0 && value.charCodeAt(mantissaStart - 1) == 45) {
            mantissaStart -= 1;
        }
        if (mantissaStart > 0 && (isDigit(value.charCodeAt(mantissaStart - 1)) || value.charCodeAt(mantissaStart - 1) == 46)) {
            return null;
        }
        final mantissa = value.substring(mantissaStart, exponentMarker);
        return new ScientificMatch(mantissa, exponent, exponentIndex);
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
    public final mantissa:String;
    public final exponent:Int;
    public final end:Int;

    public function new(mantissa:String, exponent:Int, end:Int) {
        this.mantissa = mantissa;
        this.exponent = exponent;
        this.end = end;
    }
}
