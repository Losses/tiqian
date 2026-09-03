package org.tiqian.layout;

import org.tiqian.core.LayoutDebugInfo;
import org.tiqian.font.FontPolicy.FontDecision;
import org.tiqian.clreq.ClreqPunctuationGlyphSubstitutor;
import org.tiqian.core.TextRange;
import org.tiqian.core.RoleOverrideInfo;
import org.tiqian.core.LineBox;
import org.tiqian.core.Cluster;
import org.tiqian.core.AutoSpaceDecisionInfo;
import org.tiqian.core.LineEdgeTrimDecisionInfo;
import org.tiqian.core.DecorationDecisionInfo;
import org.tiqian.core.DecorationSegmentInfo;
import org.tiqian.core.RubyDecisionInfo;
import org.tiqian.core.BopomofoDecisionInfo;
import org.tiqian.core.MandatoryBreakDecisionInfo;
import org.tiqian.core.MaxLinesDecisionInfo;
import org.tiqian.core.LineSpacingDecisionInfo;
import org.tiqian.core.RubyLineHeightDecisionInfo;
import org.tiqian.core.InlineObjectLineHeightDecisionInfo;
import org.tiqian.core.KinsokuDecisionInfo;
import org.tiqian.core.ContextualKinsokuDecisionInfo;
import org.tiqian.core.LineLengthGridDecisionInfo;
import org.tiqian.core.FirstLineIndentDecisionInfo;
import org.tiqian.core.InlineBoxDecisionInfo;
import org.tiqian.core.InlineObjectDecisionInfo;
import org.tiqian.core.InlineObjectPunctuationAttachmentDecisionInfo;
import org.tiqian.core.ZeroWidthBreakDecisionInfo;
import org.tiqian.core.BreakOpportunityDecisionInfo;
import org.tiqian.core.EmergencyTrackingEligibilityDecisionInfo;
import org.tiqian.core.ShapingDecisionInfo;
import org.tiqian.core.ClusterGeometryDecisionInfo;
import org.tiqian.layout.LineGeometryStage.ClusterMetricDecision;
import org.tiqian.layout.ProgressiveBreakDecisions.ProgressiveBreakOpportunity;
import org.tiqian.layout.PunctuationModel.PunctuationAtom;
import org.tiqian.layout.PunctuationModel.PunctuationSpacingCompressionResult;
import org.tiqian.layout.PunctuationGeometryLedger.AttachedInlinePunctuationBoundaryResult;
import org.tiqian.layout.LineOptimization.LineSolution;
import org.tiqian.layout.Justifier.JustificationPlan;
import org.tiqian.layout.ParagraphLayoutEngine.ExplainableStubParagraphLayoutEngine;

