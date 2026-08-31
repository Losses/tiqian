package org.tiqian.core;

import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;

class EastAsianSpacingTest {
    private static var testTrace:TestTraceRecorder = null;

    private static function currentTrace():TestTraceRecorder {
        if (testTrace == null) {
            testTrace = new TestTraceRecorder("EastAsianSpacingTest");
        }
        return testTrace;
    }

    @:test
    public static function chineseLanguageContextUsesPinnedMacrolanguageRegistry():Void {
        currentTrace().section("chineseLanguageContextUsesPinnedMacrolanguageRegistry");
        TracedAssertions.assertTrue(UnicodeEastAsianSpacing.isChineseLanguageContext("zh-Hans"));
        TracedAssertions.assertTrue(UnicodeEastAsianSpacing.isChineseLanguageContext("yue-Hant-HK"));
        TracedAssertions.assertFalse(UnicodeEastAsianSpacing.isChineseLanguageContext("en"));
    }

    @:test
    public static function usesPinnedUnicodeDraftDataAcrossScripts():Void {
        currentTrace().section("usesPinnedUnicodeDraftDataAcrossScripts");
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Wide, UnicodeEastAsianSpacing.propertyOf(0x63D0));
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Wide, UnicodeEastAsianSpacing.propertyOf(0x17000));
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Narrow, UnicodeEastAsianSpacing.propertyOf(0x41));
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Narrow, UnicodeEastAsianSpacing.propertyOf(0x03B1));
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Narrow, UnicodeEastAsianSpacing.propertyOf(0x044F));
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Narrow, UnicodeEastAsianSpacing.propertyOf(0x39));
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Conditional, UnicodeEastAsianSpacing.propertyOf(0x25));
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Other, UnicodeEastAsianSpacing.propertyOf(0xFF0F));
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Other, UnicodeEastAsianSpacing.propertyOf(0x1F600));
    }

    @:test
    public static function resolvesConditionalValuesFromChineseLanguageContext():Void {
        currentTrace().section("resolvesConditionalValuesFromChineseLanguageContext");
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Narrow, UnicodeEastAsianSpacing.resolvedForGraphemeCluster("%", "zh-Hans"));
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Narrow, UnicodeEastAsianSpacing.resolvedForGraphemeCluster("%", "yue-Hant-HK"));
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Other, UnicodeEastAsianSpacing.resolvedForGraphemeCluster("%", "en"));
    }

    @:test
    public static function enclosingMarkMakesTheWholeGraphemeClusterOther():Void {
        currentTrace().section("enclosingMarkMakesTheWholeGraphemeClusterOther");
        TracedAssertions.assertEqualsGeneric(EastAsianSpacingValue.Other, UnicodeEastAsianSpacing.resolvedForGraphemeCluster("A\u20DD", "zh-Hans"));
    }

    @:test
    public static function resolvesTheActualSourceUnitAtEachShapingClusterEdge():Void {
        currentTrace().section("resolvesTheActualSourceUnitAtEachShapingClusterEdge");
        TracedAssertions.assertEqualsEastAsianSpacingEdges(
            new EastAsianSpacingEdges(EastAsianSpacingValue.Other, EastAsianSpacingValue.Narrow, false),
            UnicodeEastAsianSpacing.resolvedEdges("/Hi", "zh-Hans")
        );
        TracedAssertions.assertEqualsEastAsianSpacingEdges(
            new EastAsianSpacingEdges(EastAsianSpacingValue.Other, EastAsianSpacingValue.Other, false),
            UnicodeEastAsianSpacing.resolvedEdges("A\u20DD", "zh-Hans")
        );
    }

    public static function flushTestTrace():Void {
        currentTrace().flush();
    }
}
