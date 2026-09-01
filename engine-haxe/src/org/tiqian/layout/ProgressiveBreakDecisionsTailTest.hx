package org.tiqian.layout;
import org.tiqian.core.Cluster;
import org.tiqian.core.TextRange;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;
class ProgressiveBreakDecisionsTailTest {
 static function c(i:Int):Cluster return new Cluster(new TextRange(i,i+1),"中","test",16,"中");
 static function o():Map<Int,ProgressiveBreakOpportunity>{var m=new Map<Int,ProgressiveBreakOpportunity>();var s=new TextRange(0,5);m.set(2,new ProgressiveBreakOpportunity(Whitespace,s));m.set(4,new ProgressiveBreakOpportunity(Emergency,s));return m;}
 static function t(n:String,f:Void->Void):Void{new TestTraceRecorder("ProgressiveBreakDecisionsTailTest").section(n);f();}
 public static function infiniteLineLimitWithClustersAdmitsTheCleanestTier():Void t("infiniteLineLimitWithClustersAdmitsTheCleanestTier",function(){TracedAssertions.assertEqualsInt(2,ProgressiveBreakDecisions.decideProgressiveBreak(0,4,o(),[for(i in 0...5)c(i)]));});
 public static function infiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTier():Void t("infiniteStretchCeilingWithFiniteLineLimitAdmitsTheCleanestTier",function(){TracedAssertions.assertEqualsInt(2,ProgressiveBreakDecisions.decideProgressiveBreak(0,4,o(),[for(i in 0...5)c(i)],200));});
 public static function flush():Void new TestTraceRecorder("ProgressiveBreakDecisionsTailTest").flush();
}
