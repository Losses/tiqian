package org.tiqian.protocol;

import haxe.io.Bytes;
import haxe.io.BytesBuffer;
import haxe.io.FPHelper;

/**
 * The canonical byte form of one submission (ADR 0052), single-sourced in
 * Haxe and generated for TypeScript and Rust. The bytes are the hash
 * preimage of the Paragraph and FontContracts cache layers and the content
 * encoding of the binary bridge: hashing, sending and resupplying all
 * consume the same form, so every platform agrees on one identity per
 * input. Golden vectors live in CanonicalTest and pin the same hex strings
 * the platform-side unit tests carry.
 *
 * The identity contract is "decode(encode(x)) reads exactly like the JSON
 * lane's `JSON.stringify` round trip": every field is carried the way
 * `JSON.stringify` would carry it. Non-finite numbers become absent fields
 * and both zero signs collapse to +0. The caller's logical `key` is
 * deliberately absent. The digest itself stays platform-side (node:crypto
 * on TypeScript, the sha2 crate on Rust): only the preimage crosses this
 * module.
 */
class Canonical {
    /** The canonical form's magic and version, shared with every encoder. */
    public static final MAGIC:Bytes = Bytes.ofString("TQCS");
    public static inline var VERSION:Int = 1;
    /** Snapshot paragraph submission: carries `maxWidthPx`. */
    public static inline var KIND_SNAPSHOT:Int = 0;
    /** Font contract submission: the capture width is derived in Rust. */
    public static inline var KIND_CONTRACT:Int = 1;

    static inline var SEM_ATTRS:Int = 0x01;
    static inline var SEM_ORDER:Int = 0x02;
    static inline var SEM_TAG_NAME:Int = 0x04;

    static inline var SPAN_FAMILIES:Int = 0x01;
    static inline var SPAN_FONT_SIZE_PX:Int = 0x02;
    static inline var SPAN_FONT_WEIGHT:Int = 0x04;
    static inline var SPAN_ITALIC:Int = 0x08;
    static inline var SPAN_BASELINE_SHIFT_PX:Int = 0x10;

    static inline var BOX_INLINE_START_PX:Int = 0x01;
    static inline var BOX_INLINE_END_PX:Int = 0x02;
    static inline var BOX_OUTER_SPACING:Int = 0x04;

    /**
     * Encodes one wire input into its canonical bytes. `kind` selects the
     * snapshot or contract form; snapshot inputs carry `maxWidthPx`,
     * contract inputs never do. A named issue comes back as Err; the
     * platform adapters raise it in their own error type.
     */
    public static function encode(input:WireValue, kind:Int):EncodeResult {
        final writer = new Writer();
        final text = member(input, "text");
        writer.str(text == WNull ? "" : JsCoerce.toString(text));
        if (kind == KIND_SNAPSHOT) {
            numberField(writer, numberMember(input, "maxWidthPx"));
        }
        final semantics = encodeSemantics(writer, member(input, "semantics"));
        if (semantics != null) {
            return CErr(semantics);
        }
        final spans = encodeTextSpans(writer, member(input, "textSpans"));
        if (spans != null) {
            return CErr(spans);
        }
        final boxes = encodeInlineBoxes(writer, member(input, "inlineBoxes"));
        if (boxes != null) {
            return CErr(boxes);
        }
        // A non-array reads as absent boundaries, the capture loop's own rule.
        final rawBoundaries = member(input, "sourceBoundaries");
        final boundaries = switch (rawBoundaries) {
            case WArr(items): items;
            default: new Array<WireValue>();
        };
        writer.u32(boundaries.length);
        for (boundary in boundaries) {
            final value = canonicalF64(JsCoerce.toNumber(boundary));
            // Non-finite boundaries keep the JSON lane's value: null, which
            // reads as zero through the loose number coercion.
            writer.f64(value == null ? 0.0 : value);
        }
        return COk(writer.finish());
    }

