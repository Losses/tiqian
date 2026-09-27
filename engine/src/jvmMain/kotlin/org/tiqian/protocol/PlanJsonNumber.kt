package org.tiqian.protocol

import org.tiqian.boring.runtime.FPHelper
import org.tiqian.boring.runtime.SortedMapTable
import org.tiqian.boring.runtime.SortedMapTableBuilder
import org.tiqian.boring.runtime.SortedTable
import std.UStringException

object PlanJsonNumber {
    private var fivePowersBuilder: SortedMapTableBuilder<Int, String>? = null
    private var fivePowers: SortedMapTable<Int, String>? = null
    private var twoPowersBuilder: SortedMapTableBuilder<Int, String>? = null
    private var twoPowers: SortedMapTable<Int, String>? = null

    fun ecmaJsonNumber(floatValue: Float): String {
        if (((floatValue).isNaN())) {
            return "NaN"
        }
        if (((floatValue).isFinite() == false)) {
            return (if ((floatValue < (0).toFloat())) "-Infinity" else "Infinity")
        }
        if ((floatValue == (0).toFloat())) {
            return "0"
        }
        val negative = floatValue < (0).toFloat()
        val magnitudeValue = (if ((negative)) -floatValue else floatValue)
        val shortest = PlanJsonNumber.shortestRoundTripDigits(magnitudeValue)
        val digits = PlanJsonNumber.canonicalFloatDigits(magnitudeValue, shortest.digits)
        val k = digits.length
        val n = shortest.exponent
        val sign = (if ((negative)) "-" else "")
        if ((k <= n && n <= 21)) {
            return sign + digits + PlanJsonNumber.zeros(n - k)
        }
        if ((0 < n && n <= 21)) {
            return sign + run { val _s = digits; val _pos = 0; val _len = n; val _start = if (_pos < 0) maxOf(0, _s.length + _pos) else minOf(_pos, _s.length); if (_len < 0) "" else { val _end = minOf(_s.length, _start + _len); _s.substring(_start, _end) } } + "." + run { val _s = digits; val _pos = n; val _start = if (_pos < 0) maxOf(0, _s.length + _pos) else minOf(_pos, _s.length); _s.substring(_start) }
        }
        if ((-6 < n && n <= 0)) {
            return sign + "0." + PlanJsonNumber.zeros(-n) + digits
        }
        val mantissa = (if ((k > 1)) run { val _s = digits; val _pos = 0; val _len = 1; val _start = if (_pos < 0) maxOf(0, _s.length + _pos) else minOf(_pos, _s.length); if (_len < 0) "" else { val _end = minOf(_s.length, _start + _len); _s.substring(_start, _end) } } + "." + run { val _s = digits; val _pos = 1; val _start = if (_pos < 0) maxOf(0, _s.length + _pos) else minOf(_pos, _s.length); _s.substring(_start) } else digits)
        val ev = n - 1
        val esign = (if ((ev < 0)) "-" else "+")
        val absoluteExponent = (if ((ev < 0)) -ev else ev)
        return sign + mantissa + "e" + esign + absoluteExponent
    }

