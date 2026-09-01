package org.tiqian.layout;
import org.tiqian.core.Cluster;
import org.tiqian.core.EastAsianSpacingEdges;
import org.tiqian.core.IntRange;
import org.tiqian.core.TextRange;
import org.tiqian.core.UnicodeEastAsianSpacing;
import org.tiqian.font.FontRole;
import org.tiqian.layout.PunctuationModel.GlueKind;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;

class JustifierTestSupport {
 public static var em:Float = 16.0;
 public static function cjk(at:Int):Cluster { return new Cluster(new TextRange(at, at + 1), "\u4E2D", "cjk", em); }
 public static function space(at:Int):Cluster { return new Cluster(new TextRange(at, at + 1), " ", "latin", 0.25 * em); }
 public static function latin(at:Int, w:Float):Cluster { return new Cluster(new TextRange(at, at + 2), "Hi", "latin", w); }
 public static function slashLatin(at:Int, w:Float):Cluster { return new Cluster(new TextRange(at, at + 3), "/Hi", "latin", w); }
 public static function punctuation(at:Int, ?text:String):Cluster { return new Cluster(new TextRange(at, at + 1), text == null ? "\uFF08" : text, "cjk", em); }
 public static function westernBracket(at:Int, text:String):Cluster { return new Cluster(new TextRange(at, at + 1), text, "latin", 0.5 * em); }
 public static function inlineObject(at:Int, text:String):Cluster { return new Cluster(new TextRange(at, at + text.length), text, "inline-object", 2.0 * em, ""); }
 public static function spacingEdges(clusters:Array<Cluster>):Array<EastAsianSpacingEdges> {
  var a:Array<EastAsianSpacingEdges> = []; var i = 0;
  while (i < clusters.length) { a.push(UnicodeEastAsianSpacing.resolvedEdges(clusters[i].text, "zh-Hans")); i++; }
  return a;
 }
}

class JustifierTest {
 @:test public static function westernDominantLineDoesNotStretchAroundCjkPunctuation():Void {
  new TestTraceRecorder("JustifierTest").section("westernDominantLineDoesNotStretchAroundCjkPunctuation");
  final clusters:Array<Cluster> = [
   JustifierTestSupport.latin(0, 3.0 * JustifierTestSupport.em),
   JustifierTestSupport.punctuation(2),
   JustifierTestSupport.latin(3, 3.0 * JustifierTestSupport.em),
   JustifierTestSupport.punctuation(5, "\uFF09"),
   JustifierTestSupport.punctuation(6, "\u3001"),
   JustifierTestSupport.latin(7, 3.0 * JustifierTestSupport.em),
  ];
  final roles:Array<FontRole> = [
   FontRole.LatinText,
   FontRole.CjkPunctuation,
   FontRole.LatinText,
   FontRole.CjkPunctuation,
   FontRole.CjkPunctuation,
   FontRole.LatinText,
  ];
  var natural = 0.0;
  var ni = 0;
  while (ni < clusters.length) { natural += clusters[ni].advance; ni++; }
  final plan = new Justifier().justify(clusters, roles, JustifierTestSupport.spacingEdges(clusters), new IntRange(0, clusters.length - 1), natural + 2.0 * JustifierTestSupport.em, JustifierTestSupport.em, false, null, true, 0.25 * JustifierTestSupport.em, 0.5 * JustifierTestSupport.em);
  var noneCjkInterChar = true;
  var ai = 0;
  while (ai < plan.allocations.length) {
   if (plan.allocations[ai].kind == GlueKind.CjkInterChar) { noneCjkInterChar = false; break; }
   ai++;
  }
  TracedAssertions.assertTrue(noneCjkInterChar);
  TracedAssertions.assertEqualsFloatTolerance(2.0 * JustifierTestSupport.em, plan.unfilledDeficit, 0.001);
  TracedAssertions.assertEqualsString("WesternDominantLineNaturalSpacing", plan.fallbackReason);
 }
}
