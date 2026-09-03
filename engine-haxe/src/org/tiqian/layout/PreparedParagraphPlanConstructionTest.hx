package org.tiqian.layout;

import org.tiqian.core.*;
import org.tiqian.layout.PreparedParagraph.PreparedParagraphFns;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;

class PreparedParagraphPlanConstructionTest {
    @:test public static function openTypeFeaturesAndRenderFontFamilyAttachPerCluster():Void
        PreparedParagraphPlanConstructionTestSupport.check("openTypeFeaturesAndRenderFontFamilyAttachPerCluster", "汉", "\"cells\"");

    @:test public static function multiUnitClusterMarksShapingBoundary():Void
        PreparedParagraphPlanConstructionTestSupport.check("multiUnitClusterMarksShapingBoundary", "——", "\"shapingBoundary\":true");

    @:test public static function inlineObjectCellEmitsAdvanceOverride():Void
        PreparedParagraphPlanConstructionTestSupport.check("inlineObjectCellEmitsAdvanceOverride", "汉", "\"cells\"");

    @:test public static function styleDeltaListsOnlyPaintFields():Void
        PreparedParagraphPlanConstructionTestSupport.check("styleDeltaListsOnlyPaintFields", "汉", "\"cells\"");

    @:test public static function dashClusterEmitsShapingEvidenceBlock():Void
        PreparedParagraphPlanConstructionTestSupport.check("dashClusterEmitsShapingEvidenceBlock", "汉——", "\"cells\"");

    @:test public static function punctuationInkFloorAndLatinRoleMarkCells():Void
        PreparedParagraphPlanConstructionTestSupport.check("punctuationInkFloorAndLatinRoleMarkCells", "，a", "\"cells\"");

    @:test public static function zeroWidthBreakClusterSurvivesEmptyDisplayText():Void
        PreparedParagraphPlanConstructionTestSupport.check("zeroWidthBreakClusterSurvivesEmptyDisplayText", "汉", "\"cells\"");

    @:test public static function paragraphEvidenceEmitsEverySection():Void
        PreparedParagraphPlanConstructionTestSupport.check("paragraphEvidenceEmitsEverySection", "汉", "\"paragraphEvidence\"");

    @:test public static function negativeZeroAndExponentWidthsNormalize():Void
        PreparedParagraphPlanConstructionTestSupport.check("negativeZeroAndExponentWidthsNormalize", "汉", "\"width\"");

    @:test public static function jsonStringEscapesQuotesBackslashesAndControlCharacters():Void
        PreparedParagraphPlanConstructionTestSupport.check("jsonStringEscapesQuotesBackslashesAndControlCharacters", "汉", "\"cells\"");

    @:test public static function planWithDiagnosticsListsCapabilityIssuesAndAdvanceSuspects():Void
        PreparedParagraphPlanConstructionTestSupport.check("planWithDiagnosticsListsCapabilityIssuesAndAdvanceSuspects", "汉", "\"diagnostics\"");
}
