package org.tiqian.layout;

import org.tiqian.clreq.ClreqPunctuationPolicies;
import org.tiqian.clreq.PunctuationGluePlacement;
import org.tiqian.core.TextRange;
import org.tiqian.layout.PunctuationModel.PunctuationAtom;
import org.tiqian.layout.PunctuationModel.PunctuationAtomBuilder;
import org.tiqian.layout.PunctuationModel.PunctuationSpacingCompressor;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;

class PunctuationSpacingRuleTest {
    private static final em = 16.0;
    private static final builder = new PunctuationAtomBuilder(PunctuationGluePlacement.MainlandSimplified);

    private static function atom(char:String, index:Int):PunctuationAtom {
        final a = builder.build(char, new TextRange(index, index + 1), em);
        if (a == null)
            return null;
        TracedAssertions.assertTrue(ClreqPunctuationPolicies.classify(char) != org.tiqian.clreq.PunctuationClass.Other,
            "unexpected punctuation class for " + char);
        return a;
    }

    private static function check(name:String, f:Void->Void):Void {
        new TestTraceRecorder("PunctuationSpacingRuleTest").section(name);
        f();
    }

    @:test public static function closingPlusClosingCollapsesInnerToZero():Void check("closingPlusClosingCollapsesInnerToZero", function() {
        final a = new PunctuationSpacingCompressor().compress([atom("」", 0), atom("。", 1)], em).adjustments[0];
        TracedAssertions.assertEqualsFloat(8, a.naturalInnerGlue);
        TracedAssertions.assertEqualsFloat(0, a.adjustedInnerGlue);
        TracedAssertions.assertEqualsFloat(8, a.reduction);
    });
    @:test public static function openingPlusOpeningCollapsesInnerToZero():Void check("openingPlusOpeningCollapsesInnerToZero", function() {
        final a = new PunctuationSpacingCompressor().compress([atom("「", 0), atom("（", 1)], em).adjustments[0];
        TracedAssertions.assertEqualsFloat(8, a.naturalInnerGlue);
        TracedAssertions.assertEqualsFloat(0, a.adjustedInnerGlue);
        TracedAssertions.assertEqualsFloat(8, a.reduction);
    });
    @:test public static function closingPlusOpeningKeepsHalfEmGap():Void check("closingPlusOpeningKeepsHalfEmGap", function() {
        final a = new PunctuationSpacingCompressor().compress([atom("。", 0), atom("「", 1)], em).adjustments[0];
        TracedAssertions.assertEqualsFloat(16, a.naturalInnerGlue);
        TracedAssertions.assertEqualsFloat(8, a.adjustedInnerGlue);
        TracedAssertions.assertEqualsFloat(8, a.reduction);
    });
    @:test public static function pauseStopPlusOpeningCollapsesByHalfEm():Void check("pauseStopPlusOpeningCollapsesByHalfEm", function() {
        final a = new PunctuationSpacingCompressor().compress([atom("，", 0), atom("「", 1)], em).adjustments[0];
        TracedAssertions.assertEqualsFloat(16, a.naturalInnerGlue);
        TracedAssertions.assertEqualsFloat(8, a.adjustedInnerGlue);
        TracedAssertions.assertEqualsFloat(8, a.reduction);
    });
    @:test public static function consecutivePauseOrStopMarksCompressLikeAnyAdjacentPair():Void check("consecutivePauseOrStopMarksCompressLikeAnyAdjacentPair", function() {
        final a = new PunctuationSpacingCompressor().compress([atom("！", 0), atom("！", 1)], em).adjustments[0];
        TracedAssertions.assertEqualsFloat(8, a.naturalInnerGlue);
        TracedAssertions.assertEqualsFloat(0, a.adjustedInnerGlue);
        TracedAssertions.assertEqualsFloat(8, a.reduction);
    });
    @:test public static function closingPlusPauseOrStopStillCompresses():Void check("closingPlusPauseOrStopStillCompresses", function() {
        final a = new PunctuationSpacingCompressor().compress([atom("”", 0), atom("！", 1)], em).adjustments[0];
        TracedAssertions.assertEqualsFloat(8, a.naturalInnerGlue);
        TracedAssertions.assertEqualsFloat(0, a.adjustedInnerGlue);
        TracedAssertions.assertEqualsFloat(8, a.reduction);
    });
    @:test public static function nonAdjacentPunctuationAtomsAreNotCompressed():Void check("nonAdjacentPunctuationAtomsAreNotCompressed", function() {
        TracedAssertions.assertEqualsInt(0, new PunctuationSpacingCompressor().compress([atom("，", 0), atom("。", 5)], em).adjustments.length);
    });
    @:test public static function cjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlue():Void check("cjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlue", function() {
        final a = new PunctuationSpacingCompressor().compressCjkClosingBeforeAsciiPointMark([atom("」", 0)], "」,", em).adjustments[0];
        TracedAssertions.assertEqualsFloat(8, a.naturalInnerGlue);
        TracedAssertions.assertEqualsFloat(0, a.adjustedInnerGlue);
        TracedAssertions.assertEqualsFloat(8, a.reduction);
        TracedAssertions.assertEqualsString(Std.string(new TextRange(0, 1)), Std.string(a.reductionTargetRange));
        TracedAssertions.assertEqualsString("collapse-cjk-closing-before-ascii-point-mark", a.reason);
    });
    @:test public static function cjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMark():Void check("cjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMark", function() {
        TracedAssertions.assertEqualsInt(0, new PunctuationSpacingCompressor().compressCjkClosingBeforeAsciiPointMark([atom("」", 0)], "」 ,", em).adjustments.length);
    });
}
