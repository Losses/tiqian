package org.tiqian.layout;
import org.tiqian.core.*;
import org.tiqian.layout.PreparedParagraph.PreparedParagraphFns;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;
class PreparedParagraphPlanConstructionTestSupport {
 public static function result(text:String):LayoutResult { final c=[new Cluster(new TextRange(0,text.length),text,"cjk",16*text.length)]; final g=[new Glyph(1,new TextRange(0,text.length),16*text.length)]; return new LayoutResult(new LayoutInput(new TiqianTextContent(text),new TextStyle(),null,new LayoutConstraints(200)),new Size(200,24),c,[new GlyphRun(new TextRange(0,text.length),"cjk",g,16*text.length)],[new LineBox(new TextRange(0,text.length),new IntRange(0,0),20,0,24,16*text.length,16*text.length,16*text.length,null,null,null,null,null,new LineDebugInfo(null))],new LayoutDebugInfo(null)); }
 public static function check(name:String,text:String,needle:String):Void { final t=new TestTraceRecorder("PreparedParagraphPlanConstructionTest");t.section(name); final json=PreparedParagraphFns.toPlanWithDiagnosticsJson(result(text),true,0.0001);TracedAssertions.assertTrue(json.indexOf(needle)>=0,json); }
}
