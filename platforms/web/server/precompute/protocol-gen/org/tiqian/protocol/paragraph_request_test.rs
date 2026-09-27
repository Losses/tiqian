#![cfg(test)]

use crate::org::tiqian::protocol::decoration_input::DecorationInput;
use crate::org::tiqian::protocol::inline_box_input::InlineBoxInput;
use crate::org::tiqian::protocol::inline_object_input::InlineObjectInput;
use crate::org::tiqian::protocol::line_break_span_input::LineBreakSpanInput;
use crate::org::tiqian::protocol::paragraph_request_checks::ParagraphRequestChecks;
use crate::org::tiqian::protocol::paragraph_request_test_support::ParagraphRequestTestSupport;
use crate::org::tiqian::protocol::text_span_input::TextSpanInput;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;


#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NamedErrorFault(crate::org::tiqian::protocol::paragraph_request_exception::NamedError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault) -> Self {
        match value {
            ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault> for crate::org::tiqian::protocol::paragraph_request_exception::NamedError {
    fn from(value: ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault) -> Self {
        match value {
            ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault::NamedErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault) -> Self {
        match value {
            ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::protocol::paragraph_request_exception::NamedError> for ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault {
    fn from(value: crate::org::tiqian::protocol::paragraph_request_exception::NamedError) -> Self {
        ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault::NamedErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphRequestTestValidRequestPassesFault {
    NamedErrorFault(crate::org::tiqian::protocol::paragraph_request_exception::NamedError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphRequestTestValidRequestPassesFault> for crate::org::tiqian::protocol::paragraph_request_exception::NamedError {
    fn from(value: ParagraphRequestTestValidRequestPassesFault) -> Self {
        match value {
            ParagraphRequestTestValidRequestPassesFault::NamedErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphRequestTestValidRequestPassesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphRequestTestValidRequestPassesFault) -> Self {
        match value {
            ParagraphRequestTestValidRequestPassesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphRequestTestValidRequestPassesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphRequestTestValidRequestPassesFault) -> Self {
        match value {
            ParagraphRequestTestValidRequestPassesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::protocol::paragraph_request_exception::NamedError> for ParagraphRequestTestValidRequestPassesFault {
    fn from(value: crate::org::tiqian::protocol::paragraph_request_exception::NamedError) -> Self {
        ParagraphRequestTestValidRequestPassesFault::NamedErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphRequestTestValidRequestPassesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphRequestTestValidRequestPassesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphRequestTestValidRequestPassesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphRequestTestValidRequestPassesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NamedErrorFault(crate::org::tiqian::protocol::paragraph_request_exception::NamedError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault) -> Self {
        match value {
            ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault> for crate::org::tiqian::protocol::paragraph_request_exception::NamedError {
    fn from(value: ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault) -> Self {
        match value {
            ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault::NamedErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault) -> Self {
        match value {
            ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::protocol::paragraph_request_exception::NamedError> for ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault {
    fn from(value: crate::org::tiqian::protocol::paragraph_request_exception::NamedError) -> Self {
        ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault::NamedErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NamedErrorFault(crate::org::tiqian::protocol::paragraph_request_exception::NamedError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault) -> Self {
        match value {
            ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault> for crate::org::tiqian::protocol::paragraph_request_exception::NamedError {
    fn from(value: ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault) -> Self {
        match value {
            ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault::NamedErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault) -> Self {
        match value {
            ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::protocol::paragraph_request_exception::NamedError> for ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault {
    fn from(value: crate::org::tiqian::protocol::paragraph_request_exception::NamedError) -> Self {
        ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault::NamedErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NamedErrorFault(crate::org::tiqian::protocol::paragraph_request_exception::NamedError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault) -> Self {
        match value {
            ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault> for crate::org::tiqian::protocol::paragraph_request_exception::NamedError {
    fn from(value: ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault) -> Self {
        match value {
            ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault::NamedErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault) -> Self {
        match value {
            ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::protocol::paragraph_request_exception::NamedError> for ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault {
    fn from(value: crate::org::tiqian::protocol::paragraph_request_exception::NamedError) -> Self {
        ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault::NamedErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphRequestTestParagraphChecksReportTheDomainNamesInOrderFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<ParagraphRequestTestParagraphChecksReportTheDomainNamesInOrderFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphRequestTestParagraphChecksReportTheDomainNamesInOrderFault) -> Self {
        match value {
            ParagraphRequestTestParagraphChecksReportTheDomainNamesInOrderFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphRequestTestParagraphChecksReportTheDomainNamesInOrderFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: ParagraphRequestTestParagraphChecksReportTheDomainNamesInOrderFault) -> Self {
        match value {
            ParagraphRequestTestParagraphChecksReportTheDomainNamesInOrderFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphRequestTestParagraphChecksReportTheDomainNamesInOrderFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphRequestTestParagraphChecksReportTheDomainNamesInOrderFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for ParagraphRequestTestParagraphChecksReportTheDomainNamesInOrderFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        ParagraphRequestTestParagraphChecksReportTheDomainNamesInOrderFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn valid_request_passes() {
    testlib::run("org.tiqian.protocol.ParagraphRequestTest.validRequestPasses", "org.tiqian.protocol.ParagraphRequestTest.validRequestPasses", || {
        let mut recorder = TestTraceRecorder::new("ParagraphRequestTest");
        recorder.section(&"validRequestPasses");
        let _ = ParagraphRequestChecks::paragraph_request_checks_validate(ParagraphRequestTestSupport::paragraph_request_test_support_request()).unwrap();
        let _ = recorder.record(&"validate returned silently").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(true, Some("the plain request validates".to_string())).unwrap();
    });
}

#[test]
fn paragraph_checks_report_the_domain_names_in_order() {
    testlib::run("org.tiqian.protocol.ParagraphRequestTest.paragraphChecksReportTheDomainNamesInOrder", "org.tiqian.protocol.ParagraphRequestTest.paragraphChecksReportTheDomainNamesInOrder", || {
        let mut recorder = TestTraceRecorder::new("ParagraphRequestTest");
        recorder.section(&"paragraphChecksReportTheDomainNamesInOrder");
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"EmptyParagraph", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_text(&"   ")).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"EmptyParagraph",
ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_text(u_string::from_code_point(12288).as_str())).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidMaximumMeasure", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_max_width(0.0f64)).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidMaximumMeasure", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_max_width(f64::NAN)).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidFontSize", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_font_size(-1.0f64)).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidLineHeight", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_line_height(f64::INFINITY)).as_str(),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidFirstLineIndent", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_indent(f64::NAN)).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidFontWeight", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_weight(0)).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidFontWeight", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_weight(1001)).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidEmphasisDotGapEm", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_gap(-0.1f64)).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"MissingExplicitFontFamilies",
ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_families(&vec!["  ".to_string()])).as_str(), None).unwrap();
    });
}

#[test]
fn absent_gap_and_non_blank_exotic_space_pass() {
    testlib::run("org.tiqian.protocol.ParagraphRequestTest.absentGapAndNonBlankExoticSpacePass", "org.tiqian.protocol.ParagraphRequestTest.absentGapAndNonBlankExoticSpacePass", || {
        let mut recorder = TestTraceRecorder::new("ParagraphRequestTest");
        recorder.section(&"absentGapAndNonBlankExoticSpacePass");
        let _ = ParagraphRequestChecks::paragraph_request_checks_validate(ParagraphRequestTestSupport::paragraph_request_test_support_request()).unwrap();
        let nbsp = ParagraphRequestTestSupport::paragraph_request_test_support_with_families(&vec![u_string::from_code_point(160).clone()]);
        let _ = ParagraphRequestChecks::paragraph_request_checks_validate((nbsp).clone()).unwrap();
        let _ = recorder.record(&"absent gap and U+00A0 family validated").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(true, Some("absent gap and U+00A0 family validate".to_string())).unwrap();
    });
}

#[test]
fn span_checks_cover_range_families_and_numbers() {
    testlib::run("org.tiqian.protocol.ParagraphRequestTest.spanChecksCoverRangeFamiliesAndNumbers", "org.tiqian.protocol.ParagraphRequestTest.spanChecksCoverRangeFamiliesAndNumbers", || {
        let mut recorder = TestTraceRecorder::new("ParagraphRequestTest");
        recorder.section(&"spanChecksCoverRangeFamiliesAndNumbers");
        let mut range_base = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        range_base.text_spans = vec![
    (TextSpanInput { start: 2, end: 1, families: vec!["Fake CJK".to_string()], font_size_px: 16.0f64, font_weight: 400, italic: false, baseline_shift: 0.0f64 }).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidTextSpanRange", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((range_base).clone()).as_str(), None).unwrap();
        let mut range_base2 = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        range_base2.text_spans = vec![
    (TextSpanInput { start: 0, end: 9, families: vec!["Fake CJK".to_string()], font_size_px: 16.0f64, font_weight: 400, italic: false, baseline_shift: 0.0f64 }).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidTextSpanRange", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((range_base2).clone()).as_str(), None).unwrap();
        let mut families_base = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        families_base.text_spans = vec![
    (TextSpanInput { start: 0, end: 2, families: vec![" ".to_string()], font_size_px: 16.0f64, font_weight: 400, italic: false, baseline_shift: 0.0f64 }).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"MissingTextSpanFontFamilies", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((families_base).clone()).as_str(), None).unwrap();
        let mut zero_width_base = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        zero_width_base.text_spans = vec![
    (TextSpanInput { start: 0, end: 2, families: vec![u_string::from_code_point(8203).clone()], font_size_px: 16.0f64, font_weight: 400, italic: false, baseline_shift: 0.0f64 }).clone(),
];
        let _ = ParagraphRequestChecks::paragraph_request_checks_validate((zero_width_base).clone()).unwrap();
        let mut font_size_base = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        font_size_base.text_spans = vec![
    (TextSpanInput { start: 0, end: 2, families: vec!["Fake CJK".to_string()], font_size_px: 0.0f64, font_weight: 400, italic: false, baseline_shift: 0.0f64 }).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidTextSpanFontSize", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((font_size_base).clone()).as_str(), None).unwrap();
        let mut weight_base = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        weight_base.text_spans = vec![
    (TextSpanInput { start: 0, end: 2, families: vec!["Fake CJK".to_string()], font_size_px: 16.0f64, font_weight: 1001, italic: false, baseline_shift: 0.0f64 }).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidTextSpanFontWeight", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((weight_base).clone()).as_str(), None).unwrap();
        let mut baseline_base = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        baseline_base.text_spans = vec![
    (TextSpanInput { start: 0, end: 2, families: vec!["Fake CJK".to_string()], font_size_px: 16.0f64, font_weight: 400, italic: false, baseline_shift: f64::NAN }).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidTextSpanBaselineShift", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((baseline_base).clone()).as_str(), None).unwrap();
    });
}

#[test]
fn boundaries_and_ranges_use_the_utf16_length() {
    testlib::run("org.tiqian.protocol.ParagraphRequestTest.boundariesAndRangesUseTheUtf16Length", "org.tiqian.protocol.ParagraphRequestTest.boundariesAndRangesUseTheUtf16Length", || {
        let mut recorder = TestTraceRecorder::new("ParagraphRequestTest");
        recorder.section(&"boundariesAndRangesUseTheUtf16Length");
        let mut astral = ParagraphRequestTestSupport::paragraph_request_test_support_with_text(format!("{}{}",
            u_string::from_code_point(128512),
            "字"
        ).as_str());
        astral.source_boundaries = vec![3];
        let _ = ParagraphRequestChecks::paragraph_request_checks_validate((astral).clone()).unwrap();
        astral.source_boundaries = vec![4];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidSourceBoundary", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((astral).clone()).as_str(), None).unwrap();
        let mut breaks = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        breaks.line_break_spans = vec![(LineBreakSpanInput { start: 0, end: 5, policy: "ProgressiveTechnical".to_string() }).clone()];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidLineBreakSpanRange", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((breaks).clone()).as_str(), None).unwrap();
        let mut boxes = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        boxes.inline_boxes = vec![
    (InlineBoxInput { start: 2, end: 5, inline_start: 1.0f64, inline_end: 2.0f64, outer_spacing: "Narrow".to_string() }).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidInlineBoxRange", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((boxes).clone()).as_str(), None).unwrap();
        let mut geometry = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        geometry.inline_boxes = vec![
    (InlineBoxInput { start: 0, end: 3, inline_start: 1.0f64, inline_end: f64::NAN, outer_spacing: "Narrow".to_string() }).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidInlineBoxGeometry", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((geometry).clone()).as_str(), None).unwrap();
    });
}

#[test]
fn whitespace_edge_set_matches_the_kotlin_lane() {
    testlib::run("org.tiqian.protocol.ParagraphRequestTest.whitespaceEdgeSetMatchesTheKotlinLane", "org.tiqian.protocol.ParagraphRequestTest.whitespaceEdgeSetMatchesTheKotlinLane", || {
        let mut recorder = TestTraceRecorder::new("ParagraphRequestTest");
        recorder.section(&"whitespaceEdgeSetMatchesTheKotlinLane");
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"EmptyParagraph", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_text(&" ")).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"EmptyParagraph",
ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_text(u_string::from_code_point(12288).as_str())).as_str(), None).unwrap();
        let _ = ParagraphRequestChecks::paragraph_request_checks_validate(ParagraphRequestTestSupport::paragraph_request_test_support_with_text(u_string::from_code_point(160).as_str())).unwrap();
        let _ = ParagraphRequestChecks::paragraph_request_checks_validate(ParagraphRequestTestSupport::paragraph_request_test_support_with_text(u_string::from_code_point(8239).as_str())).unwrap();
        let _ = ParagraphRequestChecks::paragraph_request_checks_validate(ParagraphRequestTestSupport::paragraph_request_test_support_with_text(u_string::from_code_point(8203).as_str())).unwrap();
        let _ = recorder.record(&"space/U+3000 blank; U+00A0/U+202F/U+200B non-blank").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(true, Some("the whitespace edge set matches Character.isWhitespace".to_string())).unwrap();
    });
}

#[test]
fn inline_object_and_decoration_checks() {
    testlib::run("org.tiqian.protocol.ParagraphRequestTest.inlineObjectAndDecorationChecks", "org.tiqian.protocol.ParagraphRequestTest.inlineObjectAndDecorationChecks", || {
        let mut recorder = TestTraceRecorder::new("ParagraphRequestTest");
        recorder.section(&"inlineObjectAndDecorationChecks");
        let mut objects = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        objects.inline_objects = vec![InlineObjectInput { start: 0, end: 9, advance: 1.0f64, ascent: 1.0f64, descent: 1.0f64 }];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidInlineObjectRange", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((objects).clone()).as_str(), None).unwrap();
        objects.inline_objects[0usize].end = 2u32;
        objects.inline_objects[0usize].advance = -1.0f64;
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidInlineObjectAdvance", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((objects).clone()).as_str(), None).unwrap();
        objects.inline_objects[0usize].advance = 1.0f64;
        objects.inline_objects[0usize].ascent = f64::NAN;
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidInlineObjectVerticalGeometry", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((objects).clone()).as_str(), None).unwrap();
        let mut decorations = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        decorations.decorations = vec![(DecorationInput { start: 3, end: 2, kind: "Underline".to_string() }).clone()];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"InvalidDecorationRange", ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((decorations).clone()).as_str(), None).unwrap();
    });
}
