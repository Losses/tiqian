package org.tiqian.layout;

import org.tiqian.clreq.*;
import org.tiqian.core.*;
import org.tiqian.layout.LineOptimization.LineCandidate;
import org.tiqian.layout.ProgressiveBreakDecisions.ShrinkOpportunity;
import org.tiqian.layout.ProgressiveBreakDecisions.ShrinkChannel;
import org.tiqian.layout.ProgressiveBreakDecisions.UnbreakableRanges;
import org.tiqian.layout.ProgressiveBreakDecisions.ProgressiveBreakOpportunity;
import std.SortedSet;
import std.SortedMap;

class LineAdjustmentPushInTestSupport {
    public static function cluster(i:Int, text:String, advance:Float):Cluster
        return new Cluster(new TextRange(i, i + 1), text, "test", advance);
    public static function ints(a:Array<Int>):SortedSet<Int> {
        var b = SortedSet.builder(); for (x in a) b.put(x); return b.build();
    }
    public static function line(r:IntRange, cs:Array<Cluster>):LineCandidate {
        var w = 0.0; for (i in r.start...r.end + 1) w += cs[i].advance;
        return new LineCandidate(r, new TextRange(r.start, r.end + 1), w, w);
    }
    public static function emptyRanges():UnbreakableRanges return new UnbreakableRanges([]);
    public static function emptyProgressive():SortedMap<Int, ProgressiveBreakOpportunity> {
        return SortedMap.builder().build();
    }
    public static function baseClusters():Array<Cluster> return [
        LineAdjustmentPushInTestSupport.cluster(0,"甲",30), LineAdjustmentPushInTestSupport.cluster(1,"乙",30),
        LineAdjustmentPushInTestSupport.cluster(2,"丙",20), LineAdjustmentPushInTestSupport.cluster(3,"丁",20),
        LineAdjustmentPushInTestSupport.cluster(4,"戊",20), LineAdjustmentPushInTestSupport.cluster(5,"己",20)];
    public static function fill(cs:Array<Cluster>, width:Float, ?shrink:Array<ShrinkOpportunity>, ?starts:Array<Int>, ?ends:Array<Int>, ?progressive:SortedMap<Int,ProgressiveBreakOpportunity>):Array<LineCandidate> {
        final a=[LineAdjustmentPushInTestSupport.line(new IntRange(0,1),cs),LineAdjustmentPushInTestSupport.line(new IntRange(2,cs.length-1),cs)];
        return LineRepair.applyFillPushIn(a,cs,cs,width,shrink==null?[]:shrink,0,1000000,null,LineAdjustmentPushInTestSupport.ints(ends==null?[]:ends),LineAdjustmentPushInTestSupport.emptyRanges(),2,LineAdjustmentPushInTestSupport.ints([0,1,2,3,4]),progressive);
    }
}

class PushInClreqResolver implements ClreqProfileResolver {
    final strategy:LineAdjustmentStrategy;
    public function new(strategy:LineAdjustmentStrategy) this.strategy = strategy;
    public function resolve(profileId:LayoutProfileId):ClreqProfile {
        final base = ClreqProfile.MainlandHorizontal;
        return new ClreqProfile(base.id, base.strictness, base.region, base.punctuationGlyphPolicy, null, base.autoSpace, base.gluePlacement,
            new AdjustmentStylePolicy(strategy), base.kinsokuMode, base.punctuationWidth);
    }
}
