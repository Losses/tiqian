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
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NamedErrorFault(crate::org::tiqian::protocol::paragraph_request_exception::NamedError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault::NamedErrorFault(value) => write!(formatter, "{}", value),
            ParagraphRequestTestWhitespaceEdgeSetMatchesTheKotlinLaneFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ParagraphRequestTestValidRequestPassesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphRequestTestValidRequestPassesFault::NamedErrorFault(value) => write!(formatter, "{}", value),
            ParagraphRequestTestValidRequestPassesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphRequestTestValidRequestPassesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault::NamedErrorFault(value) => write!(formatter, "{}", value),
            ParagraphRequestTestSpanChecksCoverRangeFamiliesAndNumbersFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault::NamedErrorFault(value) => write!(formatter, "{}", value),
            ParagraphRequestTestBoundariesAndRangesUseTheUtf16LengthFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault::NamedErrorFault(value) => write!(formatter, "{}", value),
            ParagraphRequestTestAbsentGapAndNonBlankExoticSpacePassFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for ParagraphRequestTestParagraphChecksReportTheDomainNamesInOrderFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphRequestTestParagraphChecksReportTheDomainNamesInOrderFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            ParagraphRequestTestParagraphChecksReportTheDomainNamesInOrderFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[80,97,114,97,103,114,97,112,104,82,101,113,117,101,115,116,84,101,115,116])));
        recorder.section(UStr::new(&[118,97,108,105,100,82,101,113,117,101,115,116,80,97,115,115,101,115]));
        let _ = ParagraphRequestChecks::paragraph_request_checks_validate(ParagraphRequestTestSupport::paragraph_request_test_support_request()).unwrap();
        let _ = recorder.record(UStr::new(&[118,97,108,105,100,97,116,101,32,114,101,116,117,114,110,101,100,32,115,105,108,101,110,116,108,121])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(true, Some(UString::from("the plain request validates"))).unwrap();
    });
}

#[test]
fn paragraph_checks_report_the_domain_names_in_order() {
    testlib::run("org.tiqian.protocol.ParagraphRequestTest.paragraphChecksReportTheDomainNamesInOrder", "org.tiqian.protocol.ParagraphRequestTest.paragraphChecksReportTheDomainNamesInOrder", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[80,97,114,97,103,114,97,112,104,82,101,113,117,101,115,116,84,101,115,116])));
        recorder.section(UStr::new(&[112,97,114,97,103,114,97,112,104,67,104,101,99,107,115,82,101,112,111,114,116,84,104,101,68,111,109,97,105,110,78,97,109,101,115,73,110,79,114,100,101,114]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[69,109,112,116,121,80,97,114,97,103,114,97,112,104]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_text(UStr::new(&[32,32,32]))).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[69,109,112,116,121,80,97,114,97,103,114,97,112,104]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_text(u_string::from_code_point(12288).as_ustr())).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,77,97,120,105,109,117,109,77,101,97,115,117,114,101]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_max_width(0.0f64)).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,77,97,120,105,109,117,109,77,101,97,115,117,114,101]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_max_width(f64::NAN)).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,70,111,110,116,83,105,122,101]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_font_size(-1.0f64)).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,76,105,110,101,72,101,105,103,104,116]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_line_height(f64::INFINITY)).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,70,105,114,115,116,76,105,110,101,73,110,100,101,110,116]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_indent(f64::NAN)).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,70,111,110,116,87,101,105,103,104,116]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_weight(0)).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,70,111,110,116,87,101,105,103,104,116]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_weight(1001)).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,69,109,112,104,97,115,105,115,68,111,116,71,97,112,69,109]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_gap(-0.1f64)).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[77,105,115,115,105,110,103,69,120,112,108,105,99,105,116,70,111,110,116,70,97,109,105,108,105,101,115]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_families(&vec![UString::from("  ").to_ustring()])).as_ustr(), None).unwrap();
    });
}

