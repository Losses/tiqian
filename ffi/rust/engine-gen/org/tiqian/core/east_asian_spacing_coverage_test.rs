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
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}
impl std::fmt::Display for EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingCoverageTestTestUnicodeWordCharacterFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingCoverageTestTestUnicodeScriptEvidenceFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingLoneHighSurrogatesFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingCoverageTestTestUnicodeEastAsianSpacingFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingCoverageTestTestEastAsianSpacingDataAndValuesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        TestTraceRecorder::new(&(UStr::new(&[69,97,115,116,65,115,105,97,110,83,112,97,99,105,110,103,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,85,110,105,99,111,100,101,87,111,114,100,67,104,97,114,97,99,116,101,114]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[49,55,46,48,46,48]), UnicodeWordCharacter::UNICODE_WORD_CHARACTER_DATA_REVISION.to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(UnicodeWordCharacter::UNICODE_WORD_CHARACTER_DATA_SOURCE.to_ustring()))) as i32).to_ne_bytes())) > (0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(UnicodeWordCharacter::UNICODE_WORD_CHARACTER_DATA_SHA256.to_ustring()))) as i32).to_ne_bytes())) > (0), None).unwrap();
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
        TestTraceRecorder::new(&(UStr::new(&[69,97,115,116,65,115,105,97,110,83,112,97,99,105,110,103,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,85,110,105,99,111,100,101,83,99,114,105,112,116,69,118,105,100,101,110,99,101]));
        let values = vec![UnicodeScriptEvidence::Neutral, UnicodeScriptEvidence::EastAsian, UnicodeScriptEvidence::Other];
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let value = values[usize::try_from(index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(value.name()) }.as_ustr(), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[49,55,46,48,46,48]), UnicodeScriptEvidenceClassifier::UNICODE_SCRIPT_EVIDENCE_CLASSIFIER_DATA_REVISION.to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(UnicodeScriptEvidenceClassifier::UNICODE_SCRIPT_EVIDENCE_CLASSIFIER_DATA_SOURCE.to_ustring()))) as i32).to_ne_bytes())) > (0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(UnicodeScriptEvidenceClassifier::UNICODE_SCRIPT_EVIDENCE_CLASSIFIER_DATA_SHA256.to_ustring()))) as i32).to_ne_bytes())) > (0), None).unwrap();
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
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(UnicodeScriptEvidence::EastAsian.name()).as_ustr(), UString::from(UnicodeScriptEvidenceClassifier::unicode_script_evidence_classifier_classify(19968).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(UnicodeScriptEvidence::Other.name()).as_ustr(), UString::from(UnicodeScriptEvidenceClassifier::unicode_script_evidence_classifier_classify(65).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(UnicodeScriptEvidence::Neutral.name()).as_ustr(), UString::from(UnicodeScriptEvidenceClassifier::unicode_script_evidence_classifier_classify(32).unwrap().name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn test_east_asian_spacing_data_and_values() {
    testlib::run("org.tiqian.core.EastAsianSpacingCoverageTest.testEastAsianSpacingDataAndValues", "org.tiqian.core.EastAsianSpacingCoverageTest.testEastAsianSpacingDataAndValues", || {
        TestTraceRecorder::new(&(UStr::new(&[69,97,115,116,65,115,105,97,110,83,112,97,99,105,110,103,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,69,97,115,116,65,115,105,97,110,83,112,97,99,105,110,103,68,97,116,97,65,110,100,86,97,108,117,101,115]));
        let values = vec![
    EastAsianSpacingValue::Wide,
    EastAsianSpacingValue::Narrow,
    EastAsianSpacingValue::Other,
    EastAsianSpacingValue::Conditional,
];
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let value = values[usize::try_from(index).unwrap_or(0)];
            let _ = TracedAssertions::traced_assertions_record_rendered_not_null(if false { UString::from("-") } else { UString::from(value.name()) }.as_ustr(), None).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Wide.name()).as_ustr(), UString::from(EastAsianSpacingData::east_asian_spacing_data_lookup(711).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Narrow.name()).as_ustr(), UString::from(EastAsianSpacingData::east_asian_spacing_data_lookup(48).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Conditional.name()).as_ustr(), UString::from(EastAsianSpacingData::east_asian_spacing_data_lookup(33).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Other.name()).as_ustr(), UString::from(EastAsianSpacingData::east_asian_spacing_data_lookup(0).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Other.name()).as_ustr(), UString::from(EastAsianSpacingData::east_asian_spacing_data_lookup(1114111).unwrap().name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn test_east_asian_spacing_edges_model() {
    testlib::run("org.tiqian.core.EastAsianSpacingCoverageTest.testEastAsianSpacingEdgesModel", "org.tiqian.core.EastAsianSpacingCoverageTest.testEastAsianSpacingEdgesModel", || {
        TestTraceRecorder::new(&(UStr::new(&[69,97,115,116,65,115,105,97,110,83,112,97,99,105,110,103,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,69,97,115,116,65,115,105,97,110,83,112,97,99,105,110,103,69,100,103,101,115,77,111,100,101,108]));
        let edges = EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Narrow, true);
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Wide.name()).as_ustr(), UString::from(edges.leading.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Narrow.name()).as_ustr(), UString::from(edges.trailing.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(edges.contains_wide, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_east_asian_spacing_edges(EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Narrow, true), (edges).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(edges.leading == EastAsianSpacingEdges::new(EastAsianSpacingValue::Wide, EastAsianSpacingValue::Narrow, true).leading, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&UString::from(format!("{}", edges.to_string()).as_str()), UString::from("EastAsianSpacingEdges").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[test]
fn test_unicode_east_asian_spacing() {
    testlib::run("org.tiqian.core.EastAsianSpacingCoverageTest.testUnicodeEastAsianSpacing", "org.tiqian.core.EastAsianSpacingCoverageTest.testUnicodeEastAsianSpacing", || {
        TestTraceRecorder::new(&(UStr::new(&[69,97,115,116,65,115,105,97,110,83,112,97,99,105,110,103,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[116,101,115,116,85,110,105,99,111,100,101,69,97,115,116,65,115,105,97,110,83,112,97,99,105,110,103]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[100,114,97,102,116,45,50,48,50,52,45,49,50,45,49,54]), UnicodeEastAsianSpacing::UNICODE_EAST_ASIAN_SPACING_DATA_REVISION.to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(UnicodeEastAsianSpacing::UNICODE_EAST_ASIAN_SPACING_DATA_SOURCE.to_ustring()))) as i32).to_ne_bytes())) > (0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(UnicodeEastAsianSpacing::UNICODE_EAST_ASIAN_SPACING_DATA_SHA256.to_ustring()))) as i32).to_ne_bytes())) > (0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[50,48,50,54,45,48,54,45,49,52]), UnicodeEastAsianSpacing::UNICODE_EAST_ASIAN_SPACING_LANGUAGE_REGISTRY_REVISION.to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u_string::unit_count(&(UnicodeEastAsianSpacing::UNICODE_EAST_ASIAN_SPACING_LANGUAGE_REGISTRY_SOURCE.to_ustring()))) as i32).to_ne_bytes())) > (0), None).unwrap();
        let chinese_locales = vec![
    UString::from("zh").to_ustring(),
    UString::from("zh-Hans").to_ustring(),
    UString::from("zh-Hant").to_ustring(),
    UString::from("zh-CN").to_ustring(),
    UString::from("zh_TW").to_ustring(),
    UString::from("cdo").to_ustring(),
    UString::from("cjy").to_ustring(),
    UString::from("cmn").to_ustring(),
    UString::from("cnp").to_ustring(),
    UString::from("cpx").to_ustring(),
    UString::from("csp").to_ustring(),
    UString::from("czh").to_ustring(),
    UString::from("czo").to_ustring(),
    UString::from("gan").to_ustring(),
    UString::from("hak").to_ustring(),
    UString::from("hnm").to_ustring(),
    UString::from("hsn").to_ustring(),
    UString::from("luh").to_ustring(),
    UString::from("lzh").to_ustring(),
    UString::from("mnp").to_ustring(),
    UString::from("nan").to_ustring(),
    UString::from("sjc").to_ustring(),
    UString::from("wuu").to_ustring(),
    UString::from("yue").to_ustring(),
    UString::from("yue-HK").to_ustring(),
    UString::from("cmn-Hans-CN").to_ustring(),
];
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((chinese_locales.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let locale = (chinese_locales[usize::try_from(index).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_true(UnicodeEastAsianSpacing::unicode_east_asian_spacing_is_chinese_language_context(locale.as_ustr()), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Locale ")); __s += locale.as_ustr(); __s += &(UString::from(" should be Chinese")); __s }).as_str()))).unwrap();
            index = u32::wrapping_add(index, 1);
        }
        let non_chinese_locales = vec![
    UString::from("en").to_ustring(),
    UString::from("en-US").to_ustring(),
    UString::from("ja").to_ustring(),
    UString::from("ko").to_ustring(),
    UString::from("fr").to_ustring(),
    UString::from("de").to_ustring(),
    UString::from("es").to_ustring(),
];
        index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((non_chinese_locales.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let locale = (non_chinese_locales[usize::try_from(index).unwrap_or(0)]).clone();
            let _ = TracedAssertions::traced_assertions_assert_false(UnicodeEastAsianSpacing::unicode_east_asian_spacing_is_chinese_language_context(locale.as_ustr()), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Locale ")); __s += locale.as_ustr(); __s += &(UString::from(" should not be Chinese")); __s }).as_str()))).unwrap();
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
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Other.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(UStr::new(&[]), UStr::new(&[122,104])).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Other.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(UStr::new(&[65,8413]), UStr::new(&[122,104])).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Narrow.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(UStr::new(&[33]), UStr::new(&[122,104,45,67,78])).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Other.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(UStr::new(&[33]), UStr::new(&[101,110,45,85,83])).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Wide.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(UStr::new(&[20013]), UStr::new(&[122,104])).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Narrow.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(UStr::new(&[65]), UStr::new(&[122,104])).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Other.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(UStr::new(&[0]), UStr::new(&[122,104])).unwrap().name()).as_ustr(), None).unwrap();
        let empty_edges = UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_edges(UStr::new(&[]), UStr::new(&[122,104])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Other.name()).as_ustr(), UString::from(empty_edges.leading.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Other.name()).as_ustr(), UString::from(empty_edges.trailing.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(empty_edges.contains_wide, None).unwrap();
        let mixed_edges = UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_edges(UStr::new(&[20013,97,25991]), UStr::new(&[122,104])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Wide.name()).as_ustr(), UString::from(mixed_edges.leading.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Wide.name()).as_ustr(), UString::from(mixed_edges.trailing.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(mixed_edges.contains_wide, None).unwrap();
        let western_edges = UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_edges(UStr::new(&[104,101,108,108,111]), UStr::new(&[101,110])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Narrow.name()).as_ustr(), UString::from(western_edges.leading.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Narrow.name()).as_ustr(), UString::from(western_edges.trailing.name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(western_edges.contains_wide, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Other.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832]).as_ustr(), UStr::new(&[122,104])).unwrap().name()).as_ustr(), None).unwrap();
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
