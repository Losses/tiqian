package org.tiqian.layout;

import org.tiqian.core.*;
import org.tiqian.layout.PreparedParagraph.PreparedParagraphFns;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;

class PreparedParagraphPlanConstructionTest {
    @:test public static function openTypeFeaturesAndRenderFontFamilyAttachPerCluster():Void {
        final t = new TestTraceRecorder("PreparedParagraphPlanConstructionTest");
        t.section("openTypeFeaturesAndRenderFontFamilyAttachPerCluster");
        final json = PreparedParagraphFns.toPreparedParagraphJson(PreparedParagraphPlanConstructionTestSupport.openTypeFeaturesAndRenderFontFamilyAttachPerCluster(),
            true);
        TracedAssertions.assertTrue(json.indexOf("\"openTypeFeatures\":[\"kern\",\"liga\"]") >= 0, json);
        TracedAssertions.assertTrue(json.indexOf("\"renderFontFamily\":\"Noto Serif CJK\"") >= 0, json);
        TracedAssertions.assertFalse(json.indexOf("shapingBoundary") >= 0, json);
    }

    @:test public static function multiUnitClusterMarksShapingBoundary():Void {
        final t = new TestTraceRecorder("PreparedParagraphPlanConstructionTest");
        t.section("multiUnitClusterMarksShapingBoundary");
        final json = PreparedParagraphFns.toPreparedParagraphJson(PreparedParagraphPlanConstructionTestSupport.multiUnitClusterMarksShapingBoundary());
        TracedAssertions.assertTrue(json.indexOf("\"shapingBoundary\":true") >= 0, json);
    }

    @:test public static function inlineObjectCellEmitsAdvanceOverride():Void {
        final t = new TestTraceRecorder("PreparedParagraphPlanConstructionTest");
        t.section("inlineObjectCellEmitsAdvanceOverride");
        final r = PreparedParagraphPlanConstructionTestSupport.inlineObjectCellEmitsAdvanceOverride();
        final json = PreparedParagraphFns.toPreparedParagraphJson(r, true);
        TracedAssertions.assertTrue(json.indexOf("\"inlineObject\":24") >= 0, json);
        TracedAssertions.assertTrue(json.indexOf("\"advance\":10") >= 0, json);
        final emptyClusters = [];
        for (i in 0...r.clusters.length) {
            final c = r.clusters[i];
            emptyClusters.push(new Cluster(c.range, c.text, c.fontKey, c.advance, c.range.start == 1 ? "" : c.displayText));
        }
        final emptyDisplay = new LayoutResult(r.input, r.size, emptyClusters, r.glyphRuns, r.lines, r.debug);
        final plain = PreparedParagraphFns.toPreparedParagraphJson(emptyDisplay, false);
        TracedAssertions.assertFalse(plain.indexOf("\"inlineObject\"") >= 0, plain);
        TracedAssertions.assertFalse(plain.indexOf("\"rangeStart\":1") >= 0, plain);
        final evidence = PreparedParagraphFns.toPreparedParagraphJson(emptyDisplay, true);
        TracedAssertions.assertTrue(evidence.indexOf("\"inlineObject\":24") >= 0, evidence);
    }

    @:test public static function styleDeltaListsOnlyPaintFields():Void {
        final t = new TestTraceRecorder("PreparedParagraphPlanConstructionTest");
        t.section("styleDeltaListsOnlyPaintFields");
        final json = PreparedParagraphFns.toPreparedParagraphJson(PreparedParagraphPlanConstructionTestSupport.styleDeltaListsOnlyPaintFields(), true);
        TracedAssertions.assertTrue(json.indexOf("\"style\":{\"fontSize\":20,\"fontWeight\":700,\"italic\":true}") >= 0, json);
        TracedAssertions.assertTrue(json.indexOf("\"style\":{}") >= 0, json);
        TracedAssertions.assertEquals(2, json.split("\"style\":").length - 1);
    }

    @:test public static function dashClusterEmitsShapingEvidenceBlock():Void {
        return runEvidence("dashClusterEmitsShapingEvidenceBlock", PreparedParagraphPlanConstructionTestSupport.dashClusterEmitsShapingEvidenceBlock());
    }

    @:test public static function punctuationInkFloorAndLatinRoleMarkCells():Void {
        return runPunctuation();
    }

    @:test public static function zeroWidthBreakClusterSurvivesEmptyDisplayText():Void {
        return runZeroWidth();
    }

    @:test public static function paragraphEvidenceEmitsEverySection():Void {
        return runParagraphEvidence();
    }

    @:test public static function negativeZeroAndExponentWidthsNormalize():Void {
        return runNegativeZero();
    }

    @:test public static function jsonStringEscapesQuotesBackslashesAndControlCharacters():Void {
        return runEscapes();
    }

    @:test public static function planWithDiagnosticsListsCapabilityIssuesAndAdvanceSuspects():Void {
        return runDiagnostics();
    }

    static function begin(name:String):TestTraceRecorder {
        final t = new TestTraceRecorder("PreparedParagraphPlanConstructionTest");
        t.section(name);
        return t;
    }