    fun appendJsonString(out: StringBuilder, value: String) {
        val tail = out.lastOrNull()?.code ?: -1
        if (tail >= 55296 && tail <= 56319 && "\"".length > 0 && !("\""[0].code >= 56320 && "\""[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail)
        }
        out.append("\"")
        for (i in 0 until value.length) {
            val c = run { val _s = value; val _i = i; if (_i >= 0 && _i < _s.length) _s[_i].code else null }
            if ((c == 34)) {
                val tail2 = out.lastOrNull()?.code ?: -1
                if (tail2 >= 55296 && tail2 <= 56319 && "\\\"".length > 0 && !("\\\""[0].code >= 56320 && "\\\""[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail2)
                }
                out.append("\\\"")
            } else {
                if ((c == 92)) {
                    val tail3 = out.lastOrNull()?.code ?: -1
                    if (tail3 >= 55296 && tail3 <= 56319 && "\\\\".length > 0 && !("\\\\"[0].code >= 56320 && "\\\\"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail3)
                    }
                    out.append("\\\\")
                } else {
                    if ((c == 8)) {
                        val tail4 = out.lastOrNull()?.code ?: -1
                        if (tail4 >= 55296 && tail4 <= 56319 && "\\b".length > 0 && !("\\b"[0].code >= 56320 && "\\b"[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail4)
                        }
                        out.append("\\b")
                    } else {
                        if ((c == 12)) {
                            val tail5 = out.lastOrNull()?.code ?: -1
                            if (tail5 >= 55296 && tail5 <= 56319 && "\\f".length > 0 && !("\\f"[0].code >= 56320 && "\\f"[0].code <= 57343)) {
                                throw UStringException.UnpairedSurrogate(tail5)
                            }
                            out.append("\\f")
                        } else {
                            if ((c == 10)) {
                                val tail6 = out.lastOrNull()?.code ?: -1
                                if (tail6 >= 55296 && tail6 <= 56319 && "\\n".length > 0 && !("\\n"[0].code >= 56320 && "\\n"[0].code <= 57343)) {
                                    throw UStringException.UnpairedSurrogate(tail6)
                                }
                                out.append("\\n")
                            } else {
                                if ((c == 13)) {
                                    val tail7 = out.lastOrNull()?.code ?: -1
                                    if (tail7 >= 55296 && tail7 <= 56319 && "\\r".length > 0 && !("\\r"[0].code >= 56320 && "\\r"[0].code <= 57343)) {
                                        throw UStringException.UnpairedSurrogate(tail7)
                                    }
                                    out.append("\\r")
                                } else {
                                    if ((c == 9)) {
                                        val tail8 = out.lastOrNull()?.code ?: -1
                                        if (tail8 >= 55296 && tail8 <= 56319 && "\\t".length > 0 && !("\\t"[0].code >= 56320 && "\\t"[0].code <= 57343)) {
                                            throw UStringException.UnpairedSurrogate(tail8)
                                        }
                                        out.append("\\t")
                                    } else {
                                        if ((c!! < 32)) {
                                            val tail9 = out.lastOrNull()?.code ?: -1
                                            if (tail9 >= 55296 && tail9 <= 56319 && ("\\u" + (c).toUInt().toString(16).uppercase().padStart(4, '0').lowercase()).length > 0 && !(("\\u" + (c).toUInt().toString(16).uppercase().padStart(4, '0').lowercase())[0].code >= 56320 && ("\\u" + (c).toUInt().toString(16).uppercase().padStart(4, '0').lowercase())[0].code <= 57343)) {
                                                throw UStringException.UnpairedSurrogate(tail9)
                                            }
                                            out.append(("\\u" + (c).toUInt().toString(16).uppercase().padStart(4, '0').lowercase()))
                                        } else {
                                            val tail10 = out.lastOrNull()?.code ?: -1
                                            if (c >= 56320 && c <= 57343) {
                                                if (!(tail10 >= 55296 && tail10 <= 56319)) {
                                                    throw UStringException.UnpairedSurrogate(c)
                                                }
                                            } else if (tail10 >= 55296 && tail10 <= 56319) {
                                                throw UStringException.UnpairedSurrogate(tail10)
                                            }
                                            out.append((c).toChar())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        val tail11 = out.lastOrNull()?.code ?: -1
        if (tail11 >= 55296 && tail11 <= 56319 && "\"".length > 0 && !("\""[0].code >= 56320 && "\""[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail11)
        }
        out.append("\"")
    }

    fun shortestRoundTripDigits(magnitude: Float): PlanJsonDigitsAndExponent {
        val d = PlanJsonNumber.decompose(magnitude)
        val f = d.exp2
        val expansion = PlanJsonNumber.dyadicDecimal(d.mantissa, f)
        val exact = PlanJsonNumber.trimZeros(expansion.digits)
        val n = expansion.exponent
        val base = d.mantissa
        val doubled = PlanJsonNumber.timesSmall(base, 2)
        val hi = PlanJsonNumber.dyadicDecimalString(PlanJsonNumber.addDecimal(doubled, "1"), d.exp2 - 1)
        var lo: PlanJsonDigitsAndExponent
        if ((base == "4503599627370496")) {
            lo = PlanJsonNumber.dyadicDecimalString(PlanJsonNumber.decrementDecimal(PlanJsonNumber.timesSmall(base, 4)), d.exp2 - 2)
        } else {
            lo = PlanJsonNumber.dyadicDecimalString(PlanJsonNumber.decrementDecimal(doubled), d.exp2 - 1)
        }
        val inclusive = ((run { val _s = base; val _i = base.length - 1; if (_i >= 0 && _i < _s.length) _s[_i].code else null })!! - 48) % 2 == 0
        var length = 1
        while ((length <= 17)) {
            val keep = run { val _s = exact; val _pos = 0; val _len = length; val _start = if (_pos < 0) maxOf(0, _s.length + _pos) else minOf(_pos, _s.length); if (_len < 0) "" else { val _end = minOf(_s.length, _start + _len); _s.substring(_start, _end) } }
            val up = PlanJsonNumber.incrementDecimal(keep)
            val upN = n + up.length - length
            if ((PlanJsonNumber.inInterval(keep, n, lo, hi, inclusive) || PlanJsonNumber.inInterval(up, upN, lo, hi, inclusive))) {
                val rounded = PlanJsonNumber.roundToSignificant(exact, length)
                return PlanJsonDigitsAndExponent(digits = rounded.digits, exponent = n + rounded.exponent)
            }
            length += 1
        }
        return PlanJsonDigitsAndExponent(digits = exact, exponent = n)
    }

    private fun inInterval(digits: String, n: Int, lo: PlanJsonDigitsAndExponent, hi: PlanJsonDigitsAndExponent, inclusive: Boolean): Boolean {
        val low = PlanJsonNumber.compareDecimal(digits, n, lo.digits, lo.exponent)
        val high = PlanJsonNumber.compareDecimal(digits, n, hi.digits, hi.exponent)
        return (low > 0 || low == 0 && inclusive) && (high < 0 || high == 0 && inclusive)
    }

    private fun canonicalFloatDigits(magnitude: Float, doubleDigits: String): String {
        val d = PlanJsonNumber.decomposeF32(magnitude)
        val exact = (if ((d.mant24 == 0)) "0" else (if ((d.exp2 >= 0)) PlanJsonNumber.timesLong(PlanJsonNumber.twoToThe(d.exp2), d.mant24) else PlanJsonNumber.timesLong(PlanJsonNumber.fiveToThe(-d.exp2), d.mant24)))
        val stripped = PlanJsonNumber.trimZeros(exact)
        if ((stripped.length <= doubleDigits.length)) {
            return doubleDigits
        }
        val rounded = PlanJsonNumber.roundToSignificant(stripped, doubleDigits.length)
        return (if ((rounded.digits.length == doubleDigits.length)) rounded.digits else doubleDigits)
    }

    private fun dyadicDecimal(p: String, f: Int): PlanJsonDigitsAndExponent {
        return PlanJsonNumber.dyadicDecimalString(p, f)
    }

    private fun dyadicDecimalString(p: String, f: Int): PlanJsonDigitsAndExponent {
        val digits = (if ((f < 0)) PlanJsonNumber.multiplyDecimal(PlanJsonNumber.fiveToThe(-f), p) else PlanJsonNumber.multiplyDecimal(PlanJsonNumber.twoToThe(f), p))
        return PlanJsonDigitsAndExponent(digits = digits, exponent = (if ((f < 0)) digits.length + f else digits.length))
    }

    private fun multiplyDecimal(a: String, b: String): String {
        var result = "0"
        var shift = 0
        var i = b.length - 1
        while ((i >= 0)) {
            val digit = (run { val _s = b; val _i = i; if (_i >= 0 && _i < _s.length) _s[_i].code else null })!! - 48
            if ((digit != 0)) {
                var part = PlanJsonNumber.timesSmall(a, digit)
                if ((shift > 0)) {
                    part += PlanJsonNumber.zeros(shift)
                }
                result = PlanJsonNumber.addDecimal(result, part)
            }
            shift++
            i--
        }
        return result
    }

    private fun fiveToThe(k: Int): String {
        if ((PlanJsonNumber.fivePowersBuilder == null)) {
            PlanJsonNumber.fivePowersBuilder = SortedTable.mapBuilder<Int, String>(SortedTable::compareInts)
            PlanJsonNumber.fivePowers = PlanJsonNumber.fivePowersBuilder?.build()
        }
        val cached = PlanJsonNumber.fivePowers?.get(k)
        if ((cached != null)) {
            return cached
        }
        var anchor = 0
        var digits = "1"
        var i = 0
        while ((i < PlanJsonNumber.fivePowers?.size()!!)) {
            if ((PlanJsonNumber.fivePowers?.keyAt(i)!! < k && PlanJsonNumber.fivePowers?.keyAt(i)!! > anchor)) {
                anchor = PlanJsonNumber.fivePowers?.keyAt(i)!!
                digits = PlanJsonNumber.fivePowers?.valueAt(i)!!
            }
            i++
        }
        for (i in anchor until k) {
            digits = PlanJsonNumber.timesSmall(digits, 5)
        }
        PlanJsonNumber.fivePowersBuilder?.put(k, digits)
        PlanJsonNumber.fivePowers = PlanJsonNumber.fivePowersBuilder?.build()
        return digits
    }

    private fun twoToThe(k: Int): String {
        if ((PlanJsonNumber.twoPowersBuilder == null)) {
            PlanJsonNumber.twoPowersBuilder = SortedTable.mapBuilder<Int, String>(SortedTable::compareInts)
            PlanJsonNumber.twoPowers = PlanJsonNumber.twoPowersBuilder?.build()
        }
        val cached = PlanJsonNumber.twoPowers?.get(k)
        if ((cached != null)) {
            return cached
        }
        var anchor = 0
        var digits = "1"
        var i = 0
        while ((i < PlanJsonNumber.twoPowers?.size()!!)) {
            if ((PlanJsonNumber.twoPowers?.keyAt(i)!! < k && PlanJsonNumber.twoPowers?.keyAt(i)!! > anchor)) {
                anchor = PlanJsonNumber.twoPowers?.keyAt(i)!!
                digits = PlanJsonNumber.twoPowers?.valueAt(i)!!
            }
            i++
        }
        for (i in anchor until k) {
            digits = PlanJsonNumber.timesSmall(digits, 2)
        }
        PlanJsonNumber.twoPowersBuilder?.put(k, digits)
        PlanJsonNumber.twoPowers = PlanJsonNumber.twoPowersBuilder?.build()
        return digits
    }

    private fun timesLong(digits: String, factor: Int): String {
        if ((factor == 0)) {
            return "0"
        }
        var result: String? = null
        var shift = 0
        var remaining = factor
        while ((remaining > 0)) {
            val chunk = remaining % 100000000
            remaining = (remaining / 100000000)
            if ((chunk != 0)) {
                var part = PlanJsonNumber.timesSmall(digits, chunk)
                if ((shift > 0)) {
                    part += PlanJsonNumber.zeros(shift)
                }
                result = (if ((result == null)) part else PlanJsonNumber.addDecimal(result!!, part))
            }
            shift += 8
        }
        return (if ((result == null)) "0" else result)
    }

    private fun addDecimal(a: String, b: String): String {
        val out = StringBuilder()
        var i = a.length - 1
        var j = b.length - 1
        var carry = 0
        while ((i >= 0 || j >= 0 || carry > 0)) {
            val sum = ((if ((i >= 0)) (run { val _s = a; val _i = i; if (_i >= 0 && _i < _s.length) _s[_i].code else null })!! - 48 else 0)) + ((if ((j >= 0)) (run { val _s = b; val _i = j; if (_i >= 0 && _i < _s.length) _s[_i].code else null })!! - 48 else 0)) + carry
            val tail12 = out.lastOrNull()?.code ?: -1
            if (48 + sum % 10 >= 56320 && 48 + sum % 10 <= 57343) {
                if (!(tail12 >= 55296 && tail12 <= 56319)) {
                    throw UStringException.UnpairedSurrogate(48 + sum % 10)
                }
            } else if (tail12 >= 55296 && tail12 <= 56319) {
                throw UStringException.UnpairedSurrogate(tail12)
            }
            out.append((48 + sum % 10).toChar())
            carry = (sum / 10)
            i--
            j--
        }
        val tail13 = out.lastOrNull()?.code ?: -1
        if (tail13 >= 55296 && tail13 <= 56319) {
            throw UStringException.UnpairedSurrogate(tail13)
        }
        val text = out.toString()
        return PlanJsonNumber.reverse(text)
    }

    private fun roundToSignificant(exact: String, length: Int): PlanJsonDigitsAndExponent {
        if ((length >= exact.length)) {
            return PlanJsonDigitsAndExponent(digits = exact, exponent = 0)
        }
        val keep = run { val _s = exact; val _pos = 0; val _len = length; val _start = if (_pos < 0) maxOf(0, _s.length + _pos) else minOf(_pos, _s.length); if (_len < 0) "" else { val _end = minOf(_s.length, _start + _len); _s.substring(_start, _end) } }
        val rem = run { val _s = exact; val _pos = length; val _start = if (_pos < 0) maxOf(0, _s.length + _pos) else minOf(_pos, _s.length); _s.substring(_start) }
        var up = false
        if (((run { val _s = rem; val _i = 0; if (_i >= 0 && _i < _s.length) _s[_i].code else null })!! > 53)) {
            up = true
        } else {
            if ((run { val _s = rem; val _i = 0; if (_i >= 0 && _i < _s.length) _s[_i].code else null } == 53)) {
                var tail = 1
                while ((tail < rem.length && run { val _s = rem; val _i = tail; if (_i >= 0 && _i < _s.length) _s[_i].code else null } == 48)) {
                    tail++
                }
                up = tail < rem.length || ((run { val _s = keep; val _i = keep.length - 1; if (_i >= 0 && _i < _s.length) _s[_i].code else null })!! - 48) % 2 != 0
            }
        }
        val rounded = (if ((up)) PlanJsonNumber.incrementDecimal(keep) else keep)
        return PlanJsonDigitsAndExponent(digits = PlanJsonNumber.trimZeros(rounded), exponent = rounded.length - length)
    }

    private fun compareDecimal(a: String, nA: Int, b: String, nB: Int): Int {
        val eA = nA - a.length
        val eB = nB - b.length
        val aa = (if ((eA >= eB)) a + PlanJsonNumber.zeros(eA - eB) else a)
        val bb = (if ((eB >= eA)) b + PlanJsonNumber.zeros(eB - eA) else b)
        if ((aa.length != bb.length)) {
            return aa.length - bb.length
        }
        return PlanJsonNumber.compareDigitStrings(aa, bb)
    }

    private fun timesSmall(digits: String, factor: Int): String {
        val out = StringBuilder()
        var carry = 0
        var i = digits.length - 1
        while ((i >= 0)) {
            val product = ((run { val _s = digits; val _i = i; if (_i >= 0 && _i < _s.length) _s[_i].code else null })!! - 48) * factor + carry
            val tail14 = out.lastOrNull()?.code ?: -1
            if (48 + product % 10 >= 56320 && 48 + product % 10 <= 57343) {
                if (!(tail14 >= 55296 && tail14 <= 56319)) {
                    throw UStringException.UnpairedSurrogate(48 + product % 10)
                }
            } else if (tail14 >= 55296 && tail14 <= 56319) {
                throw UStringException.UnpairedSurrogate(tail14)
            }
            out.append((48 + product % 10).toChar())
            carry = (product / 10)
            i--
        }
        while ((carry > 0)) {
            val tail15 = out.lastOrNull()?.code ?: -1
            if (48 + carry % 10 >= 56320 && 48 + carry % 10 <= 57343) {
                if (!(tail15 >= 55296 && tail15 <= 56319)) {
                    throw UStringException.UnpairedSurrogate(48 + carry % 10)
                }
            } else if (tail15 >= 55296 && tail15 <= 56319) {
                throw UStringException.UnpairedSurrogate(tail15)
            }
            out.append((48 + carry % 10).toChar())
            carry = (carry / 10)
        }
        val tail16 = out.lastOrNull()?.code ?: -1
        if (tail16 >= 55296 && tail16 <= 56319) {
            throw UStringException.UnpairedSurrogate(tail16)
        }
        val text = out.toString()
        return PlanJsonNumber.reverse(text)
    }

    private fun incrementDecimal(digits: String): String {
        val a = digits.chunked(1).toMutableList()
        var i = a.size - 1
        while ((true)) {
            if ((a[i] != "9")) {
                if (a.size <= i) { a.addAll(java.util.Collections.nCopies<String>(i + 1 - a.size, null)) }
                a[i] = (if (((run { val _s = a[i]; val _i = 0; if (_i >= 0 && _i < _s.length) _s[_i].code else null })!! + 1) > 0xFFFF) String(Character.toChars(((run { val _s = a[i]; val _i = 0; if (_i >= 0 && _i < _s.length) _s[_i].code else null })!! + 1))).toString() else ((((run { val _s = a[i]; val _i = 0; if (_i >= 0 && _i < _s.length) _s[_i].code else null })!! + 1)).toChar()).toString())
                return a.joinToString("")
            }
            if (a.size <= i) { a.addAll(java.util.Collections.nCopies<String>(i + 1 - a.size, null)) }
            a[i] = "0"
            if ((i == 0)) {
                return "1" + a.joinToString("")
            }
            i--
        }
    }

    private fun decrementDecimal(digits: String): String {
        val a = digits.chunked(1).toMutableList()
        var i = a.size - 1
        while ((i >= 0 && a[i] == "0")) {
            if (a.size <= i) { a.addAll(java.util.Collections.nCopies<String>(i + 1 - a.size, null)) }
            a[i] = "9"
            i--
        }
        if ((i >= 0)) {
            if (a.size <= i) { a.addAll(java.util.Collections.nCopies<String>(i + 1 - a.size, null)) }
            a[i] = (if (((run { val _s = a[i]; val _i = 0; if (_i >= 0 && _i < _s.length) _s[_i].code else null })!! - 1) > 0xFFFF) String(Character.toChars(((run { val _s = a[i]; val _i = 0; if (_i >= 0 && _i < _s.length) _s[_i].code else null })!! - 1))).toString() else ((((run { val _s = a[i]; val _i = 0; if (_i >= 0 && _i < _s.length) _s[_i].code else null })!! - 1)).toChar()).toString())
        }
        val out = a.joinToString("")
        var start = 0
        while ((start < out.length - 1 && run { val _s = out; val _i = start; if (_i >= 0 && _i < _s.length) _s[_i].code else null } == 48)) {
            start++
        }
        return run { val _s = out; val _pos = start; val _start = if (_pos < 0) maxOf(0, _s.length + _pos) else minOf(_pos, _s.length); _s.substring(_start) }
    }

    private fun decompose(v: Float): PlanJsonDecomposed {
        val f = PlanJsonNumber.decomposeF32(v)
        var mantissa = (f.mant24).toString()
        var exponent = f.exp2
        if ((f.mant24 >= 8388608)) {
            mantissa = PlanJsonNumber.timesLong(mantissa, 536870912)
            exponent -= 29
        }
        while ((mantissa.length < 16 || mantissa.length == 16 && PlanJsonNumber.compareDigitStrings(mantissa, "4503599627370496") < 0)) {
            mantissa = PlanJsonNumber.timesSmall(mantissa, 2)
            exponent--
        }
        return PlanJsonDecomposed(mantissa = mantissa, exp2 = exponent)
    }

    private fun decomposeF32(v: Float): PlanJsonF32Decomposed {
        val bits = FPHelper.floatToI32(v)
        val rawExponent = ((((bits) ushr (23))) and (255))
        val rawMantissa = ((bits) and (8388607))
        if ((rawExponent == 0)) {
            return PlanJsonF32Decomposed(mant24 = rawMantissa, exp2 = -149)
        }
        return PlanJsonF32Decomposed(mant24 = ((rawMantissa) or (8388608)), exp2 = rawExponent - 150)
    }

    private fun compareDigitStrings(a: String, b: String): Int {
        if ((a.length != b.length)) {
            return a.length - b.length
        }
        var i = 0
        while ((i < a.length)) {
            val da = run { val _s = a; val _i = i; if (_i >= 0 && _i < _s.length) _s[_i].code else null }
            val db = run { val _s = b; val _i = i; if (_i >= 0 && _i < _s.length) _s[_i].code else null }
            if ((da != db)) {
                return da!! - db!!
            }
            i++
        }
        return 0
    }

    private fun zeros(n: Int): String {
        var s = ""
        for (i in 0 until n) {
            s += "0"
        }
        return s
    }

    private fun trimZeros(s: String): String {
        var e = s.length
        while ((e > 1 && run { val _s = s; val _i = e - 1; if (_i >= 0 && _i < _s.length) _s[_i].code else null } == 48)) {
            e--
        }
        return run { val _s = s; val _pos = 0; val _len = e; val _start = if (_pos < 0) maxOf(0, _s.length + _pos) else minOf(_pos, _s.length); if (_len < 0) "" else { val _end = minOf(_s.length, _start + _len); _s.substring(_start, _end) } }
    }

    private fun reverse(s: String): String {
        var r = ""
        var i = s.length - 1
        while ((i >= 0)) {
            r += s[i].toString()
            i--
        }
        return r
    }
}

data class PlanJsonDigitsAndExponent(
    var digits: String,
    var exponent: Int
)

fun compare(a: PlanJsonDigitsAndExponent, b: PlanJsonDigitsAndExponent): Int {
    if (a === b) return 0
    var cmp = 0
    cmp = a.digits.compareTo(b.digits)
    if (cmp != 0) return cmp
    cmp = a.exponent.compareTo(b.exponent)
    if (cmp != 0) return cmp
    return 0
}

data class PlanJsonDecomposed(
    var mantissa: String,
    var exp2: Int
)

fun compare(a: PlanJsonDecomposed, b: PlanJsonDecomposed): Int {
    if (a === b) return 0
    var cmp = 0
    cmp = a.mantissa.compareTo(b.mantissa)
    if (cmp != 0) return cmp
    cmp = a.exp2.compareTo(b.exp2)
    if (cmp != 0) return cmp
    return 0
}

data class PlanJsonF32Decomposed(
    var mant24: Int,
    var exp2: Int
)

fun compare(a: PlanJsonF32Decomposed, b: PlanJsonF32Decomposed): Int {
    if (a === b) return 0
    var cmp = 0
    cmp = a.mant24.compareTo(b.mant24)
    if (cmp != 0) return cmp
    cmp = a.exp2.compareTo(b.exp2)
    if (cmp != 0) return cmp
    return 0
}
