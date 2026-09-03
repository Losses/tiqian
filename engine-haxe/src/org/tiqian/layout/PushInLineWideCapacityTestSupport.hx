package org.tiqian.layout;

import org.tiqian.core.Cluster;
import org.tiqian.core.TextRange;
import org.tiqian.layout.LineOptimization.RepairOption;
import org.tiqian.test.trace.TracedAssertions;

class PushInLineWideCapacityTestSupport {
    public static function cluster(s:Int, e:Int, text:String, a:Float):Cluster return new Cluster(new TextRange(s, e), text, "test", a);

    public static function assertPushIn(o:RepairOption, expectedIndexes:Array<Int>, expectedShrink:Float, expectedCapacity:Null<Float>):Void {
        switch (o) {
            case PushIn(_, _, _, alloc, shrink, cap):
                final indexes:Array<Int> = []; for (a in alloc) indexes.push(a.clusterIndex);
                TracedAssertions.assertEqualsIntArray(expectedIndexes, indexes);
                if (expectedCapacity != null) TracedAssertions.assertEqualsFloat(expectedCapacity, cap);
                TracedAssertions.assertEqualsFloat(expectedShrink, shrink);
            case Hang(_, _, _): case CarryPrevious(_, _, _, _): case CarryNext(_, _, _): case LeaveRagged(_, _, _):
        }
    }

    public static function isPushIn(o:RepairOption):Bool return switch (o) {
        case PushIn(_, _, _, _, _, _): true;
        case Hang(_, _, _): false;
        case CarryPrevious(_, _, _, _): false;
        case CarryNext(_, _, _): false;
        case LeaveRagged(_, _, _): false;
    };

    public static function isCarryPrevious(o:RepairOption):Bool return switch (o) {
        case PushIn(_, _, _, _, _, _): false;
        case Hang(_, _, _): false;
        case CarryPrevious(_, _, _, _): true;
        case CarryNext(_, _, _): false;
        case LeaveRagged(_, _, _): false;
    };

    public static function isLeaveRagged(o:RepairOption):Bool return switch (o) {
        case PushIn(_, _, _, _, _, _): false;
        case Hang(_, _, _): false;
        case CarryPrevious(_, _, _, _): false;
        case CarryNext(_, _, _): false;
        case LeaveRagged(_, _, _): true;
    };
}
