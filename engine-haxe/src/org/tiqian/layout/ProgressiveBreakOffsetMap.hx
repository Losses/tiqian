package org.tiqian.layout;

import std.StringBuf;
import org.tiqian.layout.ProgressiveBreakDecisions.ProgressiveBreakOpportunity;

/**
 * Insertion-ordered table of progressive technical break opportunities, keyed
 * by the source offset the break sits after.
 *
 * The legacy stage holds this table in a Kotlin `Map`
 * (`engine/src/commonMain/kotlin/org/tiqian/layout/ParagraphShapingStage.kt:384`
 * `mutableMapOf<Int, ProgressiveBreakOpportunity>()`, returned at line 750
 * through `progressiveBreakOffsets.toMap()`), so its iteration order and its
 * rendered `toString` follow the order the tiers recorded the offsets.
 * `std.SortedMap` reorders entries by offset, which changes the rendered
 * table. The port keeps the offsets and the opportunities in two parallel
 * arrays in recording order and renders them in that order, matching the
 * Kotlin `Map` text `{key=value, key=value}`.
 *
 * Haxe's `haxe.ds.Map` is not an option: style rule V13
 * (`docs/specs/style/01-haxe-style-standard.md`) rejects it.
 */
class ProgressiveBreakOffsetMap {
    public final offsets:Array<Int>;
    public final opportunities:Array<ProgressiveBreakOpportunity>;

    public function new(offsets:Array<Int>, opportunities:Array<ProgressiveBreakOpportunity>) {
        this.offsets = offsets;
        this.opportunities = opportunities;
    }

    public function size():Int {
        return offsets.length;
    }

    public function offsetAt(index:Int):Int {
        return offsets[index];
    }

    public function opportunityAt(index:Int):ProgressiveBreakOpportunity {
        return opportunities[index];
    }

    public function toString():String {
        final out = new StringBuf();
        out.add("{");
        for (i in 0...offsets.length) {
            if (i > 0)
                out.add(", ");
            out.add(Std.string(offsets[i]));
            out.add("=");
            out.add(opportunities[i].toString());
        }
        out.add("}");
        return out.toString();
    }
}
