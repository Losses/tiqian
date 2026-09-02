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
    static var em:Float = 16;
    static var builder = new PunctuationAtomBuilder();
    static function start(n:String):Void new TestTraceRecorder("PunctuationGeometryLedgerCoverageTest").section(n);
    static function c(text:String, start:Int, ?advance:Float=16, ?font:String="cjk"):Cluster return new Cluster(new TextRange(start,start+text.length),text,font,advance,text);
    static function ledger(texts:Array<String>):PunctuationGeometryLedger {
        var cs:Array<Cluster>=[]; var atoms:Array<PunctuationAtom>=[]; var i=0; var pos=0;
        while(i<texts.length){var x=c(texts[i],pos);cs.push(x);var aa=PunctuationGeometryStage.punctuationAtoms(x,em,builder,[],PunctuationGluePlacement.MainlandSimplified,new PunctuationWidthPolicy());for(a in aa)atoms.push(a);pos+=texts[i].length;i++;}
        return PunctuationGeometryLedger.from(cs,atoms,new PunctuationSpacingCompressor().compress(atoms,em));
    }
    static function map(entries:Array<Int>):SortedMap<Int,Float>{var b=SortedMap.builder();for(k in entries)b.put(k,4.0);return b.build();}
    static function line(s:Int,e:Int):LineOptimization.LineCandidate return new LineOptimization.LineCandidate(new IntRange(s,e),new TextRange(s,e+1),32,32);

    @:test public static function budgetsResolveAdvancesThroughRemainingGlue():Void { throw new IllegalStateException("TODO r10: budgetsResolveAdvancesThroughRemainingGlue"); }
    @:test public static function glueCapacitiesReportSidesAndPairing():Void {start("glueCapacitiesReportSidesAndPairing");var m=ledger(["。","「"]).glueCapacities();TracedAssertions.assertTrue(m.has(1));TracedAssertions.assertEqualsFloat(8,m.get(1).leading);TracedAssertions.assertEqualsFloat(0,m.get(1).trailing);TracedAssertions.assertFalse(m.get(1).paired);}
    @:test public static function sideConsumptionIsCappedAndSkipsNonPositiveAmounts():Void {start("sideConsumptionIsCappedAndSkipsNonPositiveAmounts");var x=ledger(["。","「"]).consumeLeadingByCluster(map([1])).glueCapacities();TracedAssertions.assertEqualsFloat(4,x.get(1).leading);var y=ledger(["。","「"]).consumeLeadingByCluster((function(){var b=SortedMap.builder();b.put(1,100.0);return b.build();})());TracedAssertions.assertFalse(y.glueCapacities().has(1));}
    @:test public static function justificationDeltasAndStructuralChannelsFeedResolvedAdvance():Void { throw new IllegalStateException("TODO r10: justificationDeltasAndStructuralChannelsFeedResolvedAdvance"); }
    @:test public static function geometryWithoutBudgetFallsBackToBodyWidth():Void {start("geometryWithoutBudgetFallsBackToBodyWidth");var x=ledger(["「","中"]);var e=SortedMap.builder().build();var y=new PunctuationGeometryLedger(x.naturalClusters,x.geometries,e);TracedAssertions.assertEqualsFloat(8,y.resolveClusters()[0].advance);}
    @:test public static function decisionInfoListsEveryGeometryWithBudgets():Void {start("decisionInfoListsEveryGeometryWithBudgets");var i=ledger(["。","中"]).toDecisionInfo()[0];TracedAssertions.assertEqualsFloat(8,i.bodyWidth);TracedAssertions.assertEqualsFloat(16,i.resolvedAdvance);TracedAssertions.assertEqualsString("PunctuationGeometryLedger",i.source);}
    @:test public static function spacingPlanAdjustmentsConsumeByTargetAndAnchor():Void {start("spacingPlanAdjustmentsConsumeByTargetAndAnchor");var x=ledger(["「","「"]);TracedAssertions.assertEqualsFloat(8,x.glueCapacities().get(0).leading);var a=new PunctuationSpacingAdjustment(new TextRange(0,1),new TextRange(0,1),"「","「",8,4,4,"test");var y=PunctuationGeometryLedger.from([c("「",0),c("「",1)],[],new PunctuationSpacingCompressionResult([a]));TracedAssertions.assertTrue(y.toDecisionInfo().length==0);}
    @:test public static function attachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachments():Void {start("attachedInlineBoundariesRequireAlignmentAndRunOnlyWithAttachments");var x=ledger(["。","中"]);TracedAssertions.assertFailsWith(null,function() x.resolveAttachedInlinePunctuationBoundaries([InlineAttachment.None],[],em));var r=x.resolveAttachedInlinePunctuationBoundaries([InlineAttachment.None,InlineAttachment.None],[],em);TracedAssertions.assertTrue(r.decisions.length==0);}
    @:test public static function attachedInlineBoundaryAtLineEndConsumesTrailingGlue():Void {start("attachedInlineBoundaryAtLineEndConsumesTrailingGlue");var x=ledger(["」","ref"]);var r=x.resolveAttachedInlinePunctuationBoundaries([InlineAttachment.None,InlineAttachment.Previous],[],em);TracedAssertions.assertEqualsString("AttachedInlineVirtualPunctuationBoundary:line-end",r.decisions[0].reason);TracedAssertions.assertEqualsFloat(8,r.geometry.resolveClusters()[0].advance);}
    @:test public static function attachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlue():Void {start("attachedInlineBoundaryAdjacentPunctuationHalvesTheVirtualGlue");var cs=[c("」",0),c("ref",1),c("「",4)];var atoms:Array<PunctuationAtom>=[];for(q in cs)for(a in PunctuationGeometryStage.punctuationAtoms(q,em,builder,[],PunctuationGluePlacement.MainlandSimplified,new PunctuationWidthPolicy()))atoms.push(a);var r=PunctuationGeometryLedger.from(cs,atoms,new PunctuationSpacingCompressionResult([])).resolveAttachedInlinePunctuationBoundaries([InlineAttachment.None,InlineAttachment.Previous,InlineAttachment.None],atoms,em);TracedAssertions.assertEqualsFloat(8,r.decisions[0].adjustedInnerGlue);}
    @:test public static function attachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacent():Void {start("attachedInlineBoundaryBeforeAsciiPointMarkCollapsesLikeAdjacent");var x=ledger(["」","ref",","]);var r=x.resolveAttachedInlinePunctuationBoundaries([InlineAttachment.None,InlineAttachment.Previous,InlineAttachment.None],[],em);TracedAssertions.assertTrue(r.decisions.length>=1);}
    @:test public static function attachedInlineBoundarySkipsMandatoryBreakNeighbour():Void { throw new IllegalStateException("TODO r10: attachedInlineBoundarySkipsMandatoryBreakNeighbour"); }
    @:test public static function attachedInlineBoundaryWithoutGlueEmitsNoDecision():Void {start("attachedInlineBoundaryWithoutGlueEmitsNoDecision");var r=ledger(["「","ref","中"]).resolveAttachedInlinePunctuationBoundaries([InlineAttachment.None,InlineAttachment.Previous,InlineAttachment.None],[],em);TracedAssertions.assertTrue(r.decisions.length==0);}
    @:test public static function lineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputs():Void {start("lineEdgeTrimConsumesHalfWidthAtEdgesAndSkipsEmptyInputs");var x=ledger(["」","中"]);TracedAssertions.assertTrue(x.consumeLineEdgeGlue([]).decisions.length==0);var r=x.consumeLineEdgeGlue([line(0,0)]);TracedAssertions.assertEqualsString("LineEndHalfWidthPunctuation",r.decisions[0].reason);TracedAssertions.assertEqualsFloat(8,r.geometry.resolveClusters()[0].advance);}
    @:test public static function lineEdgeTrimConsumesCentredPunctuationOncePerLine():Void {start("lineEdgeTrimConsumesCentredPunctuationOncePerLine");var atom:PunctuationAtom=builder.build('·',new TextRange(0,1),em,new PunctuationInkInput(16,new Rect(2,4,10,12),8,-2));var x=PunctuationGeometryLedger.from([c("·",0)],[atom],new PunctuationSpacingCompressionResult([]));var r=x.consumeLineEdgeGlue([line(0,0)]);TracedAssertions.assertEqualsString("both",r.decisions[0].side);TracedAssertions.assertEqualsFloat(4,r.decisions[0].trimAmount);}
    @:test public static function clusterIndexRangeFindCoveredClusters():Void {start("clusterIndexRangeFindCoveredClusters");var cs=[c("中",0),c("中",1),c("中",2)];TracedAssertions.assertTrue(PunctuationGeometryLedger.clusterIndexRangeFor(cs,new TextRange(0,3)).start==0);TracedAssertions.assertTrue(PunctuationGeometryLedger.clusterIndexRangeFor(cs,new TextRange(5,6))==null);}
}
