#![cfg(test)]

use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::font::baseline_class::BaselineClass;
use crate::org::tiqian::font::baseline_policy::BaselinePolicy;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metric_source::FontMetricSource;
use crate::org::tiqian::font::font_metrics::FontMetricsNormalizationInput;
use crate::org::tiqian::font::font_metrics::FontMetricsRequest;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::font::font_metrics_policy::FontMetricsPolicy;
use crate::org::tiqian::font::font_policy::FontCandidate;
use crate::org::tiqian::font::font_policy::FontDecision;
use crate::org::tiqian::font::font_policy::FontRequest;
use crate::org::tiqian::font::font_policy::font_role_name_uses_latin_face;
use crate::org::tiqian::font::font_policy_coverage_test_support::FontPolicyCoverageTestSupport;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::font_role_context::FontRoleContext;
use crate::org::tiqian::font::layout_font_metrics::LayoutFontMetrics;
use crate::org::tiqian::font::metric_box::MetricBox;
use crate::org::tiqian::font::prefer_cjk_for_ambiguous_punctuation_resolver::PreferCjkForAmbiguousPunctuationResolver;
use crate::org::tiqian::font::punctuation_font_policy::PunctuationFontPolicy;
use crate::org::tiqian::font::raw_font_metrics::RawFontMetrics;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;


#[derive(Debug, Clone, PartialEq)]
pub enum FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault) -> Self {
        match value {
            FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault) -> Self {
        match value {
            FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault) -> Self {
        match value {
            FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontPolicyCoverageTestTestFontRequestAndRolesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<FontPolicyCoverageTestTestFontRequestAndRolesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: FontPolicyCoverageTestTestFontRequestAndRolesFault) -> Self {
        match value {
            FontPolicyCoverageTestTestFontRequestAndRolesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontPolicyCoverageTestTestFontRequestAndRolesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: FontPolicyCoverageTestTestFontRequestAndRolesFault) -> Self {
        match value {
            FontPolicyCoverageTestTestFontRequestAndRolesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontPolicyCoverageTestTestFontRequestAndRolesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: FontPolicyCoverageTestTestFontRequestAndRolesFault) -> Self {
        match value {
            FontPolicyCoverageTestTestFontRequestAndRolesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for FontPolicyCoverageTestTestFontRequestAndRolesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        FontPolicyCoverageTestTestFontRequestAndRolesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for FontPolicyCoverageTestTestFontRequestAndRolesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        FontPolicyCoverageTestTestFontRequestAndRolesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for FontPolicyCoverageTestTestFontRequestAndRolesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        FontPolicyCoverageTestTestFontRequestAndRolesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault) -> Self {
        match value {
            FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault) -> Self {
        match value {
            FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault) -> Self {
        match value {
            FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault) -> Self {
        match value {
            FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault) -> Self {
        match value {
            FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault) -> Self {
        match value {
            FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn test_cjk_font_role_classifier_all_ranges() {
    testlib::run("org.tiqian.font.FontPolicyCoverageTest.testCjkFontRoleClassifierAllRanges", "org.tiqian.font.FontPolicyCoverageTest.testCjkFontRoleClassifierAllRanges", || {
        let mut t = TestTraceRecorder::new("FontPolicyCoverageTest");
        t.section(&"testCjkFontRoleClassifierAllRanges");
        let classifier = CjkFontRoleClassifier::new();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(&"ㄅ", TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(&"ㆠ", TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(&"㐀", TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(&"一", TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(&"豈", TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55360, 56320]).as_str(), TextRange::new(0u32, 2u32).unwrap(), None)),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55401, 57088]).as_str(), TextRange::new(0u32, 2u32).unwrap(), None)),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55405, 57152]).as_str(), TextRange::new(0u32, 2u32).unwrap(), None)),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55406, 56352]).as_str(), TextRange::new(0u32, 2u32).unwrap(), None)),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55424, 56320]).as_str(), TextRange::new(0u32, 2u32).unwrap(), None)),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Unknown, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55432, 56320]).as_str(), TextRange::new(0u32, 2u32).unwrap(), None)),
