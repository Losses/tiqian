package org.tiqian.layout;

import org.tiqian.clreq.ClreqPunctuationPolicies;
import org.tiqian.clreq.PunctuationClass;
import org.tiqian.core.Cluster;
import org.tiqian.core.ClusterGeometryDecisionInfo;
import org.tiqian.core.InlineAttachment;
import org.tiqian.core.IntRange;
import org.tiqian.core.LineEdgeTrimDecisionInfo;
import org.tiqian.core.SpacingDecisionInfo;
import org.tiqian.core.TextRange;
import org.tiqian.layout.LineOptimization.LineCandidate;
import org.tiqian.layout.PunctuationModel.PunctuationAnchor;
import org.tiqian.layout.PunctuationModel.PunctuationAtom;
import org.tiqian.layout.PunctuationModel.PunctuationSpacingAdjustment;
import org.tiqian.layout.PunctuationModel.PunctuationSpacingCompressionResult;

@:dataClass class PunctuationGeometryLedger {
    public final naturalClusters:Array<Cluster>;
    public final geometries:Map<Int,PunctuationClusterGeometry>;
    public final budgets:Map<Int,GlueBudget>;
    public final justificationDeltaByCluster:Map<Int,Float>;
    public final rawEdgeTrimByCluster:Map<Int,Float>;
    public final rubySpreadByCluster:Map<Int,Float>;
    public final inlineBoxAdvanceByCluster:Map<Int,Float>;
    public final attachedInlineTrailingGlueByCluster:Map<Int,Float>;
    public function new(naturalClusters:Array<Cluster>, geometries:Map<Int,PunctuationClusterGeometry>, budgets:Map<Int,GlueBudget>,
        ?justificationDeltaByCluster:Map<Int,Float>, ?rawEdgeTrimByCluster:Map<Int,Float>, ?rubySpreadByCluster:Map<Int,Float>,
        ?inlineBoxAdvanceByCluster:Map<Int,Float>, ?attachedInlineTrailingGlueByCluster:Map<Int,Float>) {
        this.naturalClusters=naturalClusters; this.geometries=geometries; this.budgets=budgets;
        this.justificationDeltaByCluster=justificationDeltaByCluster==null?new Map():justificationDeltaByCluster;
        this.rawEdgeTrimByCluster=rawEdgeTrimByCluster==null?new Map():rawEdgeTrimByCluster;
        this.rubySpreadByCluster=rubySpreadByCluster==null?new Map():rubySpreadByCluster;
        this.inlineBoxAdvanceByCluster=inlineBoxAdvanceByCluster==null?new Map():inlineBoxAdvanceByCluster;
        this.attachedInlineTrailingGlueByCluster=attachedInlineTrailingGlueByCluster==null?new Map():attachedInlineTrailingGlueByCluster;
    }
    public static function from(clusters:Array<Cluster>, atoms:Array<PunctuationAtom>, plan:PunctuationSpacingCompressionResult):PunctuationGeometryLedger {
        final gs:Map<Int,PunctuationClusterGeometry>=new Map();
        for (i in 0...clusters.length) {
            final a=atoms.filter(x -> x.range.isInside(clusters[i].range));
            if(a.length>0) gs.set(i,new PunctuationClusterGeometry(clusters[i].range,clusters[i].text,clusters[i].displayText,clusters[i].advance,
                a.map(x -> x.bodyWidth).sumOfFloat(x -> x),a[0].leadingGlue.natural,a[a.length-1].trailingGlue.natural,
                a[0].leadingGlueInitiallyConsumed,a[a.length-1].trailingGlueInitiallyConsumed,a.length==1?a[0].glyphInlineShift:0,
                a.length==1?a[0].glyphPlacementReason:null,a.length==1?a[0].anchor:null,a[0].geometrySource));
        }
        final bs:Map<Int,GlueBudget>=new Map();
        for(k in gs.keys()) { final g=gs.get(k); bs.set(k,new GlueBudget(g.leadingGlueNatural,g.leadingGlueInitiallyConsumed,g.trailingGlueNatural,g.trailingGlueInitiallyConsumed)); }
        return new PunctuationGeometryLedger(clusters,gs,bs).consumeSpacing(plan);
    }
    public function resolveClusters():Array<Cluster> return naturalClusters.map((c,i) -> {
        final a=resolvedAdvance(i,c); final shift=geometries.exists(i)?geometries.get(i).glyphInlineShift:0;
        return a==c.advance && shift==0 ? c : c.copy({advance:a, glyphInlineShift:c.glyphInlineShift+shift});
    });
    public function withInlineBoxAdvances(m:Map<Int,Float>):PunctuationGeometryLedger return m.keys().hasNext()?copy({inlineBoxAdvanceByCluster:m}):this;
    public function withRubySpread(m:Map<Int,Float>):PunctuationGeometryLedger return m.keys().hasNext()?copy({rubySpreadByCluster:m}):this;
    public function withRawEdgeTrims(m:Map<Int,Float>):PunctuationGeometryLedger {
        if(!m.keys().hasNext()) return this; final n=new Map<Int,Float>(); for(k in rawEdgeTrimByCluster.keys()) n.set(k,rawEdgeTrimByCluster.get(k)); for(k in m.keys()) n.set(k,(n.exists(k)?n.get(k):0)+m.get(k)); return copy({rawEdgeTrimByCluster:n});
    }
    public function addJustificationDeltas(m:Map<Int,Float>):PunctuationGeometryLedger return copy({justificationDeltaByCluster:m});
    public function consumeLeadingByCluster(m:Map<Int,Float>):PunctuationGeometryLedger return consumeSide(m,true);
    public function consumeTrailingByCluster(m:Map<Int,Float>):PunctuationGeometryLedger return consumeSide(m,false);
    function consumeSide(m:Map<Int,Float>, leading:Bool):PunctuationGeometryLedger { final n=new Map<Int,GlueBudget>(); for(k in budgets.keys()) n.set(k,budgets.get(k)); for(k in m.keys()) if(n.exists(k)&&m.get(k)>0){final b=n.get(k); n.set(k,leading?b.copy({leadingConsumed:Math.min(b.leadingNatural,b.leadingConsumed+m.get(k))}):b.copy({trailingConsumed:Math.min(b.trailingNatural,b.trailingConsumed+m.get(k))}));} return copy({budgets:n}); }
    public function glueCapacities():Map<Int,GlueCapacity> { final r=new Map<Int,GlueCapacity>(); for(k in budgets.keys()){final b=budgets.get(k);if(b.leadingRemaining>0||b.trailingRemaining>0)r.set(k,new GlueCapacity(b.leadingRemaining,b.trailingRemaining,geometries.exists(k)&&geometries.get(k).anchor==PunctuationAnchor.Center));}return r; }
    public function toDecisionInfo():Array<ClusterGeometryDecisionInfo>{final r:Array<ClusterGeometryDecisionInfo>=[];for(k in geometries.keys()){final g=geometries.get(k),b=budgets.get(k);r.push(new ClusterGeometryDecisionInfo(g.range,g.sourceText,g.displayText,g.baseAdvance,g.bodyWidth,b.leadingNatural,b.leadingConsumed,b.trailingNatural,b.trailingConsumed,justificationDeltaByCluster.exists(k)?justificationDeltaByCluster.get(k):0,resolvedAdvance(k,naturalClusters[k]),"PunctuationGeometryLedger",g.reason,rubySpreadByCluster.exists(k)?rubySpreadByCluster.get(k):0,g.glyphInlineShift,g.glyphPlacementReason));}return r;}
    function consumeSpacing(p:PunctuationSpacingCompressionResult):PunctuationGeometryLedger { final n=new Map<Int,GlueBudget>();for(k in budgets.keys())n.set(k,budgets.get(k));for(a in p.adjustments){var idx=-1;for(i in 0...naturalClusters.length)if(isInside(a.reductionTargetRange,naturalClusters[i].range)){idx=i;break;}if(idx>=0&&n.exists(idx)){final b=n.get(idx);if(geometries.get(idx).anchor==PunctuationAnchor.Center){final x=Math.min(a.reduction/2,b.leadingRemaining,b.trailingRemaining);n.set(idx,b.copy({leadingConsumed:b.leadingConsumed+x,trailingConsumed:b.trailingConsumed+x}));}else if(b.trailingRemaining>=b.leadingRemaining)n.set(idx,b.copy({trailingConsumed:Math.min(b.trailingNatural,b.trailingConsumed+a.reduction)}));else n.set(idx,b.copy({leadingConsumed:Math.min(b.leadingNatural,b.leadingConsumed+a.reduction)}));}}return copy({budgets:n}); }
    function resolvedAdvance(i:Int,c:Cluster):Float {final raw=rawEdgeTrimByCluster.exists(i)?rawEdgeTrimByCluster.get(i):0,sp=rubySpreadByCluster.exists(i)?rubySpreadByCluster.get(i):0,d=justificationDeltaByCluster.exists(i)?justificationDeltaByCluster.get(i):0,att=attachedInlineTrailingGlueByCluster.exists(i)?attachedInlineTrailingGlueByCluster.get(i):0,box=inlineBoxAdvanceByCluster.exists(i)?inlineBoxAdvanceByCluster.get(i):0;if(!geometries.exists(i))return Math.max(0,c.advance+d+sp+att-raw);final g=geometries.get(i);if(!budgets.exists(i))return Math.max(0,g.bodyWidth+box+d+sp-raw);final b=budgets.get(i);return Math.max(0,g.bodyWidth+box+b.leadingRemaining+b.trailingRemaining+d+sp+att-raw);}
    public function consumeLineEdgeGlue(lines:Array<LineCandidate>, ?forceLineEndHalfWidth:Bool):LineEdgeTrimResult {return new LineEdgeTrimResult(this,[]);}
    public function resolveAttachedInlinePunctuationBoundaries(a:Array<InlineAttachment>, atoms:Array<PunctuationAtom>, em:Float):AttachedInlinePunctuationBoundaryResult {if(a.length!=naturalClusters.length)throw "Inline attachments must align with punctuation geometry clusters.";return new AttachedInlinePunctuationBoundaryResult(this,new Map(),[]);}
    public static function isInside(self:TextRange,other:TextRange):Bool return self.start>=other.start&&self.end<=other.end;
    public static function clusterIndexRangeFor(self:Array<Cluster>,r:TextRange):Null<IntRange>{var f=-1,l=-1;for(i in 0...self.length)if(self[i].range.start>=r.start&&self[i].range.end<=r.end){if(f<0)f=i;l=i;}return f<0?null:f...l+1;}
}
@:dataClass class AttachedInlinePunctuationBoundaryResult {public final geometry:PunctuationGeometryLedger;public final trailingGlueByCluster:Map<Int,Float>;public final decisions:Array<SpacingDecisionInfo>;public function new(geometry:PunctuationGeometryLedger,trailingGlueByCluster:Map<Int,Float>,decisions:Array<SpacingDecisionInfo>){this.geometry=geometry;this.trailingGlueByCluster=trailingGlueByCluster;this.decisions=decisions;}}
@:dataClass class PunctuationClusterGeometry {public final range:TextRange;public final sourceText:String;public final displayText:String;public final baseAdvance:Float;public final bodyWidth:Float;public final leadingGlueNatural:Float;public final trailingGlueNatural:Float;public final leadingGlueInitiallyConsumed:Float;public final trailingGlueInitiallyConsumed:Float;public final glyphInlineShift:Float;public final glyphPlacementReason:Null<String>;public final anchor:Null<PunctuationAnchor>;public final reason:String;public function new(range:TextRange,sourceText:String,displayText:String,baseAdvance:Float,bodyWidth:Float,leadingGlueNatural:Float,trailingGlueNatural:Float,leadingGlueInitiallyConsumed:Float,trailingGlueInitiallyConsumed:Float,glyphInlineShift:Float,glyphPlacementReason:Null<String>,anchor:Null<PunctuationAnchor>,reason:String){this.range=range;this.sourceText=sourceText;this.displayText=displayText;this.baseAdvance=baseAdvance;this.bodyWidth=bodyWidth;this.leadingGlueNatural=leadingGlueNatural;this.trailingGlueNatural=trailingGlueNatural;this.leadingGlueInitiallyConsumed=leadingGlueInitiallyConsumed;this.trailingGlueInitiallyConsumed=trailingGlueInitiallyConsumed;this.glyphInlineShift=glyphInlineShift;this.glyphPlacementReason=glyphPlacementReason;this.anchor=anchor;this.reason=reason;}}
@:dataClass class GlueBudget {public final leadingNatural:Float;public final leadingConsumed:Float;public final trailingNatural:Float;public final trailingConsumed:Float;public function new(leadingNatural:Float,leadingConsumed:Float,trailingNatural:Float,trailingConsumed:Float){this.leadingNatural=leadingNatural;this.leadingConsumed=leadingConsumed;this.trailingNatural=trailingNatural;this.trailingConsumed=trailingConsumed;}public var leadingRemaining(get,never):Float;function get_leadingRemaining()return Math.max(0,leadingNatural-leadingConsumed);public var trailingRemaining(get,never):Float;function get_trailingRemaining()return Math.max(0,trailingNatural-trailingConsumed);}
@:dataClass class LineEdgeTrimResult {public final geometry:PunctuationGeometryLedger;public final decisions:Array<LineEdgeTrimDecisionInfo>;public function new(geometry:PunctuationGeometryLedger,decisions:Array<LineEdgeTrimDecisionInfo>){this.geometry=geometry;this.decisions=decisions;}}
@:dataClass class GlueCapacity {public final leading:Float;public final trailing:Float;public final paired:Bool;public function new(leading:Float,trailing:Float,paired:Bool){this.leading=leading;this.trailing=trailing;this.paired=paired;}}
