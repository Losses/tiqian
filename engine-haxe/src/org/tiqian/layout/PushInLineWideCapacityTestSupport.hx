package org.tiqian.layout;

import org.tiqian.core.Cluster;
import org.tiqian.core.TextRange;
import org.tiqian.core.IntRange;
import org.tiqian.layout.ProgressiveBreakDecisions.ShrinkOpportunity;
import org.tiqian.layout.ProgressiveBreakDecisions.ShrinkChannel;
import org.tiqian.layout.ProgressiveBreakDecisions.UnbreakableRanges;

class PushInLineWideCapacityTestSupport {
    public static function cluster(s:Int,e:Int,text:String,a:Float):Cluster return new Cluster(new TextRange(s,e),text,"test",a);
}
