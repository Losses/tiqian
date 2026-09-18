package org.tiqian.linebreak;

import org.tiqian.linebreak.ParseTexHyphenationPatterns;

class EnglishHyphenation {
    private static var enUsCache:Null<Hyphenator> = null;

    public static function enUs():Hyphenator {
        final cached:Null<Hyphenator> = enUsCache;
        if (cached != null) {
            return cached;
        }
        final parsed = ParseTexHyphenationPatterns.parse(EnglishHyphenationPatterns.load());
        final hyphenator:Hyphenator = new LiangHyphenator(parsed.patterns, parsed.exceptions, 2, 3);
        enUsCache = hyphenator;
        return hyphenator;
    }
}
