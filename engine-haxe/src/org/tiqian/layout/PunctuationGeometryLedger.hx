package org.tiqian.layout;

import org.tiqian.core.Cluster;
import org.tiqian.core.IllegalStateException;
import org.tiqian.core.ClusterGeometryDecisionInfo;
import org.tiqian.core.InlineAttachment;
import org.tiqian.core.IntRange;
import org.tiqian.core.LineEdgeTrimDecisionInfo;
import org.tiqian.core.SpacingDecisionInfo;
import org.tiqian.core.TextRange;
import org.tiqian.layout.LineOptimization.LineCandidate;
import org.tiqian.layout.PunctuationModel.PunctuationAnchor;
import org.tiqian.layout.PunctuationModel.PunctuationAtom;
import org.tiqian.layout.PunctuationModel.PunctuationSpacingCompressionResult;
import std.SortedMap;

@:dataClass class PunctuationGeometryLedger {
    public final naturalClusters:Array<Cluster>;
    public final geometries:SortedMap<Int,PunctuationClusterGeometry>;
    public final budgets:SortedMap<Int,GlueBudget>;
    public final justificationDeltaByCluster:SortedMap<Int,Float>;
    public final rawEdgeTrimByCluster:SortedMap<Int,Float>;
    public final rubySpreadByCluster:SortedMap<Int,Float>;
    public final inlineBoxAdvanceByCluster:SortedMap<Int,Float>;
    public final attachedInlineTrailingGlueByCluster:SortedMap<Int,Float>;
    public function new(naturalClusters:Array<Cluster>, geometries:SortedMap<Int,PunctuationClusterGeometry>, budgets:SortedMap<Int,GlueBudget>, ?justificationDeltaByCluster:Null<SortedMap<Int,Float>>, ?rawEdgeTrimByCluster:Null<SortedMap<Int,Float>>, ?rubySpreadByCluster:Null<SortedMap<Int,Float>>, ?inlineBoxAdvanceByCluster:Null<SortedMap<Int,Float>>, ?attachedInlineTrailingGlueByCluster:Null<SortedMap<Int,Float>>) {
        this.naturalClusters=naturalClusters; this.geometries=geometries; this.budgets=budgets;
        this.justificationDeltaByCluster=justificationDeltaByCluster==null?SortedMap.builder().build():justificationDeltaByCluster;
        this.rawEdgeTrimByCluster=rawEdgeTrimByCluster==null?SortedMap.builder().build():rawEdgeTrimByCluster;
        this.rubySpreadByCluster=rubySpreadByCluster==null?SortedMap.builder().build():rubySpreadByCluster;
        this.inlineBoxAdvanceByCluster=inlineBoxAdvanceByCluster==null?SortedMap.builder().build():inlineBoxAdvanceByCluster;
        this.attachedInlineTrailingGlueByCluster=attachedInlineTrailingGlueByCluster==null?SortedMap.builder().build():attachedInlineTrailingGlueByCluster;
    }
    public static function from(clusters:Array<Cluster>, atoms:Array<PunctuationAtom>, plan:PunctuationSpacingCompressionResult):PunctuationGeometryLedger throw new IllegalStateException("TODO r6: PunctuationGeometryLedger.from");
    public function resolveClusters():Array<Cluster> throw new IllegalStateException("TODO r6: PunctuationGeometryLedger.resolveClusters");
    public function withInlineBoxAdvances(m:SortedMap<Int,Float>):PunctuationGeometryLedger throw new IllegalStateException("TODO r6: PunctuationGeometryLedger.withInlineBoxAdvances");
    public function withRubySpread(m:SortedMap<Int,Float>):PunctuationGeometryLedger throw new IllegalStateException("TODO r6: PunctuationGeometryLedger.withRubySpread");
    public function withRawEdgeTrims(m:SortedMap<Int,Float>):PunctuationGeometryLedger throw new IllegalStateException("TODO r6: PunctuationGeometryLedger.withRawEdgeTrims");
    public function addJustificationDeltas(m:SortedMap<Int,Float>):PunctuationGeometryLedger throw new IllegalStateException("TODO r6: PunctuationGeometryLedger.addJustificationDeltas");
    public function consumeLeadingByCluster(m:SortedMap<Int,Float>):PunctuationGeometryLedger throw new IllegalStateException("TODO r6: PunctuationGeometryLedger.consumeLeadingByCluster");
    public function consumeTrailingByCluster(m:SortedMap<Int,Float>):PunctuationGeometryLedger throw new IllegalStateException("TODO r6: PunctuationGeometryLedger.consumeTrailingByCluster");
    public function glueCapacities():SortedMap<Int,GlueCapacity> throw new IllegalStateException("TODO r6: PunctuationGeometryLedger.glueCapacities");
    public function toDecisionInfo():Array<ClusterGeometryDecisionInfo> throw new IllegalStateException("TODO r6: PunctuationGeometryLedger.toDecisionInfo");
    public function consumeLineEdgeGlue(lines:Array<LineCandidate>, ?forceLineEndHalfWidth:Null<Bool>):LineEdgeTrimResult throw new IllegalStateException("TODO r6: PunctuationGeometryLedger.consumeLineEdgeGlue");
    public function resolveAttachedInlinePunctuationBoundaries(a:Array<InlineAttachment>, atoms:Array<PunctuationAtom>, em:Float):AttachedInlinePunctuationBoundaryResult throw new IllegalStateException("TODO r6: PunctuationGeometryLedger.resolveAttachedInlinePunctuationBoundaries");
    public static function isInside(self:TextRange,other:TextRange):Bool return self.start>=other.start&&self.end<=other.end;
    public static function clusterIndexRangeFor(self:Array<Cluster>,r:TextRange):Null<IntRange> throw new IllegalStateException("TODO r6: PunctuationGeometryLedger.clusterIndexRangeFor");
}
@:dataClass class AttachedInlinePunctuationBoundaryResult {public final geometry:PunctuationGeometryLedger;public final trailingGlueByCluster:SortedMap<Int,Float>;public final decisions:Array<SpacingDecisionInfo>;public function new(geometry:PunctuationGeometryLedger,trailingGlueByCluster:SortedMap<Int,Float>,decisions:Array<SpacingDecisionInfo>){this.geometry=geometry;this.trailingGlueByCluster=trailingGlueByCluster;this.decisions=decisions;}}
@:dataClass class PunctuationClusterGeometry {public final range:TextRange;public final sourceText:String;public final displayText:String;public final baseAdvance:Float;public final bodyWidth:Float;public final leadingGlueNatural:Float;public final trailingGlueNatural:Float;public final leadingGlueInitiallyConsumed:Float;public final trailingGlueInitiallyConsumed:Float;public final glyphInlineShift:Float;public final glyphPlacementReason:Null<String>;public final anchor:Null<PunctuationAnchor>;public final reason:String;public function new(range:TextRange,sourceText:String,displayText:String,baseAdvance:Float,bodyWidth:Float,leadingGlueNatural:Float,trailingGlueNatural:Float,leadingGlueInitiallyConsumed:Float,trailingGlueInitiallyConsumed:Float,glyphInlineShift:Float,glyphPlacementReason:Null<String>,anchor:Null<PunctuationAnchor>,reason:String){this.range=range;this.sourceText=sourceText;this.displayText=displayText;this.baseAdvance=baseAdvance;this.bodyWidth=bodyWidth;this.leadingGlueNatural=leadingGlueNatural;this.trailingGlueNatural=trailingGlueNatural;this.leadingGlueInitiallyConsumed=leadingGlueInitiallyConsumed;this.trailingGlueInitiallyConsumed=trailingGlueInitiallyConsumed;this.glyphInlineShift=glyphInlineShift;this.glyphPlacementReason=glyphPlacementReason;this.anchor=anchor;this.reason=reason;}}
@:dataClass class GlueBudget {public final leadingNatural:Float;public final leadingConsumed:Float;public final trailingNatural:Float;public final trailingConsumed:Float;public function new(leadingNatural:Float,leadingConsumed:Float,trailingNatural:Float,trailingConsumed:Float){this.leadingNatural=leadingNatural;this.leadingConsumed=leadingConsumed;this.trailingNatural=trailingNatural;this.trailingConsumed=trailingConsumed;}public var leadingRemaining(get,never):Float;function get_leadingRemaining():Float return Math.max(0,leadingNatural-leadingConsumed);public var trailingRemaining(get,never):Float;function get_trailingRemaining():Float return Math.max(0,trailingNatural-trailingConsumed);}
@:dataClass class LineEdgeTrimResult {public final geometry:PunctuationGeometryLedger;public final decisions:Array<LineEdgeTrimDecisionInfo>;public function new(geometry:PunctuationGeometryLedger,decisions:Array<LineEdgeTrimDecisionInfo>){this.geometry=geometry;this.decisions=decisions;}}
@:dataClass class GlueCapacity {public final leading:Float;public final trailing:Float;public final paired:Bool;public function new(leading:Float,trailing:Float,paired:Bool){this.leading=leading;this.trailing=trailing;this.paired=paired;}}
