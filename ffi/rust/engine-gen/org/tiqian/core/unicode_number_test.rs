#![cfg(test)]

use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::unicode_number::UnicodeNumber;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertFalseFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::TracedAssertionsAssertFalseFaultFault(value) => write!(formatter, "{}", value),
            UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
            UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault) -> Self {
        match value {
            UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault) -> Self {
        match value {
            UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault) -> Self {
        match value {
            UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault) -> Self {
        match value {
            UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault {
    fn from(value: UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault) -> Self {
        match value {
            UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::TracedAssertionsAssertFalseFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault) -> Self {
        match value {
            UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault) -> Self {
        match value {
            UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault> for UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault) -> Self {
        UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::TracedAssertionsAssertFalseFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        UnicodeNumberTestNumbersAreMembersAcrossScriptsAndNonScalarsAreRejectedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn numbers_are_members_across_scripts_and_non_scalars_are_rejected() {
    testlib::run("org.tiqian.core.UnicodeNumberTest.numbersAreMembersAcrossScriptsAndNonScalarsAreRejected", "org.tiqian.core.UnicodeNumberTest.numbersAreMembersAcrossScriptsAndNonScalarsAreRejected", || {
        TestTraceRecorder::new(&(UStr::new(&[85,110,105,99,111,100,101,78,117,109,98,101,114,84,101,115,116]))).section(UStr::new(&[110,117,109,98,101,114,115,65,114,101,77,101,109,98,101,114,115,65,99,114,111,115,115,83,99,114,105,112,116,115,65,110,100,78,111,110,83,99,97,108,97,114,115,65,114,101,82,101,106,101,99,116,101,100]));
        let positives = vec![48, 1634, 189];
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((positives.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let code_point = positives[usize::try_from(index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_assert_true(UnicodeNumber::unicode_number_contains(code_point).unwrap(), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("U+")); __s += UString::from(format!("{}", format!("{:0w$X}", code_point, w = usize::try_from(0).unwrap_or_default()).to_lowercase()).as_str()).as_ustr(); __s }).as_str()))).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let negatives = vec![97, 16397, 8217];
        index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((negatives.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let code_point = negatives[usize::try_from(index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_assert_false(UnicodeNumber::unicode_number_contains(code_point).unwrap(), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("U+")); __s += UString::from(format!("{}", format!("{:0w$X}", code_point, w = usize::try_from(0).unwrap_or_default()).to_lowercase()).as_str()).as_ustr(); __s }).as_str()))).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        UnicodeNumber::unicode_number_contains(56320).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        UnicodeNumber::unicode_number_contains(4294967295u32).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        UnicodeNumber::unicode_number_contains(1114112).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
    });
}
