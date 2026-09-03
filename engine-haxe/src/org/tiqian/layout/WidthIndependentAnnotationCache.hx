package org.tiqian.layout;

import org.tiqian.core.LayoutInput;
import org.tiqian.core.TextRange;
import org.tiqian.core.TextSpan;
import org.tiqian.core.LineBreakSpan;
import org.tiqian.core.TextStyle;
import org.tiqian.core.DecorationSpan;
import org.tiqian.core.RubySpan;
import org.tiqian.core.InlineBoxSpan;
import org.tiqian.core.InlineObjectSpan;
import org.tiqian.core.LayoutProfileId;
import org.tiqian.clreq.ClreqProfile;
import org.tiqian.clreq.ClreqPunctuationGlyphSubstitutor;
import org.tiqian.font.FontPolicy.FontDecision;
import org.tiqian.core.RoleOverrideInfo;
import org.tiqian.shaping.TextShaper.ShapingResult;
import org.tiqian.layout.ProgressiveBreakDecisions.ProgressiveBreakTier;
import org.tiqian.layout.QuotePairAnalyzer.QuotePair;
import org.tiqian.layout.ClusterRoleResolution.ResolvedClusterRange;
import org.tiqian.layout.AnnotationGeometryStage.RubyFontGeometry;
import org.tiqian.layout.ParagraphShapingStage.ParagraphShapingStageResult;
import org.tiqian.layout.LineBreakPlanningStage.ParagraphLayoutPrep;
import org.tiqian.layout.ParagraphLayoutEngine.ExplainableStubParagraphLayoutEngine;
import std.SortedSet;
import std.SortedMap;

@:dataClass class WidthIndependentAnnotationKey {
    public final text:String;
    public final spans:Array<TextSpan>;
    public final lineBreakSpans:Array<LineBreakSpan>;
    public final sourceBoundaries:SortedSet<Int>;
    public final textStyle:TextStyle;
    public final decorations:Array<DecorationSpan>;
    public final rubySpans:Array<RubySpan>;
    public final inlineBoxes:Array<InlineBoxSpan>;
    public final inlineObjects:Array<InlineObjectSpan>;
    public final profileId:LayoutProfileId;
    public final emphasisDotGapEm:Float;
    public final rejectedTechnicalTiersBySpan:SortedMap<TextRange, SortedSet<ProgressiveBreakTier>>;

    public function new(
        text:String,
        spans:Array<TextSpan>,
        lineBreakSpans:Array<LineBreakSpan>,
        sourceBoundaries:SortedSet<Int>,
        textStyle:TextStyle,
        decorations:Array<DecorationSpan>,
        rubySpans:Array<RubySpan>,
        inlineBoxes:Array<InlineBoxSpan>,
        inlineObjects:Array<InlineObjectSpan>,
        profileId:LayoutProfileId,
        emphasisDotGapEm:Float,
        rejectedTechnicalTiersBySpan:SortedMap<TextRange, SortedSet<ProgressiveBreakTier>>
    ) {
        this.text = text;
        this.spans = spans;
        this.lineBreakSpans = lineBreakSpans;
        this.sourceBoundaries = sourceBoundaries;
        this.textStyle = textStyle;
        this.decorations = decorations;
        this.rubySpans = rubySpans;
        this.inlineBoxes = inlineBoxes;
        this.inlineObjects = inlineObjects;
        this.profileId = profileId;
        this.emphasisDotGapEm = emphasisDotGapEm;
        this.rejectedTechnicalTiersBySpan = rejectedTechnicalTiersBySpan;
    }
}

class WidthIndependentParagraphAnnotation {
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
    public final quotePairs:Array<QuotePair>;
    public final roleOverrideInfos:Array<RoleOverrideInfo>;
    public final fontDecisions:Array<FontDecision>;
    public final clusterRanges:Array<ResolvedClusterRange>;
    public final fontDecisionByRange:SortedMap<TextRange, FontDecision>;
    public final inlineObjectByRange:SortedMap<TextRange, InlineObjectSpan>;
    public final segmentShapingCache:SortedMap<TextRange, ShapingResult>;
    public final substitutionRollbacks:SortedMap<TextRange, String>;
    public final rubyFontGeometryBySpan:SortedMap<RubySpan, RubyFontGeometry>;
    public final baseShapingStage:ParagraphShapingStageResult;

