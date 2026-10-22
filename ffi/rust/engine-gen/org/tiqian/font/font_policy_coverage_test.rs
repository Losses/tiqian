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
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            FontPolicyCoverageTestTestPreferCjkForAmbiguousPunctuationResolverFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for FontPolicyCoverageTestTestFontRequestAndRolesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontPolicyCoverageTestTestFontRequestAndRolesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            FontPolicyCoverageTestTestFontRequestAndRolesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            FontPolicyCoverageTestTestFontRequestAndRolesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            FontPolicyCoverageTestTestCjkFontRoleClassifierLoneHighSurrogatesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            FontPolicyCoverageTestTestCjkFontRoleClassifierAllRangesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        let mut t = TestTraceRecorder::new(&(UStr::new(&[70,111,110,116,80,111,108,105,99,121,67,111,118,101,114,97,103,101,84,101,115,116])));
        t.section(UStr::new(&[116,101,115,116,67,106,107,70,111,110,116,82,111,108,101,67,108,97,115,115,105,102,105,101,114,65,108,108,82,97,110,103,101,115]));
        let classifier = CjkFontRoleClassifier::new();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(UStr::new(&[12549]), TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(UStr::new(&[12704]), TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(UStr::new(&[13312]), TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(UStr::new(&[19968]), TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(UStr::new(&[63744]), TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55360, 56320]).as_ustr(), TextRange::new(0u32, 2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55401, 57088]).as_ustr(), TextRange::new(0u32, 2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55405, 57152]).as_ustr(), TextRange::new(0u32, 2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55406, 56352]).as_ustr(), TextRange::new(0u32, 2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55424, 56320]).as_ustr(), TextRange::new(0u32, 2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Unknown, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55432, 56320]).as_ustr(), TextRange::new(0u32, 2u32).unwrap(), None)), None).unwrap();
        let punct = vec![
    UString::from("　").to_ustring(),
    UString::from("—").to_ustring(),
    UString::from("–").to_ustring(),
    UString::from("‼").to_ustring(),
    UString::from("⁇").to_ustring(),
    UString::from("…").to_ustring(),
    UString::from("‧").to_ustring(),
    UString::from("⋯").to_ustring(),
    UString::from("・").to_ustring(),
    UString::from("⸺").to_ustring(),
    UString::from("·").to_ustring(),
    UString::from("•").to_ustring(),
    UString::from("！").to_ustring(),
    UString::from("？").to_ustring(),
    UString::from("，").to_ustring(),
    UString::from("．").to_ustring(),
    UString::from("／").to_ustring(),
    UString::from("：").to_ustring(),
    UString::from("；").to_ustring(),
    UString::from("（").to_ustring(),
    UString::from("）").to_ustring(),
    UString::from("～").to_ustring(),
];
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((punct.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let ch = (punct[usize::try_from(i).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(ch.as_ustr(), TextRange::new(0u32, u_string::unit_count(&(ch))).unwrap(), None)), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Expected CjkPunctuation for ")); __s += ch.as_ustr(); __s }).as_str()))).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(classifier.classify(UStr::new(&[97,8217,98]), TextRange::new(1u32, 2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(classifier.classify(UStr::new(&[97,8221,98]), TextRange::new(1u32, 2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(UStr::new(&[8217,98]), TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(UStr::new(&[97,8217]), TextRange::new(1u32, 2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(UStr::new(&[20013,8217,25991]), TextRange::new(1u32, 2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55357, 56832, 8217, 98]).as_ustr(), TextRange::new(2u32, 3u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![65, 56320, 8217, 98]).as_ustr(), TextRange::new(2u32, 3u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![57344, 56320, 8217, 98]).as_ustr(), TextRange::new(2u32, 3u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![56320, 8217, 98]).as_ustr(), TextRange::new(1u32, 2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(UStr::new(&[57344,8217,98]), TextRange::new(1u32, 2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkPunctuation, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![97, 8217, 55357, 56832]).as_ustr(), TextRange::new(1u32, 2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Unknown, Some(classifier.classify(UStr::new(&[57344]), TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Unknown, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55300, 56320]).as_ustr(), TextRange::new(0u32, 2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(classifier.classify(UStr::new(&[65]), TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(classifier.classify(UStr::new(&[122]), TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(classifier.classify(UStr::new(&[48]), TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(classifier.classify(UStr::new(&[32]), TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(classifier.classify(UStr::new(&[43]), TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(classifier.classify(UStr::new(&[192]), TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::LatinText, Some(classifier.classify(UStr::new(&[336]), TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Emoji, Some(classifier.classify(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55357, 56832]).as_ustr(), TextRange::new(0u32, 2u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Symbol, Some(classifier.classify(UStr::new(&[8800]), TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Symbol, Some(classifier.classify(UStr::new(&[8364]), TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Symbol, Some(classifier.classify(UStr::new(&[728]), TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Symbol, Some(classifier.classify(UStr::new(&[169]), TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::Unknown, Some(classifier.classify(UStr::new(&[1]), TextRange::new(0u32, 1u32).unwrap(), None)), None).unwrap();
    });
}

