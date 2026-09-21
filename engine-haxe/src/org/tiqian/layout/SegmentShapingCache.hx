package org.tiqian.layout;

import org.tiqian.core.TextRange;
import std.StringBuf;
import org.tiqian.shaping.TextShaper.ShapingResult;

/**
 * Insertion-ordered keyed table of shaped text segments, keyed by the source
 * range each segment was shaped from.
 *
 * The legacy stage holds this table in a Kotlin `Map`
 * (`engine/src/commonMain/kotlin/org/tiqian/layout/ParagraphShapingStage.kt:147`
 * `cachedSegmentShaping.toMutableMap()`, returned at line 751 through
 * `segmentShapingCache.toMap()`), so its iteration order and its rendered
 * `toString` follow insertion order. `std.SortedMap` reorders entries by
 * key, which changes both the re-shaping order of a cached table and the
 * rendered cache. The port therefore keeps the keys and the values in two
 * parallel arrays in insertion order and renders them in that same order,
 * matching the Kotlin `Map` text `{key=value, key=value}`.
 *
 * Haxe's `haxe.ds.Map` is not an option: style rule V13
 * (`docs/specs/style/01-haxe-style-standard.md`) rejects it.
 */
class SegmentShapingCache {
    public final keys:Array<TextRange>;
    public final values:Array<ShapingResult>;

    public function new(keys:Array<TextRange>, values:Array<ShapingResult>) {
        this.keys = keys;
        this.values = values;
    }

    public function size():Int {
        return keys.length;
    }

    public function keyAt(index:Int):TextRange {
        return keys[index];
    }

    public function valueAt(index:Int):ShapingResult {
        return values[index];
    }

    public function get(range:TextRange):Null<ShapingResult> {
        for (i in 0...keys.length) {
            if (keys[i].start == range.start && keys[i].end == range.end)
                return values[i];
        }
        return null;
    }

    public function toString():String {
        final out = new StringBuf();
        out.add("{");
        for (i in 0...keys.length) {
            if (i > 0)
                out.add(", ");
            out.add(keys[i].toString());
            out.add("=");
            out.add(values[i].toString());
        }
        out.add("}");
        return out.toString();
    }
}
