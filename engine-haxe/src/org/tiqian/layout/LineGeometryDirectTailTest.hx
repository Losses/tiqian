package org.tiqian.layout;

import org.tiqian.core.*;
import org.tiqian.font.FontMetrics.FontMetricsRequest;
import org.tiqian.font.FontRole;
import org.tiqian.font.RawFontMetrics;
import org.tiqian.font.LayoutFontMetrics;
import org.tiqian.font.FontMetricsPolicy;
import org.tiqian.font.BaselinePolicy;
import org.tiqian.layout.AnnotationGeometryStage.RubyFontGeometry;
import org.tiqian.layout.LineGeometryStage.LineGeometryStageFns;
import org.tiqian.layout.LineGeometryStage.LineVerticalGeometryStageResult;
import org.tiqian.layout.LineGeometryStage.ClusterMetricDecision;
import org.tiqian.layout.LineGeometryStage.ResolvedLineMetrics;
import org.tiqian.layout.LineOptimization.LineCandidate;
import org.tiqian.layout.LineOptimization.LineSolution;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;
import std.SortedMap;

@:test class LineGeometryDirectTailTest {
    static function start(n:String):Void new TestTraceRecorder("LineGeometryDirectTailTest").section(n);
    static function c(index:Int):Cluster return new Cluster(new TextRange(index * 2, index * 2 + 2), "𠀀", "k", 16.0, "𠀀");
    static function clusters():Array<Cluster> { var r:Array<Cluster>=[]; var i=0; while(i<4){r.push(c(i));i++;} return r; }
    static function input():LayoutInput return new LayoutInput(new TiqianTextContent("中文测试"), new TextStyle(),
        new ParagraphStyle(LastLineAlignment.Start, WritingMode.HorizontalTb, null, null, Ic.Zero,
            new MeasureAdaptiveFirstLineIndent(14.0, 1.0, 2.0), new LineLengthGrid(true, null)), new LayoutConstraints(320.0));
    static function line(range:IntRange, cs:Array<Cluster>):LineCandidate return new LineCandidate(range, new TextRange(range.start * 2, (range.end + 1) * 2), 32.0, 32.0);
    static function mapRuby(entries:Array<{s:RubySpan,g:RubyFontGeometry}>):SortedMap<RubySpan,RubyFontGeometry> {
        var b=SortedMap.builder(); var i=0; while(i<entries.length){b.put(entries[i].s,entries[i].g);i++;} return b.build();
    }
    static function geometry(?spans:Array<RubySpan>, lines:Array<LineCandidate>, ?rmap:SortedMap<RubySpan,RubyFontGeometry>, ?objects:Map<Int,InlineObjectSpan>, ?existing:Float=0.0, ?ascent:Float=8.0, ?descent:Float=4.0):LineVerticalGeometryStageResult {
        var cs=clusters();
        return LineGeometryStageFns.resolveLineVerticalGeometry(input(),16.0,spans==null?[]:spans,cs,new LineSolution(lines),rmap==null?SortedMap.builder().build():rmap,existing,new ResolvedLineMetrics(12.0,16.0),16.0,0.0,objects,ascent,descent);
    }
    static function ruby(range:TextRange, text:String):RubySpan return new RubySpan(range,text,[],RubyKind.Pinyin);
    static function rg(width:Float, required:Float):RubyFontGeometry return new RubyFontGeometry(width,6.0,2.0,required,[]);

    @:test public static function rubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtents():Void { start("rubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtents"); var a=ruby(new TextRange(4,8),"y"); var m=ruby(new TextRange(1,3),"w"); var z=ruby(new TextRange(0,4),"z"); var r=geometry([a,m,z],[line(new IntRange(0,3),clusters())],mapRuby([{s:a,g:rg(12,8)}])); TracedAssertions.assertEqualsInt(1,r.lineBaseline.length); TracedAssertions.assertTrue(r.lineBaseline[0]>0); }
    @:test public static function rubiesOnBothLinesExerciseBothSidesOfTheOverlapTest():Void { start("rubiesOnBothLinesExerciseBothSidesOfTheOverlapTest"); var a=ruby(new TextRange(0,4),"a"); var b=ruby(new TextRange(4,8),"b"); var r=geometry([a,b],[line(new IntRange(0,1),clusters()),line(new IntRange(2,3),clusters())],mapRuby([{s:a,g:rg(12,8)},{s:b,g:rg(12,6)}])); TracedAssertions.assertEqualsInt(2,r.lineBaseline.length); TracedAssertions.assertTrue(r.lineBaseline[1]>r.lineBaseline[0]); }
    @:test public static function emptyLineSolutionYieldsZeroArraysAndZeroMaxExtra():Void { start("emptyLineSolutionYieldsZeroArraysAndZeroMaxExtra"); var r=geometry([ruby(new TextRange(0,2),"y")],[]); TracedAssertions.assertEqualsInt(0,r.lineBaseline.length); TracedAssertions.assertEqualsInt(0,r.lineTop.length); TracedAssertions.assertEqualsInt(0,r.lineBottom.length); TracedAssertions.assertTrue(r.rubyLineHeightDecision!=null); TracedAssertions.assertEqualsFloat(0,r.rubyLineHeightDecision.maxExtra); }
    static function objectCase(objectAscent:Float, extent:Float):LineVerticalGeometryStageResult { var r=ruby(new TextRange(4,8),"y"); return geometry([r],[line(new IntRange(0,1),clusters()),line(new IntRange(2,3),clusters())],mapRuby([{s:r,g:rg(12,extent)}]),null,0,8,4); }
    @:test public static function objectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZero():Void { start("objectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZero"); var r=objectCase(10,8); TracedAssertions.assertEqualsInt(2,r.lineBaseline.length); TracedAssertions.assertTrue(r.lineBaseline[1]>r.lineBaseline[0]); }
    @:test public static function objectTopIntrusionDominatingRubyDemandAddsBoundaryClearance():Void { start("objectTopIntrusionDominatingRubyDemandAddsBoundaryClearance"); var r=objectCase(20,8); TracedAssertions.assertEqualsInt(2,r.lineBaseline.length); TracedAssertions.assertTrue(r.lineBaseline[1]>r.lineBaseline[0]); }
    @:test public static function objectFlushWithBaseTopSkipsIntrusionConjunctionEarly():Void { start("objectFlushWithBaseTopSkipsIntrusionConjunctionEarly"); var r=objectCase(8,8); TracedAssertions.assertEqualsInt(2,r.lineBaseline.length); TracedAssertions.assertTrue(r.lineBaseline[1]>r.lineBaseline[0]); }
    @:test public static function metricListWithoutIdeographicEmBoxFallsBackToAllClusters():Void { start("metricListWithoutIdeographicEmBoxFallsBackToAllClusters"); var q=new FontMetricsRequest("latin",16,FontRole.LatinText,"zh-Hans"); var d=new ClusterMetricDecision(new TextRange(0,1),"a",q,new RawFontMetrics(14,4),new LayoutFontMetrics(14,4,0,FontMetricsPolicy.Raw,BaselinePolicy.Alphabetic)); var r=LineGeometryStageFns.lineMetrics([d],null,24,0); TracedAssertions.assertTrue(r.baseline>=14); TracedAssertions.assertTrue(r.height>=18); }
    @:test public static function emptyMetricListTakesEmptyParagraphBaselineFallback():Void { start("emptyMetricListTakesEmptyParagraphBaselineFallback"); var a=LineGeometryStageFns.lineMetrics([],null,24,0); TracedAssertions.assertEqualsFloat(24,a.height); TracedAssertions.assertEqualsFloat(18,a.baseline); var b=LineGeometryStageFns.lineMetrics([],30,24,0); TracedAssertions.assertEqualsFloat(30,b.height); TracedAssertions.assertEqualsFloat(22.5,b.baseline); }
}
