package;

import js.Syntax;

/**
 * Binds sorted key-indexed array implementations to the boring std
 * SortedSet/SortedMap names for the stage A test runner. Keys are unique and
 * kept in ascending order, matching the BTreeMap semantics of the boring
 * runtime tables: builder put overwrites an existing key, has/get do a binary
 * search, and at/keyAt/valueAt index in ascending key order.
 */
class SortedTablesOracle {
    public static function install():Void {
        Syntax.code("globalThis.std = globalThis.std || {}; globalThis.std.SortedSet = {0}; globalThis.std.SortedMap = {1};", SortedSetOracle, SortedMapOracle);
    }
}

class SortedSetOracle {
    private final keys:Array<Int>;

    public function new(keys:Array<Int>) {
        this.keys = keys;
    }

    public function has(key:Int):Bool {
        return indexOf(key) >= 0;
    }

    public function size():Int {
        return keys.length;
    }

    public function at(index:Int):Int {
        return keys[index];
    }

    private function indexOf(key:Int):Int {
        var low = 0;
        var high = keys.length;
        while (low < high) {
            final mid = (low + high) >> 1;
            if (keys[mid] < key) {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        return low < keys.length && keys[low] == key ? low : -1;
    }

    public static function builder():SortedSetBuilderOracle {
        return new SortedSetBuilderOracle();
    }
}

class SortedSetBuilderOracle {
    public function new() {}

    private final pending:Array<Int> = [];

    public function put(key:Int):Void {
        pending.push(key);
    }

    public function build():SortedSetOracle {
        final sorted = pending.copy();
        sorted.sort(function(a, b) return a - b);
        final unique:Array<Int> = [];
        var i = 0;
        while (i < sorted.length) {
            if (unique.length == 0 || unique[unique.length - 1] != sorted[i]) {
                unique.push(sorted[i]);
            }
            i += 1;
        }
        return new SortedSetOracle(unique);
    }
}

class SortedMapOracle<V> {
    private final keys:Array<Int>;
    private final values:Array<V>;

    public function new(keys:Array<Int>, values:Array<V>) {
        this.keys = keys;
        this.values = values;
    }

    public function get(key:Int):Null<V> {
        final index = indexOf(key);
        return index < 0 ? null : values[index];
    }

    public function has(key:Int):Bool {
        return indexOf(key) >= 0;
    }

    public function size():Int {
        return keys.length;
    }

    public function keyAt(index:Int):Int {
        return keys[index];
    }

    public function valueAt(index:Int):V {
        return values[index];
    }

    private function indexOf(key:Int):Int {
        var low = 0;
        var high = keys.length;
        while (low < high) {
            final mid = (low + high) >> 1;
            if (keys[mid] < key) {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        return low < keys.length && keys[low] == key ? low : -1;
    }

    public static function builder<V>():SortedMapBuilderOracle<V> {
        return new SortedMapBuilderOracle<V>();
    }
}

class SortedMapBuilderOracle<V> {
    public function new() {}

    private final keys:Array<Int> = [];
    private final values:Array<V> = [];

    public function put(key:Int, value:V):Void {
        var low = 0;
        var high = keys.length;
        while (low < high) {
            final mid = (low + high) >> 1;
            if (keys[mid] < key) {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        if (low < keys.length && keys[low] == key) {
            values[low] = value;
        } else {
            keys.insert(low, key);
            values.insert(low, value);
        }
    }

    public function get(key:Int):Null<V> {
        var low = 0;
        var high = keys.length;
        while (low < high) {
            final mid = (low + high) >> 1;
            if (keys[mid] < key) {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        return low < keys.length && keys[low] == key ? values[low] : null;
    }

    public function build():SortedMapOracle<V> {
        return new SortedMapOracle<V>(keys.copy(), values.copy());
    }
}
