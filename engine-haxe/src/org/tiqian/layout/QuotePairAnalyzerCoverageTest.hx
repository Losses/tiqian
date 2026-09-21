package org.tiqian.layout;

import org.tiqian.layout.QuotePairAnalyzer.QuotePairAwareFontRoleClassifier;
import org.tiqian.core.TextRange;
import org.tiqian.font.CjkFontRoleClassifier;
import org.tiqian.font.FontRole;
import org.tiqian.font.FontRoleContext;
import org.tiqian.layout.QuotePairAnalyzer.QuoteType;
import org.tiqian.test.TestHelpers;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;
import org.tiqian.test.trace.ClassTestEntry;

class QuotePairAnalyzerCoverageTest {
    private static function rec(n:String):Void
        new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section(n);

    private static function a():QuotePairAnalyzer
        return new QuotePairAnalyzer();

    private static function nonEmpty(t:String):Void {
        var d = a().classifyQuoteRoles(t, []);
        TracedAssertions.assertTrue(d.length > 0);
    }

    public static function deprecatedClassifyPairsWithFontRoleClassifierDelegates():Void {
        rec("deprecatedClassifyPairsWithFontRoleClassifierDelegates");
        var t = "\u4ED6\u8BF4\u201C\u4F60\u597D\u201D";
        TracedAssertions.assertEqualsFontRole(FontRole.CjkPunctuation, a().classifyPairsWithClassifier(t, a().analyze(t), new CjkFontRoleClassifier()).get(2));
    }

    public static function deprecatedClassifyQuoteRolesWithFontRoleClassifierDelegates():Void {
        rec("deprecatedClassifyQuoteRolesWithFontRoleClassifierDelegates");
        var t = "\u4ED6\u8BF4\u201C\u4F60\u597D\u201D";
        TracedAssertions.assertTrue(a().classifyQuoteRolesWithClassifier(t, a().analyze(t), new CjkFontRoleClassifier()).length > 0);
    }

    public static function codePointBeforeSurrogatePairReturnsSupplementary():Void {
        rec("codePointBeforeSurrogatePairReturnsSupplementary");
        TracedAssertions.assertEquals(0, a().analyze(TestHelpers.surrogateText([0xD83D, 0xDE00, 0x2019])).length);
    }

    public static function codePointAtOrNullSurrogatePairReturnsSupplementary():Void {
        rec("codePointAtOrNullSurrogatePairReturnsSupplementary");
        nonEmpty(TestHelpers.surrogateText([0x2019, 0xD83D, 0xDE00]));
    }

    public static function codePointAtOrNullNonSurrogateReturnsSelf():Void {
        rec("codePointAtOrNullNonSurrogateReturnsSelf");
        TracedAssertions.assertTrue(a().classifyQuoteRoles("abc", []).length == 0);
    }

    public static function codePointBeforeReturnsNullAtStart():Void {
        rec("codePointBeforeReturnsNullAtStart");
        nonEmpty("\u2019");
    }

    public static function codePointBeforeReturnsSupplementaryForSurrogatePair():Void {
        rec("codePointBeforeReturnsSupplementaryForSurrogatePair");
        nonEmpty(TestHelpers.surrogateText([0xD83D, 0xDE00, 0x2019]));
    }

    public static function quotePairAwareFontRoleClassifierUsesOverride():Void {
        rec("quotePairAwareFontRoleClassifierUsesOverride");
        var b = std.SortedMap.builder();
        b.put(2, FontRole.LatinText);
        var c = new QuotePairAwareFontRoleClassifier(new CjkFontRoleClassifier(), b.build());
        TracedAssertions.assertEqualsFontRole(FontRole.LatinText, c.classify("ab", new TextRange(0, 2), new FontRoleContext()));
    }