    static function runEvidence(name:String, r:LayoutResult):Void {
        final t = begin(name);
        final j = PreparedParagraphFns.toPreparedParagraphJson(r, true);
        TracedAssertions.assertTrue(j.indexOf("\"dashStrategy\":\"PairedEmDash\"") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"shapingLanguage\":\"zh-Hans\"") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"resolvedFace\":\"NotoSansCJK\"") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"glyphIds\":\"9,10\"") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"shapingEvidence\":\"dash-reason\"") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"naturalWidth\":32") >= 0, j);
    }

    static function runPunctuation():Void {
        final t = begin("punctuationInkFloorAndLatinRoleMarkCells");
        final j = PreparedParagraphFns.toPreparedParagraphJson(PreparedParagraphPlanConstructionTestSupport.punctuationInkFloorAndLatinRoleMarkCells(), true);
        TracedAssertions.assertTrue(j.indexOf("\"punctuationInkFloor\":6") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"punctuationBodyWidth\":16") >= 0, j);
        TracedAssertions.assertEquals(1, j.split("\"punctuationInkFloor\":").length - 1);
        TracedAssertions.assertTrue(j.indexOf("\"latin\":true") >= 0, j);
    }

    static function runZeroWidth():Void {
        final t = begin("zeroWidthBreakClusterSurvivesEmptyDisplayText");
        final r = PreparedParagraphPlanConstructionTestSupport.zeroWidthBreakClusterSurvivesEmptyDisplayText();
        final j = PreparedParagraphFns.toPreparedParagraphJson(r);
        TracedAssertions.assertTrue(j.indexOf("\"display\":\"\",\"drawX\":16") >= 0, j);
        TracedAssertions.assertEquals(3, j.split("\"source\":").length - 1);
        final e = PreparedParagraphFns.toPreparedParagraphJson(r, true);
        TracedAssertions.assertTrue(e.indexOf("\"dashStrategy\":\"ZeroWidthNoShape\"") >= 0, e);
        TracedAssertions.assertTrue(e.indexOf("\"shapingEvidence\":\"no-shape\"") >= 0, e);
        TracedAssertions.assertFalse(e.indexOf("shapingLanguage") >= 0, e);
        TracedAssertions.assertFalse(e.indexOf("resolvedFace") >= 0, e);
        TracedAssertions.assertFalse(e.indexOf("glyphIds") >= 0, e);
    }

    static function runParagraphEvidence():Void {
        final t = begin("paragraphEvidenceEmitsEverySection");
        final j = PreparedParagraphFns.toPreparedParagraphJson(PreparedParagraphPlanConstructionTestSupport.paragraphEvidenceEmitsEverySection(), true);
        TracedAssertions.assertTrue(j.indexOf("\"emphasisRanges\":[[0,1],[1,2]]") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"inlineEdges\":[{\"offset\":0,\"inlineStart\":2.5},{\"offset\":2,\"inlineEnd\":4.5}]") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"rubyDecisions\":[{\"baseRangeStart\":0") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"ascent\":6") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"fontFamilies\":[\"RubyKai\",\"RubyLatin\"]") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"bopomofoDecisions\":[{\"baseRangeStart\":1") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"role\":\"Symbol\"") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"role\":\"Tone\"") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"fontFamilies\":[\"BopomofoKai\",\"BopomofoLatin\"]") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"decorationSegments\":[{\"kind\":\"ProperNoun\"") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"kind\":\"BookTitle\"") >= 0, j);
        TracedAssertions.assertFalse(j.indexOf("Emphasis") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"emphasisDots\":[{\"clusterRangeStart\":0,\"anchorX\":8,\"anchorY\":22,\"dotDiameter\":2}]") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"fontSize\":16") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"overlayWidth\":480") >= 0, j);
    }

    static function runNegativeZero():Void {
        final t = begin("negativeZeroAndExponentWidthsNormalize");
        final j = PreparedParagraphFns.toPreparedParagraphJson(PreparedParagraphPlanConstructionTestSupport.negativeZeroAndExponentWidthsNormalize());
        TracedAssertions.assertTrue(j.indexOf("\"width\":1.0000000200408773e+21") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"height\":0") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"indent\":0") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("\"hyphenAdvance\":0") >= 0, j);
    }

    static function runEscapes():Void {
        final t = begin("jsonStringEscapesQuotesBackslashesAndControlCharacters");
        final j = PreparedParagraphFns.toPreparedParagraphJson(PreparedParagraphPlanConstructionTestSupport.escapes());
        final ss = ["\\\"", "\\\\", "\\b", "\\f", "\\n", "\\r", "\\t", "\\u0001"];
        for (i in 0...ss.length) {
            final s = ss[i];
            TracedAssertions.assertTrue(j.indexOf(s) >= 0, j);
        }
    }

    static function runDiagnostics():Void {
        final t = begin("planWithDiagnosticsListsCapabilityIssuesAndAdvanceSuspects");
        final j = PreparedParagraphFns.toPlanWithDiagnosticsJson(PreparedParagraphPlanConstructionTestSupport.diagnostics(), false, 0.5);
        final d = j.substr(j.indexOf("\"diagnostics\":"));
        for (s in [
            "\"name\":\"InvalidWebShapingAdvance\"",
            "\"reason\":\"capability-reason\"",
            "\"rangeStart\":0",
            "\"rangeEnd\":1",
            "\"displayText\":\"零\"",
            "\"advance\":\"0\"",
            "\"advance\":\"NaN\"",
            "\"advance\":\"Infinity\""
        ])
            TracedAssertions.assertTrue(d.indexOf(s) >= 0, j);
        TracedAssertions.assertFalse(d.indexOf("\"advance\":\"32\"") >= 0, j);
        TracedAssertions.assertTrue(j.indexOf("{\"plan\":\"") == 0, j.substr(0, 20));
    }
}
