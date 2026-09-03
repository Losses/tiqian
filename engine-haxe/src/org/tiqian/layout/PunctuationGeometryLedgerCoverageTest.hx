package org.tiqian.layout;

import org.tiqian.core.*;
import org.tiqian.layout.PunctuationModel.PunctuationAtomBuilder;
import org.tiqian.layout.PunctuationModel.PunctuationAtom;
import org.tiqian.clreq.PunctuationGluePlacement;
import org.tiqian.clreq.PunctuationWidthPolicy;
import org.tiqian.layout.PunctuationModel.PunctuationSpacingCompressor;
import org.tiqian.layout.PunctuationModel.PunctuationSpacingAdjustment;
import org.tiqian.layout.PunctuationModel.PunctuationSpacingCompressionResult;
import org.tiqian.layout.PunctuationModel.PunctuationInkInput;
import org.tiqian.test.trace.*;
import std.SortedMap;

@:test class PunctuationGeometryLedgerCoverageTest {

    @:test public static function budgetsResolveAdvancesThroughRemainingGlue():Void {
        PunctuationGeometryLedgerCoverageSupport.start("budgetsResolveAdvancesThroughRemainingGlue");
        var x=PunctuationGeometryLedgerCoverageSupport.ledger(["。","「","中"]);
        var resolved=x.resolveClusters();
        TracedAssertions.assertEqualsFloat(8,resolved[0].advance);
        TracedAssertions.assertEqualsFloat(16,resolved[1].advance);
        TracedAssertions.assertEqualsFloat(16,resolved[2].advance);
        TracedAssertions.assertTrue(resolved[2]==x.resolveClusters()[2]);
    }
    @:test public static function glueCapacitiesReportSidesAndPairing():Void {PunctuationGeometryLedgerCoverageSupport.start("glueCapacitiesReportSidesAndPairing");var m=PunctuationGeometryLedgerCoverageSupport.ledger(["。","「"]).glueCapacities();TracedAssertions.assertTrue(m.has(1));var mk:Array<Int>=[];for(i in 0...m.size())mk.push(m.keyAt(i));TracedAssertions.assertEqualsRendered("[1]",Std.string(mk));TracedAssertions.assertEqualsFloat(8,m.get(1).leading);TracedAssertions.assertEqualsFloat(0,m.get(1).trailing);TracedAssertions.assertFalse(m.get(1).paired);}
    @:test public static function sideConsumptionIsCappedAndSkipsNonPositiveAmounts():Void {PunctuationGeometryLedgerCoverageSupport.start("sideConsumptionIsCappedAndSkipsNonPositiveAmounts");var x=PunctuationGeometryLedgerCoverageSupport.ledger(["。","「"]).consumeLeadingByCluster(PunctuationGeometryLedgerCoverageSupport.budgetAt([1],4.0)).glueCapacities();TracedAssertions.assertEqualsFloat(4,x.get(1).leading);var y=PunctuationGeometryLedgerCoverageSupport.ledger(["。","「"]).consumeLeadingByCluster(PunctuationGeometryLedgerCoverageSupport.budgetAt([1],100.0));TracedAssertions.assertFalse(y.glueCapacities().has(1));}
    @:test public static function justificationDeltasAndStructuralChannelsFeedResolvedAdvance():Void {
        PunctuationGeometryLedgerCoverageSupport.start("justificationDeltasAndStructuralChannelsFeedResolvedAdvance");
        var base=PunctuationGeometryLedgerCoverageSupport.ledger(["「","中"]);
        TracedAssertions.assertEqualsFloat(16,base.resolveClusters()[0].advance);
        var b=SortedMap.builder();b.put(0,1.5);var justified=base.addJustificationDeltas(b.build());
        TracedAssertions.assertEqualsFloat(17.5,justified.resolveClusters()[0].advance);
        TracedAssertions.assertEqualsFloat(1.5,justified.toDecisionInfo()[0].justificationDelta);
        TracedAssertions.assertEqualsFloat(0,base.toDecisionInfo()[0].justificationDelta);
        var s=SortedMap.builder();s.put(0,2.0);var spread=base.withRubySpread(s.build());
        TracedAssertions.assertEqualsFloat(18,spread.resolveClusters()[0].advance);
        TracedAssertions.assertEqualsFloat(2,spread.toDecisionInfo()[0].rubySpread);
        var r=SortedMap.builder();r.put(0,3.0);var trimmed=base.withRawEdgeTrims(r.build());
        TracedAssertions.assertEqualsFloat(13,trimmed.resolveClusters()[0].advance);
        var r2=SortedMap.builder();r2.put(0,20.0);var trimmedTwice=trimmed.withRawEdgeTrims(r2.build());
        TracedAssertions.assertEqualsFloat(0,trimmedTwice.resolveClusters()[0].advance);
        var empty=SortedMap.builder().build();
        TracedAssertions.assertTrue(base.withRubySpread(empty)==base);
        TracedAssertions.assertTrue(base.withRawEdgeTrims(empty)==base);
        TracedAssertions.assertTrue(base.withInlineBoxAdvances(empty)==base);
        var box=SortedMap.builder();box.put(0,4.0);var boxed=base.withInlineBoxAdvances(box.build());
        TracedAssertions.assertEqualsFloat(20,boxed.resolveClusters()[0].advance);
    }
    @:test public static function geometryWithoutBudgetFallsBackToBodyWidth():Void {PunctuationGeometryLedgerCoverageSupport.start("geometryWithoutBudgetFallsBackToBodyWidth");var x=PunctuationGeometryLedgerCoverageSupport.ledger(["「","中"]);var e=SortedMap.builder().build();var y=new PunctuationGeometryLedger(x.naturalClusters,x.geometries,e);TracedAssertions.assertEqualsFloat(8,y.resolveClusters()[0].advance);}
    @:test public static function decisionInfoListsEveryGeometryWithBudgets():Void {PunctuationGeometryLedgerCoverageSupport.start("decisionInfoListsEveryGeometryWithBudgets");var i=PunctuationGeometryLedgerCoverageSupport.ledger(["。","中"]).toDecisionInfo()[0];TracedAssertions.assertEqualsFloat(8,i.bodyWidth);TracedAssertions.assertEqualsFloat(16,i.resolvedAdvance);TracedAssertions.assertEqualsString("PunctuationGeometryLedger",i.source);}
    @:test public static function spacingPlanAdjustmentsConsumeByTargetAndAnchor():Void {PunctuationGeometryLedgerCoverageSupport.start("spacingPlanAdjustmentsConsumeByTargetAndAnchor");var x=PunctuationGeometryLedgerCoverageSupport.ledger(["「","「"]);TracedAssertions.assertEqualsFloat(8,x.glueCapacities().get(0).leading);var a=new PunctuationSpacingAdjustment(new TextRange(0,1),new TextRange(0,1),"「","「",8,4,4,"test");var y=PunctuationGeometryLedger.from([PunctuationGeometryLedgerCoverageSupport.c("「",0),PunctuationGeometryLedgerCoverageSupport.c("「",1)],[],new PunctuationSpacingCompressionResult([a]));TracedAssertions.assertTrue(y.toDecisionInfo().length==0);}
    @:test public static function attachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachments():Void {PunctuationGeometryLedgerCoverageSupport.start("attachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachments");var x=PunctuationGeometryLedgerCoverageSupport.ledger(["。","中"]);TracedAssertions.assertFailsWith(null,function() x.resolveAttachedInlinePunctuationBoundaries([InlineAttachment.None],[],PunctuationGeometryLedgerCoverageSupport.em));var r=x.resolveAttachedInlinePunctuationBoundaries([InlineAttachment.None,InlineAttachment.None],[],PunctuationGeometryLedgerCoverageSupport.em);TracedAssertions.assertTrue(r.decisions.length==0);}
    @:test public static function attachedInlineBoundaryAtLineEndConsumesTrailingGlue():Void {PunctuationGeometryLedgerCoverageSupport.start("attachedInlineBoundaryAtLineEndConsumesTrailingGlue");var x=PunctuationGeometryLedgerCoverageSupport.ledger(["」","ref"]);var r=x.resolveAttachedInlinePunctuationBoundaries([InlineAttachment.None,InlineAttachment.Previous],[],PunctuationGeometryLedgerCoverageSupport.em);TracedAssertions.assertEqualsString("AttachedInlineVirtualPunctuationBoundary:line-end",r.decisions[0].reason);TracedAssertions.assertEqualsFloat(8,r.geometry.resolveClusters()[0].advance);}
    @:test public static function attachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlue():Void {PunctuationGeometryLedgerCoverageSupport.start("attachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlue");var cs=[PunctuationGeometryLedgerCoverageSupport.c("」",0),PunctuationGeometryLedgerCoverageSupport.c("ref",1),PunctuationGeometryLedgerCoverageSupport.c("「",4)];var atoms:Array<PunctuationAtom>=[];for(q in cs)for(a in PunctuationGeometryStage.punctuationAtoms(q,PunctuationGeometryLedgerCoverageSupport.em,PunctuationGeometryLedgerCoverageSupport.builder,[],PunctuationGluePlacement.MainlandSimplified,new PunctuationWidthPolicy()))atoms.push(a);var r=PunctuationGeometryLedger.from(cs,atoms,new PunctuationSpacingCompressionResult([])).resolveAttachedInlinePunctuationBoundaries([InlineAttachment.None,InlineAttachment.Previous,InlineAttachment.None],atoms,PunctuationGeometryLedgerCoverageSupport.em);TracedAssertions.assertEqualsFloat(8,r.decisions[0].adjustedInnerGlue);}
    @:test public static function attachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacent():Void {PunctuationGeometryLedgerCoverageSupport.start("attachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacent");var x=PunctuationGeometryLedgerCoverageSupport.ledger(["」","ref",","]);var r=x.resolveAttachedInlinePunctuationBoundaries([InlineAttachment.None,InlineAttachment.Previous,InlineAttachment.None],[],PunctuationGeometryLedgerCoverageSupport.em);TracedAssertions.assertTrue(r.decisions.length>=1);}
    @:test public static function attachedInlineBoundarySkipsMandatoryBreakNeighbour():Void {
        PunctuationGeometryLedgerCoverageSupport.start("attachedInlineBoundarySkipsMandatoryBreakNeighbour");
        var cs:Array<Cluster>=[PunctuationGeometryLedgerCoverageSupport.c("」",0),PunctuationGeometryLedgerCoverageSupport.c("ref",1,16,"latin"),new Cluster(new TextRange(3,4),"\n","mandatory-break",0,"")];
        var atoms:Array<PunctuationAtom>=PunctuationGeometryStage.punctuationAtoms(cs[0],PunctuationGeometryLedgerCoverageSupport.em,PunctuationGeometryLedgerCoverageSupport.builder,[],PunctuationGluePlacement.MainlandSimplified,new PunctuationWidthPolicy());
        var x=PunctuationGeometryLedger.from(cs,atoms,new PunctuationSpacingCompressionResult([]));
        var r=x.resolveAttachedInlinePunctuationBoundaries([InlineAttachment.None,InlineAttachment.Previous,InlineAttachment.None],atoms,PunctuationGeometryLedgerCoverageSupport.em);
        TracedAssertions.assertEqualsString("AttachedInlineVirtualPunctuationBoundary:line-end",r.decisions[0].reason);
    }
    @:test public static function attachedInlineBoundaryWithoutGlueEmitsNoDecision():Void {PunctuationGeometryLedgerCoverageSupport.start("attachedInlineBoundaryWithoutGlueEmitsNoDecision");var r=PunctuationGeometryLedgerCoverageSupport.ledger(["「","ref","中"]).resolveAttachedInlinePunctuationBoundaries([InlineAttachment.None,InlineAttachment.Previous,InlineAttachment.None],[],PunctuationGeometryLedgerCoverageSupport.em);TracedAssertions.assertTrue(r.decisions.length==0);}
    @:test public static function lineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputs():Void {PunctuationGeometryLedgerCoverageSupport.start("lineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputs");var x=PunctuationGeometryLedgerCoverageSupport.ledger(["」","中"]);TracedAssertions.assertTrue(x.consumeLineEdgeGlue([]).decisions.length==0);var r=x.consumeLineEdgeGlue([PunctuationGeometryLedgerCoverageSupport.line(0,0)]);TracedAssertions.assertEqualsString("LineEndHalfWidthPunctuation",r.decisions[0].reason);TracedAssertions.assertEqualsFloat(8,r.geometry.resolveClusters()[0].advance);}
    @:test public static function lineEdgeTrimConsumesCentredPunctuationOncePerLine():Void {PunctuationGeometryLedgerCoverageSupport.start("lineEdgeTrimConsumesCentredPunctuationOncePerLine");var atom:PunctuationAtom=PunctuationGeometryLedgerCoverageSupport.builder.build('·',new TextRange(0,1),PunctuationGeometryLedgerCoverageSupport.em,new PunctuationInkInput(16,new Rect(2,4,10,12),8,-2));var x=PunctuationGeometryLedger.from([PunctuationGeometryLedgerCoverageSupport.c("·",0)],[atom],new PunctuationSpacingCompressionResult([]));var r=x.consumeLineEdgeGlue([PunctuationGeometryLedgerCoverageSupport.line(0,0)]);TracedAssertions.assertEqualsString("both",r.decisions[0].side);TracedAssertions.assertEqualsFloat(4,r.decisions[0].trimAmount);}
    @:test public static function clusterIndexRangeFindCoveredClusters():Void {PunctuationGeometryLedgerCoverageSupport.start("clusterIndexRangeFindCoveredClusters");var cs=[PunctuationGeometryLedgerCoverageSupport.c("中",0),PunctuationGeometryLedgerCoverageSupport.c("中",1),PunctuationGeometryLedgerCoverageSupport.c("中",2)];TracedAssertions.assertTrue(PunctuationGeometryLedger.clusterIndexRangeFor([],new TextRange(0,3))==null);var a=PunctuationGeometryLedger.clusterIndexRangeFor(cs,new TextRange(0,3));TracedAssertions.assertEqualsIntRange(new IntRange(0,2),a);var b=PunctuationGeometryLedger.clusterIndexRangeFor(cs,new TextRange(1,2));TracedAssertions.assertEqualsIntRange(new IntRange(1,1),b);TracedAssertions.assertTrue(PunctuationGeometryLedger.clusterIndexRangeFor(cs,new TextRange(5,6))==null);var d=PunctuationGeometryLedger.clusterIndexRangeFor(cs,new TextRange(0,1));TracedAssertions.assertEqualsIntRange(new IntRange(0,0),d);}
}

