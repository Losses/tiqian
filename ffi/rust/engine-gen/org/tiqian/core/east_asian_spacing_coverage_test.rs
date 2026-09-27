#![cfg(test)]

use crate::org::tiqian::core::east_asian_spacing_data::EastAsianSpacingData;
use crate::org::tiqian::core::east_asian_spacing_edges::EastAsianSpacingEdges;
use crate::org::tiqian::core::east_asian_spacing_value::EastAsianSpacingValue;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::unicode_east_asian_spacing::UnicodeEastAsianSpacing;
use crate::org::tiqian::core::unicode_script_evidence::UnicodeScriptEvidence;
use crate::org::tiqian::core::unicode_script_evidence_classifier::UnicodeScriptEvidenceClassifier;
use crate::org::tiqian::core::unicode_word_character::UnicodeWordCharacter;
use crate::org::tiqian::test::test_helpers::TestHelpers;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault) -> Self {
        match value {
            EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn test_unicode_word_character() {
    testlib::run("org.tiqian.core.EastAsianSpacingCoverageTest.testUnicodeWordCharacter", "org.tiqian.core.EastAsianSpacingCoverageTest.testUnicodeWordCharacter", || {
        TestTraceRecorder::new("EastAsianSpacingCoverageTest").section(&"testUnicodeWordCharacter");
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"17.0.0", UnicodeWordCharacter::UNICODE_WORD_CHARACTER_DATA_REVISION.to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u_string::unit_count(&(UnicodeWordCharacter::UNICODE_WORD_CHARACTER_DATA_SOURCE.to_string()))).to_ne_bytes())) > (0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u_string::unit_count(&(UnicodeWordCharacter::UNICODE_WORD_CHARACTER_DATA_SHA256.to_string()))).to_ne_bytes())) > (0), None).unwrap();
        let _ = EastAsianSpacingCoverageTestHelpers::east_asian_spacing_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        UnicodeWordCharacter::unicode_word_character_contains(4294967295u32).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = EastAsianSpacingCoverageTestHelpers::east_asian_spacing_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        UnicodeWordCharacter::unicode_word_character_contains(1114112).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = EastAsianSpacingCoverageTestHelpers::east_asian_spacing_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        UnicodeWordCharacter::unicode_word_character_contains(55296).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = EastAsianSpacingCoverageTestHelpers::east_asian_spacing_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        UnicodeWordCharacter::unicode_word_character_contains(57343).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(UnicodeWordCharacter::unicode_word_character_contains(65).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(UnicodeWordCharacter::unicode_word_character_contains(20013).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(UnicodeWordCharacter::unicode_word_character_contains(32).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(UnicodeWordCharacter::unicode_word_character_contains(33).unwrap(), None).unwrap();
    });
}