#[test]
fn test_cjk_font_role_classifier_lone_high_surrogates() {
    testlib::record_not_applicable("org.tiqian.font.FontPolicyCoverageTest.testCjkFontRoleClassifierLoneHighSurrogates", "org.tiqian.font.FontPolicyCoverageTest.testCjkFontRoleClassifierLoneHighSurrogates");
}

#[test]
fn test_font_enums_and_models() {
    testlib::run("org.tiqian.font.FontPolicyCoverageTest.testFontEnumsAndModels", "org.tiqian.font.FontPolicyCoverageTest.testFontEnumsAndModels", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[70,111,110,116,80,111,108,105,99,121,67,111,118,101,114,97,103,101,84,101,115,116])));
        t.section(UStr::new(&[116,101,115,116,70,111,110,116,69,110,117,109,115,65,110,100,77,111,100,101,108,115]));
        let metrics_policies = FontMetricsPolicy::ALL;
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (4) {
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(UString::from(metrics_policies[usize::try_from(i).unwrap_or(0)].name()).as_ustr(), None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let baseline_policies = BaselinePolicy::ALL;
        i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (3) {
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(UString::from(baseline_policies[usize::try_from(i).unwrap_or(0)].name()).as_ustr(), None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let punctuation_policies = PunctuationFontPolicy::ALL;
        i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (4) {
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(UString::from(punctuation_policies[usize::try_from(i).unwrap_or(0)].name()).as_ustr(), None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let raw = RawFontMetrics::new(16 as f64 as f64, 4 as f64 as f64, Some(2 as f64), Some(FontMetricSource::RawTables), Some(14 as f64), Some(2 as f64));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, raw.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, raw.descent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, raw.leading, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14 as f64, *(raw.typo_ascent).as_ref().unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, *(raw.typo_descent).as_ref().unwrap(), None).unwrap();
        let raw_copy = RawFontMetrics::new(raw.ascent, raw.descent, Some(raw.leading), Some(raw.source), raw.typo_ascent, raw.typo_descent);
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", raw.to_string()).as_str()).as_ustr(), UString::from(format!("{}", raw_copy.to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(raw.to_string() == raw_copy.to_string(), None).unwrap();
        let layout = LayoutFontMetrics::new(14 as f64 as f64, 2 as f64 as f64, 0 as f64 as f64, FontMetricsPolicy::IdeographicBox, BaselinePolicy::Ideographic, Some(BaselineClass::IdeographicLow), Some(MetricBox::IdeographicEmBox), Some(FontMetricSource::RawTables), Some(UString::from("test")));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14 as f64, layout.ascent, None).unwrap();
        let layout_copy = LayoutFontMetrics::new(layout.ascent, layout.descent, layout.baseline_offset, layout.policy, layout.baseline_policy, Some(layout.baseline_class), Some(layout.metric_box), Some(layout.source), Some((layout.reason).to_ustring()));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", layout.to_string()).as_str()).as_ustr(), UString::from(format!("{}", layout_copy.to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(layout.to_string() == layout_copy.to_string(), None).unwrap();
    });
}

#[test]
fn test_font_metrics_request_and_resolvers() {
    testlib::run("org.tiqian.font.FontPolicyCoverageTest.testFontMetricsRequestAndResolvers", "org.tiqian.font.FontPolicyCoverageTest.testFontMetricsRequestAndResolvers", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[70,111,110,116,80,111,108,105,99,121,67,111,118,101,114,97,103,101,84,101,115,116])));
        t.section(UStr::new(&[116,101,115,116,70,111,110,116,77,101,116,114,105,99,115,82,101,113,117,101,115,116,65,110,100,82,101,115,111,108,118,101,114,115]));
        let request = FontMetricsRequest::new(&(UStr::new(&[107,101,121,49])), 16 as f64 as f64, FontRole::CjkText, &(UStr::new(&[122,104,45,72,97,110,115])), Some(vec![UString::from("FontA").to_ustring()]), Some(700), Some(true), Some(UString::from("测试")));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[107,101,121,49]), (request.font_key).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, request.font_size, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(request.role), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[122,104,45,72,97,110,115]), (request.locale).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("FontA").to_ustring()], &request.font_families, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(700, request.font_weight, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(request.italic, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[27979,35797]), (request.face_selection_text).to_ustring().as_ustr(), None).unwrap();
        let request_copy = FontMetricsRequest::new((request.font_key).to_ustring().as_ustr(), request.font_size, request.role, (request.locale).to_ustring().as_ustr(), Some(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_copy_strings(&request.font_families)), Some(request.font_weight), Some(request.italic), Some((request.face_selection_text).to_ustring()));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", request.to_string()).as_str()).as_ustr(), UString::from(format!("{}", request_copy.to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(request.to_string() == request_copy.to_string(), None).unwrap();
        let resolver = StubFontMetricsResolver::new();
        let cjk = resolver.resolve((request).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18.56f64, cjk.ascent, None).unwrap();
        let typo_ascent = cjk.typo_ascent;
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14.08f64, match &(typo_ascent) { None => 0 as f64, Some(__option1) => *__option1 }, None).unwrap();
        let punct = resolver.resolve(FontMetricsRequest::new((request.font_key).to_ustring().as_ustr(), request.font_size, FontRole::CjkPunctuation, (request.locale).to_ustring().as_ustr(), Some(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_copy_strings(&request.font_families)), Some(request.font_weight), Some(request.italic), Some((request.face_selection_text).to_ustring())));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18.56f64, punct.ascent, None).unwrap();
        let latin = resolver.resolve(FontMetricsRequest::new((request.font_key).to_ustring().as_ustr(), request.font_size, FontRole::LatinText, (request.locale).to_ustring().as_ustr(), Some(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_copy_strings(&request.font_families)), Some(request.font_weight), Some(request.italic), Some((request.face_selection_text).to_ustring())));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(12.8f64, latin.ascent, None).unwrap();
        let symbol = resolver.resolve(FontMetricsRequest::new((request.font_key).to_ustring().as_ustr(), request.font_size, FontRole::Symbol, (request.locale).to_ustring().as_ustr(), Some(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_copy_strings(&request.font_families)), Some(request.font_weight), Some(request.italic), Some((request.face_selection_text).to_ustring())));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14.4f64, symbol.ascent, None).unwrap();
        let emoji = resolver.resolve(FontMetricsRequest::new((request.font_key).to_ustring().as_ustr(), request.font_size, FontRole::Emoji, (request.locale).to_ustring().as_ustr(), Some(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_copy_strings(&request.font_families)), Some(request.font_weight), Some(request.italic), Some((request.face_selection_text).to_ustring())));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14.4f64, emoji.ascent, None).unwrap();
        let unknown = resolver.resolve(FontMetricsRequest::new((request.font_key).to_ustring().as_ustr(), request.font_size, FontRole::Unknown, (request.locale).to_ustring().as_ustr(), Some(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_copy_strings(&request.font_families)), Some(request.font_weight), Some(request.italic), Some((request.face_selection_text).to_ustring())));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14.4f64, unknown.ascent, None).unwrap();
    });
}