@:dataClass class LayoutDebugStageInput {
    public final text:String;
    public final fontDecisions:Array<FontDecision>;
    public final punctuationGlyphSubstitutor:ClreqPunctuationGlyphSubstitutor;
    public final substitutionRollbacks:std.SortedMap<TextRange, String>;
    public final shapingDecisions:Array<ShapingDecisionInfo>;
    public final metricDecisions:Array<ClusterMetricDecision>;
    public final punctuationAtoms:Array<PunctuationAtom>;
    public final geometryDecisions:Array<ClusterGeometryDecisionInfo>;
    public final spacingPlan:PunctuationSpacingCompressionResult;
    public final attachedPunctuationBoundary:AttachedInlinePunctuationBoundaryResult;
    public final roleOverrideInfos:Array<RoleOverrideInfo>;
    public final laidOutLines:Array<LineBox>;
    public final lineSolution:LineSolution;
    public final clusters:Array<Cluster>;
    public final justificationPlans:Array<Null<JustificationPlan>>;
    public final autoSpaceDecisions:Array<AutoSpaceDecisionInfo>;
    public final edgeTrimDecisions:Array<LineEdgeTrimDecisionInfo>;
    public final decorationDecisions:Array<DecorationDecisionInfo>;
    public final decorationSegments:Array<DecorationSegmentInfo>;
    public final rubyDecisions:Array<RubyDecisionInfo>;
    public final bopomofoDecisions:Array<BopomofoDecisionInfo>;
    public final mandatoryBreakDecisions:Array<MandatoryBreakDecisionInfo>;
    public final maxLinesDecision:Null<MaxLinesDecisionInfo>;
    public final lineSpacingDecision:Null<LineSpacingDecisionInfo>;
    public final rubyLineHeightDecision:Null<RubyLineHeightDecisionInfo>;
    public final inlineObjectLineHeightDecision:Null<InlineObjectLineHeightDecisionInfo>;
    public final kinsokuDecision:KinsokuDecisionInfo;
    public final contextualKinsokuDecisions:Array<ContextualKinsokuDecisionInfo>;
    public final lineLengthGridDecision:LineLengthGridDecisionInfo;
    public final firstLineIndentDecision:FirstLineIndentDecisionInfo;
    public final inlineBoxDecisions:Array<InlineBoxDecisionInfo>;
    public final inlineObjectDecisions:Array<InlineObjectDecisionInfo>;
    public final inlineObjectPunctuationAttachmentDecisions:Array<InlineObjectPunctuationAttachmentDecisionInfo>;
    public final zeroWidthBreakDecisions:Array<ZeroWidthBreakDecisionInfo>;
    public final breakOpportunityDecisions:Array<BreakOpportunityDecisionInfo>;
    public final emergencyTrackingEligibilityDecisions:Array<EmergencyTrackingEligibilityDecisionInfo>;
    public final progressiveBreakOpportunities:std.SortedMap<Int, ProgressiveBreakOpportunity>;

    public function new(
        
        text:String,
        fontDecisions:Array<FontDecision>,
        punctuationGlyphSubstitutor:ClreqPunctuationGlyphSubstitutor,
        substitutionRollbacks:std.SortedMap<TextRange, String>,
        shapingDecisions:Array<ShapingDecisionInfo>,
        metricDecisions:Array<ClusterMetricDecision>,
        punctuationAtoms:Array<PunctuationAtom>,
        geometryDecisions:Array<ClusterGeometryDecisionInfo>,
        spacingPlan:PunctuationSpacingCompressionResult,
        attachedPunctuationBoundary:AttachedInlinePunctuationBoundaryResult,
        roleOverrideInfos:Array<RoleOverrideInfo>,
        laidOutLines:Array<LineBox>,
        lineSolution:LineSolution,
        clusters:Array<Cluster>,
        justificationPlans:Array<Null<JustificationPlan>>,
        autoSpaceDecisions:Array<AutoSpaceDecisionInfo>,
        edgeTrimDecisions:Array<LineEdgeTrimDecisionInfo>,
        decorationDecisions:Array<DecorationDecisionInfo>,
        decorationSegments:Array<DecorationSegmentInfo>,
        rubyDecisions:Array<RubyDecisionInfo>,
        bopomofoDecisions:Array<BopomofoDecisionInfo>,
        mandatoryBreakDecisions:Array<MandatoryBreakDecisionInfo>,
        maxLinesDecision:Null<MaxLinesDecisionInfo>,
        lineSpacingDecision:Null<LineSpacingDecisionInfo>,
        rubyLineHeightDecision:Null<RubyLineHeightDecisionInfo>,
        inlineObjectLineHeightDecision:Null<InlineObjectLineHeightDecisionInfo>,
        kinsokuDecision:KinsokuDecisionInfo,
        contextualKinsokuDecisions:Array<ContextualKinsokuDecisionInfo>,
        lineLengthGridDecision:LineLengthGridDecisionInfo,
        firstLineIndentDecision:FirstLineIndentDecisionInfo,
        inlineBoxDecisions:Array<InlineBoxDecisionInfo>,
        inlineObjectDecisions:Array<InlineObjectDecisionInfo>,
        inlineObjectPunctuationAttachmentDecisions:Array<InlineObjectPunctuationAttachmentDecisionInfo>,
        zeroWidthBreakDecisions:Array<ZeroWidthBreakDecisionInfo>,
        breakOpportunityDecisions:Array<BreakOpportunityDecisionInfo>,
        emergencyTrackingEligibilityDecisions:Array<EmergencyTrackingEligibilityDecisionInfo>,
        progressiveBreakOpportunities:std.SortedMap<Int, ProgressiveBreakOpportunity>
    ) {
        this.text = text;
        this.fontDecisions = fontDecisions;
        this.punctuationGlyphSubstitutor = punctuationGlyphSubstitutor;
        this.substitutionRollbacks = substitutionRollbacks;
        this.shapingDecisions = shapingDecisions;
        this.metricDecisions = metricDecisions;
        this.punctuationAtoms = punctuationAtoms;
        this.geometryDecisions = geometryDecisions;
        this.spacingPlan = spacingPlan;
        this.attachedPunctuationBoundary = attachedPunctuationBoundary;
        this.roleOverrideInfos = roleOverrideInfos;
        this.laidOutLines = laidOutLines;
        this.lineSolution = lineSolution;
        this.clusters = clusters;
        this.justificationPlans = justificationPlans;
        this.autoSpaceDecisions = autoSpaceDecisions;
        this.edgeTrimDecisions = edgeTrimDecisions;
        this.decorationDecisions = decorationDecisions;
        this.decorationSegments = decorationSegments;
        this.rubyDecisions = rubyDecisions;
        this.bopomofoDecisions = bopomofoDecisions;
        this.mandatoryBreakDecisions = mandatoryBreakDecisions;
        this.maxLinesDecision = maxLinesDecision;
        this.lineSpacingDecision = lineSpacingDecision;
        this.rubyLineHeightDecision = rubyLineHeightDecision;
        this.inlineObjectLineHeightDecision = inlineObjectLineHeightDecision;
        this.kinsokuDecision = kinsokuDecision;
        this.contextualKinsokuDecisions = contextualKinsokuDecisions;
        this.lineLengthGridDecision = lineLengthGridDecision;
        this.firstLineIndentDecision = firstLineIndentDecision;
        this.inlineBoxDecisions = inlineBoxDecisions;
        this.inlineObjectDecisions = inlineObjectDecisions;
        this.inlineObjectPunctuationAttachmentDecisions = inlineObjectPunctuationAttachmentDecisions;
        this.zeroWidthBreakDecisions = zeroWidthBreakDecisions;
        this.breakOpportunityDecisions = breakOpportunityDecisions;
        this.emergencyTrackingEligibilityDecisions = emergencyTrackingEligibilityDecisions;
        this.progressiveBreakOpportunities = progressiveBreakOpportunities;
    }
}


class LayoutDebugAssembly {
    public static function buildLayoutDebugInfo(
        engine:ExplainableStubParagraphLayoutEngine,
        stage:LayoutDebugStageInput
    ):LayoutDebugInfo {
        throw new haxe.exceptions.NotImplementedException("ring-r1 not-ported marker: buildLayoutDebugInfo");
    }
}
