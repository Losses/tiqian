// Stage-A JS runtime binding for std.Functional.sortedBy.
// The boring samples declare @:native("__functional_shim") externs; the Haxe
// test bundle must install the global itself, the same way boring's own test
// collector does. Contract: ascending by key, stable on equal keys via index
// tiebreak. Keys are Int in current use; extend per adopted member or key
// type. The core-kotlin gate emits Kotlin sortBy natively and never runs the oracle.
class FunctionalOracle {
    public static function install():Void {
        js.Syntax.code("globalThis.__functional_shim = { sortedBy: {0} }", sortedBy);
    }

    private static function sortedBy(arr:Array<Dynamic>, keyFn:(Dynamic) -> Int):Array<Dynamic> {
        final decorated:Array<{item:Dynamic, key:Int, idx:Int}> = [];
        var i:Int = 0;
        while (i < arr.length) {
            decorated.push({item: arr[i], key: keyFn(arr[i]), idx: i});
            i++;
        }
        decorated.sort(function(a:{item:Dynamic, key:Int, idx:Int}, b:{item:Dynamic, key:Int, idx:Int}):Int {
            if (a.key < b.key) return -1;
            if (a.key > b.key) return 1;
            return a.idx - b.idx;
        });
        final out:Array<Dynamic> = [];
        i = 0;
        while (i < decorated.length) {
            out.push(decorated[i].item);
            i++;
        }
        return out;
    }
}
