package org.tiqian.layout;

using std.Functional;

import org.tiqian.core.Cluster;
import org.tiqian.core.Glyph;
import org.tiqian.core.IntRange;
import org.tiqian.core.InlineObjectLineHeightDecisionInfo;
import org.tiqian.core.InlineObjectSpan;
import org.tiqian.core.LastLineAlignment;
import org.tiqian.core.LayoutInput;
import org.tiqian.core.LineBox;
import org.tiqian.core.LineDebugInfo;
import org.tiqian.core.LineEndReason;
import org.tiqian.core.MaxLinesDecisionInfo;
import org.tiqian.core.RubyLineHeightDecisionInfo;
import org.tiqian.core.RubyLineHeightMode;
import org.tiqian.core.RubySpan;
import org.tiqian.core.TextRange;
import org.tiqian.font.FontMetrics.FontMetricsRequest;
import org.tiqian.font.LayoutFontMetrics;
import org.tiqian.font.MetricBox;
import org.tiqian.font.RawFontMetrics;
import org.tiqian.layout.AnnotationGeometryStage.RubyFontGeometry;
import org.tiqian.layout.Justifier.JustificationPlan;
import org.tiqian.layout.LineOptimization.LineSolution;
import org.tiqian.layout.LineOptimization.LineCandidate;

@:dataClass class LineBoxStageResult { public final laidOutLines:Array<LineBox>; public final visibleLines:Array<LineBox>; public final maxLinesDecision:Null<MaxLinesDecisionInfo>; public final visibleLineRanges:Array<IntRange>; public function new(laidOutLines:Array<LineBox>,visibleLines:Array<LineBox>,maxLinesDecision:Null<MaxLinesDecisionInfo>,visibleLineRanges:Array<IntRange>){this.laidOutLines=laidOutLines;this.visibleLines=visibleLines;this.maxLinesDecision=maxLinesDecision;this.visibleLineRanges=visibleLineRanges;} }
@:dataClass class LineVerticalGeometryStageResult { public final rubyLineHeightDecision:Null<RubyLineHeightDecisionInfo>; public final inlineObjectLineHeightDecision:Null<InlineObjectLineHeightDecisionInfo>; public final lineBaseline:Array<Float>; public final lineTop:Array<Float>; public final lineBottom:Array<Float>; public function new(rubyLineHeightDecision:Null<RubyLineHeightDecisionInfo>,inlineObjectLineHeightDecision:Null<InlineObjectLineHeightDecisionInfo>,lineBaseline:Array<Float>,lineTop:Array<Float>,lineBottom:Array<Float>){this.rubyLineHeightDecision=rubyLineHeightDecision;this.inlineObjectLineHeightDecision=inlineObjectLineHeightDecision;this.lineBaseline=lineBaseline;this.lineTop=lineTop;this.lineBottom=lineBottom;} }
@:dataClass class ClusterMetricDecision {public final range:TextRange;public final sourceText:String;public final request:FontMetricsRequest;public final rawMetrics:RawFontMetrics;public final layoutMetrics:LayoutFontMetrics;public function new(range:TextRange,sourceText:String,request:FontMetricsRequest,rawMetrics:RawFontMetrics,layoutMetrics:LayoutFontMetrics){this.range=range;this.sourceText=sourceText;this.request=request;this.rawMetrics=rawMetrics;this.layoutMetrics=layoutMetrics;}}
@:dataClass class ResolvedLineMetrics {public final baseline:Float;public final height:Float;public final extraLeading:Float;public function new(baseline:Float,height:Float,?extraLeading:Null<Float>){this.baseline=baseline;this.height=height;this.extraLeading=extraLeading==null?0:extraLeading;}}
class LineGeometryStageFns {
 public static function resolveLineVerticalGeometry(input:LayoutInput,fontSize:Float,pinyinSpans:Array<RubySpan>,naturalClusters:Array<Cluster>,lineSolution:LineSolution,rubyFontGeometryBySpan:std.SortedMap<RubySpan,RubyFontGeometry>,existingInterlineSpace:Float,baseLineMetrics:ResolvedLineMetrics,baseFaceHeight:Float,rubyExtent:Float,inlineObjectByClusterIndex:Map<Int,InlineObjectSpan>,baseAscent:Float,baseDescent:Float):LineVerticalGeometryStageResult {
  final ext:Array<Float>=[]; for(line in lineSolution.lines){var x=0.0;for(ruby in pinyinSpans){final range=clusterIndexRangeFor(naturalClusters,ruby.baseRange);if(range!=null&&range.start<=line.clusterRange.end&&range.end>=line.clusterRange.start&&rubyFontGeometryBySpan.has(ruby))x=Math.max(x,rubyFontGeometryBySpan.get(ruby).requiredExtent);}ext.push(x);}
  final extras=ext.map(x -> Math.max(0,x-existingInterlineSpace)); final baselines:Array<Float>=[]; final tops:Array<Float>=[];final bottoms:Array<Float>=[];for(i in 0...ext.length){baselines.push((i==0?baseLineMetrics.baseline:baselines[i-1]+baseLineMetrics.height)+extras[i]);tops.push(0);bottoms.push(baselines[i]+baseLineMetrics.height-baseLineMetrics.baseline);}
  var hasExtra=false; for(x in extras) if(x>0) hasExtra=true;
  final reason=hasExtra?"ConditionalRubyLineHeight":"ExistingInterlineSpaceFitsRuby";
  final rd=pinyinSpans.length==0?null:new RubyLineHeightDecisionInfo(Type.enumConstructor(input.paragraphStyle.rubyLineHeightMode),baseLineMetrics.height,baseFaceHeight,rubyExtent,existingInterlineSpace,extras.length==0?0:extras[0],extras,[],reason);
  return new LineVerticalGeometryStageResult(rd,null,baselines,tops,bottoms);
 }
 public static function lineMetrics(self:Array<ClusterMetricDecision>,explicitLineHeight:Null<Float>,defaultLineHeight:Float,?spacingFloor:Null<Float>):ResolvedLineMetrics {var floorValue=0.0;if(spacingFloor!=null)floorValue=spacingFloor;final floor:Float=floorValue;if(self.length==0){var hValue=defaultLineHeight;if(explicitLineHeight!=null)hValue=explicitLineHeight;final h:Float=hValue;return new ResolvedLineMetrics(h*.75,h);}var src=self.filter(x -> x.layoutMetrics.metricBox==MetricBox.IdeographicEmBox);if(src.length==0)src=self;var a=src[0].layoutMetrics.ascent;var d=src[0].layoutMetrics.descent;for(x in src){a=Math.max(a,x.layoutMetrics.ascent);d=Math.max(d,x.layoutMetrics.descent);}final natural=a+d;var requestedValue=defaultLineHeight;if(explicitLineHeight!=null)requestedValue=explicitLineHeight;final requested:Float=requestedValue;final h=Math.max(requested,natural+floor);return new ResolvedLineMetrics(a+(h-natural)/2,h,h-natural);}
 public static function resolveInlineObjectLineBoundaryExtent(nominalBoundaryExtent:Float,currentContentBottomExtent:Float,baselineDistance:Float,nextContentTopExtent:Float):Float{return Math.max(currentContentBottomExtent,Math.min(nominalBoundaryExtent,Math.max(currentContentBottomExtent,baselineDistance-nextContentTopExtent)));}
 public static function clusterIndexRangeFor(self:Array<Cluster>,r:TextRange):Null<IntRange>{return PunctuationGeometryLedger.clusterIndexRangeFor(self,r);}
}