#[test]
fn test_unicode_script_evidence() {
    testlib::run("org.tiqian.core.EastAsianSpacingCoverageTest.testUnicodeScriptEvidence", "org.tiqian.core.EastAsianSpacingCoverageTest.testUnicodeScriptEvidence", || {
        TestTraceRecorder::new("EastAsianSpacingCoverageTest").section(&"testUnicodeScriptEvidence");
        let values = vec![UnicodeScriptEvidence::Neutral, UnicodeScriptEvidence::EastAsian, UnicodeScriptEvidence::Other];
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let value = values[usize::try_from(index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { value.name().to_string() }.as_str(), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"17.0.0", UnicodeScriptEvidenceClassifier::UNICODE_SCRIPT_EVIDENCE_CLASSIFIER_DATA_REVISION.to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u_string::unit_count(&(UnicodeScriptEvidenceClassifier::UNICODE_SCRIPT_EVIDENCE_CLASSIFIER_DATA_SOURCE.to_string()))).to_ne_bytes())) > (0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u_string::unit_count(&(UnicodeScriptEvidenceClassifier::UNICODE_SCRIPT_EVIDENCE_CLASSIFIER_DATA_SHA256.to_string()))).to_ne_bytes())) > (0), None).unwrap();
        let _ = EastAsianSpacingCoverageTestHelpers::east_asian_spacing_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        UnicodeScriptEvidenceClassifier::unicode_script_evidence_classifier_classify(4294967295u32).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = EastAsianSpacingCoverageTestHelpers::east_asian_spacing_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        UnicodeScriptEvidenceClassifier::unicode_script_evidence_classifier_classify(1114112).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = EastAsianSpacingCoverageTestHelpers::east_asian_spacing_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        UnicodeScriptEvidenceClassifier::unicode_script_evidence_classifier_classify(55296).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = EastAsianSpacingCoverageTestHelpers::east_asian_spacing_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        UnicodeScriptEvidenceClassifier::unicode_script_evidence_classifier_classify(57343).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UnicodeScriptEvidence::EastAsian.name().to_string().as_str(), UnicodeScriptEvidenceClassifier::unicode_script_evidence_classifier_classify(19968).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UnicodeScriptEvidence::Other.name().to_string().as_str(), UnicodeScriptEvidenceClassifier::unicode_script_evidence_classifier_classify(65).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UnicodeScriptEvidence::Neutral.name().to_string().as_str(), UnicodeScriptEvidenceClassifier::unicode_script_evidence_classifier_classify(32).unwrap().name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn test_east_asian_spacing_data_and_values() {
    testlib::run("org.tiqian.core.EastAsianSpacingCoverageTest.testEastAsianSpacingDataAndValues", "org.tiqian.core.EastAsianSpacingCoverageTest.testEastAsianSpacingDataAndValues", || {
        TestTraceRecorder::new("EastAsianSpacingCoverageTest").section(&"testEastAsianSpacingDataAndValues");
        let values = vec![
    EastAsianSpacingValue::Wide,
    EastAsianSpacingValue::Narrow,
    EastAsianSpacingValue::Other,
    EastAsianSpacingValue::Conditional,
];
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let value = values[usize::try_from(index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { "-".to_string() } else { value.name().to_string() }.as_str(), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Wide.name().to_string().as_str(), EastAsianSpacingData::east_asian_spacing_data_lookup(711).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Narrow.name().to_string().as_str(), EastAsianSpacingData::east_asian_spacing_data_lookup(48).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Conditional.name().to_string().as_str(), EastAsianSpacingData::east_asian_spacing_data_lookup(33).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Other.name().to_string().as_str(), EastAsianSpacingData::east_asian_spacing_data_lookup(0).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Other.name().to_string().as_str(), EastAsianSpacingData::east_asian_spacing_data_lookup(1114111).unwrap().name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn test_east_asian_spacing_edges_model() {
    testlib::run("org.tiqian.core.EastAsianSpacingCoverageTest.testEastAsianSpacingEdgesModel", "org.tiqian.core.EastAsianSpacingCoverageTest.testEastAsianSpacingEdgesModel", || {
        TestTraceRecorder::new("EastAsianSpacingCoverageTest").section(&"testEastAsianSpacingEdgesModel");
        let edges = EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Narrow, true);
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Wide.name().to_string().as_str(), edges.leading.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Narrow.name().to_string().as_str(), edges.trailing.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(edges.contains_wide, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_east_asian_spacing_edges(EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Narrow, true), (edges).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(edges.leading == EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Narrow, true).leading, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&edges.to_string(), "EastAsianSpacingEdges", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[test]
fn test_unicode_east_asian_spacing() {
    testlib::run("org.tiqian.core.EastAsianSpacingCoverageTest.testUnicodeEastAsianSpacing", "org.tiqian.core.EastAsianSpacingCoverageTest.testUnicodeEastAsianSpacing", || {
        TestTraceRecorder::new("EastAsianSpacingCoverageTest").section(&"testUnicodeEastAsianSpacing");
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"draft-2024-12-16", UnicodeEastAsianSpacing::UNICODE_EAST_ASIAN_SPACING_DATA_REVISION.to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u_string::unit_count(&(UnicodeEastAsianSpacing::UNICODE_EAST_ASIAN_SPACING_DATA_SOURCE.to_string()))).to_ne_bytes())) > (0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u_string::unit_count(&(UnicodeEastAsianSpacing::UNICODE_EAST_ASIAN_SPACING_DATA_SHA256.to_string()))).to_ne_bytes())) > (0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"2026-06-14", UnicodeEastAsianSpacing::UNICODE_EAST_ASIAN_SPACING_LANGUAGE_REGISTRY_REVISION.to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u_string::unit_count(&(UnicodeEastAsianSpacing::UNICODE_EAST_ASIAN_SPACING_LANGUAGE_REGISTRY_SOURCE.to_string()))).to_ne_bytes())) > (0), None).unwrap();
        let chinese_locales = vec![
    "zh".to_string(),
    "zh-Hans".to_string(),
    "zh-Hant".to_string(),
    "zh-CN".to_string(),
    "zh_TW".to_string(),
    "cdo".to_string(),
    "cjy".to_string(),
    "cmn".to_string(),
    "cnp".to_string(),
    "cpx".to_string(),
    "csp".to_string(),
    "czh".to_string(),
    "czo".to_string(),
    "gan".to_string(),
    "hak".to_string(),
    "hnm".to_string(),
    "hsn".to_string(),
    "luh".to_string(),
    "lzh".to_string(),
    "mnp".to_string(),
    "nan".to_string(),
    "sjc".to_string(),
    "wuu".to_string(),
    "yue".to_string(),
    "yue-HK".to_string(),
    "cmn-Hans-CN".to_string(),
];
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((chinese_locales.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let locale = (chinese_locales[usize::try_from(index).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_true(UnicodeEastAsianSpacing::unicode_east_asian_spacing_is_chinese_language_context(locale.as_str()), Some((format!("{}{}{}",
            "Locale ",
            locale,
            " should be Chinese"
        )).to_string())).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let non_chinese_locales = vec![
    "en".to_string(),
    "en-US".to_string(),
    "ja".to_string(),
    "ko".to_string(),
    "fr".to_string(),
    "de".to_string(),
    "es".to_string(),
];
        index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((non_chinese_locales.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let locale = (non_chinese_locales[usize::try_from(index).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_false(UnicodeEastAsianSpacing::unicode_east_asian_spacing_is_chinese_language_context(locale.as_str()), Some((format!("{}{}{}",
            "Locale ",
            locale,
            " should not be Chinese"
        )).to_string())).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let _ = EastAsianSpacingCoverageTestHelpers::east_asian_spacing_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(4294967295u32).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = EastAsianSpacingCoverageTestHelpers::east_asian_spacing_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(1114112).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = EastAsianSpacingCoverageTestHelpers::east_asian_spacing_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(55296).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = EastAsianSpacingCoverageTestHelpers::east_asian_spacing_coverage_test_helpers_expect_argument_failure({  Arc::new(move || {
        UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(57343).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Other.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(&"", &"zh").unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Other.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(&"A⃝", &"zh").unwrap().name().to_string().as_str(),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Narrow.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(&"!", &"zh-CN").unwrap().name().to_string().as_str(),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Other.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(&"!", &"en-US").unwrap().name().to_string().as_str(),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Wide.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(&"中", &"zh").unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Narrow.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(&"A", &"zh").unwrap().name().to_string().as_str(),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Other.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(&" ", &"zh").unwrap().name().to_string().as_str(),
None).unwrap();
        let empty_edges = UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_edges(&"", &"zh").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Other.name().to_string().as_str(), empty_edges.leading.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Other.name().to_string().as_str(), empty_edges.trailing.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(empty_edges.contains_wide, None).unwrap();
        let mixed_edges = UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_edges(&"中a文", &"zh").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Wide.name().to_string().as_str(), mixed_edges.leading.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Wide.name().to_string().as_str(), mixed_edges.trailing.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(mixed_edges.contains_wide, None).unwrap();
        let western_edges = UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_edges(&"hello", &"en").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Narrow.name().to_string().as_str(), western_edges.leading.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Narrow.name().to_string().as_str(), western_edges.trailing.name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(western_edges.contains_wide, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Other.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(TestHelpers::test_helpers_surrogate_text(&vec![55357,
56832]).as_str(), &"zh").unwrap().name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn test_unicode_east_asian_spacing_lone_high_surrogates() {
    testlib::record_not_applicable("org.tiqian.core.EastAsianSpacingCoverageTest.testUnicodeEastAsianSpacingLoneHighSurrogates", "org.tiqian.core.EastAsianSpacingCoverageTest.testUnicodeEastAsianSpacingLoneHighSurrogates");
}

#[derive(Clone, Copy)]
pub struct EastAsianSpacingCoverageTestHelpers;

impl EastAsianSpacingCoverageTestHelpers {
    pub fn east_asian_spacing_coverage_test_helpers_expect_argument_failure(block: Arc<dyn Fn() -> Result<(), IllegalStateException> + Send + Sync + 'static>) -> Result<(), TracedAssertionsAssertFailsWithFault> {
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), (block).clone())?;
        Ok(())
    }
}
