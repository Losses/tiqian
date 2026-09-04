package org.tiqian.layout;

import org.tiqian.clreq.*;
import org.tiqian.core.*;
import org.tiqian.linebreak.*;
import org.tiqian.linebreak.Hyphenator.NoHyphenator;
import org.tiqian.layout.LineBreaker.LookaheadLineBreaker;
import org.tiqian.test.trace.*;
import org.tiqian.layout.ParagraphLayoutEngine.ExplainableStubParagraphLayoutEngine;

class LineBreakRepairEngineTestSupport {
    public static function input(text:String, width:Float, ?spans:Array<LineBreakSpan>):LayoutInput
        return new LayoutInput(new TiqianTextContent(text, spans == null ? [] : spans), null,
            new ParagraphStyle(null, null, null, Ic.Zero), new LayoutConstraints(width));
    public static function layout(text:String, width:Float, ?breaker:LineBreaker, ?hyphenator:Hyphenator, ?spans:Array<LineBreakSpan>):LayoutResult {
        return new ExplainableStubParagraphLayoutEngine(null,null,null,null,null,null,null,null,breaker,null,null,hyphenator,null).layout(input(text,width,spans));
    }
    public static function lineText(r:LayoutResult, n:Int):String {
        final l=r.lines[n]; var s=""; for(i in l.clusterRange.start...l.clusterRange.end) s+=r.clusters[i].text; return s;
    }
    public static function hasText(r:LayoutResult, s:String):Bool { for(i in 0...r.clusters.length) if(r.clusters[i].text==s)return true; return false; }
    public static function noHyphen(r:LayoutResult):Bool { for(i in 0...r.lines.length) if(r.lines[i].hyphenAdvance!=0)return false; return true; }
    public static function fixed():ExplainableStubParagraphLayoutEngine {
        return new ExplainableStubParagraphLayoutEngine(null,null,new FixedProfileResolver(),null,null,null,null,null,null,null,null,new NoHyphenator(),null);
    }
    public static function blobTest(t:TestTraceRecorder,w:Float, ignored:Bool):Void { final p="为什么历史是 ",s=StringTools.lpad("", "s", 40)+"herstory";final r=LineBreakRepairEngineTestSupport.layout(p+s,w, new LookaheadLineBreaker(),new NoHyphenator());var x=LineBreakRepairEngineTestSupport.lineText(r,0);TracedAssertions.assertTrue(x.length>7,"first line should carry part of the long opaque token instead of stretching only '"+p+"': "+x);TracedAssertions.assertTrue(LineBreakRepairEngineTestSupport.noHyphen(r));TracedAssertions.assertTrue(!LineBreakRepairEngineTestSupport.hasText(r,s)); }
    public static function kinsokuStart(n:String):TestTraceRecorder { final t=new TestTraceRecorder("KinsokuAndCohesionRepairEngineTest");t.section(n);return t; }
}
class FixedProfileResolver implements ClreqProfileResolver {
    public function new() {}
    public function resolve(id:LayoutProfileId):ClreqProfile {
        final b=ClreqProfile.MainlandHorizontal;
        return new ClreqProfile(b.id,b.strictness,b.region,b.punctuationGlyphPolicy,null,b.autoSpace,b.gluePlacement,b.adjustment,KinsokuMode.Fixed(KinsokuLevel.Basic,HangingPunctuationStyle.Disabled),b.punctuationWidth);
    }
}
