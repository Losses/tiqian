package org.tiqian.linebreak;

interface Hyphenator { public function hyphenate(word:String):std.ReadOnlyArray<Int>; }
class NoHyphenator implements Hyphenator {
    public function new() {}
    public function hyphenate(word:String):std.ReadOnlyArray<Int> return [];
}
