package org.tiqian.protocol;

/**
 * The canonical encoder's cross-language golden vectors (ADR 0052). The hex
 * strings pin the bytes the TypeScript lane asserts in
 * platforms/web/server/core/test/canonical.test.ts and the Rust lane asserts
 * in canonical.rs unit tests: the canonical bytes are the cache identity, so
 * one bit of divergence would split every cache entry.
 */
class CanonicalTest {
    @:test
    public static function goldenVectorsMatchTheSharedBytes():Void {
        final recorder = new org.tiqian.test.trace.TestTraceRecorder("CanonicalTest");
        recorder.section("goldenVectorsMatchTheSharedBytes");
        assertHex(recorder, "vector0", "54514353010006000000e4b8ade6968701000000000000624000000000000000000000000000000000",
            encode(WObj([
                { name: "key", value: WStr("p-1") },
                { name: "text", value: WStr("中文") },
                { name: "maxWidthPx", value: WNum(144) },
            ]), Canonical.KIND_SNAPSHOT));
        assertHex(recorder, "vector1", "54514353010106000000e4b8ade6968700000000000000000000000000000000",
            encode(WObj([
                { name: "key", value: WStr("fc-1") },
                { name: "text", value: WStr("中文") },
            ]), Canonical.KIND_CONTRACT));
        assertHex(recorder, "vector2", FULL_VECTOR_HEX,
            encode(WObj([
                { name: "key", value: WStr("p-2") },
                { name: "text", value: WStr("中文字排版") },
                { name: "maxWidthPx", value: WNum(144) },
                { name: "semantics", value: WArr([
                    WObj([
                        { name: "tagName", value: WStr("a") },
                        { name: "start", value: WNum(2) },
                        { name: "end", value: WNum(4) },
                        { name: "attributes", value: WObj([
                            { name: "href", value: WStr("https://example.com") },
                            { name: "class", value: WStr("link") },
                        ]) },
                        { name: "order", value: WNum(1) },
                    ]),
                    WObj([
                        { name: "tagName", value: WStr("em") },
                        { name: "start", value: WNum(0) },
                        { name: "end", value: WNum(1) },
                    ]),
                ]) },
                { name: "textSpans", value: WArr([
                    WObj([
                        { name: "start", value: WNum(0) },
                        { name: "end", value: WNum(2) },
                        { name: "fontFamilies", value: WArr([WStr("Dela Gothic One")]) },
                        { name: "fontSizePx", value: WNum(18) },
                        { name: "fontWeight", value: WNum(400) },
                        { name: "italic", value: WBool(true) },
                        { name: "baselineShiftPx", value: WNum(-0.5) },
                    ]),
                    WObj([
                        { name: "start", value: WNum(2) },
                        { name: "end", value: WNum(4) },
                        { name: "fontFamilies", value: WArr([]) },
                    ]),
                ]) },
                { name: "inlineBoxes", value: WArr([
                    WObj([
                        { name: "start", value: WNum(1) },
                        { name: "end", value: WNum(2) },
                        { name: "inlineStartPx", value: WNum(8) },
                        { name: "inlineEndPx", value: WNum(4) },
                        { name: "outerSpacing", value: WStr("Source") },
                    ]),
                    WObj([
                        { name: "start", value: WNum(3) },
                        { name: "end", value: WNum(4) },
                    ]),
                ]) },
                { name: "sourceBoundaries", value: WArr([WNum(0), WNum(18), WNum(36)]) },
            ]), Canonical.KIND_SNAPSHOT));
    }

