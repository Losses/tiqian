package org.tiqian.layout;

import org.tiqian.core.*;
import org.tiqian.clreq.*;
import org.tiqian.font.FontRole;
import org.tiqian.shaping.TextShaper.ITextShaper;
import org.tiqian.shaping.TextShaper.ShapingInput;
import org.tiqian.shaping.TextShaper.ShapingResult;
import org.tiqian.shaping.TextShaper.ExplainableStubTextShaper;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.layout.ParagraphLayoutEngine.ExplainableStubParagraphLayoutEngine;
import std.SortedSet;

class QuoteClassificationEngineTestSupport {
    public static function begin(name:String):TestTraceRecorder { var t=new TestTraceRecorder("QuoteClassificationEngineTest"); t.section(name); return t; }
    public static function input(text:String, width:Float):LayoutInput return new LayoutInput(new TiqianTextContent(text),null,new ParagraphStyle(null,null,null,Ic.Zero),new LayoutConstraints(width));
    public static function layout(text:String, width:Float, ?engine:ExplainableStubParagraphLayoutEngine):LayoutResult return (engine == null ? new ExplainableStubParagraphLayoutEngine() : engine).layout(input(text,width));
    public static function set(values:Array<Int>):SortedSet<Int> { var b=SortedSet.builder(); for (v in values) b.put(v); return b.build(); }
    public static function indices(text:String):Array<Int> { var r:Array<Int>=[]; for (i in 0...text.length) if (isCurlyQuoteForTest(text.substring(i,i+1))) r.push(i); return r; }
    public static function roleAt(result:LayoutResult,index:Int):String { for (i in 0...result.debug.fontDecisions.length) { var d=result.debug.fontDecisions[i]; if (index>=d.range.start && index<d.range.end) return d.role; } return ""; }
    public static function isCurlyQuoteForTest(ch:String):Bool return ch=="\u2018" || ch=="\u2019" || ch=="\u201C" || ch=="\u201D";
    public static function lastIndex(text:String, mark:String):Int { var result=-1; var start=0; while (true) { var next=text.indexOf(mark,start); if (next<0) return result; result=next; start=next+1; } }
}

class ProportionalQuoteTextShaper implements ITextShaper {
    private final delegate:ExplainableStubTextShaper;
    public function new() delegate=new ExplainableStubTextShaper();
    public function shape(input:ShapingInput):ShapingResult {
        final result=delegate.shape(input);
        if (input.displayText!="\u201C" && input.displayText!="\u201D") return result;
        org.tiqian.test.trace.TracedAssertions.assertEqualsStringArray(["fwid=1"],input.openTypeFeatures);
        var clusters:Array<Cluster>=[]; for (i in 0...result.clusters.length) { var c=result.clusters[i]; clusters.push(new Cluster(c.range,c.text,c.fontKey,6.0,c.displayText,c.baselineShift,c.leadingLayoutAdvance,c.glyphInlineShift)); }
        var runs:Array<GlyphRun>=[];
        for (ri in 0...result.glyphRuns.length) { var run=result.glyphRuns[ri]; var gs:Array<Glyph>=[]; for (gi in 0...run.glyphs.length) { var g=run.glyphs[gi]; gs.push(new Glyph(g.id,g.clusterRange,6.0,g.x,g.y,g.renderFontKey,new Rect(1,-10,5,0),g.haltAdvance,g.haltPlacementX)); } var features:Array<String>=[]; for (fi in 0...run.openTypeFeatures.length) features.push(run.openTypeFeatures[fi]); runs.push(new GlyphRun(run.range,run.fontKey,gs,6.0,features)); }
        var decisions:Array<ShapingDecisionInfo>=[]; for (i in 0...result.decisions.length) { var d=result.decisions[i]; decisions.push(new ShapingDecisionInfo(d.range,d.sourceText,d.displayText,d.fontKey,d.glyphCount,6.0,d.source,d.reason,d.glyphsWithoutInkBounds,d.missingGlyphs,d.resolvedFace,d.script,d.language,d.strategy,d.featureEvidence,d.capabilityIssue)); }
        return new ShapingResult(clusters,runs,decisions);
    }
}
