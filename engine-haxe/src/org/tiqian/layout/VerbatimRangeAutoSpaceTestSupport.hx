package org.tiqian.layout;

import org.tiqian.core.*;
import org.tiqian.layout.ParagraphLayoutEngine.ExplainableStubParagraphLayoutEngine;

class VerbatimRangeAutoSpaceTestSupport {
    public static function layout(text:String, suppressed:Array<TextRange>):LayoutResult {
        return new ExplainableStubParagraphLayoutEngine().layout(new LayoutInput(new TiqianTextContent(text, null, null, null, suppressed), null,
            new ParagraphStyle(null, null, null, Ic.Zero), new LayoutConstraints(640.0)));
    }
    public static function rendered(result:LayoutResult):String {
        final parts:Array<Dynamic> = [];
        for (i in 0...result.debug.autoSpaceDecisions.length) parts.push(result.debug.autoSpaceDecisions[i]);
        return Std.string(parts);
    }
    public static function count(result:LayoutResult, reason:String):Int {
        var n = 0;
        for (i in 0...result.debug.autoSpaceDecisions.length) {
            final d = result.debug.autoSpaceDecisions[i];
            if (d.reason == reason) n++;
        }
        return n;
    }
}
