#![cfg(test)]

use crate::org::tiqian::font::font_metric_source::FontMetricSource;
use crate::org::tiqian::font::font_metrics::FontMetricsNormalizationInput;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::raw_font_metrics::RawFontMetrics;
use crate::org::tiqian::font::script_aware_font_metrics_normalizer_test_support::ScriptAwareFontMetricsNormalizerTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[test]
fn cjk_text_uses_font_declared_typo_box_instead_of_synthesized_square() {
    testlib::run("org.tiqian.font.ScriptAwareFontMetricsNormalizerTest.cjkTextUsesFontDeclaredTypoBoxInsteadOfSynthesizedSquare", "org.tiqian.font.ScriptAwareFontMetricsNormalizerTest.cjkTextUsesFontDeclaredTypoBoxInsteadOfSynthesizedSquare", || {
        TestTraceRecorder::new(&(UStr::new(&[83,99,114,105,112,116,65,119,97,114,101,70,111,110,116,77,101,116,114,105,99,115,78,111,114,109,97,108,105,122,101,114,84,101,115,116]))).section(UStr::new(&[99,106,107,84,101,120,116,85,115,101,115,70,111,110,116,68,101,99,108,97,114,101,100,84,121,112,111,66,111,120,73,110,115,116,101,97,100,79,102,83,121,110,116,104,101,115,105,122,101,100,83,113,117,97,114,101]));
        let r = ScriptAwareFontMetricsNormalizerTestSupport::script_aware_font_metrics_normalizer_test_support_req(UStr::new(&[99,106,107,45,112,114,105,109,97,114,121]), FontRole::CjkText);
        let raw = StubFontMetricsResolver::new().resolve((r).clone());
        let l = ScriptAwareFontMetricsNormalizer::new().normalize(FontMetricsNormalizationInput::new((r).clone(), (raw).clone()));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14.08f64, *(raw.typo_ascent).as_ref().unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.92f64, *(raw.typo_descent).as_ref().unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14.08f64, l.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.92f64, l.descent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[73,100,101,111,103,114,97,112,104,105,99,76,111,119]), UString::from(l.baseline_class.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[73,100,101,111,103,114,97,112,104,105,99,69,109,66,111,120]), UString::from(l.metric_box.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,97,119,84,97,98,108,101,115]), UString::from(l.source.name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn cjk_text_falls_back_to_hhea_when_font_has_no_typo_metrics() {
    testlib::run("org.tiqian.font.ScriptAwareFontMetricsNormalizerTest.cjkTextFallsBackToHheaWhenFontHasNoTypoMetrics", "org.tiqian.font.ScriptAwareFontMetricsNormalizerTest.cjkTextFallsBackToHheaWhenFontHasNoTypoMetrics", || {
        TestTraceRecorder::new(&(UStr::new(&[83,99,114,105,112,116,65,119,97,114,101,70,111,110,116,77,101,116,114,105,99,115,78,111,114,109,97,108,105,122,101,114,84,101,115,116]))).section(UStr::new(&[99,106,107,84,101,120,116,70,97,108,108,115,66,97,99,107,84,111,72,104,101,97,87,104,101,110,70,111,110,116,72,97,115,78,111,84,121,112,111,77,101,116,114,105,99,115]));
        let r = ScriptAwareFontMetricsNormalizerTestSupport::script_aware_font_metrics_normalizer_test_support_req(UStr::new(&[99,106,107,45,98,97,100]), FontRole::CjkText);
        let l = ScriptAwareFontMetricsNormalizer::new().normalize(FontMetricsNormalizationInput::new((r).clone(), RawFontMetrics::new(18.4f64, 4 as f64 as f64, Some(0 as f64), Some(FontMetricSource::RawTables), None, None)));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18.4f64, l.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, l.descent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,97,119]), UString::from(l.policy.name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn latin_text_keeps_roman_raw_metrics() {
    testlib::run("org.tiqian.font.ScriptAwareFontMetricsNormalizerTest.latinTextKeepsRomanRawMetrics", "org.tiqian.font.ScriptAwareFontMetricsNormalizerTest.latinTextKeepsRomanRawMetrics", || {
        TestTraceRecorder::new(&(UStr::new(&[83,99,114,105,112,116,65,119,97,114,101,70,111,110,116,77,101,116,114,105,99,115,78,111,114,109,97,108,105,122,101,114,84,101,115,116]))).section(UStr::new(&[108,97,116,105,110,84,101,120,116,75,101,101,112,115,82,111,109,97,110,82,97,119,77,101,116,114,105,99,115]));
        let r = ScriptAwareFontMetricsNormalizerTestSupport::script_aware_font_metrics_normalizer_test_support_req(UStr::new(&[108,97,116,105,110,45,112,114,105,109,97,114,121]), FontRole::LatinText);
        let raw = StubFontMetricsResolver::new().resolve((r).clone());
        let l = ScriptAwareFontMetricsNormalizer::new().normalize(FontMetricsNormalizationInput::new((r).clone(), (raw).clone()));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(raw.ascent, l.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(raw.descent, l.descent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,111,109,97,110]), UString::from(l.baseline_class.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,97,119,70,111,110,116,66,111,120]), UString::from(l.metric_box.name()).as_ustr(), None).unwrap();
    });
}
