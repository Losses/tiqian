package org.tiqian.font;
import org.tiqian.font.FontPolicy.FontRequest;
import org.tiqian.core.TextRange;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;
import org.tiqian.font.FontMetrics.FontMetricsRequest;
import org.tiqian.font.FontMetrics.StubFontMetricsResolver;
import org.tiqian.font.FontMetrics.ScriptAwareFontMetricsNormalizer;
import org.tiqian.font.FontMetrics.FontMetricsNormalizationInput;
class FontPolicyCoverageTest {
 @:test public static function testFontRequestAndRoles():Void { new TestTraceRecorder("FontPolicyCoverageTest").section("testFontRequestAndRoles");var r=new FontRequest(["Source Han Sans"],"zh-Hans",CjkText);TracedAssertions.assertEqualsString("zh-Hans",r.locale);TracedAssertions.assertTrue(FontRoleFns.usesLatinFace(LatinText)); }
 @:test public static function testCjkFontRoleClassifierAllRanges():Void { new TestTraceRecorder("FontPolicyCoverageTest").section("testCjkFontRoleClassifierAllRanges");var c=new CjkFontRoleClassifier();TracedAssertions.assertEqualsRendered("CjkText",Std.string(c.classify("提",new TextRange(0,1))));TracedAssertions.assertEqualsRendered("Emoji",Std.string(c.classify("😀",new TextRange(0,2)))); }
 @:test public static function testPreferCjkForAmbiguousPunctuationResolver():Void { new TestTraceRecorder("FontPolicyCoverageTest").section("testPreferCjkForAmbiguousPunctuationResolver");var d=new PreferCjkForAmbiguousPunctuationResolver("cjk-key","latin-key","symbol-key").resolve("中",new TextRange(0,1),new FontRequest([],"zh-Hans",CjkText));TracedAssertions.assertEqualsString("cjk-key",d.candidate.key); }
 @:test public static function testFontEnumsAndModels():Void { new TestTraceRecorder("FontPolicyCoverageTest").section("testFontEnumsAndModels");var r=new RawFontMetrics(16,4,2,RawTables,14,2);TracedAssertions.assertEqualsFloat(16,r.ascent);var l=new LayoutFontMetrics(14,2,0,IdeographicBox,Ideographic,IdeographicLow,IdeographicEmBox,RawTables,"test");TracedAssertions.assertEqualsFloat(14,l.ascent); }
 @:test public static function testFontMetricsRequestAndResolvers():Void { new TestTraceRecorder("FontPolicyCoverageTest").section("testFontMetricsRequestAndResolvers");var r=new FontMetricsRequest("key1",16,CjkText,"zh-Hans");var m=new StubFontMetricsResolver().resolve(r);TracedAssertions.assertEqualsFloat(16*1.16,m.ascent); }
 @:test public static function testScriptAwareFontMetricsNormalizerBranches():Void { new TestTraceRecorder("FontPolicyCoverageTest").section("testScriptAwareFontMetricsNormalizerBranches");var r=new FontMetricsRequest("key",16,CjkText,"zh-Hans");var l=new ScriptAwareFontMetricsNormalizer().normalize(new FontMetricsNormalizationInput(r,new RawFontMetrics(18,5,0,RawTables,14,2)));TracedAssertions.assertEqualsFloat(14,l.ascent); }
}
