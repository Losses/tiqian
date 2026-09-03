package org.tiqian.layout;

import org.tiqian.core.LayoutInput;
import org.tiqian.core.TextRange;
import org.tiqian.core.IntRange;
import org.tiqian.core.TextStyle;
import org.tiqian.core.RubySpan;
import org.tiqian.clreq.ClreqProfile;
import org.tiqian.clreq.ClreqPunctuationGlyphSubstitutor;
import org.tiqian.font.FontPolicy.FontDecision;
import org.tiqian.core.Glyph;
import org.tiqian.core.RoleOverrideInfo;
import org.tiqian.core.BreakOpportunityDecisionInfo;
import org.tiqian.core.EmergencyTrackingEligibilityDecisionInfo;
import org.tiqian.layout.ProgressiveBreakDecisions.ProgressiveBreakOpportunity;
import org.tiqian.core.ShapingDecisionInfo;
import org.tiqian.core.AutoSpaceDecisionInfo;
import org.tiqian.core.Cluster;
import org.tiqian.core.InlineObjectSpan;
import org.tiqian.core.InlineObjectPreferredStretch;
import org.tiqian.font.FontRole;
import org.tiqian.layout.KinsokuRule.ClreqKinsokuRule;
import org.tiqian.layout.PunctuationGeometryStage.InlineObjectAttachedMark;
import org.tiqian.core.InlineObjectPunctuationAttachmentDecisionInfo;
import org.tiqian.core.MandatoryBreakDecisionInfo;
import org.tiqian.core.ZeroWidthBreakDecisionInfo;
import org.tiqian.core.InlineAttachment;
import org.tiqian.core.LineLengthGridDecisionInfo;
import org.tiqian.layout.LineGeometryStage.ClusterMetricDecision;
import org.tiqian.layout.LineGeometryStage.ResolvedLineMetrics;
import org.tiqian.core.LineSpacingDecisionInfo;
import org.tiqian.core.FirstLineIndentDecisionInfo;
import org.tiqian.core.KinsokuDecisionInfo;
import org.tiqian.layout.ProgressiveBreakDecisions.ProgressiveBreakTier;
import org.tiqian.layout.QuotePairAnalyzer.QuotePair;
import org.tiqian.layout.AnnotationGeometryStage.RubyFontGeometry;
import org.tiqian.layout.LineOptimization.LineSolution;
import org.tiqian.clreq.AdjustmentStylePolicy;
import org.tiqian.clreq.PunctuationClass;
import org.tiqian.layout.PunctuationModel.PunctuationAtom;
import org.tiqian.layout.PunctuationModel.PunctuationSpacingCompressionResult;
import org.tiqian.layout.PunctuationGeometryLedger.AttachedInlinePunctuationBoundaryResult;
import org.tiqian.layout.PunctuationGeometryLedger;
import org.tiqian.layout.ProgressiveBreakDecisions.ShrinkOpportunity;
import org.tiqian.layout.PunctuationGeometryStage.ContextualKinsoku;
import org.tiqian.layout.UnicodePunctuationBoundaryResolver.UnicodePunctuationBoundaries;
import org.tiqian.core.EastAsianSpacingEdges;
import org.tiqian.clreq.ResolvedKinsoku;
import org.tiqian.layout.PunctuationGeometryStage.InlineBoxApplicationResult;
import org.tiqian.layout.ParagraphLayoutEngine.ExplainableStubParagraphLayoutEngine;

