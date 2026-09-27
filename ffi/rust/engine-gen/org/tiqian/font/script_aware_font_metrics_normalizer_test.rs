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


#[test]
fn cjk_text_uses_font_declared_typo_box_instead_of_synthesized_square() {
    testlib::run("org.tiqian.font.ScriptAwareFontMetricsNormalizerTest.cjkTextUsesFontDeclaredTypoBoxInsteadOfSynthesizedSquare", "org.tiqian.font.ScriptAwareFontMetricsNormalizerTest.cjkTextUsesFontDeclaredTypoBoxInsteadOfSynthesizedSquare", || {
        TestTraceRecorder::new("ScriptAwareFontMetricsNormalizerTest").section(&"cjkTextUsesFontDeclaredTypoBoxInsteadOfSynthesizedSquare");
        let r = ScriptAwareFontMetricsNormalizerTestSupport::script_aware_font_metrics_normalizer_test_support_req(&"cjk-primary", FontRole::CjkText);
        let raw = StubFontMetricsResolver::new().resolve((r).clone());
        let l = ScriptAwareFontMetricsNormalizer::new().normalize(FontMetricsNormalizationInput::new((r).clone(), (raw).clone()));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14.08f64, *(raw.typo_ascent).as_ref().unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.92f64, *(raw.typo_descent).as_ref().unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14.08f64, l.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1.92f64, l.descent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"IdeographicLow", l.baseline_class.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"IdeographicEmBox", l.metric_box.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"RawTables", l.source.name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn cjk_text_falls_back_to_hhea_when_font_has_no_typo_metrics() {
    testlib::run("org.tiqian.font.ScriptAwareFontMetricsNormalizerTest.cjkTextFallsBackToHheaWhenFontHasNoTypoMetrics", "org.tiqian.font.ScriptAwareFontMetricsNormalizerTest.cjkTextFallsBackToHheaWhenFontHasNoTypoMetrics", || {
        TestTraceRecorder::new("ScriptAwareFontMetricsNormalizerTest").section(&"cjkTextFallsBackToHheaWhenFontHasNoTypoMetrics");
        let r = ScriptAwareFontMetricsNormalizerTestSupport::script_aware_font_metrics_normalizer_test_support_req(&"cjk-bad", FontRole::CjkText);
        let l = ScriptAwareFontMetricsNormalizer::new().normalize(FontMetricsNormalizationInput::new((r).clone(), RawFontMetrics::new(18.4f64, 4 as f64 as f64, Some(0 as f64), Some(FontMetricSource::RawTables), None, None)));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18.4f64, l.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, l.descent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Raw", l.policy.name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn latin_text_keeps_roman_raw_metrics() {
    testlib::run("org.tiqian.font.ScriptAwareFontMetricsNormalizerTest.latinTextKeepsRomanRawMetrics", "org.tiqian.font.ScriptAwareFontMetricsNormalizerTest.latinTextKeepsRomanRawMetrics", || {
        TestTraceRecorder::new("ScriptAwareFontMetricsNormalizerTest").section(&"latinTextKeepsRomanRawMetrics");
        let r = ScriptAwareFontMetricsNormalizerTestSupport::script_aware_font_metrics_normalizer_test_support_req(&"latin-primary", FontRole::LatinText);
        let raw = StubFontMetricsResolver::new().resolve((r).clone());
        let l = ScriptAwareFontMetricsNormalizer::new().normalize(FontMetricsNormalizationInput::new((r).clone(), (raw).clone()));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(raw.ascent, l.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(raw.descent, l.descent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Roman", l.baseline_class.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"RawFontBox", l.metric_box.name().to_string().as_str(), None).unwrap();
    });
}