    /** The coalesced member: absent and null both read as WNull. */
    public static function member(input:WireValue, name:String):WireValue {
        return switch (input) {
            case WObj(fields):
                for (field in fields) {
                    if (field.name == name) {
                        return field.value;
                    }
                }
                WNull;
            default: WNull;
        }
    }

    /**
     * A number the way `JSON.stringify` carries it: non-finite values drop
     * out (they serialize as null, which readers coalesce away) and both
     * zero signs collapse to +0.
     */
    static function canonicalF64(value:Float):Null<Float> {
        if (!Math.isFinite(value)) {
            return null;
        }
        return value == 0.0 ? 0.0 : value;
    }

    /** `Number(member)` when the member survives, absent otherwise. */
    static function numberMember(input:WireValue, name:String):Null<Float> {
        final resolved = member(input, name);
        return resolved == WNull ? null : canonicalF64(JsCoerce.toNumber(resolved));
    }

    /** An optional numeric field: present flag plus the f64 bytes. */
    static function numberField(writer:Writer, value:Null<Float>):Void {
        if (value == null) {
            writer.u8(0);
        } else {
            writer.u8(1);
            writer.f64(value);
        }
    }

    /** One list value: absent reads as empty, a non-array is a named issue. */
    static function listMember(value:WireValue):ListShape {
        return switch (value) {
            case WNull: LAbsent;
            case WArr(items): LArr(items);
            default: LBad;
        }
    }

    static function encodeSemantics(writer:Writer, value:WireValue):String {
        final items = switch (listMember(value)) {
            case LAbsent: new Array<WireValue>();
            case LArr(items): items;
            case LBad: return "InvalidSnapshotSemantics";
        };
        writer.u32(items.length);
        for (span in items) {
            final attributes = member(span, "attributes");
            final order = numberMember(span, "order");
            final tagName = member(span, "tagName");
            var flags = 0;
            if (attributes != WNull) {
                flags |= SEM_ATTRS;
            }
            if (order != null) {
                flags |= SEM_ORDER;
            }
            if (tagName != WNull) {
                flags |= SEM_TAG_NAME;
            }
            writer.u8(flags);
            numberField(writer, numberMember(span, "start"));
            numberField(writer, numberMember(span, "end"));
            if (tagName != WNull) {
                writer.str(JsCoerce.toString(tagName));
            }
            if (order != null) {
                writer.f64(order);
            }
            if (attributes != WNull) {
                encodeAttributes(writer, attributes);
            }
        }
        return null;
    }

    static function encodeTextSpans(writer:Writer, value:WireValue):String {
        final items = switch (listMember(value)) {
            case LAbsent: new Array<WireValue>();
            case LArr(items): items;
            case LBad: return "InvalidSnapshotTextSpans";
        };
        writer.u32(items.length);
        for (span in items) {
            final families = switch (member(span, "fontFamilies")) {
                case WArr(list):
                    final names = new Array<String>();
                    for (item in list) {
                        names.push(JsCoerce.toString(item));
                    }
                    names;
                default: null;
            };
            final fontSizePx = numberMember(span, "fontSizePx");
            final fontWeight = numberMember(span, "fontWeight");
            final italic = switch (member(span, "italic")) {
                case WBool(inner): inner;
                default: null;
            };
            final baselineShiftPx = numberMember(span, "baselineShiftPx");
            var flags = 0;
            if (families != null) {
                flags |= SPAN_FAMILIES;
            }
            if (fontSizePx != null) {
                flags |= SPAN_FONT_SIZE_PX;
            }
            if (fontWeight != null) {
                flags |= SPAN_FONT_WEIGHT;
            }
            if (italic != null) {
                flags |= SPAN_ITALIC;
            }
            if (baselineShiftPx != null) {
                flags |= SPAN_BASELINE_SHIFT_PX;
            }
            writer.u8(flags);
            numberField(writer, numberMember(span, "start"));
            numberField(writer, numberMember(span, "end"));
            if (families != null) {
                writer.u32(families.length);
                for (name in families) {
                    writer.str(name);
                }
            }
            if (fontSizePx != null) {
                writer.f64(fontSizePx);
            }
            if (fontWeight != null) {
                writer.f64(fontWeight);
            }
            if (italic != null) {
                writer.u8(italic ? 1 : 0);
            }
            if (baselineShiftPx != null) {
                writer.f64(baselineShiftPx);
            }
        }
        return null;
    }

