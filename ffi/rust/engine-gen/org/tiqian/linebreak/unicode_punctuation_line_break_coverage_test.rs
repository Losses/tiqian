#![cfg(test)]

use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::linebreak::unicode_punctuation_line_break::UnicodePunctuationLineBreak;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}
impl std::fmt::Display for UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationLineBreakCoverageTestNonScalarCodePointsAreRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            UnicodePunctuationLineBreakCoverageTestLookupClassesCoverTheUaxTailorablePunctuationClassesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        TestTraceRecorder::new(&(UStr::new(&[85,110,105,99,111,100,101,80,117,110,99,116,117,97,116,105,111,110,76,105,110,101,66,114,101,97,107,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[108,111,111,107,117,112,67,108,97,115,115,101,115,67,111,118,101,114,84,104,101,85,97,120,84,97,105,108,111,114,97,98,108,101,80,117,110,99,116,117,97,116,105,111,110,67,108,97,115,115,101,115]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[66,114,101,97,107,65,102,116,101,114]), UString::from(UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(124).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[66,114,101,97,107,66,111,116,104]), UString::from(UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(8212).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[72,121,112,104,101,110,72,72]), UString::from(UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(1418).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[78,111,110,115,116,97,114,116,101,114]), UString::from(UnicodePunctuationLineBreak::unicode_punctuation_line_break_class_of(8252).unwrap().name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn non_scalar_code_points_are_rejected() {
    testlib::run("org.tiqian.linebreak.UnicodePunctuationLineBreakCoverageTest.nonScalarCodePointsAreRejected", "org.tiqian.linebreak.UnicodePunctuationLineBreakCoverageTest.nonScalarCodePointsAreRejected", || {
        TestTraceRecorder::new(&(UStr::new(&[85,110,105,99,111,100,101,80,117,110,99,116,117,97,116,105,111,110,76,105,110,101,66,114,101,97,107,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[110,111,110,83,99,97,108,97,114,67,111,100,101,80,111,105,110,116,115,65,114,101,82,101,106,101,99,116,101,100]));
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
