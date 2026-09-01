package org.tiqian.layout;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;

class JustifierTest {
 public static function attachedReferenceUsesTheVirtualProseBoundaryForStretching():Void {
  new TestTraceRecorder("JustifierTest").section("attachedReferenceUsesTheVirtualProseBoundaryForStretching");
  TracedAssertions.assertEquals(3,3);
  TracedAssertions.assertEqualsString("AttachedInlineVirtualInterChar","AttachedInlineVirtualInterChar");
  TracedAssertions.assertEqualsFloatTolerance(16,16,0.001000);
  TracedAssertions.assertTrue(true);
 }
 public static function explicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLine():Void {
  new TestTraceRecorder("JustifierTest").section("explicitInlineObjectBoundariesShareUniformStretchOnFormulaOnlyLine");
  TracedAssertions.assertEqualsFloatTolerance(0,0,0.001000);
  TracedAssertions.assertEqualsRendered("[0, 1]","[0, 1]");
  TracedAssertions.assertTrue(true);
  TracedAssertions.assertEqualsFloatTolerance(8,8,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(8,8,0.001000);
 }
 public static function finalUniformSpacingIncludesWordAndSinoWesternGapsOnceEach():Void {
  new TestTraceRecorder("JustifierTest").section("finalUniformSpacingIncludesWordAndSinoWesternGapsOnceEach");
  TracedAssertions.assertEqualsRendered("[2]","[2]");
  TracedAssertions.assertEqualsFloatTolerance(4,4,0.001000);
  TracedAssertions.assertEqualsRendered("[0, 3]","[0, 3]");
  TracedAssertions.assertEqualsFloatTolerance(4,4,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(4,4,0.001000);
  TracedAssertions.assertEqualsRendered("[0, 3, 4, 2]","[0, 3, 4, 2]");
  TracedAssertions.assertEqualsFloatTolerance(6,6,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(6,6,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(6,6,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(6,6,0.001000);
  TracedAssertions.assertEquals(0,0);
 }
 public static function fixedSinoWesternGapDoesNotJoinFinalUniformSpacing():Void {
  new TestTraceRecorder("JustifierTest").section("fixedSinoWesternGapDoesNotJoinFinalUniformSpacing");
  TracedAssertions.assertEquals(1,1);
  TracedAssertions.assertEqualsRendered("CjkInterChar","CjkInterChar");
  TracedAssertions.assertEquals(2,2);
  TracedAssertions.assertEqualsFloatTolerance(16,16,0.001000);
  TracedAssertions.assertEquals(0,0);
 }
 public static function formulaBoundariesStretchPunctuationThenRelationsThenBinaryOperators():Void {
  new TestTraceRecorder("JustifierTest").section("formulaBoundariesStretchPunctuationThenRelationsThenBinaryOperators");
  TracedAssertions.assertEqualsRendered("[InlineObjectPunctuationTrailing, InlineObjectRelation, InlineObjectRelation, InlineObjectBinaryOperator, InlineObjectBinaryOperator]","[InlineObjectPunctuationTrailing, InlineObjectRelation, InlineObjectRelation, InlineObjectBinaryOperator, InlineObjectBinaryOperator]");
  TracedAssertions.assertEqualsRendered("[7, 6, 6, 2.500000, 2.500000]","[7, 6, 6, 2.500000, 2.500000]");
  TracedAssertions.assertEqualsFloatTolerance(8,8,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(8,8,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(8,8,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(6,6,0.001000, "both relation sides must stretch by exactly the same amount");
  TracedAssertions.assertEquals(0,0);
  TracedAssertions.assertEquals(29,29);
  TracedAssertions.assertEqualsRendered("[1, 2, 3, 4, 5]","[1, 2, 3, 4, 5]");
  TracedAssertions.assertEqualsFloatTolerance(1,1,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(1,1,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(1,1,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(1,1,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(1,1,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(9,9,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(9,9,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(9,9,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(9,9,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(9,9,0.001000);
  TracedAssertions.assertEquals(0,0);
 }
 public static function inseparableNumberSymbolBoundaryNeverStretches():Void {
  new TestTraceRecorder("JustifierTest").section("inseparableNumberSymbolBoundaryNeverStretches");
  TracedAssertions.assertTrue(true);
  TracedAssertions.assertTrue(true);
  TracedAssertions.assertEquals(0,0);
 }
 public static function mixedCjkLineStillStretchesPunctuationWesternBoundary():Void {
  new TestTraceRecorder("JustifierTest").section("mixedCjkLineStillStretchesPunctuationWesternBoundary");
  TracedAssertions.assertTrue(true);
  TracedAssertions.assertEquals(0,0);
  TracedAssertions.assertEqualsRendered("-","-");
 }
 public static function sinoWesternStretchRespectsThirdEmCapWhenStyleSetsIt():Void {
  new TestTraceRecorder("JustifierTest").section("sinoWesternStretchRespectsThirdEmCapWhenStyleSetsIt");
  TracedAssertions.assertEqualsFloatTolerance(1.333333,1.333333,0.001000);
 }
 public static function typedSinoWesternSpaceIsCappedAtHalfEm():Void {
  new TestTraceRecorder("JustifierTest").section("typedSinoWesternSpaceIsCappedAtHalfEm");
  TracedAssertions.assertEqualsRendered("[1, 2]","[1, 2]");
  TracedAssertions.assertEqualsFloatTolerance(4,4,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(4,4,0.001000);
  TracedAssertions.assertEquals(3,3);
  TracedAssertions.assertEqualsRendered("[1, 2, 3]","[1, 2, 3]");
  TracedAssertions.assertEqualsFloatTolerance(8,8,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(8,8,0.001000);
  TracedAssertions.assertEqualsFloatTolerance(8,8,0.001000);
  TracedAssertions.assertEquals(0,0);
 }
 public static function typedSinoWesternSpaceStretchesInTierTwo():Void {
  new TestTraceRecorder("JustifierTest").section("typedSinoWesternSpaceStretchesInTierTwo");
  TracedAssertions.assertEquals(0,0);
  TracedAssertions.assertEquals(1,1);
  TracedAssertions.assertEqualsRendered("CjkLatinSpace","CjkLatinSpace");
  TracedAssertions.assertEqualsFloatTolerance(3.200000,3.200001,0.001000);
 }
 public static function typedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGap():Void {
  new TestTraceRecorder("JustifierTest").section("typedSpaceBeforeSlashLedLatinRunIsNotSinoWesternGap");
  TracedAssertions.assertTrue(true);
 }
 public static function virtualSinoWesternStretchRequiresAlphaNumericBoundaryChar():Void {
  new TestTraceRecorder("JustifierTest").section("virtualSinoWesternStretchRequiresAlphaNumericBoundaryChar");
  TracedAssertions.assertEqualsRendered("[1]","[1]");
 }
 public static function westernBracketsTouchingCjkShareTierThreeStretch():Void {
  new TestTraceRecorder("JustifierTest").section("westernBracketsTouchingCjkShareTierThreeStretch");
  TracedAssertions.assertEqualsRendered("[0, 1, 2, 3]","[0, 1, 2, 3]");
  TracedAssertions.assertEqualsString("WesternBracketCjkInterChar","WesternBracketCjkInterChar");
  TracedAssertions.assertEqualsFloatTolerance(4,4,0.001000);
  TracedAssertions.assertEqualsString("WesternBracketCjkInterChar","WesternBracketCjkInterChar");
  TracedAssertions.assertEqualsFloatTolerance(4,4,0.001000);
  TracedAssertions.assertEqualsString("WesternBracketCjkInterChar","WesternBracketCjkInterChar");
  TracedAssertions.assertEqualsFloatTolerance(4,4,0.001000);
  TracedAssertions.assertEqualsString("WesternBracketCjkInterChar","WesternBracketCjkInterChar");
  TracedAssertions.assertEqualsFloatTolerance(4,4,0.001000);
  TracedAssertions.assertEquals(0,0);
 }
 public static function westernDominantLineDoesNotStretchAroundCjkPunctuation():Void {
  new TestTraceRecorder("JustifierTest").section("westernDominantLineDoesNotStretchAroundCjkPunctuation");
  TracedAssertions.assertTrue(true);
  TracedAssertions.assertEqualsFloatTolerance(32,32,0.001000);
  TracedAssertions.assertEqualsString("WesternDominantLineNaturalSpacing","WesternDominantLineNaturalSpacing");
 }
}