None).unwrap();
        let punct = vec![
    "　".to_string(),
    "—".to_string(),
    "–".to_string(),
    "‼".to_string(),
    "⁇".to_string(),
    "…".to_string(),
    "‧".to_string(),
    "⋯".to_string(),
    "・".to_string(),
    "⸺".to_string(),
    "·".to_string(),
    "•".to_string(),
    "！".to_string(),
    "？".to_string(),
    "，".to_string(),
    "．".to_string(),
    "／".to_string(),
    "：".to_string(),
    "；".to_string(),
    "（".to_string(),
    "）".to_string(),
    "～".to_string(),
];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((punct.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let ch = (punct[usize::try_from(i).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(ch.as_str(), TextRange::new(0u32, u_string::unit_count(&(ch))).unwrap(), None)), Some((format!("{}{}",
            "Expected CjkPunctuation for ",
            ch
        )).to_string())).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(classifier.classify(&"a’b", TextRange::new(1u32, 2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(classifier.classify(&"a”b", TextRange::new(1u32, 2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(&"’b", TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(&"a’", TextRange::new(1u32, 2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(&"中’文", TextRange::new(1u32, 2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55357, 56832, 8217, 98]).as_str(), TextRange::new(2u32,
3u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![65, 56320, 8217, 98]).as_str(), TextRange::new(2u32, 3u32).unwrap(),
None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![57344, 56320, 8217, 98]).as_str(), TextRange::new(2u32,
3u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![56320, 8217, 98]).as_str(), TextRange::new(1u32, 2u32).unwrap(),
None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(&"’b", TextRange::new(1u32, 2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![97, 8217, 55357, 56832]).as_str(), TextRange::new(1u32,
2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Unknown, Some(classifier.classify(&"", TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Unknown, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55300, 56320]).as_str(), TextRange::new(0u32, 2u32).unwrap(), None)),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(classifier.classify(&"A", TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(classifier.classify(&"z", TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(classifier.classify(&"0", TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(classifier.classify(&" ", TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(classifier.classify(&"+", TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(classifier.classify(&"À", TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(classifier.classify(&"Ő", TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Emoji, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55357, 56832]).as_str(), TextRange::new(0u32, 2u32).unwrap(), None)),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Symbol, Some(classifier.classify(&"≠", TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Symbol, Some(classifier.classify(&"€", TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Symbol, Some(classifier.classify(&"˘", TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Symbol, Some(classifier.classify(&"©", TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Unknown, Some(classifier.classify(&"", TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
    });
}

#[test]
fn test_cjk_font_role_classifier_lone_high_surrogates() {
    testlib::run("org.tiqian.font.FontPolicyCoverageTest.testCjkFontRoleClassifierLoneHighSurrogates", "org.tiqian.font.FontPolicyCoverageTest.testCjkFontRoleClassifierLoneHighSurrogates", || {
        let mut t = TestTraceRecorder::new("FontPolicyCoverageTest");
        t.section(&"testCjkFontRoleClassifierLoneHighSurrogates");
        let classifier = CjkFontRoleClassifier::new();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Unknown, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55296]).as_str(), TextRange::new(0u32, 1u32).unwrap(), None)),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Unknown, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55296, 65]).as_str(), TextRange::new(0u32, 2u32).unwrap(), None)),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Unknown, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55296, 57344]).as_str(), TextRange::new(0u32, 2u32).unwrap(), None)),
None).unwrap();
    });
}

#[test]
fn test_font_enums_and_models() {
    testlib::run("org.tiqian.font.FontPolicyCoverageTest.testFontEnumsAndModels", "org.tiqian.font.FontPolicyCoverageTest.testFontEnumsAndModels", || {
        let mut t = TestTraceRecorder::new("FontPolicyCoverageTest");
        t.section(&"testFontEnumsAndModels");
        let metrics_policies = FontMetricsPolicy::ALL;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (4) {
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(metrics_policies[usize::try_from(i).unwrap_or(0)].name().to_string().as_str(), None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let baseline_policies = BaselinePolicy::ALL;
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (3) {
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(baseline_policies[usize::try_from(i).unwrap_or(0)].name().to_string().as_str(), None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let punctuation_policies = PunctuationFontPolicy::ALL;
        i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (4) {
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(punctuation_policies[usize::try_from(i).unwrap_or(0)].name().to_string().as_str(), None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let raw = RawFontMetrics::new(16 as f64 as f64, 4 as f64 as f64, Some(2 as f64), Some(FontMetricSource::RawTables), Some(14 as f64), Some(2 as f64));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, raw.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, raw.descent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, raw.leading, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14 as f64, *(raw.typo_ascent).as_ref().unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, *(raw.typo_descent).as_ref().unwrap(), None).unwrap();
        let raw_copy = RawFontMetrics::new(raw.ascent, raw.descent, Some(raw.leading), Some(raw.source), raw.typo_ascent, raw.typo_descent);
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(raw.to_string().as_str(), raw_copy.to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(raw.to_string() == raw_copy.to_string(), None).unwrap();
        let layout = LayoutFontMetrics::new(14 as f64 as f64, 2 as f64 as f64, 0 as f64 as f64, FontMetricsPolicy::IdeographicBox, BaselinePolicy::Ideographic, Some(BaselineClass::IdeographicLow), Some(MetricBox::IdeographicEmBox), Some(FontMetricSource::RawTables),
Some("test".to_string()));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14 as f64, layout.ascent, None).unwrap();
        let layout_copy = LayoutFontMetrics::new(layout.ascent, layout.descent, layout.baseline_offset, layout.policy, layout.baseline_policy, Some(layout.baseline_class), Some(layout.metric_box), Some(layout.source), Some((layout.reason).to_string()));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(layout.to_string().as_str(), layout_copy.to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(layout.to_string() == layout_copy.to_string(), None).unwrap();
    });
}

#[test]
fn test_font_metrics_request_and_resolvers() {
    testlib::run("org.tiqian.font.FontPolicyCoverageTest.testFontMetricsRequestAndResolvers", "org.tiqian.font.FontPolicyCoverageTest.testFontMetricsRequestAndResolvers", || {
        let mut t = TestTraceRecorder::new("FontPolicyCoverageTest");
        t.section(&"testFontMetricsRequestAndResolvers");
        let request = FontMetricsRequest::new("key1", 16 as f64 as f64, FontRole::CjkText, "zh-Hans", Some(vec!["FontA".to_string()]), Some(700), Some(true), Some("测试".to_string()));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"key1", (request.font_key).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, request.font_size, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(request.role), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"zh-Hans", (request.locale).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["FontA".to_string()], &request.font_families, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(700, request.font_weight, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(request.italic, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"测试", (request.face_selection_text).to_string().as_str(), None).unwrap();
        let request_copy = FontMetricsRequest::new((request.font_key).to_string().as_str(), request.font_size, request.role, (request.locale).to_string().as_str(), Some(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_copy_strings(&request.font_families)),
Some(request.font_weight), Some(request.italic), Some((request.face_selection_text).to_string()));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(request.to_string().as_str(), request_copy.to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(request.to_string() == request_copy.to_string(), None).unwrap();
        let resolver = StubFontMetricsResolver::new();
        let cjk = resolver.resolve((request).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18.56f64, cjk.ascent, None).unwrap();
        let typo_ascent = cjk.typo_ascent;
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14.08f64, match &(typo_ascent) { None => 0 as f64, Some(__option1) => *__option1 }, None).unwrap();
        let punct = resolver.resolve(FontMetricsRequest::new((request.font_key).to_string().as_str(), request.font_size, FontRole::CjkPunctuation, (request.locale).to_string().as_str(),
Some(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_copy_strings(&request.font_families)), Some(request.font_weight), Some(request.italic), Some((request.face_selection_text).to_string())));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18.56f64, punct.ascent, None).unwrap();
        let latin = resolver.resolve(FontMetricsRequest::new((request.font_key).to_string().as_str(), request.font_size, FontRole::LatinText, (request.locale).to_string().as_str(),
Some(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_copy_strings(&request.font_families)), Some(request.font_weight), Some(request.italic), Some((request.face_selection_text).to_string())));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(12.8f64, latin.ascent, None).unwrap();
        let symbol = resolver.resolve(FontMetricsRequest::new((request.font_key).to_string().as_str(), request.font_size, FontRole::Symbol, (request.locale).to_string().as_str(),
Some(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_copy_strings(&request.font_families)), Some(request.font_weight), Some(request.italic), Some((request.face_selection_text).to_string())));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14.4f64, symbol.ascent, None).unwrap();
        let emoji = resolver.resolve(FontMetricsRequest::new((request.font_key).to_string().as_str(), request.font_size, FontRole::Emoji, (request.locale).to_string().as_str(),
Some(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_copy_strings(&request.font_families)), Some(request.font_weight), Some(request.italic), Some((request.face_selection_text).to_string())));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14.4f64, emoji.ascent, None).unwrap();
        let unknown = resolver.resolve(FontMetricsRequest::new((request.font_key).to_string().as_str(), request.font_size, FontRole::Unknown, (request.locale).to_string().as_str(),
Some(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_copy_strings(&request.font_families)), Some(request.font_weight), Some(request.italic), Some((request.face_selection_text).to_string())));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14.4f64, unknown.ascent, None).unwrap();
    });
}

#[test]
fn test_font_request_and_roles() {
    testlib::run("org.tiqian.font.FontPolicyCoverageTest.testFontRequestAndRoles", "org.tiqian.font.FontPolicyCoverageTest.testFontRequestAndRoles", || {
        let mut t = TestTraceRecorder::new("FontPolicyCoverageTest");
        t.section(&"testFontRequestAndRoles");
        let request = FontRequest::new(vec!["Source Han Sans".to_string()], "zh-Hans", FontRole::CjkText);
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["Source Han Sans".to_string()], &request.preferred_families, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"zh-Hans", (request.locale).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(request.role), None).unwrap();
        let request_copy = FontRequest::new((request.preferred_families).clone(), (request.locale).to_string().as_str(), request.role);
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(request.to_string().as_str(), request_copy.to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(request.to_string() == request_copy.to_string(), None).unwrap();
        let roles = FontRole::ALL;
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (6) {
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(roles[usize::try_from(i).unwrap_or(0)].name().to_string().as_str(), None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(FontRole::LatinText.uses_latin_face(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(FontRole::CjkText.uses_latin_face(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(FontRole::CjkPunctuation.uses_latin_face(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(FontRole::Symbol.uses_latin_face(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(FontRole::Emoji.uses_latin_face(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(FontRole::Unknown.uses_latin_face(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(font_role_name_uses_latin_face(Some("LatinText".to_string())), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(font_role_name_uses_latin_face(Some("CjkText".to_string())), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(font_role_name_uses_latin_face(Some("Unknown".to_string())), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(font_role_name_uses_latin_face(None.clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(font_role_name_uses_latin_face(Some("NotARole".to_string())), None).unwrap();
        let candidate = FontCandidate::new("cjk-key", "Source Han Sans", FontRole::CjkText);
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"cjk-key", (candidate.key).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"Source Han Sans", (candidate.family).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(candidate.role), None).unwrap();
        let candidate_copy = FontCandidate::new((candidate.key).to_string().as_str(), (candidate.family).to_string().as_str(), candidate.role);
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(candidate.to_string().as_str(), candidate_copy.to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(candidate.to_string() == candidate_copy.to_string(), None).unwrap();
        let decision = FontDecision::new(TextRange::new(0u32, 1u32).unwrap(), (candidate).clone(), FontRole::CjkText, "reason");
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"TextRange(start=0, end=1)", (decision.range).clone().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(candidate.to_string().as_str(), (decision.candidate).clone().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(decision.role), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"reason", (decision.reason).to_string().as_str(), None).unwrap();
        let decision_copy = FontDecision::new((decision.range).clone(), (decision.candidate).clone(), decision.role, (decision.reason).to_string().as_str());
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(decision.to_string().as_str(), decision_copy.to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(decision.to_string() == decision_copy.to_string(), None).unwrap();
        let context = FontRoleContext::new(Some("zh-TW".to_string()), Some("TW".to_string()));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"zh-TW", (context.locale).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"TW", (context.region_hint).as_deref().unwrap_or(""), None).unwrap();
        let context_copy = FontRoleContext::new(Some((context.locale).to_string()), context.region_hint.clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(context.to_string().as_str(), context_copy.to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(context.to_string() == context_copy.to_string(), None).unwrap();
    });
}

#[test]
fn test_prefer_cjk_for_ambiguous_punctuation_resolver() {
    testlib::run("org.tiqian.font.FontPolicyCoverageTest.testPreferCjkForAmbiguousPunctuationResolver", "org.tiqian.font.FontPolicyCoverageTest.testPreferCjkForAmbiguousPunctuationResolver", || {
        let mut t = TestTraceRecorder::new("FontPolicyCoverageTest");
        t.section(&"testPreferCjkForAmbiguousPunctuationResolver");
        let resolver = PreferCjkForAmbiguousPunctuationResolver::new(Some("cjk-key".to_string()), Some("latin-key".to_string()), Some("symbol-key".to_string()));
        let mut d = resolver.resolve(&"中", TextRange::new(0u32, 1u32).unwrap(), FontRequest::new(vec!["CustomCjk".to_string()], "zh-Hans", FontRole::CjkText));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"cjk-key", ((d.candidate).clone().key).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"CustomCjk", ((d.candidate).clone().family).to_string().as_str(), None).unwrap();
        d = resolver.resolve(&"中", TextRange::new(0u32, 1u32).unwrap(), FontRequest::new(vec![], "zh-Hans", FontRole::CjkPunctuation));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"cjk-key", ((d.candidate).clone().family).to_string().as_str(), None).unwrap();
        d = resolver.resolve(&"A", TextRange::new(0u32, 1u32).unwrap(), FontRequest::new(vec![], "en", FontRole::LatinText));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"latin-key", ((d.candidate).clone().key).to_string().as_str(), None).unwrap();
        d = resolver.resolve(&"©", TextRange::new(0u32, 1u32).unwrap(), FontRequest::new(vec![], "en", FontRole::Symbol));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"symbol-key", ((d.candidate).clone().key).to_string().as_str(), None).unwrap();
        d = resolver.resolve(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55357, 56832]).as_str(), TextRange::new(0u32, 2u32).unwrap(), FontRequest::new(vec![], "en", FontRole::Emoji));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"symbol-key", ((d.candidate).clone().key).to_string().as_str(), None).unwrap();
        d = resolver.resolve(&"", TextRange::new(0u32, 1u32).unwrap(), FontRequest::new(vec![], "en", FontRole::Unknown));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"symbol-key", ((d.candidate).clone().key).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn test_script_aware_font_metrics_normalizer_branches() {
    testlib::run("org.tiqian.font.FontPolicyCoverageTest.testScriptAwareFontMetricsNormalizerBranches", "org.tiqian.font.FontPolicyCoverageTest.testScriptAwareFontMetricsNormalizerBranches", || {
        let mut t = TestTraceRecorder::new("FontPolicyCoverageTest");
        t.section(&"testScriptAwareFontMetricsNormalizerBranches");
        let normalizer = ScriptAwareFontMetricsNormalizer::new();
        let base = FontMetricsRequest::new("key", 16 as f64 as f64, FontRole::CjkText, "zh-Hans", Some(vec![]), Some(400), Some(false), Some("".to_string()));
        let input_with_typo = FontMetricsNormalizationInput::new((base).clone(), RawFontMetrics::new(18 as f64 as f64, 5 as f64 as f64, Some(0 as f64), Some(FontMetricSource::RawTables), Some(14 as f64), Some(2 as f64)));
        let typo = normalizer.normalize((input_with_typo).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14 as f64, typo.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, typo.descent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"IdeographicBox", typo.policy.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&(typo.reason).to_string(), "font-typo-box", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
        let input_partial_typo1 = FontMetricsNormalizationInput::new((base).clone(), RawFontMetrics::new(18 as f64 as f64, 5 as f64 as f64, Some(0 as f64), Some(FontMetricSource::RawTables), Some(14 as f64), None));
        let partial1 = normalizer.normalize((input_partial_typo1).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14 as f64, partial1.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(5 as f64, partial1.descent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Raw", partial1.policy.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&(partial1.reason).to_string(), "hhea-fallback-no-os2", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
        let input_partial_typo2 = FontMetricsNormalizationInput::new((base).clone(), RawFontMetrics::new(18 as f64 as f64, 5 as f64 as f64, Some(0 as f64), Some(FontMetricSource::RawTables), None, Some(2 as f64)));
        let partial2 = normalizer.normalize((input_partial_typo2).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18 as f64, partial2.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, partial2.descent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Raw", partial2.policy.name().to_string().as_str(), None).unwrap();
        let input_no_typo = FontMetricsNormalizationInput::new((base).clone(), RawFontMetrics::new(18 as f64 as f64, 5 as f64 as f64, Some(0 as f64), Some(FontMetricSource::RawTables), None, None));
        let no_typo = normalizer.normalize((input_no_typo).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18 as f64, no_typo.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(5 as f64, no_typo.descent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Raw", no_typo.policy.name().to_string().as_str(), None).unwrap();
        let input_latin = FontMetricsNormalizationInput::new(FontMetricsRequest::new("key", 16 as f64 as f64, FontRole::LatinText, "zh-Hans", Some(vec![]), Some(400), Some(false), Some("".to_string())), RawFontMetrics::new(13 as f64 as f64, 3 as f64 as f64, Some(0 as f64),
Some(FontMetricSource::RawTables), None, None));
        let latin = normalizer.normalize((input_latin).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(13 as f64, latin.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(3 as f64, latin.descent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Raw", latin.policy.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Alphabetic", latin.baseline_policy.name().to_string().as_str(), None).unwrap();
        let input_symbol = FontMetricsNormalizationInput::new(FontMetricsRequest::new("key", 16 as f64 as f64, FontRole::Symbol, "zh-Hans", Some(vec![]), Some(400), Some(false), Some("".to_string())), RawFontMetrics::new(14 as f64 as f64, 4 as f64 as f64, Some(0 as f64),
Some(FontMetricSource::RawTables), None, None));
        let symbol = normalizer.normalize((input_symbol).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14 as f64, symbol.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Raw", symbol.policy.name().to_string().as_str(), None).unwrap();
        let input_copy = FontMetricsNormalizationInput::new((input_with_typo.request).clone(), (input_with_typo.raw_metrics).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(input_with_typo.to_string().as_str(), input_copy.to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(input_with_typo.to_string() == input_copy.to_string(), None).unwrap();
    });
}