/** Shared fixtures and cluster builders for PunctuationGeometryLedgerCoverageTest; the Kotlin test-class lowering admits test functions only. */
class PunctuationGeometryLedgerCoverageSupport {
    public static final em:Float = 16;
    public static final builder = new PunctuationAtomBuilder();
    public static function start(n:String):Void new TestTraceRecorder("PunctuationGeometryLedgerCoverageTest").section(n);
    public static function c(text:String, start:Int, ?advance:Float=16, ?font:String="cjk"):Cluster return new Cluster(new TextRange(start,start+text.length),text,font,advance,text);
    public static function ledger(texts:Array<String>):PunctuationGeometryLedger {
        var cs:Array<Cluster>=[]; var atoms:Array<PunctuationAtom>=[]; var i=0; var pos=0;
        while(i<texts.length){var x=c(texts[i],pos);cs.push(x);var aa=PunctuationGeometryStage.punctuationAtoms(x,em,builder,[],PunctuationGluePlacement.MainlandSimplified,new PunctuationWidthPolicy());for(a in aa)atoms.push(a);pos+=texts[i].length;i++;}
        return PunctuationGeometryLedger.from(cs,atoms,new PunctuationSpacingCompressor().compress(atoms,em));
    }
    public static function budgetAt(entries:Array<Int>, value:Float):SortedMap<Int,Float>{var b=SortedMap.builder();for(k in entries)b.put(k,value);return b.build();}
    public static function line(s:Int,e:Int):LineOptimization.LineCandidate return new LineOptimization.LineCandidate(new IntRange(s,e),new TextRange(s,e+1),32,32);
}
