package org.tiqian.font;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;
import org.tiqian.font.FontMetrics.FontMetricsRequest;
import org.tiqian.font.FontMetrics.StubFontMetricsResolver;
import org.tiqian.font.FontMetrics.ScriptAwareFontMetricsNormalizer;
import org.tiqian.font.FontMetrics.FontMetricsNormalizationInput;
class ScriptAwareFontMetricsNormalizerTest {
 static function req(key:String,role:FontRole):FontMetricsRequest return new FontMetricsRequest(key,16,role,"zh-Hans");
 @:test public static function cjkTextUsesFontDeclaredTypoBoxInsteadOfSynthesizedSquare():Void { new TestTraceRecorder("ScriptAwareFontMetricsNormalizerTest").section("cjkTextUsesFontDeclaredTypoBoxInsteadOfSynthesizedSquare");var r=req("cjk-primary",CjkText);var raw=new StubFontMetricsResolver().resolve(r);var l=new ScriptAwareFontMetricsNormalizer().normalize(new FontMetricsNormalizationInput(r,raw));TracedAssertions.assertEqualsFloat(14.08,raw.typoAscent);TracedAssertions.assertEqualsFloat(14.08,l.ascent);TracedAssertions.assertEqualsRendered("IdeographicLow",Std.string(l.baselineClass)); }
 @:test public static function cjkTextFallsBackToHheaWhenFontHasNoTypoMetrics():Void { new TestTraceRecorder("ScriptAwareFontMetricsNormalizerTest").section("cjkTextFallsBackToHheaWhenFontHasNoTypoMetrics");var r=req("cjk-bad",CjkText);var l=new ScriptAwareFontMetricsNormalizer().normalize(new FontMetricsNormalizationInput(r,new RawFontMetrics(18.4,4)));TracedAssertions.assertEqualsFloat(18.4,l.ascent);TracedAssertions.assertEqualsRendered("Raw",Std.string(l.policy)); }
 @:test public static function latinTextKeepsRomanRawMetrics():Void { new TestTraceRecorder("ScriptAwareFontMetricsNormalizerTest").section("latinTextKeepsRomanRawMetrics");var r=req("latin-primary",LatinText);var raw=new StubFontMetricsResolver().resolve(r);var l=new ScriptAwareFontMetricsNormalizer().normalize(new FontMetricsNormalizationInput(r,raw));TracedAssertions.assertEqualsFloat(raw.ascent,l.ascent);TracedAssertions.assertEqualsRendered("Roman",Std.string(l.baselineClass)); }
}
