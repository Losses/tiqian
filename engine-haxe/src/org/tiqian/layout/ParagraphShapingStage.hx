package org.tiqian.layout;

import org.tiqian.core.LayoutInput;
import org.tiqian.core.TextRange;
import org.tiqian.core.Glyph;
import org.tiqian.shaping.ShapingResult;
import org.tiqian.core.BreakOpportunityDecisionInfo;
import org.tiqian.core.EmergencyTrackingEligibilityDecisionInfo;
import org.tiqian.layout.ProgressiveBreakDecisions.ProgressiveBreakOpportunity;
import std.SortedSet;
import std.SortedMap;

@:dataClass class ParagraphShapingStageResult {
    public final shapingResults:Array<ShapingResult>;
    public final hyphenOffsets:SortedSet<Int>;
    public final hyphenAdvance:Float;
    public final hyphenGlyphs:Array<Glyph>;
    public final substitutionRollbacks:haxe.ds.ObjectMap<TextRange, String>;
    public final breakOpportunityDecisions:Array<BreakOpportunityDecisionInfo>;
    public final emergencyTrackingEligibilityDecisions:Array<EmergencyTrackingEligibilityDecisionInfo>;
    public final progressiveBreakOffsets:SortedMap<Int, ProgressiveBreakOpportunity>;
    public final segmentShapingCache:haxe.ds.ObjectMap<TextRange, ShapingResult>;

    public function new(
        shapingResults:Array<ShapingResult>,
        hyphenOffsets:SortedSet<Int>,
        hyphenAdvance:Float,
        hyphenGlyphs:Array<Glyph>,
        substitutionRollbacks:haxe.ds.ObjectMap<TextRange, String>,
        breakOpportunityDecisions:Array<BreakOpportunityDecisionInfo>,
        emergencyTrackingEligibilityDecisions:Array<EmergencyTrackingEligibilityDecisionInfo>,
        progressiveBreakOffsets:SortedMap<Int, ProgressiveBreakOpportunity>,
        ?segmentShapingCache:haxe.ds.ObjectMap<TextRange, ShapingResult>
    ) {
        this.shapingResults = shapingResults;
        this.hyphenOffsets = hyphenOffsets;
        this.hyphenAdvance = hyphenAdvance;
        this.hyphenGlyphs = hyphenGlyphs;
        this.substitutionRollbacks = substitutionRollbacks;
        this.breakOpportunityDecisions = breakOpportunityDecisions;
        this.emergencyTrackingEligibilityDecisions = emergencyTrackingEligibilityDecisions;
        this.progressiveBreakOffsets = progressiveBreakOffsets;
        this.segmentShapingCache = segmentShapingCache != null ? segmentShapingCache : new haxe.ds.ObjectMap();
    }
}

class ParagraphShapingStage {
    public static function shapeParagraph(
        engine:ExplainableStubParagraphLayoutEngine,
        input:LayoutInput,
        rejectedTechnicalTiersBySpan:haxe.ds.ObjectMap<TextRange, SortedSet<Int>>
    ):ParagraphShapingStageResult {
        throw new haxe.exceptions.NotImplementedException("ring-r1 not-ported marker: shapeParagraph");
    }
}
