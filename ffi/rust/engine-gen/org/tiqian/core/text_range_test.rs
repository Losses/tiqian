#![cfg(test)]

use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum TextRangeTestRejectsNegativeStartFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<TextRangeTestRejectsNegativeStartFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TextRangeTestRejectsNegativeStartFault) -> Self {
        match value {
            TextRangeTestRejectsNegativeStartFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextRangeTestRejectsNegativeStartFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: TextRangeTestRejectsNegativeStartFault) -> Self {
        match value {
            TextRangeTestRejectsNegativeStartFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextRangeTestRejectsNegativeStartFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: TextRangeTestRejectsNegativeStartFault) -> Self {
        match value {
            TextRangeTestRejectsNegativeStartFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TextRangeTestRejectsNegativeStartFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TextRangeTestRejectsNegativeStartFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for TextRangeTestRejectsNegativeStartFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        TextRangeTestRejectsNegativeStartFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for TextRangeTestRejectsNegativeStartFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        TextRangeTestRejectsNegativeStartFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextRangeTestExposesLengthFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<TextRangeTestExposesLengthFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TextRangeTestExposesLengthFault) -> Self {
        match value {
            TextRangeTestExposesLengthFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextRangeTestExposesLengthFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: TextRangeTestExposesLengthFault) -> Self {
        match value {
            TextRangeTestExposesLengthFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextRangeTestExposesLengthFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: TextRangeTestExposesLengthFault) -> Self {
        match value {
            TextRangeTestExposesLengthFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TextRangeTestExposesLengthFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TextRangeTestExposesLengthFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for TextRangeTestExposesLengthFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        TextRangeTestExposesLengthFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for TextRangeTestExposesLengthFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        TextRangeTestExposesLengthFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn exposes_length() {
    testlib::run("org.tiqian.core.TextRangeTest.exposesLength", "org.tiqian.core.TextRangeTest.exposesLength", || {
        TestTraceRecorder::new("TextRangeTest").section(&"exposesLength");
        let _ = TracedAssertions::traced_assertions_assert_equals(3, TextRange::new(2u32, 5u32).unwrap().get_length(), None).unwrap();
    });
}

#[test]
fn rejects_negative_start() {
    testlib::run("org.tiqian.core.TextRangeTest.rejectsNegativeStart", "org.tiqian.core.TextRangeTest.rejectsNegativeStart", || {
        TestTraceRecorder::new("TextRangeTest").section(&"rejectsNegativeStart");
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        TextRange::new(4294967295u32, 1u32).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}