@:dataClass class ParagraphLayoutPrep {
    public final input:LayoutInput;
    public final rejectedTechnicalTiersBySpan:haxe.ds.ObjectMap<TextRange, std.SortedSet<ProgressiveBreakTier>>;
    public final text:String;
    public final fontSize:Float;
    public final styleAt:Int->TextStyle;
    public final fontSizeAt:Int->Float;
    public final bopomofoFontWeightAt:Int->Int;
    public final rubyFontSize:Float;
    public final rubyStackGap:Float;
    public final rubyFontWeight:Int;
    public final pinyinSpans:Array<RubySpan>;
    public final clreqProfile:ClreqProfile;
    public final punctuationGlyphSubstitutor:ClreqPunctuationGlyphSubstitutor;
    public final measure:Float;
    public final measureEm:Float;
    public final gridBodyOffset:Float;
    public final lineLengthGridDecision:LineLengthGridDecisionInfo;
    public final quotePairs:Array<QuotePair>;
    public final roleOverrideInfos:Array<RoleOverrideInfo>;
    public final fontDecisions:Array<FontDecision>;
    public final hyphenOffsets:std.SortedSet<Int>;
    public final hyphenAdvance:Float;
    public final hyphenGlyphs:Array<Glyph>;
    public final substitutionRollbacks:haxe.ds.ObjectMap<TextRange, String>;
    public final breakOpportunityDecisions:Array<BreakOpportunityDecisionInfo>;
    public final emergencyTrackingEligibilityDecisions:Array<EmergencyTrackingEligibilityDecisionInfo>;
    public final progressiveBreakOffsets:std.SortedMap<Int, ProgressiveBreakOpportunity>;
    public final shapedGlyphsByClusterRange:haxe.ds.ObjectMap<TextRange, Array<Glyph>>;
    public final openTypeFeaturesByClusterRange:haxe.ds.ObjectMap<TextRange, Array<String>>;
    public final shapingDecisions:Array<ShapingDecisionInfo>;
    public final eastAsianSpacingEdges:Array<EastAsianSpacingEdges>;
    public final autoSpaceDecisions:Array<AutoSpaceDecisionInfo>;
    public final inlineBoxResult:InlineBoxApplicationResult;
    public final naturalClusters:Array<Cluster>;
    public final inlineObjectByClusterIndex:std.SortedMap<Int, InlineObjectSpan>;
    public final uniformInlineObjectBoundaryAfterClusters:std.SortedSet<Int>;
    public final preferredInlineObjectBoundaryAfterClusters:std.SortedMap<Int, InlineObjectPreferredStretch>;
    public final inlineObjectBoundaryUnbreakableRanges:Array<IntRange>;
    public final clusterRoles:Array<FontRole>;
    public final resolvedKinsoku:ResolvedKinsoku;
    public final kinsokuRule:ClreqKinsokuRule;
    public final inlineObjectAttachedMarks:Array<InlineObjectAttachedMark>;
    public final inlineObjectSeparatorSpaceTrims:std.SortedMap<Int, Float>;
    public final inlineObjectAttachmentNoStretchBoundaries:std.SortedSet<Int>;
    public final inlineObjectPunctuationAttachmentDecisions:Array<InlineObjectPunctuationAttachmentDecisionInfo>;
    public final mandatoryBreakClusters:std.SortedSet<Int>;
    public final zeroWidthBreakClusters:std.SortedSet<Int>;
    public final mandatoryBreakDecisions:Array<MandatoryBreakDecisionInfo>;
    public final zeroWidthBreakDecisions:Array<ZeroWidthBreakDecisionInfo>;
    public final punctuationAtoms:Array<PunctuationAtom>;
    public final spacingPlan:PunctuationSpacingCompressionResult;
    public final rubyFontGeometryBySpan:haxe.ds.ObjectMap<RubySpan, RubyFontGeometry>;
    public final rubyAndBopomofoSpread:std.SortedMap<Int, Float>;
    public final naturalInlineAttachments:Array<InlineAttachment>;
    public final attachedPunctuationBoundary:AttachedInlinePunctuationBoundaryResult;
    public final baseGeometry:PunctuationGeometryLedger;
    public final attachedPunctuationTrailingGlueByCluster:std.SortedMap<Int, Float>;
    public final clusters:Array<Cluster>;
    public final adjustmentStyle:AdjustmentStylePolicy;
    public final atomClassByRange:haxe.ds.ObjectMap<TextRange, PunctuationClass>;
    public final shrinkOpportunities:Array<ShrinkOpportunity>;

    public function new(
        
        input:LayoutInput,
        rejectedTechnicalTiersBySpan:haxe.ds.ObjectMap<TextRange, std.SortedSet<ProgressiveBreakTier>>,
        text:String,
        fontSize:Float,
        styleAt:Int->TextStyle,
        fontSizeAt:Int->Float,
        bopomofoFontWeightAt:Int->Int,
        rubyFontSize:Float,
        rubyStackGap:Float,
        rubyFontWeight:Int,
        pinyinSpans:Array<RubySpan>,
        clreqProfile:ClreqProfile,
        punctuationGlyphSubstitutor:ClreqPunctuationGlyphSubstitutor,
        measure:Float,
        measureEm:Float,
        gridBodyOffset:Float,
        lineLengthGridDecision:LineLengthGridDecisionInfo,
        quotePairs:Array<QuotePair>,
        roleOverrideInfos:Array<RoleOverrideInfo>,
        fontDecisions:Array<FontDecision>,
        hyphenOffsets:std.SortedSet<Int>,
        hyphenAdvance:Float,
        hyphenGlyphs:Array<Glyph>,
        substitutionRollbacks:haxe.ds.ObjectMap<TextRange, String>,
        breakOpportunityDecisions:Array<BreakOpportunityDecisionInfo>,
        emergencyTrackingEligibilityDecisions:Array<EmergencyTrackingEligibilityDecisionInfo>,
        progressiveBreakOffsets:std.SortedMap<Int, ProgressiveBreakOpportunity>,
        shapedGlyphsByClusterRange:haxe.ds.ObjectMap<TextRange, Array<Glyph>>,
        openTypeFeaturesByClusterRange:haxe.ds.ObjectMap<TextRange, Array<String>>,
        shapingDecisions:Array<ShapingDecisionInfo>,
        eastAsianSpacingEdges:Array<EastAsianSpacingEdges>,
        autoSpaceDecisions:Array<AutoSpaceDecisionInfo>,
        inlineBoxResult:InlineBoxApplicationResult,
        naturalClusters:Array<Cluster>,
        inlineObjectByClusterIndex:std.SortedMap<Int, InlineObjectSpan>,
        uniformInlineObjectBoundaryAfterClusters:std.SortedSet<Int>,
        preferredInlineObjectBoundaryAfterClusters:std.SortedMap<Int, InlineObjectPreferredStretch>,
        inlineObjectBoundaryUnbreakableRanges:Array<IntRange>,
        clusterRoles:Array<FontRole>,
        resolvedKinsoku:ResolvedKinsoku,
        kinsokuRule:ClreqKinsokuRule,
        inlineObjectAttachedMarks:Array<InlineObjectAttachedMark>,
        inlineObjectSeparatorSpaceTrims:std.SortedMap<Int, Float>,
        inlineObjectAttachmentNoStretchBoundaries:std.SortedSet<Int>,
        inlineObjectPunctuationAttachmentDecisions:Array<InlineObjectPunctuationAttachmentDecisionInfo>,
        mandatoryBreakClusters:std.SortedSet<Int>,
        zeroWidthBreakClusters:std.SortedSet<Int>,
        mandatoryBreakDecisions:Array<MandatoryBreakDecisionInfo>,
        zeroWidthBreakDecisions:Array<ZeroWidthBreakDecisionInfo>,
        punctuationAtoms:Array<PunctuationAtom>,
        spacingPlan:PunctuationSpacingCompressionResult,
        rubyFontGeometryBySpan:haxe.ds.ObjectMap<RubySpan, RubyFontGeometry>,
        rubyAndBopomofoSpread:std.SortedMap<Int, Float>,
        naturalInlineAttachments:Array<InlineAttachment>,
        attachedPunctuationBoundary:AttachedInlinePunctuationBoundaryResult,
        baseGeometry:PunctuationGeometryLedger,
        attachedPunctuationTrailingGlueByCluster:std.SortedMap<Int, Float>,
        clusters:Array<Cluster>,
        adjustmentStyle:AdjustmentStylePolicy,
        atomClassByRange:haxe.ds.ObjectMap<TextRange, PunctuationClass>,
        shrinkOpportunities:Array<ShrinkOpportunity>
    ) {
        this.input = input;
        this.rejectedTechnicalTiersBySpan = rejectedTechnicalTiersBySpan;
        this.text = text;
        this.fontSize = fontSize;
        this.styleAt = styleAt;
        this.fontSizeAt = fontSizeAt;
        this.bopomofoFontWeightAt = bopomofoFontWeightAt;
        this.rubyFontSize = rubyFontSize;
        this.rubyStackGap = rubyStackGap;
        this.rubyFontWeight = rubyFontWeight;
        this.pinyinSpans = pinyinSpans;
        this.clreqProfile = clreqProfile;
        this.punctuationGlyphSubstitutor = punctuationGlyphSubstitutor;
        this.measure = measure;
        this.measureEm = measureEm;
        this.gridBodyOffset = gridBodyOffset;
        this.lineLengthGridDecision = lineLengthGridDecision;
        this.quotePairs = quotePairs;
        this.roleOverrideInfos = roleOverrideInfos;
        this.fontDecisions = fontDecisions;
        this.hyphenOffsets = hyphenOffsets;
        this.hyphenAdvance = hyphenAdvance;
        this.hyphenGlyphs = hyphenGlyphs;
        this.substitutionRollbacks = substitutionRollbacks;
        this.breakOpportunityDecisions = breakOpportunityDecisions;
        this.emergencyTrackingEligibilityDecisions = emergencyTrackingEligibilityDecisions;
        this.progressiveBreakOffsets = progressiveBreakOffsets;
        this.shapedGlyphsByClusterRange = shapedGlyphsByClusterRange;
        this.openTypeFeaturesByClusterRange = openTypeFeaturesByClusterRange;
        this.shapingDecisions = shapingDecisions;
        this.eastAsianSpacingEdges = eastAsianSpacingEdges;
        this.autoSpaceDecisions = autoSpaceDecisions;
        this.inlineBoxResult = inlineBoxResult;
        this.naturalClusters = naturalClusters;
        this.inlineObjectByClusterIndex = inlineObjectByClusterIndex;
        this.uniformInlineObjectBoundaryAfterClusters = uniformInlineObjectBoundaryAfterClusters;
        this.preferredInlineObjectBoundaryAfterClusters = preferredInlineObjectBoundaryAfterClusters;
        this.inlineObjectBoundaryUnbreakableRanges = inlineObjectBoundaryUnbreakableRanges;
        this.clusterRoles = clusterRoles;
        this.resolvedKinsoku = resolvedKinsoku;
        this.kinsokuRule = kinsokuRule;
        this.inlineObjectAttachedMarks = inlineObjectAttachedMarks;
        this.inlineObjectSeparatorSpaceTrims = inlineObjectSeparatorSpaceTrims;
        this.inlineObjectAttachmentNoStretchBoundaries = inlineObjectAttachmentNoStretchBoundaries;
        this.inlineObjectPunctuationAttachmentDecisions = inlineObjectPunctuationAttachmentDecisions;
        this.mandatoryBreakClusters = mandatoryBreakClusters;
        this.zeroWidthBreakClusters = zeroWidthBreakClusters;
        this.mandatoryBreakDecisions = mandatoryBreakDecisions;
        this.zeroWidthBreakDecisions = zeroWidthBreakDecisions;
        this.punctuationAtoms = punctuationAtoms;
        this.spacingPlan = spacingPlan;
        this.rubyFontGeometryBySpan = rubyFontGeometryBySpan;
        this.rubyAndBopomofoSpread = rubyAndBopomofoSpread;
        this.naturalInlineAttachments = naturalInlineAttachments;
        this.attachedPunctuationBoundary = attachedPunctuationBoundary;
        this.baseGeometry = baseGeometry;
        this.attachedPunctuationTrailingGlueByCluster = attachedPunctuationTrailingGlueByCluster;
        this.clusters = clusters;
        this.adjustmentStyle = adjustmentStyle;
        this.atomClassByRange = atomClassByRange;
        this.shrinkOpportunities = shrinkOpportunities;
    }
}

