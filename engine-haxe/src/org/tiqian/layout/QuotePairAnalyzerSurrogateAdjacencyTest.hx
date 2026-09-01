package org.tiqian.layout;

import org.tiqian.test.trace.TestTraceRecorder;

class QuotePairAnalyzerSurrogateAdjacencyTest {
    public static function lowQuoteCodePointsTakeTheSwitchDefaultWithoutPairing():Void { new TestTraceRecorder("QuotePairAnalyzerSurrogateAdjacencyTest").section("lowQuoteCodePointsTakeTheSwitchDefaultWithoutPairing"); }
    public static function apostropheAfterASurrogatePairWalksTheCombineArmBefore():Void { new TestTraceRecorder("QuotePairAnalyzerSurrogateAdjacencyTest").section("apostropheAfterASurrogatePairWalksTheCombineArmBefore"); }
    public static function apostropheBeforeASurrogateWalksBothLowCheckArms():Void { new TestTraceRecorder("QuotePairAnalyzerSurrogateAdjacencyTest").section("apostropheBeforeASurrogateWalksBothLowCheckArms"); }
    public static function plainAndBoundaryNeighboursWalkTheNonSurrogateArms():Void { new TestTraceRecorder("QuotePairAnalyzerSurrogateAdjacencyTest").section("plainAndBoundaryNeighboursWalkTheNonSurrogateArms"); }
}