#[test]
fn absent_gap_and_non_blank_exotic_space_pass() {
    testlib::run("org.tiqian.protocol.ParagraphRequestTest.absentGapAndNonBlankExoticSpacePass", "org.tiqian.protocol.ParagraphRequestTest.absentGapAndNonBlankExoticSpacePass", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[80,97,114,97,103,114,97,112,104,82,101,113,117,101,115,116,84,101,115,116])));
        recorder.section(UStr::new(&[97,98,115,101,110,116,71,97,112,65,110,100,78,111,110,66,108,97,110,107,69,120,111,116,105,99,83,112,97,99,101,80,97,115,115]));
        let _ = ParagraphRequestChecks::paragraph_request_checks_validate(ParagraphRequestTestSupport::paragraph_request_test_support_request()).unwrap();
        let nbsp = ParagraphRequestTestSupport::paragraph_request_test_support_with_families(&vec![u_string::from_code_point(160).clone()]);
        let _ = ParagraphRequestChecks::paragraph_request_checks_validate((nbsp).clone()).unwrap();
        let _ = recorder.record(UStr::new(&[97,98,115,101,110,116,32,103,97,112,32,97,110,100,32,85,43,48,48,65,48,32,102,97,109,105,108,121,32,118,97,108,105,100,97,116,101,100])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(true, Some(UString::from("absent gap and U+00A0 family validate"))).unwrap();
    });
}

#[test]
fn span_checks_cover_range_families_and_numbers() {
    testlib::run("org.tiqian.protocol.ParagraphRequestTest.spanChecksCoverRangeFamiliesAndNumbers", "org.tiqian.protocol.ParagraphRequestTest.spanChecksCoverRangeFamiliesAndNumbers", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[80,97,114,97,103,114,97,112,104,82,101,113,117,101,115,116,84,101,115,116])));
        recorder.section(UStr::new(&[115,112,97,110,67,104,101,99,107,115,67,111,118,101,114,82,97,110,103,101,70,97,109,105,108,105,101,115,65,110,100,78,117,109,98,101,114,115]));
        let mut range_base = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        range_base.text_spans = vec![
    (TextSpanInput { start: 2, end: 1, families: vec![UString::from("Fake CJK").to_ustring()], font_size_px: 16.0f64, font_weight: 400, italic: false, baseline_shift: 0.0f64 }).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,84,101,120,116,83,112,97,110,82,97,110,103,101]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((range_base).clone()).as_ustr(), None).unwrap();
        let mut range_base2 = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        range_base2.text_spans = vec![
    (TextSpanInput { start: 0, end: 9, families: vec![UString::from("Fake CJK").to_ustring()], font_size_px: 16.0f64, font_weight: 400, italic: false, baseline_shift: 0.0f64 }).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,84,101,120,116,83,112,97,110,82,97,110,103,101]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((range_base2).clone()).as_ustr(), None).unwrap();
        let mut families_base = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        families_base.text_spans = vec![
    (TextSpanInput { start: 0, end: 2, families: vec![UString::from(" ").to_ustring()], font_size_px: 16.0f64, font_weight: 400, italic: false, baseline_shift: 0.0f64 }).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[77,105,115,115,105,110,103,84,101,120,116,83,112,97,110,70,111,110,116,70,97,109,105,108,105,101,115]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((families_base).clone()).as_ustr(), None).unwrap();
        let mut zero_width_base = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        zero_width_base.text_spans = vec![
    (TextSpanInput { start: 0, end: 2, families: vec![u_string::from_code_point(8203).clone()], font_size_px: 16.0f64, font_weight: 400, italic: false, baseline_shift: 0.0f64 }).clone(),
];
        let _ = ParagraphRequestChecks::paragraph_request_checks_validate((zero_width_base).clone()).unwrap();
        let mut font_size_base = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        font_size_base.text_spans = vec![
    (TextSpanInput { start: 0, end: 2, families: vec![UString::from("Fake CJK").to_ustring()], font_size_px: 0.0f64, font_weight: 400, italic: false, baseline_shift: 0.0f64 }).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,84,101,120,116,83,112,97,110,70,111,110,116,83,105,122,101]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((font_size_base).clone()).as_ustr(), None).unwrap();
        let mut weight_base = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        weight_base.text_spans = vec![
    (TextSpanInput { start: 0, end: 2, families: vec![UString::from("Fake CJK").to_ustring()], font_size_px: 16.0f64, font_weight: 1001, italic: false, baseline_shift: 0.0f64 }).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,84,101,120,116,83,112,97,110,70,111,110,116,87,101,105,103,104,116]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((weight_base).clone()).as_ustr(), None).unwrap();
        let mut baseline_base = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        baseline_base.text_spans = vec![
    (TextSpanInput { start: 0, end: 2, families: vec![UString::from("Fake CJK").to_ustring()], font_size_px: 16.0f64, font_weight: 400, italic: false, baseline_shift: f64::NAN }).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,84,101,120,116,83,112,97,110,66,97,115,101,108,105,110,101,83,104,105,102,116]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((baseline_base).clone()).as_ustr(), None).unwrap();
    });
}

