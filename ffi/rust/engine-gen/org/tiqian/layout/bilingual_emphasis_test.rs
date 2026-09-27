#![cfg(test)]

use crate::org::tiqian::layout::bilingual_emphasis_test_support::BilingualEmphasisTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum BilingualEmphasisTestEmphasisDotsHanButNotWesternFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    SupportLayoutFault(crate::org::tiqian::layout::bilingual_emphasis_test_support::BilingualEmphasisTestSupportLayoutFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}

impl From<BilingualEmphasisTestEmphasisDotsHanButNotWesternFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: BilingualEmphasisTestEmphasisDotsHanButNotWesternFault) -> Self {
        match value {
            BilingualEmphasisTestEmphasisDotsHanButNotWesternFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BilingualEmphasisTestEmphasisDotsHanButNotWesternFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: BilingualEmphasisTestEmphasisDotsHanButNotWesternFault) -> Self {
        match value {
            BilingualEmphasisTestEmphasisDotsHanButNotWesternFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BilingualEmphasisTestEmphasisDotsHanButNotWesternFault> for crate::org::tiqian::layout::bilingual_emphasis_test_support::BilingualEmphasisTestSupportLayoutFault {
    fn from(value: BilingualEmphasisTestEmphasisDotsHanButNotWesternFault) -> Self {
        match value {
            BilingualEmphasisTestEmphasisDotsHanButNotWesternFault::SupportLayoutFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BilingualEmphasisTestEmphasisDotsHanButNotWesternFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: BilingualEmphasisTestEmphasisDotsHanButNotWesternFault) -> Self {
        match value {
            BilingualEmphasisTestEmphasisDotsHanButNotWesternFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BilingualEmphasisTestEmphasisDotsHanButNotWesternFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: BilingualEmphasisTestEmphasisDotsHanButNotWesternFault) -> Self {
        match value {
            BilingualEmphasisTestEmphasisDotsHanButNotWesternFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for BilingualEmphasisTestEmphasisDotsHanButNotWesternFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        BilingualEmphasisTestEmphasisDotsHanButNotWesternFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for BilingualEmphasisTestEmphasisDotsHanButNotWesternFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        BilingualEmphasisTestEmphasisDotsHanButNotWesternFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::bilingual_emphasis_test_support::BilingualEmphasisTestSupportLayoutFault> for BilingualEmphasisTestEmphasisDotsHanButNotWesternFault {
    fn from(value: crate::org::tiqian::layout::bilingual_emphasis_test_support::BilingualEmphasisTestSupportLayoutFault) -> Self {
        BilingualEmphasisTestEmphasisDotsHanButNotWesternFault::SupportLayoutFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for BilingualEmphasisTestEmphasisDotsHanButNotWesternFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        BilingualEmphasisTestEmphasisDotsHanButNotWesternFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for BilingualEmphasisTestEmphasisDotsHanButNotWesternFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        BilingualEmphasisTestEmphasisDotsHanButNotWesternFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[test]
fn emphasis_dots_han_but_not_western() {
    testlib::run("org.tiqian.layout.BilingualEmphasisTest.emphasisDotsHanButNotWestern", "org.tiqian.layout.BilingualEmphasisTest.emphasisDotsHanButNotWestern", || {
        let mut t = TestTraceRecorder::new("BilingualEmphasisTest");
        t.section(&"emphasisDotsHanButNotWestern");
        let r = BilingualEmphasisTestSupport::bilingual_emphasis_test_support_layout().unwrap();
        let decisions = ((r.debug).clone().decoration_decisions).clone();
        if i32::from_ne_bytes((u32::try_from((decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) < (3) {
            return;
        }
        let han_first = (decisions[0usize]).clone();
        let western = (decisions[1usize]).clone();
        let han_last = (decisions[2usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_true(han_first.applied, Some("Han 中 gets a 着重号 dot".to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(han_last.applied, Some("Han 中 gets a 着重号 dot".to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!western.applied, Some("Western A must not get a dot".to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"no-dot-on-non-han", (western.reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, western.dot_diameter, None).unwrap();
    });
}
