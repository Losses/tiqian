package org.tiqian.layout;

import org.tiqian.layout.QuotePairAnalyzer.QuotePair;
import org.tiqian.layout.QuotePairAnalyzer.QuoteType;
import org.tiqian.test.TestHelpers;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;
import org.tiqian.test.trace.ClassTestEntry;

class QuotePairAnalyzerSurrogateAdjacencyTest {
    private static function rec(n:String):Void
        new TestTraceRecorder("QuotePairAnalyzerSurrogateAdjacencyTest").section(n);

    public static function lowQuoteCodePointsTakeTheSwitchDefaultWithoutPairing():Void {
        rec("lowQuoteCodePointsTakeTheSwitchDefaultWithoutPairing");
        TracedAssertions.assertEqualsQuotePairArray([new QuotePair(2, 3, QuoteType.Double)], new QuotePairAnalyzer().analyze("\u201A\u201B\u201C\u201D"));
    }

    public static function apostropheAfterASurrogatePairWalksTheCombineArmBefore():Void {
        rec("apostropheAfterASurrogatePairWalksTheCombineArmBefore");
        TracedAssertions.assertFalse(QuotePairAnalyzer.isNonCjkInWordApostrophe(TestHelpers.surrogateText([0xD83D, 0xDE00, 0x2019, 0x78]), 2));
        TracedAssertions.assertFailsWith(null, () -> QuotePairAnalyzer.isNonCjkInWordApostrophe(TestHelpers.surrogateText([0xDC00, 0x2019, 0x78]), 1));
        TracedAssertions.assertFailsWith(null,
            () -> QuotePairAnalyzer.isNonCjkInWordApostrophe(TestHelpers.surrogateText([0x61, 0x62, 0xDC00, 0x2019, 0x78]), 3));
        TracedAssertions.assertFalse(QuotePairAnalyzer.isNonCjkInWordApostrophe("\uE000\u2019b", 1));
        TracedAssertions.assertFailsWith(null,
            () -> QuotePairAnalyzer.isNonCjkInWordApostrophe(TestHelpers.surrogateText([0x78, 0xDC00, 0xDC00, 0x2019, 0x78]), 3));
    }

    public static function apostropheBeforeASurrogateWalksBothLowCheckArms():Void {
        rec("apostropheBeforeASurrogateWalksBothLowCheckArms");
        TracedAssertions.assertFalse(QuotePairAnalyzer.isNonCjkInWordApostrophe(TestHelpers.surrogateText([0x61, 0x2019, 0xD83D, 0xDE00]), 1));
        TracedAssertions.assertFailsWith(null, () -> QuotePairAnalyzer.isNonCjkInWordApostrophe(TestHelpers.surrogateText([0x61, 0x2019, 0xD83D, 0x62]), 1));
    }

    public static function plainAndBoundaryNeighboursWalkTheNonSurrogateArms():Void {
        rec("plainAndBoundaryNeighboursWalkTheNonSurrogateArms");
        TracedAssertions.assertTrue(QuotePairAnalyzer.isNonCjkInWordApostrophe("a\u2019b", 1));
        TracedAssertions.assertFalse(QuotePairAnalyzer.isNonCjkInWordApostrophe("\u2019a", 0));
        TracedAssertions.assertFalse(QuotePairAnalyzer.isNonCjkInWordApostrophe("a\u2019", 1));
        TracedAssertions.assertFailsWith(null, () -> QuotePairAnalyzer.isNonCjkInWordApostrophe(TestHelpers.surrogateText([0x61, 0x2019, 0xDC00]), 1));
        TracedAssertions.assertFailsWith(null, () -> QuotePairAnalyzer.isNonCjkInWordApostrophe(TestHelpers.surrogateText([0x61, 0x2019, 0xD83D]), 1));
        TracedAssertions.assertFailsWith(null, () -> QuotePairAnalyzer.isNonCjkInWordApostrophe(TestHelpers.surrogateText([0x61, 0x2019, 0xD83D, 0xE000]), 1));
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
        ClassTestEntry.run(lowQuoteCodePointsTakeTheSwitchDefaultWithoutPairing);
        ClassTestEntry.run(apostropheAfterASurrogatePairWalksTheCombineArmBefore);
        ClassTestEntry.run(apostropheBeforeASurrogateWalksBothLowCheckArms);
        ClassTestEntry.run(plainAndBoundaryNeighboursWalkTheNonSurrogateArms);
        TestTraceRecorder.flushClass("QuotePairAnalyzerSurrogateAdjacencyTest");
    }
}
