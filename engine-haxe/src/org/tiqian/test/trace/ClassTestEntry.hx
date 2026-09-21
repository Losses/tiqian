package org.tiqian.test.trace;

import org.tiqian.core.TiqianIllegalArgumentException;

/**
    Failure accounting for the conventional class test entry (boring feature
    spec 19). engine-haxe/tests/Main.hx drives the same entry points through
    its private run(name, test) helper, which catches TraceAssertionException
    and TiqianIllegalArgumentException and counts each one into a shared
    failures field. A class the @:test collection leaves out reaches the
    Kotlin runner through one runTestEntries call, and that runner catches
    outside the call only: a throwing entry point would skip the remaining
    entry points of its class. The entries route through this helper instead,
    which keeps every entry point of the class reachable and keeps the Haxe
    bundle's accounting.

    One domain per region (style rule V20) forbids a single region carrying
    two clauses, so the two handlers nest: the inner region owns
    TraceAssertionException and the outer one TiqianIllegalArgumentException.

    The Haxe runner writes a "FAIL <name>: <message>" line through std.Console
    for every counted failure. This helper carries no such line: the Kotlin
    std.Console shim emits `object Console` in the runtime package while every
    reference to that extern class renders under its @:native name `console`,
    so the generated import (org.tiqian.boring.runtime.console) does not
    resolve and the Kotlin compilation stops. The counter below records the
    failures the Haxe side counts; a failed entry point also writes its own
    class trace, which is what the trace comparison sees.
**/
class ClassTestEntry {
    /**
        Failures the entry points counted; the Haxe runner keeps one counter
        for the whole run the same way.
    **/
    public static var failures:Int = 0;

    /**
        Runs one entry point of a class and accounts for its failure. A
        throwing entry point leaves the remaining entry points of its class
        reachable, which is what the per-class entry call needs.
    **/
    public static function run(test:() -> Void):Void {
        try {
            try {
                test();
            } catch (error:TraceAssertionException) {
                failures += 1;
            }
        } catch (error:TiqianIllegalArgumentException) {
            failures += 1;
        }
    }
}
