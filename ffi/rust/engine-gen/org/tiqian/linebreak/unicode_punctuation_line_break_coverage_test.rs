#![cfg(test)]

use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::linebreak::unicode_punctuation_line_break::UnicodePunctuationLineBreak;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault) -> Self {
        match value {
            UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault) -> Self {
        match value {
            UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault) -> Self {
        match value {
            UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault) -> Self {
        match value {
            UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault) -> Self {
        match value {
            UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault) -> Self {
        match value {
            UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn lookup_classes_cover_the_uax_tailorable_punctuation_classes() {
    testlib::run("org.tiqian.linebreak.UnicodePunctuationLineBreakCoverageTest.lookupClassesCoverTheUaxTailorablePunctuationClasses", "org.tiqian.linebreak.UnicodePunctuationLineBreakCoverageTest.lookupClassesCoverTheUaxTailorablePunctuationClasses", || {
        TestTraceRecorder::new("UnicodePunctuationLineBreakCoverageTest").section(&"lookupClassesCoverTheUaxTailorablePunctuationClasses");
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"BreakAfter", UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(124).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"BreakBoth", UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(8212).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"HyphenHH", UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(1418).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"Nonstarter", UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(8252).unwrap().name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn non_scalar_code_points_are_rejected() {
    testlib::run("org.tiqian.linebreak.UnicodePunctuationLineBreakCoverageTest.nonScalarCodePointsAreRejected", "org.tiqian.linebreak.UnicodePunctuationLineBreakCoverageTest.nonScalarCodePointsAreRejected", || {
        TestTraceRecorder::new("UnicodePunctuationLineBreakCoverageTest").section(&"nonScalarCodePointsAreRejected");
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(4294967295u32).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(1114112).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(55296).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}
