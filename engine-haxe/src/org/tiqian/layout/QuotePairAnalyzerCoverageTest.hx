package org.tiqian.layout;

import org.tiqian.test.trace.TestTraceRecorder;

class QuotePairAnalyzerCoverageTest {
    public static function deprecatedClassifyPairsWithFontRoleClassifierDelegates():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("deprecatedClassifyPairsWithFontRoleClassifierDelegates"); }
    public static function deprecatedClassifyQuoteRolesWithFontRoleClassifierDelegates():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("deprecatedClassifyQuoteRolesWithFontRoleClassifierDelegates"); }
    public static function codePointBeforeSurrogatePairReturnsSupplementary():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("codePointBeforeSurrogatePairReturnsSupplementary"); }
    public static function codePointAtOrNullSurrogatePairReturnsSupplementary():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("codePointAtOrNullSurrogatePairReturnsSupplementary"); }
    public static function codePointAtOrNullNonSurrogateReturnsSelf():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("codePointAtOrNullNonSurrogateReturnsSelf"); }
    public static function codePointBeforeReturnsNullAtStart():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("codePointBeforeReturnsNullAtStart"); }
    public static function codePointBeforeReturnsSupplementaryForSurrogatePair():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("codePointBeforeReturnsSupplementaryForSurrogatePair"); }
    public static function quotePairAwareFontRoleClassifierUsesOverride():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("quotePairAwareFontRoleClassifierUsesOverride"); }
    public static function quotePairAwareFontRoleClassifierDelegatesWhenNoOverride():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("quotePairAwareFontRoleClassifierDelegatesWhenNoOverride"); }
    public static function doubleQuoteCloseWithEmptyStackIgnores():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("doubleQuoteCloseWithEmptyStackIgnores"); }
    public static function singleQuoteCloseWithEmptyStackIgnores():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("singleQuoteCloseWithEmptyStackIgnores"); }
    public static function inWordApostropheAfterSupplementaryDoesNotClose():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("inWordApostropheAfterSupplementaryDoesNotClose"); }
    public static function codePointAtOrNullWithSupplementaryAfterQuote():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("codePointAtOrNullWithSupplementaryAfterQuote"); }
    public static function codePointBeforeWithHighSurrogateBeforeQuote():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("codePointBeforeWithHighSurrogateBeforeQuote"); }
    public static function codePointBeforeWithLowSurrogateAtStart():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("codePointBeforeWithLowSurrogateAtStart"); }
    public static function codePointBeforeWithLowSurrogateAfterNonHighSurrogate():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("codePointBeforeWithLowSurrogateAfterNonHighSurrogate"); }
    public static function codePointAtOrNullWithIndexOutOfRange():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("codePointAtOrNullWithIndexOutOfRange"); }
    public static function codePointAtOrNullWithHighSurrogateAtEnd():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("codePointAtOrNullWithHighSurrogateAtEnd"); }
    public static function codePointAtOrNullWithHighSurrogateFollowedByNonLowSurrogate():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("codePointAtOrNullWithHighSurrogateFollowedByNonLowSurrogate"); }
    public static function analyzeWithDoubleQuoteOpen():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("analyzeWithDoubleQuoteOpen"); }
    public static function codePointAtOrNullHighSurrogateNotInRangeReturnsHigh():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("codePointAtOrNullHighSurrogateNotInRangeReturnsHigh"); }
    public static function codePointBeforeLowInRangeIndexGe2HighNotInRange():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("codePointBeforeLowInRangeIndexGe2HighNotInRange"); }
    public static function singleQuotePairMatch():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("singleQuotePairMatch"); }
    public static function codePointAtOrNullLoneHighSurrogateAfterQuote():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("codePointAtOrNullLoneHighSurrogateAfterQuote"); }
    public static function codePointAtOrNullHighSurrogateAtStringEnd():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("codePointAtOrNullHighSurrogateAtStringEnd"); }
    public static function analyzeWithAllQuoteTypes():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("analyzeWithAllQuoteTypes"); }
    public static function codePointBeforeNonSurrogateBmpChar():Void { new TestTraceRecorder("QuotePairAnalyzerCoverageTest").section("codePointBeforeNonSurrogateBmpChar"); }
}