    public static function quotePairAwareFontRoleClassifierDelegatesWhenNoOverride():Void {
        rec("quotePairAwareFontRoleClassifierDelegatesWhenNoOverride");
        var b = std.SortedMap.builder();
        var c = new CjkFontRoleClassifier();
        var w = new QuotePairAwareFontRoleClassifier(c, b.build());
        TracedAssertions.assertEqualsFontRole(c.classify("ab", new TextRange(0, 2), new FontRoleContext()),
            w.classify("ab", new TextRange(0, 2), new FontRoleContext()));
    }

    public static function doubleQuoteCloseWithEmptyStackIgnores():Void {
        rec("doubleQuoteCloseWithEmptyStackIgnores");
        TracedAssertions.assertEquals(0, a().analyze("\u201D").length);
    }

    public static function singleQuoteCloseWithEmptyStackIgnores():Void {
        rec("singleQuoteCloseWithEmptyStackIgnores");
        TracedAssertions.assertEquals(0, a().analyze("\u2019").length);
    }

    public static function inWordApostropheAfterSupplementaryDoesNotClose():Void {
        rec("inWordApostropheAfterSupplementaryDoesNotClose");
        nonEmpty(TestHelpers.surrogateText([0xD83D, 0xDE00, 0x2019, 0x78]));
    }

    public static function codePointAtOrNullWithSupplementaryAfterQuote():Void {
        rec("codePointAtOrNullWithSupplementaryAfterQuote");
        nonEmpty(TestHelpers.surrogateText([0x61, 0x2019, 0xD83D, 0xDE00]));
    }

    public static function codePointBeforeWithHighSurrogateBeforeQuote():Void {
        rec("codePointBeforeWithHighSurrogateBeforeQuote");
        nonEmpty(TestHelpers.surrogateText([0xD83D, 0xDE00, 0x2019]));
    }

    private static function failLow(t:String):Void
        TracedAssertions.assertFailsWith(null, () -> a().classifyQuoteRoles(t, []));

    public static function codePointBeforeWithLowSurrogateAtStart():Void {
        rec("codePointBeforeWithLowSurrogateAtStart");
        failLow(TestHelpers.surrogateText([0xDC00, 0x2019]));
    }

    public static function codePointBeforeWithLowSurrogateAfterNonHighSurrogate():Void {
        rec("codePointBeforeWithLowSurrogateAfterNonHighSurrogate");
        failLow(TestHelpers.surrogateText([0x61, 0xDC00, 0x2019]));
    }

    public static function codePointAtOrNullWithIndexOutOfRange():Void {
        rec("codePointAtOrNullWithIndexOutOfRange");
        nonEmpty("a\u2019");
    }

    public static function codePointAtOrNullWithHighSurrogateAtEnd():Void {
        rec("codePointAtOrNullWithHighSurrogateAtEnd");
        failLow(TestHelpers.surrogateText([0x2019, 0xD800]));
    }

    public static function codePointAtOrNullWithHighSurrogateFollowedByNonLowSurrogate():Void {
        rec("codePointAtOrNullWithHighSurrogateFollowedByNonLowSurrogate");
        failLow(TestHelpers.surrogateText([0x2019, 0xD800, 0x61]));
    }

    public static function analyzeWithDoubleQuoteOpen():Void {
        rec("analyzeWithDoubleQuoteOpen");
        TracedAssertions.assertEquals(0, a().analyze("\u201Cabc").length);
    }

    public static function codePointAtOrNullHighSurrogateNotInRangeReturnsHigh():Void {
        rec("codePointAtOrNullHighSurrogateNotInRangeReturnsHigh");
        nonEmpty("x\u2019a");
    }

    public static function codePointBeforeLowInRangeIndexGe2HighNotInRange():Void {
        rec("codePointBeforeLowInRangeIndexGe2HighNotInRange");
        failLow(TestHelpers.surrogateText([0x61, 0xDC00, 0x2019]));
    }

    public static function singleQuotePairMatch():Void {
        rec("singleQuotePairMatch");
        var p = a().analyze("\u2018\u2019");
        TracedAssertions.assertEquals(1, p.length);
        TracedAssertions.assertEqualsQuoteType(QuoteType.Single, p[0].quoteType);
    }

