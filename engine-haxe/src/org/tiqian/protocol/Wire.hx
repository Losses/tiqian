package org.tiqian.protocol;

/**
 * One entry of a WObj. The list order inside WObj is the identity order: the
 * canonical encoder carries attribute pairs in exactly this order, so every
 * adapter that builds a WObj from a platform map must keep insertion order
 * (JavaScript's own property order, the Rust Json parser's document order).
 */
typedef WireField = {
    final name:String;
    final value:WireValue;
}

/**
 * The JSON-lane wire value, the single-source input of the canonical byte
 * form (ADR 0052). Both platform callers parse their submission into this
 * shape first: the TypeScript caller walks its live JS object, the Rust
 * caller mirrors its parsed crate::json::Json. Absent members and explicit
 * nulls both read as WNull, the same coalescing the wire readers apply.
 */
enum WireValue {
    WNull;
    WBool(value:Bool);
    WNum(value:Float);
    WStr(value:String);
    WArr(items:Array<WireValue>);
    WObj(fields:Array<WireField>);
}
