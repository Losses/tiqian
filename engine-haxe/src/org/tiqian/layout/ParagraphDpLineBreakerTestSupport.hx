package org.tiqian.layout;
import org.tiqian.core.Cluster;
import org.tiqian.core.TextRange;
import org.tiqian.core.IntRange;
import std.SortedSet;
import std.SortedMap;
import org.tiqian.layout.ProgressiveBreakDecisions.ShrinkOpportunity;
import org.tiqian.layout.ProgressiveBreakDecisions.UnbreakableRanges;
import org.tiqian.layout.ProgressiveBreakDecisions.ProgressiveBreakOpportunity;
import org.tiqian.layout.ProgressiveBreakDecisions.ProgressiveBreakTier;
import org.tiqian.layout.LineOptimization.LineSolution;
import org.tiqian.test.trace.TracedAssertions;
import org.tiqian.layout.ProgressiveBreakDecisions.ProgressiveBreakTier;
import org.tiqian.test.trace.TracedAssertions;
class ParagraphDpLineBreakerTestSupport {
 public static function cluster(i:Int, ?text:String, ?advance:Float):Cluster return new Cluster(new TextRange(i,i+1), text == null ? "中" : text, "test", advance == null ? 16.0 : advance);
 public static function han(n:Int, ?advance:Float):Array<Cluster> { var a=[]; for(i in 0...n) a.push(cluster(i,"中",advance)); return a; }
 public static function latin():Array<Cluster> return [cluster(0,"a",30),cluster(1,"/",30),cluster(2,"b",25),cluster(3,"c",30),cluster(4,"d",30)];
 public static function ints(v:Array<Int>):SortedSet<Int> { var b=SortedSet.builder(); for(x in v)b.put(x); return b.build(); }
 public static function opportunities(v:Array<Int>, spans:Array<TextRange>, tiers:Array<ProgressiveBreakTier>):SortedMap<Int,ProgressiveBreakOpportunity> { var b=SortedMap.builder(); for(i in 0...v.length)b.put(v[i],new ProgressiveBreakOpportunity(tiers[i],spans[i])); return b.build(); }
 public static function solve(c:Array<Cluster>, width:Float, ?shrink:Array<ShrinkOpportunity>, ?hard:Array<Int>, ?push:Bool, ?ranges:UnbreakableRanges, ?progressive:SortedMap<Int,ProgressiveBreakOpportunity>, ?window:Int):LineSolution {
  var x=new ParagraphDpLineBreaker(window == null ? 8 : window); return x.breakLines(c,c,width,shrink,ranges,null,null,null,null,null,null,ints([]),8,null,null,push,null,ints(hard==null?[]:hard),null,progressive);
 }
 public static function tiles(s:LineSolution,n:Int):Void { var e=0; for(l in s.lines) if(l.clusterRange.start<=l.clusterRange.end){ TracedAssertions.assertEqualsInt(e,l.clusterRange.start); e=l.clusterRange.end+1; } TracedAssertions.assertEqualsInt(n,e); }
}
