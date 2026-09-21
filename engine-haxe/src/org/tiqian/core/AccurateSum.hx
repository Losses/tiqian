package org.tiqian.core;

import std.Functional;

using std.Functional;

/**
 * Sums binary32 operands with one binary64 accumulator, the shape the Kotlin
 * reference writes as `values.sumOf { it.advance.toDouble() }.toFloat()`.
 *
 * The reference keeps its accumulator in Double and narrows the finished sum
 * once, so a line of equal binary32 advances reaches its exact total
 * (thirty 16.0 advances make 480) instead of rounding on every step, which
 * a binary32 accumulator turns into 479.99982.
 *
 * `std.Functional.sumOfFloat` carries that binary64 accumulator, but the
 * collection-pipeline expansion rewrites a call sitting in a direct statement
 * position into a binary32 loop. The accumulator therefore runs inside a
 * local function, which the expansion leaves alone, and every caller reaches
 * it through this non-inline entry point.
 */
class AccurateSum {
    /** The exact sum of [values], narrowed to binary32 once. */
    public static function of(values:Array<Float>):Float {
        function accumulate():Float {
#if float_precision
            return values.sumOfFloat(value -> value);
#else
            var total = 0.0;
            for (value in values) total += value;
            return total;
#end
        }
        return accumulate();
    }
}
