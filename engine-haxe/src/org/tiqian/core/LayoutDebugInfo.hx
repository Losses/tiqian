package org.tiqian.core;

import std.ReadOnlyArray;

class LayoutDebugInfo {
    public final metricDecisions:ReadOnlyArray<MetricDecisionInfo>;
    public final geometryDecisions:ReadOnlyArray<ClusterGeometryDecisionInfo>;
    public final autoSpaceDecisions:ReadOnlyArray<AutoSpaceDecisionInfo>;
    public final rubyDecisions:ReadOnlyArray<RubyDecisionInfo>;
    public final bopomofoDecisions:ReadOnlyArray<BopomofoDecisionInfo>;
    public final maxLinesDecision:Null<MaxLinesDecisionInfo>;

    public function new(maxLinesDecision:Null<MaxLinesDecisionInfo>, ?metricDecisions:Array<MetricDecisionInfo>, ?geometryDecisions:Array<ClusterGeometryDecisionInfo>, ?autoSpaceDecisions:Array<AutoSpaceDecisionInfo>, ?rubyDecisions:Array<RubyDecisionInfo>, ?bopomofoDecisions:Array<BopomofoDecisionInfo>) {
        this.maxLinesDecision = maxLinesDecision;
        this.metricDecisions = metricDecisions == null ? [] : metricDecisions;
        this.geometryDecisions = geometryDecisions == null ? [] : geometryDecisions;
        this.autoSpaceDecisions = autoSpaceDecisions == null ? [] : autoSpaceDecisions;
        this.rubyDecisions = rubyDecisions == null ? [] : rubyDecisions;
        this.bopomofoDecisions = bopomofoDecisions == null ? [] : bopomofoDecisions;
    }

    public static function withMetricDecisions(values:Array<MetricDecisionInfo>):LayoutDebugInfo {
        return new LayoutDebugInfo(null, values, [], [], [], []);
    }

    public static function withGeometryDecisions(values:Array<ClusterGeometryDecisionInfo>):LayoutDebugInfo {
        return new LayoutDebugInfo(null, [], values, [], [], []);
    }

    public static function withAutoSpaceDecisions(values:Array<AutoSpaceDecisionInfo>):LayoutDebugInfo {
        return new LayoutDebugInfo(null, [], [], values, [], []);
    }

    public static function withRubyDecisions(values:Array<RubyDecisionInfo>):LayoutDebugInfo {
        return new LayoutDebugInfo(null, [], [], [], values, []);
    }

    public static function withBopomofoDecisions(values:Array<BopomofoDecisionInfo>):LayoutDebugInfo {
        return new LayoutDebugInfo(null, [], [], [], [], values);
    }
}
