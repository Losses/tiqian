package org.tiqian.layout;

import org.tiqian.test.trace.TestTraceRecorder;

class QuotePairAnalyzerTest {
    public static function matchesDoubleQuotePair():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("matchesDoubleQuotePair"); }
    public static function matchesSingleQuotePair():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("matchesSingleQuotePair"); }
    public static function matchesNestedQuotePairs():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("matchesNestedQuotePairs"); }
    public static function unmatchedQuotesProduceNoPairs():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("unmatchedQuotesProduceNoPairs"); }
    public static function contractionApostropheDoesNotCloseOuterSingleQuote():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("contractionApostropheDoesNotCloseOuterSingleQuote"); }
    public static function contractionInsideCjkSingleQuotesKeepsApostropheLatin():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("contractionInsideCjkSingleQuotesKeepsApostropheLatin"); }
    public static function inWordApostropheMatrixDoesNotConsumeOuterQuotePairs():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("inWordApostropheMatrixDoesNotConsumeOuterQuotePairs"); }
    public static function unmatchedCurlyQuotesUseDirectionalContext():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("unmatchedCurlyQuotesUseDirectionalContext"); }
    public static function mismatchedNestingLeavesQuotesUnmatched():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("mismatchedNestingLeavesQuotesUnmatched"); }
    public static function classifiesPairAsCjkWhenOuterContextIsCjk():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("classifiesPairAsCjkWhenOuterContextIsCjk"); }
    public static function classifiesPairAsLatinWhenOuterContextIsLatin():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("classifiesPairAsLatinWhenOuterContextIsLatin"); }
    public static function classifiesBothQuotesAsCjkForCjkQuotedLatinContent():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("classifiesBothQuotesAsCjkForCjkQuotedLatinContent"); }
    public static function whitespaceDelimitedLatinQuotePairOverridesCjkOuterContext():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("whitespaceDelimitedLatinQuotePairOverridesCjkOuterContext"); }
    public static function unspacedCjkQuotationOfLatinTextRemainsCjk():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("unspacedCjkQuotationOfLatinTextRemainsCjk"); }
    public static function adjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContext():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("adjacentQuotedListItemsDoNotUsePreviousItemContentAsOuterContext"); }
    public static function spacedCjkQuotedContentRemainsCjk():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("spacedCjkQuotedContentRemainsCjk"); }
    public static function classifiesPairAsCjkAtTextBoundary():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("classifiesPairAsCjkAtTextBoundary"); }
    public static function classifiesTextStartLatinPairFromQuotedContent():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("classifiesTextStartLatinPairFromQuotedContent"); }
    public static function mixedChineseQuestionAtParagraphStartUsesParagraphLanguage():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("mixedChineseQuestionAtParagraphStartUsesParagraphLanguage"); }
    public static function explicitEnglishParagraphLanguageWinsForMixedQuotation():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("explicitEnglishParagraphLanguageWinsForMixedQuotation"); }
    public static function commonDigitsDoNotChooseTheQuoteRole():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("commonDigitsDoNotChooseTheQuoteRole"); }
    public static function nonLatinWesternScriptsParticipateAsStrongScriptEvidence():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("nonLatinWesternScriptsParticipateAsStrongScriptEvidence"); }
    public static function numberedCjkQuotePrefixUsesQuotedContent():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("numberedCjkQuotePrefixUsesQuotedContent"); }
    public static function numberedLatinQuotePrefixStillUsesLatinContent():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("numberedLatinQuotePrefixStillUsesLatinContent"); }
    public static function classifiesNestedPairsByOutermostContext():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("classifiesNestedPairsByOutermostContext"); }
    public static function classifiesLatinNestedQuotesByOuterContext():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("classifiesLatinNestedQuotesByOuterContext"); }
    public static function skipsAsciiPunctuationWhenResolvingContext():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("skipsAsciiPunctuationWhenResolvingContext"); }
    public static function skipsNeutralDashWhenResolvingContext():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("skipsNeutralDashWhenResolvingContext"); }
    public static function endOfTextQuotePairClassifiedByOuterContext():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("endOfTextQuotePairClassifiedByOuterContext"); }
    public static function representativeQuoteContextMatrixRemainsStable():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("representativeQuoteContextMatrixRemainsStable"); }
    public static function roleDecisionSourcesStayExplainableAcrossFallbackPaths():Void { new TestTraceRecorder("QuotePairAnalyzerTest").section("roleDecisionSourcesStayExplainableAcrossFallbackPaths"); }
}
