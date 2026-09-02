package org.tiqian.layout;

import std.StringBuf;
import std.SortedMap;
import org.tiqian.core.LayoutResult;

typedef PreparedParagraphDigitsAndExponent = { digits:String, exponent:Int };
typedef PreparedParagraphDecomposed = { mant24:Int, exp2:Int };

/** Prepared paragraph JSON serialization functions. */
class PreparedParagraphFns {

    private static var fivePowersBuilder = null;
    private static var fivePowers:std.SortedMap<Int,String> = null;
    private static var twoPowersBuilder = null;
    private static var twoPowers:std.SortedMap<Int,String> = null;

    public static function toPreparedParagraphJson(result:LayoutResult, renderEvidence:Bool = false):String {
        throw new org.tiqian.core.IllegalStateException("TODO r4: toPreparedParagraphJson");
    }

    public static function toPlanWithDiagnosticsJson(result:LayoutResult, renderEvidence:Bool, zeroAdvanceEpsilonPx:Float):String {
        throw new org.tiqian.core.IllegalStateException("TODO r4: toPlanWithDiagnosticsJson");
    }

    public static function ecmaJsonNumber(floatValue:Float):String {
        if (Math.isNaN(floatValue)) return "NaN";
        if (Math.isFinite(floatValue) == false) return floatValue < 0 ? "-Infinity" : "Infinity";
        if (floatValue == 0) return "0";
        final negative = floatValue < 0;
        final magnitudeValue = negative ? -floatValue : floatValue;
        final shortest = shortestRoundTripDigits(magnitudeValue);
        final digits = canonicalFloatDigits(magnitudeValue, shortest.digits);
        final k = digits.length;
        final n = shortest.exponent;
        final sign = negative ? "-" : "";
        if (k <= n && n <= 21) return sign + digits + zeros(n-k);
        if (0 < n && n <= 21) return sign + digits.substr(0,n) + "." + digits.substr(n);
        if (-6 < n && n <= 0) return sign + "0." + zeros(-n) + digits;
        final mantissa = k > 1 ? digits.substr(0,1) + "." + digits.substr(1) : digits;
        final ev = n - 1;
        final esign = ev < 0 ? "-" : "+";
        final absoluteExponent:Int = ev < 0 ? -ev : ev;
        return sign + mantissa + "e" + esign + Std.string(absoluteExponent);
    }

    public static function shortestRoundTripDigits(magnitude:Float):PreparedParagraphDigitsAndExponent {
        final d = decompose(magnitude);
        final f = d.exp2;
        final expansion = dyadicDecimal(d.mant24, f);
        final exact = trimZeros(expansion.digits);
        final n = expansion.exponent;
        final halfExponent = d.mant24 < 0x800000 ? d.exp2 - 53 : d.exp2 - 30;
        final scale = d.mant24 < 0x800000 ? 52 : 30;
        final base = timesLong(twoToThe(scale), d.mant24);
        final hi = dyadicDecimalString(addDecimal(base, "1"), halfExponent);
        final lo = dyadicDecimalString(decrementDecimal(base), halfExponent);
        final inclusive = true;
        var length = 1;
        while (length <= 17) {
            final keep = exact.substr(0, length);
            final up = incrementDecimal(keep);
            final upN = n + up.length - length;
            if (inInterval(keep, n, lo, hi, inclusive) || inInterval(up, upN, lo, hi, inclusive)) {
                final rounded = roundToSignificant(exact, length);
                return {digits:rounded.digits, exponent:n + rounded.exponent};
            }
            length += 1;
        }
        return {digits:exact, exponent:n};
    }

    private static function inInterval(digits:String, n:Int, lo:PreparedParagraphDigitsAndExponent,
            hi:PreparedParagraphDigitsAndExponent, inclusive:Bool):Bool {
        final low = compareDecimal(digits, n, lo.digits, lo.exponent);
        final high = compareDecimal(digits, n, hi.digits, hi.exponent);
        return (low > 0 || (low == 0 && inclusive)) && (high < 0 || (high == 0 && inclusive));
    }

    private static function canonicalFloatDigits(magnitude:Float, doubleDigits:String):String {
        final d = decompose(magnitude);
        final exact = d.mant24 == 0 ? "0" : (d.exp2 >= 0
            ? timesLong(twoToThe(d.exp2), d.mant24)
            : timesLong(fiveToThe(-d.exp2), d.mant24));
        final stripped = trimZeros(exact);
        if (stripped.length <= doubleDigits.length) return doubleDigits;
        final rounded = roundToSignificant(stripped, doubleDigits.length);
        return rounded.digits.length == doubleDigits.length ? rounded.digits : doubleDigits;
    }

    private static function dyadicDecimal(p:Int, f:Int):PreparedParagraphDigitsAndExponent {
        return dyadicDecimalString(Std.string(p), f);
    }

    private static function dyadicDecimalString(p:String, f:Int):PreparedParagraphDigitsAndExponent {
        final digits = f < 0 ? multiplyDecimal(fiveToThe(-f), p) : multiplyDecimal(twoToThe(f), p);
        return {digits:digits, exponent:f < 0 ? digits.length + f : digits.length};
    }

    private static function multiplyDecimal(a:String, b:String):String {
        var result = "0";
        var shift = 0;
        var i = b.length - 1;
        while (i >= 0) {
            final digit = b.charCodeAt(i) - 48;
            if (digit != 0) {
                var part = timesSmall(a, digit);
                if (shift > 0) part += zeros(shift);
                result = addDecimal(result, part);
            }
            shift++;
            i--;
        }
        return result;
    }