#[test]
fn boundaries_and_ranges_use_the_utf16_length() {
    testlib::run("org.tiqian.protocol.ParagraphRequestTest.boundariesAndRangesUseTheUtf16Length", "org.tiqian.protocol.ParagraphRequestTest.boundariesAndRangesUseTheUtf16Length", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[80,97,114,97,103,114,97,112,104,82,101,113,117,101,115,116,84,101,115,116])));
        recorder.section(UStr::new(&[98,111,117,110,100,97,114,105,101,115,65,110,100,82,97,110,103,101,115,85,115,101,84,104,101,85,116,102,49,54,76,101,110,103,116,104]));
        let mut astral = ParagraphRequestTestSupport::paragraph_request_test_support_with_text(UString::from(format!("{}", { let mut __s = UString::new(); __s += u_string::from_code_point(128512).as_ustr(); __s += &(UString::from("字")); __s }).as_str()).as_ustr());
        astral.source_boundaries = vec![3];
        let _ = ParagraphRequestChecks::paragraph_request_checks_validate((astral).clone()).unwrap();
        astral.source_boundaries = vec![4];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,83,111,117,114,99,101,66,111,117,110,100,97,114,121]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((astral).clone()).as_ustr(), None).unwrap();
        let mut breaks = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        breaks.line_break_spans = vec![
    (LineBreakSpanInput { start: 0, end: 5, policy: UString::from("ProgressiveTechnical").to_ustring() }).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,76,105,110,101,66,114,101,97,107,83,112,97,110,82,97,110,103,101]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((breaks).clone()).as_ustr(), None).unwrap();
        let mut boxes = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        boxes.inline_boxes = vec![
    (InlineBoxInput { start: 2, end: 5, inline_start: 1.0f64, inline_end: 2.0f64, outer_spacing: UString::from("Narrow").to_ustring() }).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,73,110,108,105,110,101,66,111,120,82,97,110,103,101]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((boxes).clone()).as_ustr(), None).unwrap();
        let mut geometry = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        geometry.inline_boxes = vec![
    (InlineBoxInput { start: 0, end: 3, inline_start: 1.0f64, inline_end: f64::NAN, outer_spacing: UString::from("Narrow").to_ustring() }).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,73,110,108,105,110,101,66,111,120,71,101,111,109,101,116,114,121]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((geometry).clone()).as_ustr(), None).unwrap();
    });
}

