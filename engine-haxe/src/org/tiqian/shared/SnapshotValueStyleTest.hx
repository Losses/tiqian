package org.tiqian.shared;

import org.tiqian.test.trace.TracedAssertions;

/**
 * Golden pins for the value-style class generation and internment, taken
 * from the hand-written TypeScript lane (prepared-dom.ts snapshotValueStyleClass
 * and runtimeValueStyleKey) and the Rust engine's to_string_36 assertions
 * (snapshot_bundle.rs). The same literals appear in no other place; a change
 * to the fold must move every pin here, item by item, or the browser and the
 * Node precompute drift.
 */
class SnapshotValueStyleTest {
    @:test
    public static function snapshotClassMatchesJsBase36():Void {
        TracedAssertions.assertEqualsString("tqv-0", SnapshotValueStyle.snapshotValueStyleClass(0), "index 0");
        TracedAssertions.assertEqualsString("tqv-9", SnapshotValueStyle.snapshotValueStyleClass(9), "index 9");
        TracedAssertions.assertEqualsString("tqv-a", SnapshotValueStyle.snapshotValueStyleClass(10), "index 10");
        TracedAssertions.assertEqualsString("tqv-z", SnapshotValueStyle.snapshotValueStyleClass(35), "index 35");
        TracedAssertions.assertEqualsString("tqv-10", SnapshotValueStyle.snapshotValueStyleClass(36), "index 36");
        TracedAssertions.assertEqualsString("tqv-1z", SnapshotValueStyle.snapshotValueStyleClass(71), "index 71");
        TracedAssertions.assertEqualsString("tqv-100", SnapshotValueStyle.snapshotValueStyleClass(1296), "index 1296");
    }

    @:test
    public static function unsignedBase36MatchesRustToString36():Void {
        TracedAssertions.assertEqualsString("0", SnapshotValueStyle.toBase36Unsigned(0), "0");
        TracedAssertions.assertEqualsString("9", SnapshotValueStyle.toBase36Unsigned(9), "9");
        TracedAssertions.assertEqualsString("a", SnapshotValueStyle.toBase36Unsigned(10), "10");
        TracedAssertions.assertEqualsString("z", SnapshotValueStyle.toBase36Unsigned(35), "35");
        TracedAssertions.assertEqualsString("10", SnapshotValueStyle.toBase36Unsigned(36), "36");
        TracedAssertions.assertEqualsString("1z", SnapshotValueStyle.toBase36Unsigned(71), "71");
        TracedAssertions.assertEqualsString("100", SnapshotValueStyle.toBase36Unsigned(1296), "1296");
        // The two FNV seeds, unsigned. 0x811c9dc5 is a negative Int literal,
        // so this also pins the unsigned conversion of the high-bit pattern.
        TracedAssertions.assertEqualsString("ztntfp", SnapshotValueStyle.toBase36Unsigned(0x811c9dc5), "fnv offset basis");
        TracedAssertions.assertEqualsString("9zloz", SnapshotValueStyle.toBase36Unsigned(0x01000193), "fnv prime");
    }

    @:test
    public static function runtimeKeyMatchesPreparedDom():Void {
        TracedAssertions.assertEqualsString("ztntfp9zloz", SnapshotValueStyle.runtimeValueStyleKey(""), "empty");
        TracedAssertions.assertEqualsString("h66u19128vzvp", SnapshotValueStyle.runtimeValueStyleKey("color:red"), "color:red");
        TracedAssertions.assertEqualsString("1llh9akp4u4k", SnapshotValueStyle.runtimeValueStyleKey("letter-spacing:0.5px"), "letter-spacing:0.5px");
        TracedAssertions.assertEqualsString("amwry4d1zyhi", SnapshotValueStyle.runtimeValueStyleKey("margin-right:2px"), "margin-right:2px");
        TracedAssertions.assertEqualsString("dl16ta1ep87no", SnapshotValueStyle.runtimeValueStyleKey("font-family:Arial"), "font-family:Arial");
        TracedAssertions.assertEqualsString("1s6sor8mafpnq", SnapshotValueStyle.runtimeValueStyleKey("letter-spacing:0.25em"), "letter-spacing:0.25em");
    }

    @:test
    public static function runtimeClassWrapsKey():Void {
        TracedAssertions.assertEqualsString("tqvr-h66u19128vzvp", SnapshotValueStyle.runtimeValueStyleClass("h66u19128vzvp"), "color:red key");
    }

    @:test
    public static function internerReturnsFirstSeenIndex():Void {
        final reg = new ValueStyleInterner();
        TracedAssertions.assertEquals(0, reg.intern("color:red"), "first");
        TracedAssertions.assertEquals(1, reg.intern("letter-spacing:0.5px"), "second");
        TracedAssertions.assertEquals(0, reg.intern("color:red"), "repeat");
        TracedAssertions.assertEquals(2, reg.size(), "size");
        TracedAssertions.assertEqualsString("color:red", reg.at(0), "at 0");
        TracedAssertions.assertEqualsString("letter-spacing:0.5px", reg.at(1), "at 1");
    }
}
