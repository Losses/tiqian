package org.tiqian.test;

class TestHelpers {
    public static function f32Literal(value:Float):Float {
        return haxe.io.FPHelper.i32ToFloat(haxe.io.FPHelper.floatToI32(value));
    }

    public static function f32Bits(bits:Int):Float {
        return haxe.io.FPHelper.i32ToFloat(bits);
    }

    public static function surrogateText(codeUnits:Array<Int>):String {
        // One fromCharCode per unit. Merging a surrogate pair into a single
        // scalar shortens the text wherever the target's fromCharCode builds a
        // 16-bit character: the Kotlin backend renders String.fromCharCode(x)
        // as (x).toChar(), which keeps the low sixteen bits, so the merged
        // scalar loses the high surrogate and every boundary expectation that
        // counts units (SourceInteractionBoundariesCoverageTest, CoreBoundaryTest,
        // LayoutQueriesTest and its residual coverage) stops matching.
        var output = "";
        var index = 0;
        while (index < codeUnits.length) {
            output += String.fromCharCode(codeUnits[index]);
            index += 1;
        }
        return output;
    }
}
