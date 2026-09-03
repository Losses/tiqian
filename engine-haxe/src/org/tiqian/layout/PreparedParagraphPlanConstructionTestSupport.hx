package org.tiqian.layout;

import org.tiqian.core.*;
import org.tiqian.layout.PreparedParagraph.PreparedParagraphFns;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;

class PreparedParagraphPlanConstructionTestSupport {
    public static function result(text:String):LayoutResult {
        final width = 480.0;
        final advance = 16 * text.length;
        final c = [new Cluster(new TextRange(0, text.length), text, "cjk", advance)];
        final g = [new Glyph(1, new TextRange(0, text.length), advance)];
        return new LayoutResult(new LayoutInput(new TiqianTextContent(text), new TextStyle(), null, new LayoutConstraints(width)), new Size(width, 24), c,
            [new GlyphRun(new TextRange(0, text.length), "cjk", g, advance)], [
                new LineBox(new TextRange(0, text.length), new IntRange(0, 0), 20, 0, 24, advance, advance, advance, null, null, null, null, null,
                    new LineDebugInfo(null))
            ], new LayoutDebugInfo(null));
    }

    public static function check(name:String, text:String, needle:String):Void {
        final t = new TestTraceRecorder("PreparedParagraphPlanConstructionTest");
        t.section(name);
        final json = PreparedParagraphFns.toPlanWithDiagnosticsJson(result(text), true, 0.0001);
        TracedAssertions.assertTrue(json.indexOf(needle) >= 0, json);
    }
}
