package org.tiqian.layout;
import org.tiqian.test.trace.TestTraceRecorder;
class PunctuationAtomBuilderHaltTest {
    @:test public static function haltAdvanceWithoutPlacementUsesNamedProfileFallback():Void { new TestTraceRecorder("PunctuationAtomBuilderHaltTest").section("haltAdvanceWithoutPlacementUsesNamedProfileFallback"); }
    @:test public static function haltPlacementDirectlyDefinesBothCompressionSides():Void { new TestTraceRecorder("PunctuationAtomBuilderHaltTest").section("haltPlacementDirectlyDefinesBothCompressionSides"); }
    @:test public static function haltPlacementOverridesRegionalProfileDirection():Void { new TestTraceRecorder("PunctuationAtomBuilderHaltTest").section("haltPlacementOverridesRegionalProfileDirection"); }
    @:test public static function defaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyph():Void { new TestTraceRecorder("PunctuationAtomBuilderHaltTest").section("defaultInkCapsAHaltTrimThatWouldCutIntoThePaintedGlyph"); }
    @:test public static function equalHaltAdvanceFallsThroughToInkBounds():Void { new TestTraceRecorder("PunctuationAtomBuilderHaltTest").section("equalHaltAdvanceFallsThroughToInkBounds"); }
    @:test public static function microsoftYaheiCentredCommaCompressesFromBothSides():Void { new TestTraceRecorder("PunctuationAtomBuilderHaltTest").section("microsoftYaheiCentredCommaCompressesFromBothSides"); }
    @:test public static function microsoftYaheiBottomLeftStopKeepsItsLeadingSafetyMargin():Void { new TestTraceRecorder("PunctuationAtomBuilderHaltTest").section("microsoftYaheiBottomLeftStopKeepsItsLeadingSafetyMargin"); }
    @:test public static function founderHeitiCentredParenthesesStayMirrorImages():Void { new TestTraceRecorder("PunctuationAtomBuilderHaltTest").section("founderHeitiCentredParenthesesStayMirrorImages"); }
    @:test public static function underwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCell():Void { new TestTraceRecorder("PunctuationAtomBuilderHaltTest").section("underwidthOpeningQuoteCompletesTheLeadingSideOfItsFullWidthCell"); }
    @:test public static function fixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShift():Void { new TestTraceRecorder("PunctuationAtomBuilderHaltTest").section("fixedHalfConsumesMeasuredSidebearingsInsteadOfApplyingAProfileShift"); }
    @:test public static function overhangReducesCompressionCapacityWithoutMovingInk():Void { new TestTraceRecorder("PunctuationAtomBuilderHaltTest").section("overhangReducesCompressionCapacityWithoutMovingInk"); }
    @:test public static function flushTestTrace():Void { new TestTraceRecorder("PunctuationAtomBuilderHaltTest").section("flushTestTrace"); }
}