#[test]
fn whitespace_edge_set_matches_the_kotlin_lane() {
    testlib::run("org.tiqian.protocol.ParagraphRequestTest.whitespaceEdgeSetMatchesTheKotlinLane", "org.tiqian.protocol.ParagraphRequestTest.whitespaceEdgeSetMatchesTheKotlinLane", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[80,97,114,97,103,114,97,112,104,82,101,113,117,101,115,116,84,101,115,116])));
        recorder.section(UStr::new(&[119,104,105,116,101,115,112,97,99,101,69,100,103,101,83,101,116,77,97,116,99,104,101,115,84,104,101,75,111,116,108,105,110,76,97,110,101]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[69,109,112,116,121,80,97,114,97,103,114,97,112,104]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_text(UStr::new(&[32]))).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[69,109,112,116,121,80,97,114,97,103,114,97,112,104]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of(ParagraphRequestTestSupport::paragraph_request_test_support_with_text(u_string::from_code_point(12288).as_ustr())).as_ustr(), None).unwrap();
        let _ = ParagraphRequestChecks::paragraph_request_checks_validate(ParagraphRequestTestSupport::paragraph_request_test_support_with_text(u_string::from_code_point(160).as_ustr())).unwrap();
        let _ = ParagraphRequestChecks::paragraph_request_checks_validate(ParagraphRequestTestSupport::paragraph_request_test_support_with_text(u_string::from_code_point(8239).as_ustr())).unwrap();
        let _ = ParagraphRequestChecks::paragraph_request_checks_validate(ParagraphRequestTestSupport::paragraph_request_test_support_with_text(u_string::from_code_point(8203).as_ustr())).unwrap();
        let _ = recorder.record(UStr::new(&[115,112,97,99,101,47,85,43,51,48,48,48,32,98,108,97,110,107,59,32,85,43,48,48,65,48,47,85,43,50,48,50,70,47,85,43,50,48,48,66,32,110,111,110,45,98,108,97,110,107])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(true, Some(UString::from("the whitespace edge set matches Character.isWhitespace"))).unwrap();
    });
}

#[test]
fn inline_object_and_decoration_checks() {
    testlib::run("org.tiqian.protocol.ParagraphRequestTest.inlineObjectAndDecorationChecks", "org.tiqian.protocol.ParagraphRequestTest.inlineObjectAndDecorationChecks", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[80,97,114,97,103,114,97,112,104,82,101,113,117,101,115,116,84,101,115,116])));
        recorder.section(UStr::new(&[105,110,108,105,110,101,79,98,106,101,99,116,65,110,100,68,101,99,111,114,97,116,105,111,110,67,104,101,99,107,115]));
        let mut objects = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        objects.inline_objects = vec![InlineObjectInput { start: 0, end: 9, advance: 1.0f64, ascent: 1.0f64, descent: 1.0f64 }];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,73,110,108,105,110,101,79,98,106,101,99,116,82,97,110,103,101]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((objects).clone()).as_ustr(), None).unwrap();
        objects.inline_objects[0usize].end = 2u32;
        objects.inline_objects[0usize].advance = -1.0f64;
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,73,110,108,105,110,101,79,98,106,101,99,116,65,100,118,97,110,99,101]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((objects).clone()).as_ustr(), None).unwrap();
        objects.inline_objects[0usize].advance = 1.0f64;
        objects.inline_objects[0usize].ascent = f64::NAN;
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,73,110,108,105,110,101,79,98,106,101,99,116,86,101,114,116,105,99,97,108,71,101,111,109,101,116,114,121]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((objects).clone()).as_ustr(), None).unwrap();
        let mut decorations = ParagraphRequestTestSupport::paragraph_request_test_support_request();
        decorations.decorations = vec![(DecorationInput { start: 3, end: 2, kind: UString::from("Underline").to_ustring() }).clone()];
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[73,110,118,97,108,105,100,68,101,99,111,114,97,116,105,111,110,82,97,110,103,101]), ParagraphRequestTestSupport::paragraph_request_test_support_issue_of((decorations).clone()).as_ustr(), None).unwrap();
    });
}