    public function new(
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
        quotePairs:Array<QuotePair>,
        roleOverrideInfos:Array<RoleOverrideInfo>,
        fontDecisions:Array<FontDecision>,
        clusterRanges:Array<ResolvedClusterRange>,
        fontDecisionByRange:SortedMap<TextRange, FontDecision>,
        inlineObjectByRange:SortedMap<TextRange, InlineObjectSpan>,
        segmentShapingCache:SortedMap<TextRange, ShapingResult>,
        substitutionRollbacks:SortedMap<TextRange, String>,
        rubyFontGeometryBySpan:SortedMap<RubySpan, RubyFontGeometry>,
        baseShapingStage:ParagraphShapingStageResult
    ) {
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
        this.quotePairs = quotePairs;
        this.roleOverrideInfos = roleOverrideInfos;
        this.fontDecisions = fontDecisions;
        this.clusterRanges = clusterRanges;
        this.fontDecisionByRange = fontDecisionByRange;
        this.inlineObjectByRange = inlineObjectByRange;
        this.segmentShapingCache = segmentShapingCache;
        this.substitutionRollbacks = substitutionRollbacks;
        this.rubyFontGeometryBySpan = rubyFontGeometryBySpan;
        this.baseShapingStage = baseShapingStage;
    }
}

interface WidthIndependentAnnotationCache {
    function get(key:WidthIndependentAnnotationKey):Any;
    function put(key:WidthIndependentAnnotationKey, annotation:Any):Void;
    function clear():Void;
    public var size(get, never):Int;
}

class LruWidthIndependentAnnotationCache implements WidthIndependentAnnotationCache {
    public final maxEntries:Int;
    public var size(get, never):Int;
    
    public function new(maxEntries:Int = 512) {
        this.maxEntries = maxEntries;
    }
    
    public function get(key:WidthIndependentAnnotationKey):Any {
        throw new haxe.exceptions.NotImplementedException("ring-r1 not-ported marker: LruWidthIndependentAnnotationCache.get");
    }
    
    public function put(key:WidthIndependentAnnotationKey, annotation:Any):Void {
        throw new haxe.exceptions.NotImplementedException("ring-r1 not-ported marker: LruWidthIndependentAnnotationCache.put");
    }
    
    public function clear():Void {
        throw new haxe.exceptions.NotImplementedException("ring-r1 not-ported marker: LruWidthIndependentAnnotationCache.clear");
    }
    
    public function get_size():Int {
        throw new haxe.exceptions.NotImplementedException("ring-r1 not-ported marker: LruWidthIndependentAnnotationCache.size");
    }
}

class WidthIndependentAnnotationCacheFns {
    public static function toWidthIndependentAnnotationKey(
        input:LayoutInput,
        ?rejectedTechnicalTiersBySpan:SortedMap<TextRange, SortedSet<ProgressiveBreakTier>>
    ):WidthIndependentAnnotationKey {
        throw new haxe.exceptions.NotImplementedException("ring-r1 not-ported marker: toWidthIndependentAnnotationKey");
    }

    public static function prepareWidthIndependentAnnotation(
        engine:ExplainableStubParagraphLayoutEngine,
        input:LayoutInput,
        rejectedTechnicalTiersBySpan:SortedMap<TextRange, SortedSet<ProgressiveBreakTier>>
    ):WidthIndependentParagraphAnnotation {
        throw new haxe.exceptions.NotImplementedException("ring-r1 not-ported marker: prepareWidthIndependentAnnotation");
    }

    public static function buildParagraphLayoutPrep(
        engine:ExplainableStubParagraphLayoutEngine,
        input:LayoutInput,
        annotation:WidthIndependentParagraphAnnotation,
        rejectedTechnicalTiersBySpan:SortedMap<TextRange, SortedSet<ProgressiveBreakTier>>
    ):ParagraphLayoutPrep {
        throw new haxe.exceptions.NotImplementedException("ring-r1 not-ported marker: buildParagraphLayoutPrep");
    }
}