#[test]
fn test_font_request_and_roles() {
    testlib::run("org.tiqian.font.FontPolicyCoverageTest.testFontRequestAndRoles", "org.tiqian.font.FontPolicyCoverageTest.testFontRequestAndRoles", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[70,111,110,116,80,111,108,105,99,121,67,111,118,101,114,97,103,101,84,101,115,116])));
        t.section(UStr::new(&[116,101,115,116,70,111,110,116,82,101,113,117,101,115,116,65,110,100,82,111,108,101,115]));
        let request = FontRequest::new(vec![UString::from("Source Han Sans").to_ustring()], &(UStr::new(&[122,104,45,72,97,110,115])), FontRole::CjkText);
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("Source Han Sans").to_ustring()], &request.preferred_families, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[122,104,45,72,97,110,115]), (request.locale).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(request.role), None).unwrap();
        let request_copy = FontRequest::new((request.preferred_families).clone(), (request.locale).to_ustring().as_ustr(), request.role);
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", request.to_string()).as_str()).as_ustr(), UString::from(format!("{}", request_copy.to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(request.to_string() == request_copy.to_string(), None).unwrap();
        let roles = FontRole::ALL;
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (6) {
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(UString::from(roles[usize::try_from(i).unwrap_or(0)].name()).as_ustr(), None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(FontRole::LatinText.uses_latin_face(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(FontRole::CjkText.uses_latin_face(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(FontRole::CjkPunctuation.uses_latin_face(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(FontRole::Symbol.uses_latin_face(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(FontRole::Emoji.uses_latin_face(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(FontRole::Unknown.uses_latin_face(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(font_role_name_uses_latin_face(Some(UString::from("LatinText"))), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(font_role_name_uses_latin_face(Some(UString::from("CjkText"))), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(font_role_name_uses_latin_face(Some(UString::from("Unknown"))), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(font_role_name_uses_latin_face(None.clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(font_role_name_uses_latin_face(Some(UString::from("NotARole"))), None).unwrap();
        let candidate = FontCandidate::new(&(UStr::new(&[99,106,107,45,107,101,121])), &(UStr::new(&[83,111,117,114,99,101,32,72,97,110,32,83,97,110,115])), FontRole::CjkText);
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[99,106,107,45,107,101,121]), (candidate.key).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[83,111,117,114,99,101,32,72,97,110,32,83,97,110,115]), (candidate.family).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(candidate.role), None).unwrap();
        let candidate_copy = FontCandidate::new((candidate.key).to_ustring().as_ustr(), (candidate.family).to_ustring().as_ustr(), candidate.role);
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", candidate.to_string()).as_str()).as_ustr(), UString::from(format!("{}", candidate_copy.to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(candidate.to_string() == candidate_copy.to_string(), None).unwrap();
        let decision = FontDecision::new(TextRange::new(0u32, 1u32).unwrap(), (candidate).clone(), FontRole::CjkText, &(UStr::new(&[114,101,97,115,111,110])));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,48,44,32,101,110,100,61,49,41]), UString::from(format!("{}", (decision.range).clone().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", candidate.to_string()).as_str()).as_ustr(), UString::from(format!("{}", (decision.candidate).clone().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_font_role(FontRole::CjkText, Some(decision.role), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[114,101,97,115,111,110]), (decision.reason).to_ustring().as_ustr(), None).unwrap();
        let decision_copy = FontDecision::new((decision.range).clone(), (decision.candidate).clone(), decision.role, (decision.reason).to_ustring().as_ustr());
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", decision.to_string()).as_str()).as_ustr(), UString::from(format!("{}", decision_copy.to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(decision.to_string() == decision_copy.to_string(), None).unwrap();
        let context = FontRoleContext::new(Some(UString::from("zh-TW")), Some(UString::from("TW")));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[122,104,45,84,87]), (context.locale).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[84,87]), (context.region_hint).as_deref().unwrap_or(UStr::new(&[])), None).unwrap();
        let context_copy = FontRoleContext::new(Some((context.locale).to_ustring()), context.region_hint.clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", context.to_string()).as_str()).as_ustr(), UString::from(format!("{}", context_copy.to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(context.to_string() == context_copy.to_string(), None).unwrap();
    });
}

#[test]
fn test_prefer_cjk_for_ambiguous_punctuation_resolver() {
    testlib::run("org.tiqian.font.FontPolicyCoverageTest.testPreferCjkForAmbiguousPunctuationResolver", "org.tiqian.font.FontPolicyCoverageTest.testPreferCjkForAmbiguousPunctuationResolver", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[70,111,110,116,80,111,108,105,99,121,67,111,118,101,114,97,103,101,84,101,115,116])));
        t.section(UStr::new(&[116,101,115,116,80,114,101,102,101,114,67,106,107,70,111,114,65,109,98,105,103,117,111,117,115,80,117,110,99,116,117,97,116,105,111,110,82,101,115,111,108,118,101,114]));
        let resolver = PreferCjkForAmbiguousPunctuationResolver::new(Some(UString::from("cjk-key")), Some(UString::from("latin-key")), Some(UString::from("symbol-key")));
        let mut d = resolver.resolve(UStr::new(&[20013]), TextRange::new(0u32, 1u32).unwrap(), FontRequest::new(vec![UString::from("CustomCjk").to_ustring()], &(UStr::new(&[122,104,45,72,97,110,115])), FontRole::CjkText));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[99,106,107,45,107,101,121]), ((d.candidate).clone().key).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[67,117,115,116,111,109,67,106,107]), ((d.candidate).clone().family).to_ustring().as_ustr(), None).unwrap();
        d = resolver.resolve(UStr::new(&[20013]), TextRange::new(0u32, 1u32).unwrap(), FontRequest::new(vec![], &(UStr::new(&[122,104,45,72,97,110,115])), FontRole::CjkPunctuation));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[99,106,107,45,107,101,121]), ((d.candidate).clone().family).to_ustring().as_ustr(), None).unwrap();
        d = resolver.resolve(UStr::new(&[65]), TextRange::new(0u32, 1u32).unwrap(), FontRequest::new(vec![], &(UStr::new(&[101,110])), FontRole::LatinText));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[108,97,116,105,110,45,107,101,121]), ((d.candidate).clone().key).to_ustring().as_ustr(), None).unwrap();
        d = resolver.resolve(UStr::new(&[169]), TextRange::new(0u32, 1u32).unwrap(), FontRequest::new(vec![], &(UStr::new(&[101,110])), FontRole::Symbol));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[115,121,109,98,111,108,45,107,101,121]), ((d.candidate).clone().key).to_ustring().as_ustr(), None).unwrap();
        d = resolver.resolve(FontPolicyCoverageTestSupport::font_policy_coverage_test_support_surrogate_text(&vec![55357, 56832]).as_ustr(), TextRange::new(0u32, 2u32).unwrap(), FontRequest::new(vec![], &(UStr::new(&[101,110])), FontRole::Emoji));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[115,121,109,98,111,108,45,107,101,121]), ((d.candidate).clone().key).to_ustring().as_ustr(), None).unwrap();
        d = resolver.resolve(UStr::new(&[1]), TextRange::new(0u32, 1u32).unwrap(), FontRequest::new(vec![], &(UStr::new(&[101,110])), FontRole::Unknown));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[115,121,109,98,111,108,45,107,101,121]), ((d.candidate).clone().key).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn test_script_aware_font_metrics_normalizer_branches() {
    testlib::run("org.tiqian.font.FontPolicyCoverageTest.testScriptAwareFontMetricsNormalizerBranches", "org.tiqian.font.FontPolicyCoverageTest.testScriptAwareFontMetricsNormalizerBranches", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[70,111,110,116,80,111,108,105,99,121,67,111,118,101,114,97,103,101,84,101,115,116])));
        t.section(UStr::new(&[116,101,115,116,83,99,114,105,112,116,65,119,97,114,101,70,111,110,116,77,101,116,114,105,99,115,78,111,114,109,97,108,105,122,101,114,66,114,97,110,99,104,101,115]));
        let normalizer = ScriptAwareFontMetricsNormalizer::new();
        let base = FontMetricsRequest::new(&(UStr::new(&[107,101,121])), 16 as f64 as f64, FontRole::CjkText, &(UStr::new(&[122,104,45,72,97,110,115])), Some(vec![]), Some(400), Some(false), Some(UString::from("")));
        let input_with_typo = FontMetricsNormalizationInput::new((base).clone(), RawFontMetrics::new(18 as f64 as f64, 5 as f64 as f64, Some(0 as f64), Some(FontMetricSource::RawTables), Some(14 as f64), Some(2 as f64)));
        let typo = normalizer.normalize((input_with_typo).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14 as f64, typo.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, typo.descent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[73,100,101,111,103,114,97,112,104,105,99,66,111,120]), UString::from(typo.policy.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&((typo.reason).to_ustring()), UString::from("font-typo-box").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let input_partial_typo1 = FontMetricsNormalizationInput::new((base).clone(), RawFontMetrics::new(18 as f64 as f64, 5 as f64 as f64, Some(0 as f64), Some(FontMetricSource::RawTables), Some(14 as f64), None));
        let partial1 = normalizer.normalize((input_partial_typo1).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14 as f64, partial1.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(5 as f64, partial1.descent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,97,119]), UString::from(partial1.policy.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&((partial1.reason).to_ustring()), UString::from("hhea-fallback-no-os2").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let input_partial_typo2 = FontMetricsNormalizationInput::new((base).clone(), RawFontMetrics::new(18 as f64 as f64, 5 as f64 as f64, Some(0 as f64), Some(FontMetricSource::RawTables), None, Some(2 as f64)));
        let partial2 = normalizer.normalize((input_partial_typo2).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18 as f64, partial2.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2 as f64, partial2.descent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,97,119]), UString::from(partial2.policy.name()).as_ustr(), None).unwrap();
        let input_no_typo = FontMetricsNormalizationInput::new((base).clone(), RawFontMetrics::new(18 as f64 as f64, 5 as f64 as f64, Some(0 as f64), Some(FontMetricSource::RawTables), None, None));
        let no_typo = normalizer.normalize((input_no_typo).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18 as f64, no_typo.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(5 as f64, no_typo.descent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,97,119]), UString::from(no_typo.policy.name()).as_ustr(), None).unwrap();
        let input_latin = FontMetricsNormalizationInput::new(FontMetricsRequest::new(&(UStr::new(&[107,101,121])), 16 as f64 as f64, FontRole::LatinText, &(UStr::new(&[122,104,45,72,97,110,115])), Some(vec![]), Some(400), Some(false), Some(UString::from(""))), RawFontMetrics::new(13 as f64 as f64, 3 as f64 as f64, Some(0 as f64), Some(FontMetricSource::RawTables), None, None));
        let latin = normalizer.normalize((input_latin).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(13 as f64, latin.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(3 as f64, latin.descent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,97,119]), UString::from(latin.policy.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[65,108,112,104,97,98,101,116,105,99]), UString::from(latin.baseline_policy.name()).as_ustr(), None).unwrap();
        let input_symbol = FontMetricsNormalizationInput::new(FontMetricsRequest::new(&(UStr::new(&[107,101,121])), 16 as f64 as f64, FontRole::Symbol, &(UStr::new(&[122,104,45,72,97,110,115])), Some(vec![]), Some(400), Some(false), Some(UString::from(""))), RawFontMetrics::new(14 as f64 as f64, 4 as f64 as f64, Some(0 as f64), Some(FontMetricSource::RawTables), None, None));
        let symbol = normalizer.normalize((input_symbol).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14 as f64, symbol.ascent, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[82,97,119]), UString::from(symbol.policy.name()).as_ustr(), None).unwrap();
        let input_copy = FontMetricsNormalizationInput::new((input_with_typo.request).clone(), (input_with_typo.raw_metrics).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", input_with_typo.to_string()).as_str()).as_ustr(), UString::from(format!("{}", input_copy.to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(input_with_typo.to_string() == input_copy.to_string(), None).unwrap();
    });
}