    public static function codePointAtOrNullLoneHighSurrogateAfterQuote():Void {
        rec("codePointAtOrNullLoneHighSurrogateAfterQuote");
        failLow(TestHelpers.surrogateText([0x61, 0x2019, 0xD800, 0x61]));
    }

    public static function codePointAtOrNullHighSurrogateAtStringEnd():Void {
        rec("codePointAtOrNullHighSurrogateAtStringEnd");
        failLow(TestHelpers.surrogateText([0x61, 0x2019, 0xD800]));
    }

    public static function analyzeWithAllQuoteTypes():Void {
        rec("analyzeWithAllQuoteTypes");
        TracedAssertions.assertEquals(2, a().analyze("\u201C\u2018abc\u2019\u201D").length);
    }

    public static function codePointBeforeNonSurrogateBmpChar():Void {
        rec("codePointBeforeNonSurrogateBmpChar");
        nonEmpty("A\u2019");
    }

    /**
        The conventional class test entry (boring feature spec 19).
        The class carries no test marker, so the Kotlin runner generated from
        that marker's collection never instantiates it; this entry is how the
        runner calls the class once. It registers no test id, so the cross-target
        test id set is unchanged. The body repeats the calls
        engine-haxe/tests/Main.hx makes for this class, through the same failure
        accounting Main.hx uses, and then flushes the class trace the way Main.hx
        does.
    **/
    public static function runTestEntries():Void {
        ClassTestEntry.run(deprecatedClassifyPairsWithFontRoleClassifierDelegates);
        ClassTestEntry.run(deprecatedClassifyQuoteRolesWithFontRoleClassifierDelegates);
        ClassTestEntry.run(codePointBeforeSurrogatePairReturnsSupplementary);
        ClassTestEntry.run(codePointAtOrNullSurrogatePairReturnsSupplementary);
        ClassTestEntry.run(codePointAtOrNullNonSurrogateReturnsSelf);
        ClassTestEntry.run(codePointBeforeReturnsNullAtStart);
        ClassTestEntry.run(codePointBeforeReturnsSupplementaryForSurrogatePair);
        ClassTestEntry.run(quotePairAwareFontRoleClassifierUsesOverride);
        ClassTestEntry.run(quotePairAwareFontRoleClassifierDelegatesWhenNoOverride);
        ClassTestEntry.run(doubleQuoteCloseWithEmptyStackIgnores);
        ClassTestEntry.run(singleQuoteCloseWithEmptyStackIgnores);
        ClassTestEntry.run(inWordApostropheAfterSupplementaryDoesNotClose);
        ClassTestEntry.run(codePointAtOrNullWithSupplementaryAfterQuote);
        ClassTestEntry.run(codePointBeforeWithHighSurrogateBeforeQuote);
        ClassTestEntry.run(codePointBeforeWithLowSurrogateAtStart);
        ClassTestEntry.run(codePointBeforeWithLowSurrogateAfterNonHighSurrogate);
        ClassTestEntry.run(codePointAtOrNullWithIndexOutOfRange);
        ClassTestEntry.run(codePointAtOrNullWithHighSurrogateAtEnd);
        ClassTestEntry.run(codePointAtOrNullWithHighSurrogateFollowedByNonLowSurrogate);
        ClassTestEntry.run(analyzeWithDoubleQuoteOpen);
        ClassTestEntry.run(codePointAtOrNullHighSurrogateNotInRangeReturnsHigh);
        ClassTestEntry.run(codePointBeforeLowInRangeIndexGe2HighNotInRange);
        ClassTestEntry.run(singleQuotePairMatch);
        ClassTestEntry.run(codePointAtOrNullLoneHighSurrogateAfterQuote);
        ClassTestEntry.run(codePointAtOrNullHighSurrogateAtStringEnd);
        ClassTestEntry.run(analyzeWithAllQuoteTypes);
        ClassTestEntry.run(codePointBeforeNonSurrogateBmpChar);
        TestTraceRecorder.flushClass("QuotePairAnalyzerCoverageTest");
    }
}