@:dataClass class LineBreakPlanningStageResult {
    public final metricDecisions:Array<ClusterMetricDecision>;
    public final metricDecisionByRange:haxe.ds.ObjectMap<TextRange, ClusterMetricDecision>;
    public final baseAscent:Float;
    public final baseDescent:Float;
    public final baseBoxDescent:Float;
    public final baseFaceHeight:Float;
    public final existingInterlineSpace:Float;
    public final rubyExtent:Float;
    public final baseLineMetrics:ResolvedLineMetrics;
    public final lineSpacingDecision:Null<LineSpacingDecisionInfo>;
    public final blockIndent:Float;
    public final firstLineIndent:Float;
    public final firstLineIndentDecision:FirstLineIndentDecisionInfo;
    public final kinsokuDecision:KinsokuDecisionInfo;
    public final asciiPointMarkKinsoku:ContextualKinsoku;
    public final inlineObjectKinsoku:ContextualKinsoku;
    public final unicodePunctuationBoundaries:UnicodePunctuationBoundaries;
    public final westernBracketCjkInterCharBoundaryAfterClusters:std.SortedSet<Int>;
    public final attachedInlinePhysicalBoundaryAfterClusters:std.SortedSet<Int>;
    public final attachedInlineVirtualBoundaryAfterClusters:std.SortedMap<Int, Int>;
    public final attachedInlineVirtualSinoWesternBoundaryAfterClusters:std.SortedSet<Int>;
    public final noStretchBoundaryClusters:std.SortedSet<Int>;
    public final noStretchBoundaryAfterClusters:std.SortedSet<Int>;
    public final technicalBoundaryAfterClusters:std.SortedMap<Int, ProgressiveBreakTier>;
    public final emergencyTrackingBoundaryAfterClusters:std.SortedMap<Int, String>;
    public final progressiveBreakOpportunities:std.SortedMap<Int, ProgressiveBreakOpportunity>;
    public final lineSolution:LineSolution;

    public function new(
        
        metricDecisions:Array<ClusterMetricDecision>,
        metricDecisionByRange:haxe.ds.ObjectMap<TextRange, ClusterMetricDecision>,
        baseAscent:Float,
        baseDescent:Float,
        baseBoxDescent:Float,
        baseFaceHeight:Float,
        existingInterlineSpace:Float,
        rubyExtent:Float,
        baseLineMetrics:ResolvedLineMetrics,
        lineSpacingDecision:Null<LineSpacingDecisionInfo>,
        blockIndent:Float,
        firstLineIndent:Float,
        firstLineIndentDecision:FirstLineIndentDecisionInfo,
        kinsokuDecision:KinsokuDecisionInfo,
        asciiPointMarkKinsoku:ContextualKinsoku,
        inlineObjectKinsoku:ContextualKinsoku,
        unicodePunctuationBoundaries:UnicodePunctuationBoundaries,
        westernBracketCjkInterCharBoundaryAfterClusters:std.SortedSet<Int>,
        attachedInlinePhysicalBoundaryAfterClusters:std.SortedSet<Int>,
        attachedInlineVirtualBoundaryAfterClusters:std.SortedMap<Int, Int>,
        attachedInlineVirtualSinoWesternBoundaryAfterClusters:std.SortedSet<Int>,
        noStretchBoundaryClusters:std.SortedSet<Int>,
        noStretchBoundaryAfterClusters:std.SortedSet<Int>,
        technicalBoundaryAfterClusters:std.SortedMap<Int, ProgressiveBreakTier>,
        emergencyTrackingBoundaryAfterClusters:std.SortedMap<Int, String>,
        progressiveBreakOpportunities:std.SortedMap<Int, ProgressiveBreakOpportunity>,
        lineSolution:LineSolution
    ) {
        this.metricDecisions = metricDecisions;
        this.metricDecisionByRange = metricDecisionByRange;
        this.baseAscent = baseAscent;
        this.baseDescent = baseDescent;
        this.baseBoxDescent = baseBoxDescent;
        this.baseFaceHeight = baseFaceHeight;
        this.existingInterlineSpace = existingInterlineSpace;
        this.rubyExtent = rubyExtent;
        this.baseLineMetrics = baseLineMetrics;
        this.lineSpacingDecision = lineSpacingDecision;
        this.blockIndent = blockIndent;
        this.firstLineIndent = firstLineIndent;
        this.firstLineIndentDecision = firstLineIndentDecision;
        this.kinsokuDecision = kinsokuDecision;
        this.asciiPointMarkKinsoku = asciiPointMarkKinsoku;
        this.inlineObjectKinsoku = inlineObjectKinsoku;
        this.unicodePunctuationBoundaries = unicodePunctuationBoundaries;
        this.westernBracketCjkInterCharBoundaryAfterClusters = westernBracketCjkInterCharBoundaryAfterClusters;
        this.attachedInlinePhysicalBoundaryAfterClusters = attachedInlinePhysicalBoundaryAfterClusters;
        this.attachedInlineVirtualBoundaryAfterClusters = attachedInlineVirtualBoundaryAfterClusters;
        this.attachedInlineVirtualSinoWesternBoundaryAfterClusters = attachedInlineVirtualSinoWesternBoundaryAfterClusters;
        this.noStretchBoundaryClusters = noStretchBoundaryClusters;
        this.noStretchBoundaryAfterClusters = noStretchBoundaryAfterClusters;
        this.technicalBoundaryAfterClusters = technicalBoundaryAfterClusters;
        this.emergencyTrackingBoundaryAfterClusters = emergencyTrackingBoundaryAfterClusters;
        this.progressiveBreakOpportunities = progressiveBreakOpportunities;
        this.lineSolution = lineSolution;
    }
}


class LineBreakPlanningStage {
    public static function planParagraphLines(
        engine:ExplainableStubParagraphLayoutEngine,
        prep:ParagraphLayoutPrep
    ):LineBreakPlanningStageResult {
        throw new haxe.exceptions.NotImplementedException("ring-r1 not-ported marker: planParagraphLines");
    }
}
