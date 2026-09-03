package;

import js.Syntax;
import runtime.SortedTable;

/**
 * Binds the boring runtime sorted tables to the std.SortedSet and
 * std.SortedMap extern names for the stage A test runner. Every stage-one
 * set and map value is a resident SortedSetTable or SortedMapTable, so
 * Std.string prints the ruled text from the boring runtime and lookup
 * semantics match the generated targets. Stage-one keys are integers or
 * ASCII range keys, so one comparator serves both by dispatching on the
 * runtime key type.
 */
class SortedTablesOracle {
    public static function install():Void {
        Syntax.code("globalThis.std = globalThis.std || {}; globalThis.std.SortedSet = { builder: function() { return {0}.setBuilder({1}); } }; globalThis.std.SortedMap = { builder: function() { return {0}.mapBuilder({1}); } };",
            SortedTable, compareKeys);
    }

    static function compareKeys(left:Dynamic, right:Dynamic):Int {
        if (Std.isOfType(left, String)) {
            return SortedTable.compareStrings(left, right);
        }
        return SortedTable.compareInts(Std.int(left), Std.int(right));
    }
}