    @:test
    public static function looseCoercionsCarryTheJsonLane():Void {
        final recorder = new org.tiqian.test.trace.TestTraceRecorder("CanonicalTest");
        recorder.section("looseCoercionsCarryTheJsonLane");
        assertHex(recorder, "vector3", "5451435301000800000020636f6572636520010000000000106240010000000501000000000000f03f010000000000000040010000006902130000005b5b2261222c2231225d2c5b2262222c325d5d010000000001000000000000000001000000000000f03f010000000401000000000000000001000000000000f03f01000000370200000000000000000008400000000000000000",
            encode(WObj([
                { name: "key", value: WStr("p-3") },
                { name: "text", value: WStr(" coerce ") },
                { name: "maxWidthPx", value: WStr("144.5") },
                { name: "semantics", value: WArr([
                    WObj([
                        { name: "tagName", value: WStr("i") },
                        { name: "start", value: WStr("1") },
                        { name: "end", value: WNum(2) },
                        { name: "attributes", value: WArr([
                            WArr([WStr("a"), WStr("1")]),
                            WArr([WStr("b"), WNum(2)]),
                        ]) },
                    ]),
                ]) },
                { name: "textSpans", value: WArr([
                    WObj([
                        { name: "start", value: WNum(0) },
                        { name: "end", value: WNum(1) },
                        { name: "italic", value: WStr("no") },
                    ]),
                ]) },
                { name: "inlineBoxes", value: WArr([
                    WObj([
                        { name: "start", value: WNum(0) },
                        { name: "end", value: WNum(1) },
                        { name: "outerSpacing", value: WNum(7) },
                    ]),
                ]) },
                { name: "sourceBoundaries", value: WArr([WStr("3"), WNull]) },
            ]), Canonical.KIND_SNAPSHOT));
    }

    @:test
    public static function nonArraySemanticsIsTheNamedIssue():Void {
        final recorder = new org.tiqian.test.trace.TestTraceRecorder("CanonicalTest");
        recorder.section("nonArraySemanticsIsTheNamedIssue");
        final result = encode(WObj([
            { name: "text", value: WStr("a") },
            { name: "semantics", value: WStr("no") },
        ]), Canonical.KIND_SNAPSHOT);
        switch (result) {
            case CErr(issue):
                org.tiqian.test.trace.TracedAssertions.assertEqualsString("InvalidSnapshotSemantics", issue);
            case COk(_):
                recorder.record("encode unexpectedly succeeded");
                org.tiqian.test.trace.TracedAssertions.assertTrue(false, "non-array semantics must be the named issue");
        }
    }

    @:test
    public static function nonFiniteAndMinusZeroCollapse():Void {
        final recorder = new org.tiqian.test.trace.TestTraceRecorder("CanonicalTest");
        recorder.section("nonFiniteAndMinusZeroCollapse");
        final withNaN = encode(WObj([
            { name: "text", value: WStr("a") },
            { name: "maxWidthPx", value: WNum(Math.NaN) },
        ]), Canonical.KIND_SNAPSHOT);
        final withoutWidth = encode(WObj([
            { name: "text", value: WStr("a") },
        ]), Canonical.KIND_SNAPSHOT);
        assertHex(recorder, "withNaN", bytesToHex(bytesOf(withoutWidth)), withNaN);
        final minusZero = encode(WObj([
            { name: "text", value: WStr("a") },
            { name: "maxWidthPx", value: WNum(-0.0) },
        ]), Canonical.KIND_SNAPSHOT);
        assertHex(recorder, "minusZero", bytesToHex(bytesOf(withoutWidth)), minusZero);
    }

    static function bytesOf(result:EncodeResult):haxe.io.Bytes {
        return switch (result) {
            case COk(bytes): bytes;
            case CErr(issue): throw "unexpected issue " + issue;
        };
    }

    static function assertHex(recorder:org.tiqian.test.trace.TestTraceRecorder, label:String, expected:String, result:EncodeResult):Void {
        recorder.record(label);
        org.tiqian.test.trace.TracedAssertions.assertEqualsString(expected, bytesToHex(bytesOf(result)), label);
    }

    static function bytesToHex(bytes:haxe.io.Bytes):String {
        final digits = "0123456789abcdef";
        final out = new StringBuf();
        for (index in 0...bytes.length) {
            final byte = bytes.get(index);
            out.addChar(digits.charCodeAt((byte >> 4) & 15));
            out.addChar(digits.charCodeAt(byte & 15));
        }
        return out.toString();
    }

    static final FULL_VECTOR_HEX:String = "5451435301000f000000e4b8ade69687e5ad97e68e92e7898801000000000000624002000000070100000000000000400100000000000010400100000061000000000000f03f010200000004000000687265661300000068747470733a2f2f6578616d706c652e636f6d05000000636c617373040000006c696e6b0401000000000000000001000000000000f03f02000000656d020000001f010000000000000000010000000000000040010000000f00000044656c6120476f74686963204f6e650000000000003240000000000000794001000000000000e0bf0101000000000000004001000000000000104000000000020000000701000000000000f03f0100000000000000400000000000002040000000000000104006000000536f757263650001000000000000084001000000000000104003000000000000000000000000000000000032400000000000004240";
}