    private static function fiveToThe(k:Int):String {
        if (fivePowersBuilder == null) { fivePowersBuilder = std.SortedMap.builder(); fivePowers = fivePowersBuilder.build(); }
        final cached = fivePowers.get(k);
        if (cached != null) return cached;
        var anchor = 0;
        var digits = "1";
        var i = 0;
        while (i < fivePowers.size()) { if (fivePowers.keyAt(i) < k && fivePowers.keyAt(i) > anchor) { anchor=fivePowers.keyAt(i); digits=fivePowers.valueAt(i); } i++; }
        i = anchor;
        while (i < k) { digits=timesSmall(digits,5); i++; }
        fivePowersBuilder.put(k,digits); fivePowers=fivePowersBuilder.build();
        return digits;
    }

    private static function twoToThe(k:Int):String {
        if (twoPowersBuilder == null) { twoPowersBuilder = std.SortedMap.builder(); twoPowers = twoPowersBuilder.build(); }
        final cached = twoPowers.get(k);
        if (cached != null) return cached;
        var anchor = 0;
        var digits = "1";
        var i = 0;
        while (i < twoPowers.size()) { if (twoPowers.keyAt(i) < k && twoPowers.keyAt(i) > anchor) { anchor=twoPowers.keyAt(i); digits=twoPowers.valueAt(i); } i++; }
        i = anchor;
        while (i < k) { digits=timesSmall(digits,2); i++; }
        twoPowersBuilder.put(k,digits); twoPowers=twoPowersBuilder.build();
        return digits;
    }

    private static function timesLong(digits:String, factor:Int):String {
        if (factor == 0) return "0";
        var result:String = null;
        var shift = 0;
        var remaining = factor;
        while (remaining > 0) {
            final chunk = remaining % 100000000;
            remaining = Std.int(remaining / 100000000);
            if (chunk != 0) {
                var part = timesSmall(digits, chunk);
                if (shift > 0) part += zeros(shift);
                result = result == null ? part : addDecimal(result, part);
            }
            shift += 8;
        }
        return result == null ? "0" : result;
    }

    private static function addDecimal(a:String,b:String):String {
        final out = new StringBuf(); var i=a.length-1; var j=b.length-1; var carry=0;
        while (i>=0 || j>=0 || carry>0) { final sum=(i>=0?a.charCodeAt(i)-48:0)+(j>=0?b.charCodeAt(j)-48:0)+carry; out.addChar(48+sum%10); carry=Std.int(sum/10); i--; j--; }
        return reverse(out.toString());
    }

    private static function roundToSignificant(exact:String,length:Int):PreparedParagraphDigitsAndExponent {
        if (length >= exact.length) return {digits:exact, exponent:0};
        final keep=exact.substr(0,length); final rem=exact.substr(length); var up=false;
        if (rem.charCodeAt(0)>53) up=true; else if (rem.charCodeAt(0)==53) { var tail=1; while(tail<rem.length && rem.charCodeAt(tail)==48) tail++; up=tail<rem.length || ((keep.charCodeAt(keep.length-1)-48)%2!=0); }
        final rounded=up?incrementDecimal(keep):keep;
        return {digits:trimZeros(rounded), exponent:rounded.length-length};
    }

    private static function compareDecimal(a:String, nA:Int, b:String, nB:Int):Int {
        final eA = nA - a.length; final eB = nB - b.length;
        final aa = eA >= eB ? a + zeros(eA-eB) : a;
        final bb = eB >= eA ? b + zeros(eB-eA) : b;
        if (aa.length != bb.length) return aa.length - bb.length;
        return aa < bb ? -1 : (aa > bb ? 1 : 0);
    }
    private static function timesSmall(digits:String,factor:Int):String {
        final out=new StringBuf(); var carry=0; var i=digits.length-1;
        while(i>=0) { final product=(digits.charCodeAt(i)-48)*factor+carry; out.addChar(48+product%10); carry=Std.int(product/10); i--; }
        while(carry>0) { out.addChar(48+carry%10); carry=Std.int(carry/10); }
        return reverse(out.toString());
    }
    private static function incrementDecimal(digits:String):String { var a=digits.split(""); var i=a.length-1; while(true) { if(a[i]!="9") { a[i]=String.fromCharCode(a[i].charCodeAt(0)+1); return a.join(""); } a[i]="0"; if(i==0)return "1"+a.join(""); i--; } }
    private static function decrementDecimal(digits:String):String {
        var a = digits.split("");
        var i = a.length - 1;
        while (i >= 0 && a[i] == "0") { a[i] = "9"; i--; }
        if (i >= 0) a[i] = String.fromCharCode(a[i].charCodeAt(0) - 1);
        var out = a.join("");
        var start = 0;
        while (start < out.length - 1 && out.charCodeAt(start) == 48) start++;
        return out.substr(start);
    }
    private static function decompose(v:Float):PreparedParagraphDecomposed {
        final bits = haxe.io.FPHelper.floatToI32(v);
        final rawExponent = (bits >>> 23) & 0xff;
        final rawMantissa = bits & 0x7fffff;
        if (rawExponent == 0) return {mant24:rawMantissa, exp2: -149};
        return {mant24:rawMantissa | 0x800000, exp2:rawExponent - 150};
    }
    private static function zeros(n:Int):String { var s=""; var i=0; while(i<n){s+="0";i++;} return s; }
    private static function trimZeros(s:String):String { var e=s.length; while(e>1 && s.charCodeAt(e-1)==48)e--; return s.substr(0,e); }
    private static function reverse(s:String):String { var r=""; var i=s.length-1; while(i>=0){r+=s.charAt(i);i--;} return r; }
}