    static function encodeInlineBoxes(writer:Writer, value:WireValue):String {
        final items = switch (listMember(value)) {
            case LAbsent: new Array<WireValue>();
            case LArr(items): items;
            case LBad: return "InvalidSnapshotInlineBoxes";
        };
        writer.u32(items.length);
        for (item in items) {
            final inlineStartPx = numberMember(item, "inlineStartPx");
            final inlineEndPx = numberMember(item, "inlineEndPx");
            final outerSpacing = member(item, "outerSpacing");
            var flags = 0;
            if (inlineStartPx != null) {
                flags |= BOX_INLINE_START_PX;
            }
            if (inlineEndPx != null) {
                flags |= BOX_INLINE_END_PX;
            }
            if (outerSpacing != WNull) {
                flags |= BOX_OUTER_SPACING;
            }
            writer.u8(flags);
            numberField(writer, numberMember(item, "start"));
            numberField(writer, numberMember(item, "end"));
            if (inlineStartPx != null) {
                writer.f64(inlineStartPx);
            }
            if (inlineEndPx != null) {
                writer.f64(inlineEndPx);
            }
            if (outerSpacing != WNull) {
                writer.str(JsCoerce.toString(outerSpacing));
            }
        }
        return null;
    }

    /**
     * Attributes keep their two wire shapes: an object becomes its string
     * pairs in insertion order, an array rides as its JSON text so invalid
     * pair shapes reproduce the reader's named error on the other side. Any
     * other shape reads as empty attributes.
     */
    static function encodeAttributes(writer:Writer, value:WireValue):Void {
        switch (value) {
            case WObj(fields):
                writer.u8(1);
                writer.u32(fields.length);
                for (field in fields) {
                    writer.str(field.name);
                    writer.str(JsCoerce.toString(field.value));
                }
            case WArr(_):
                writer.u8(2);
                writer.bytes(Bytes.ofString(JsCoerce.renderJson(value)));
            default:
                writer.u8(0);
        }
    }
}

/** The encode outcome: bytes or the named issue the platform adapter raises. */
enum EncodeResult {
    COk(bytes:Bytes);
    CErr(issue:String);
}

private enum ListShape {
    LAbsent;
    LArr(items:Array<WireValue>);
    LBad;
}

/**
 * Grows the canonical buffer in chunks; every multi-byte integer is
 * little-endian and every f64 rides its IEEE 754 bit pattern.
 */
private class Writer {
    final buf:BytesBuffer = new BytesBuffer();

    public function new(kind:Int) {
        for (index in 0...Canonical.MAGIC.length) {
            buf.addByte(Canonical.MAGIC.get(index));
        }
        u8(Canonical.VERSION);
        u8(kind);
    }

    public function u8(value:Int):Void {
        buf.addByte(value & 0xFF);
    }

    public function u32(value:Int):Void {
        emitBits(value);
    }

    public function f64(value:Float):Void {
        final bits = FPHelper.doubleToI64(value);
        // Low four bytes first: the f64 form is little-endian end to end.
        emitBits(bits.low);
        emitBits(bits.high);
    }

    function emitBits(value:Int):Void {
        buf.addByte(value & 0xFF);
        buf.addByte((value >>> 8) & 0xFF);
        buf.addByte((value >>> 16) & 0xFF);
        buf.addByte((value >>> 24) & 0xFF);
    }

    public function str(value:String):Void {
        bytes(Bytes.ofString(value));
    }

    public function bytes(value:Bytes):Void {
        u32(value.length);
        buf.add(value);
    }

    public function finish():Bytes {
        return buf.getBytes();
    }
}
