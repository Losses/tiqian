package org.tiqian.layout;

import org.tiqian.core.*;
import org.tiqian.test.LayoutFixtures.LayoutFixture;
import org.tiqian.font.FontMetrics.StubFontMetricsResolver;
import org.tiqian.shaping.TextShaper.ITextShaper;
import org.tiqian.shaping.TextShaper.ExplainableStubTextShaper;
import org.tiqian.linebreak.Hyphenator.NoHyphenator;
import org.tiqian.linebreak.EnglishHyphenation;
import org.tiqian.layout.LineBreaker.GreedyLineBreaker;
import org.tiqian.layout.LineBreaker.LookaheadLineBreaker;
import org.tiqian.clreq.ClreqProfileResolver;
import org.tiqian.clreq.ClreqProfile;
import org.tiqian.clreq.KinsokuMode;
import org.tiqian.layout.ParagraphLayoutEngine.ExplainableStubParagraphLayoutEngine;

class LayoutDumpFormat {
    static function joinInts(a:std.ReadOnlyArray<Int>):String { final b=[]; for (i in 0...a.length) b.push(Std.string(a[i])); return b.join(","); }
    public static function dumpFmt(value:Float):String {
        if (Math.isNaN(value)) return "NaN";
        if (!Math.isFinite(value)) return value > 0 ? "Infinity" : "-Infinity";
        final negative = haxe.io.FPHelper.floatToI32(value) < 0;
        final magnitude = Math.floor(Math.abs(value) * 10 + 0.5);
        return (negative ? "-" : "") + Std.string(Math.floor(magnitude / 10)) + "." + Std.string(Std.int(magnitude) % 10);
    }
    public static function escapeDumpText(value:String):String {
        final out = new StringBuf();
        for (i in 0...value.length) {
            final c = value.charCodeAt(i);
            if (c == 10) out.add("\\n"); else if (c == 13) out.add("\\r"); else if (c == 11) out.add("\\v"); else if (c == 12) out.add("\\f");
            else if (c == 0x85) out.add("\\u0085"); else if (c == 0x2028) out.add("\\u2028"); else if (c == 0x2029) out.add("\\u2029"); else if (c == 0x200b) out.add("\\u200B");
            else out.addChar(c);
        }
        return out.toString();
    }
    public static function layoutFixtureDump(f:LayoutFixture):String {
        final out = new StringBuf(); out.add("fixture: "); out.add(f.id); out.add("\ntext: "); out.add(escapeDumpText(f.text));
        out.add("\nmaxWidth: "); out.add(dumpFmt(f.constraints.maxWidth)); out.add("\n");
        final shaper:ITextShaper = new ExplainableStubTextShaper();
        for (n in 0...3) {
            final breaker:LineBreaker = n == 0 ? new GreedyLineBreaker() : n == 1 ? new LookaheadLineBreaker() : new ParagraphDpLineBreaker();
            final hyphenator = f.useEnglishHyphenation ? EnglishHyphenation.enUs() : new NoHyphenator();
            final resolver:Null<ClreqProfileResolver> = f.pinBasicNoHang ? new BasicProfileResolver() : null;
            final engine = new ExplainableStubParagraphLayoutEngine(null, null, resolver, new StubFontMetricsResolver(), null, null, null, null,
                breaker, null, shaper, hyphenator, null);
            final indent:Null<Ic> = f.firstLineIndentEm == null ? null : new Ic(f.firstLineIndentEm);
            final input = new LayoutInput(new TiqianTextContent(f.text, f.lineBreakSpans), null,
                new ParagraphStyle(null, null, f.lineHeight, indent, null, null, f.lineLengthGrid, f.rubyLineHeightMode), f.constraints,
                null, f.decorations, f.rubySpans);
            out.add(decisionDump(engine.layout(input), n == 0 ? "greedy" : n == 1 ? "lookahead" : "paragraph-dp"));
        }
        return out.toString();
    }
    public static function decisionDump(r:LayoutResult, label:String):String {
        final o = new StringBuf(); final d = r.debug;
        o.add("== "); o.add(label); o.add(" ==\nsize "); o.add(dumpFmt(r.size.width)); o.add("x"); o.add(dumpFmt(r.size.height)); o.add("\n");
        if (d.lineLengthGridDecision != null && d.lineLengthGridDecision.enabled && d.lineLengthGridDecision.slack > 0) { final g=d.lineLengthGridDecision; o.add("grid container="+dumpFmt(g.containerWidth)+" measure="+dumpFmt(g.measure)+"("+g.cells+"字) slack="+dumpFmt(g.slack)+" body="+g.bodyAlignment+"@"+dumpFmt(g.bodyOffset)+"\n"); }
        if (d.firstLineIndentDecision != null && d.firstLineIndentDecision.source != "Explicit") { final x=d.firstLineIndentDecision; o.add("firstindent "+dumpFmt(x.resolvedEm)+"字 measure="+dumpFmt(x.measureEm)+"字 threshold="+dumpFmt(x.thresholdEm)+"字 "+x.source+"\n"); }
        if (d.kinsokuDecision != null) { final x=d.kinsokuDecision; o.add("kinsoku measure="+dumpFmt(x.measureEm)+"字 level="+x.level+" hang="+x.hanging+" reason="+x.reason+"\n"); }
        for (i in 0...d.contextualKinsokuDecisions.length) { final x=d.contextualKinsokuDecisions[i]; o.add("context-kinsoku "+x.range.start+"-"+x.range.end+" source='"+escapeDumpText(x.sourceText)+"' cluster="+x.clusterIndex+" forbid="+x.forbiddenPosition+" reason="+x.reason+(x.impossibleMeasureFallback == null ? "" : " fallback="+x.impossibleMeasureFallback)+"\n"); }
        for (i in 0...d.breakOpportunityDecisions.length) { final x=d.breakOpportunityDecisions[i]; o.add("break-opportunity "+x.range.start+"-"+x.range.end+" source='"+escapeDumpText(x.sourceText)+"' offsets="+joinInts(x.breakOffsets)+(x.tier == null ? "" : " tier="+x.tier)+" reason="+x.reason+"\n"); }
        for (i in 0...d.emergencyTrackingEligibilityDecisions.length) { final x=d.emergencyTrackingEligibilityDecisions[i]; o.add("tracking-eligibility "+x.range.start+"-"+x.range.end+" source='"+escapeDumpText(x.sourceText)+"' reason="+x.reason+"\n"); }
        for (i in 0...r.lines.length) {
            final line=r.lines[i]; final x=i<d.lineDecisions.length ? d.lineDecisions[i] : null;
            final repair=x == null || x.repairDecision == null ? "-" : x.repairDecision.kind+"("+x.repairDecision.reasonCode+" shrink="+dumpFmt(x.repairDecision.shrink)+")";
            var candidates = "-"; if (x != null && x.repairCandidates.length > 0) { final a=[]; for (z in x.repairCandidates) a.push(z.kind+(z.accepted?"+":"-")); candidates=a.join(","); }
            var justify="-"; for (j in 0...d.justificationDecisions.length) if (d.justificationDecisions[j].lineRange.start==line.range.start && d.justificationDecisions[j].lineRange.end==line.range.end) { final q=d.justificationDecisions[j]; justify="deficit="+dumpFmt(q.deficitBefore)+"->"+dumpFmt(q.deficitAfter); for (a in q.allocations) justify += " "+a.kind+"@"+a.clusterRange.start+"+"+dumpFmt(a.delta)+(a.reason==a.kind?"":"("+a.reason+")"); break; }
            o.add("line["+i+"] "+line.range.start+"-"+line.range.end+" "+(line.indent>0?"indent="+dumpFmt(line.indent)+" ":"")+(line.hyphenAdvance>0?"hyphen="+dumpFmt(line.hyphenAdvance)+" ":"")+"natural="+dumpFmt(line.naturalWidth)+" adjusted="+dumpFmt(line.adjustedWidth)+" visual="+dumpFmt(line.visualWidth)+" repair="+repair+" candidates="+candidates+" justify="+justify+"\n");
        }
        for (c in r.clusters) o.add("cluster "+c.range.start+"-"+c.range.end+" '"+c.displayText+"' adv="+dumpFmt(c.advance)+(c.glyphInlineShift!=0?" glyphShift="+dumpFmt(c.glyphInlineShift):"")+"\n");
        for (x in d.fontDecisions) o.add("font "+x.range.start+"-"+x.range.end+" role="+x.role+" key="+x.fontKey+" display='"+x.displayText+"' sub="+x.substitutionReason+"\n");
        for (x in d.roleOverrides) o.add("role-override "+x.range.start+"-"+x.range.end+" source='"+escapeDumpText(x.sourceText)+"' "+x.originalRole+"->"+x.overriddenRole+" policy="+x.source+" reason="+x.reason+"\n");
        for (x in d.punctuationDecisions) o.add("punct "+x.range.start+"-"+x.range.end+" '"+x.char+"' class="+x.punctuationClass+" adv="+dumpFmt(x.advance)+" body="+dumpFmt(x.bodyWidth)+" lead="+dumpFmt(x.leadingGlueNatural)+" trail="+dumpFmt(x.trailingGlueNatural)+(x.leadingGlueInitiallyConsumed!=0||x.trailingGlueInitiallyConsumed!=0?" initial="+dumpFmt(x.leadingGlueInitiallyConsumed)+"/"+dumpFmt(x.trailingGlueInitiallyConsumed):"")+" anchor="+x.anchor+" source="+x.geometrySource+(x.advanceExpansion!=0?" expand="+dumpFmt(x.advanceExpansion):"")+(x.glyphInlineShift!=0?" glyphShift="+dumpFmt(x.glyphInlineShift):"")+(x.glyphPlacementReason==null?"":" placement="+x.glyphPlacementReason)+(x.haltAdvance==null?"":" halt="+dumpFmt(x.haltAdvance))+(x.inkBoundsFallback==null?"":" fallback="+x.inkBoundsFallback)+(x.haltValidation==null?"":" haltWarn="+x.haltValidation)+"\n");
        for (x in d.geometryDecisions) o.add("geom "+x.range.start+"-"+x.range.end+" body="+dumpFmt(x.bodyWidth)+" lead="+dumpFmt(x.leadingGlueConsumed)+"/"+dumpFmt(x.leadingGlueNatural)+" trail="+dumpFmt(x.trailingGlueConsumed)+"/"+dumpFmt(x.trailingGlueNatural)+" justify="+dumpFmt(x.justificationDelta)+(x.rubySpread!=0?" ruby="+dumpFmt(x.rubySpread):"")+(x.glyphInlineShift!=0?" glyphShift="+dumpFmt(x.glyphInlineShift):"")+(x.glyphPlacementReason==null?"":" placement="+x.glyphPlacementReason)+" resolved="+dumpFmt(x.resolvedAdvance)+"\n");
        for (x in d.inlineBoxDecisions) o.add("inline-box "+x.range.start+"-"+x.range.end+" start="+dumpFmt(x.inlineStart)+" end="+dumpFmt(x.inlineEnd)+" outer="+x.outerSpacing+" clusters="+x.firstClusterIndex+"-"+x.lastClusterIndex+" reason="+x.reason+"\n");
        for (x in d.spacingDecisions) o.add("spacing "+x.range.start+"-"+x.range.end+" '"+x.leftChar+x.rightChar+"' inner="+dumpFmt(x.naturalInnerGlue)+"->"+dumpFmt(x.adjustedInnerGlue)+" target="+x.reductionTargetRange.start+"-"+x.reductionTargetRange.end+"\n");
        for (x in d.autoSpaceDecisions) o.add("autospace "+x.clusterRange.start+"-"+x.clusterRange.end+" side="+x.side+" boundary="+x.boundaryRole+" reduction="+dumpFmt(x.totalReduction)+"\n");
        for (x in d.mandatoryBreakDecisions) o.add("mandatorybreak "+x.range.start+"-"+x.range.end+" afterCluster="+x.breakAfterClusterIndex+" reason="+x.reason+"\n");
        for (x in d.zeroWidthBreakDecisions) o.add("zerowidthbreak "+x.range.start+"-"+x.range.end+" source='"+escapeDumpText(x.sourceText)+"' cluster="+x.clusterIndex+" reason="+x.reason+"\n");
        for (x in d.lineEdgeTrimDecisions) o.add("edgetrim "+x.clusterRange.start+"-"+x.clusterRange.end+" side="+x.side+" trim="+dumpFmt(x.trimAmount)+" reason="+x.reason+"\n");
        for (x in d.decorationDecisions) o.add("deco "+x.clusterRange.start+"-"+x.clusterRange.end+" '"+x.sourceText+"' kind="+x.kind+" applied="+x.applied+" anchor="+dumpFmt(x.anchorX)+","+dumpFmt(x.anchorY)+" diameter="+dumpFmt(x.dotDiameter)+" reason="+x.reason+"\n");
        if (d.lineSpacingDecision != null) { final x=d.lineSpacingDecision; o.add("linespacing natural="+dumpFmt(x.naturalHeight)+" requested="+(x.requestedLineHeight==null?"-":dumpFmt(x.requestedLineHeight))+" resolved="+dumpFmt(x.resolvedHeight)+" floor="+dumpFmt(x.spacingFloor)+" applied="+x.floorApplied+" reason="+x.reason+"\n"); }
        if (d.maxLinesDecision != null) { final x=d.maxLinesDecision; o.add("maxlines laidOut="+x.laidOutLines+" visible="+x.visibleLines+" reason="+x.reason+"\n"); }
        return o.toString();
    }
    public static function layoutDumpDiffMessage(id:String, expected:String, actual:String):String {
        final e=expected.split("\n"), a=actual.split("\n"), diffs=[]; final n=e.length>a.length?e.length:a.length; for(i in 0...n) if((i<e.length?e[i]:null)!=(i<a.length?a[i]:null)){ diffs.push("  line "+(i+1)+":\n    golden: "+(i<e.length?e[i]:"<missing>")+"\n    actual: "+(i<a.length?a[i]:"<missing>")); if(diffs.length>=8){diffs.push("  …");break;} } return "golden mismatch for fixture '"+id+"':\n"+diffs.join("\n");
    }
}
class BasicProfileResolver implements ClreqProfileResolver {
    public function new() {}
    public function resolve(id:LayoutProfileId):ClreqProfile { final p=ClreqProfile.MainlandHorizontal; return new ClreqProfile(p.id,p.strictness,p.region,p.punctuationGlyphPolicy,null,p.autoSpace,p.gluePlacement,p.adjustment,KinsokuMode.Fixed(org.tiqian.clreq.KinsokuLevel.Basic, org.tiqian.clreq.HangingPunctuationStyle.Disabled),p.punctuationWidth); }
}
