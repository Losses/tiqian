#![cfg(test)]

use crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PunctuationSpacingRuleTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault) -> Self {
        match value {
            PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault) -> Self {
        match value {
            PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault) -> Self {
        match value {
            PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault) -> Self {
        match value {
            PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault) -> Self {
        match value {
            PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault) -> Self {
        match value {
            PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationSpacingRuleTestOpeningPlusOpeningCollapsesInnerToZeroFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault) -> Self {
        match value {
            PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault) -> Self {
        match value {
            PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault) -> Self {
        match value {
            PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationSpacingRuleTestNonAdjacentPunctuationAtomsAreNotCompressedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault) -> Self {
        match value {
            PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault) -> Self {
        match value {
            PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault) -> Self {
        match value {
            PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationSpacingRuleTestConsecutivePauseOrStopMarksCompressLikeAnyAdjacentPairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault) -> Self {
        match value {
            PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault) -> Self {
        match value {
            PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault) -> Self {
        match value {
            PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationSpacingRuleTestClosingPlusPauseOrStopStillCompressesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault) -> Self {
        match value {
            PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault) -> Self {
        match value {
            PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault) -> Self {
        match value {
            PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationSpacingRuleTestClosingPlusOpeningKeepsHalfEmGapFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault) -> Self {
        match value {
            PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault) -> Self {
        match value {
            PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault) -> Self {
        match value {
            PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationSpacingRuleTestClosingPlusClosingCollapsesInnerToZeroFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault) -> Self {
        match value {
            PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault) -> Self {
        match value {
            PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault) -> Self {
        match value {
            PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationSpacingRuleTestCjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMarkFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault) -> Self {
        match value {
            PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault) -> Self {
        match value {
            PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault) -> Self {
        match value {
            PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationSpacingRuleTestCjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlueFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn closing_plus_closing_collapses_inner_to_zero() {
    testlib::run("org.tiqian.layout.PunctuationSpacingRuleTest.closingPlusClosingCollapsesInnerToZero", "org.tiqian.layout.PunctuationSpacingRuleTest.closingPlusClosingCollapsesInnerToZero", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,83,112,97,99,105,110,103,82,117,108,101,84,101,115,116])));
        test_trace.section(UStr::new(&[99,108,111,115,105,110,103,80,108,117,115,67,108,111,115,105,110,103,67,111,108,108,97,112,115,101,115,73,110,110,101,114,84,111,90,101,114,111]));
        let a = ((*crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_COMPRESSOR).clone().compress(&vec![
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(UStr::new(&[12301]), 0).unwrap()).clone(),
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(UStr::new(&[12290]), 1).unwrap()).clone(),
], PunctuationSpacingRuleTestSupport::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM).unwrap().adjustments[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.natural_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, a.adjusted_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.reduction, None).unwrap();
    });
}

#[test]
fn opening_plus_opening_collapses_inner_to_zero() {
    testlib::run("org.tiqian.layout.PunctuationSpacingRuleTest.openingPlusOpeningCollapsesInnerToZero", "org.tiqian.layout.PunctuationSpacingRuleTest.openingPlusOpeningCollapsesInnerToZero", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,83,112,97,99,105,110,103,82,117,108,101,84,101,115,116])));
        test_trace.section(UStr::new(&[111,112,101,110,105,110,103,80,108,117,115,79,112,101,110,105,110,103,67,111,108,108,97,112,115,101,115,73,110,110,101,114,84,111,90,101,114,111]));
        let a = ((*crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_COMPRESSOR).clone().compress(&vec![
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(UStr::new(&[12300]), 0).unwrap()).clone(),
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(UStr::new(&[65288]), 1).unwrap()).clone(),
], PunctuationSpacingRuleTestSupport::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM).unwrap().adjustments[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.natural_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, a.adjusted_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.reduction, None).unwrap();
    });
}

#[test]
fn closing_plus_opening_keeps_half_em_gap() {
    testlib::run("org.tiqian.layout.PunctuationSpacingRuleTest.closingPlusOpeningKeepsHalfEmGap", "org.tiqian.layout.PunctuationSpacingRuleTest.closingPlusOpeningKeepsHalfEmGap", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,83,112,97,99,105,110,103,82,117,108,101,84,101,115,116])));
        test_trace.section(UStr::new(&[99,108,111,115,105,110,103,80,108,117,115,79,112,101,110,105,110,103,75,101,101,112,115,72,97,108,102,69,109,71,97,112]));
        let a = ((*crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_COMPRESSOR).clone().compress(&vec![
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(UStr::new(&[12290]), 0).unwrap()).clone(),
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(UStr::new(&[12300]), 1).unwrap()).clone(),
], PunctuationSpacingRuleTestSupport::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM).unwrap().adjustments[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, a.natural_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.adjusted_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.reduction, None).unwrap();
    });
}

#[test]
fn pause_stop_plus_opening_collapses_by_half_em() {
    testlib::run("org.tiqian.layout.PunctuationSpacingRuleTest.pauseStopPlusOpeningCollapsesByHalfEm", "org.tiqian.layout.PunctuationSpacingRuleTest.pauseStopPlusOpeningCollapsesByHalfEm", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,83,112,97,99,105,110,103,82,117,108,101,84,101,115,116])));
        test_trace.section(UStr::new(&[112,97,117,115,101,83,116,111,112,80,108,117,115,79,112,101,110,105,110,103,67,111,108,108,97,112,115,101,115,66,121,72,97,108,102,69,109]));
        let a = ((*crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_COMPRESSOR).clone().compress(&vec![
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(UStr::new(&[65292]), 0).unwrap()).clone(),
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(UStr::new(&[12300]), 1).unwrap()).clone(),
], PunctuationSpacingRuleTestSupport::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM).unwrap().adjustments[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, a.natural_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.adjusted_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.reduction, None).unwrap();
    });
}

#[test]
fn consecutive_pause_or_stop_marks_compress_like_any_adjacent_pair() {
    testlib::run("org.tiqian.layout.PunctuationSpacingRuleTest.consecutivePauseOrStopMarksCompressLikeAnyAdjacentPair", "org.tiqian.layout.PunctuationSpacingRuleTest.consecutivePauseOrStopMarksCompressLikeAnyAdjacentPair", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,83,112,97,99,105,110,103,82,117,108,101,84,101,115,116])));
        test_trace.section(UStr::new(&[99,111,110,115,101,99,117,116,105,118,101,80,97,117,115,101,79,114,83,116,111,112,77,97,114,107,115,67,111,109,112,114,101,115,115,76,105,107,101,65,110,121,65,100,106,97,99,101,110,116,80,97,105,114]));
        let a = ((*crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_COMPRESSOR).clone().compress(&vec![
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(UStr::new(&[65281]), 0).unwrap()).clone(),
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(UStr::new(&[65281]), 1).unwrap()).clone(),
], PunctuationSpacingRuleTestSupport::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM).unwrap().adjustments[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.natural_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, a.adjusted_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.reduction, None).unwrap();
    });
}

#[test]
fn closing_plus_pause_or_stop_still_compresses() {
    testlib::run("org.tiqian.layout.PunctuationSpacingRuleTest.closingPlusPauseOrStopStillCompresses", "org.tiqian.layout.PunctuationSpacingRuleTest.closingPlusPauseOrStopStillCompresses", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,83,112,97,99,105,110,103,82,117,108,101,84,101,115,116])));
        test_trace.section(UStr::new(&[99,108,111,115,105,110,103,80,108,117,115,80,97,117,115,101,79,114,83,116,111,112,83,116,105,108,108,67,111,109,112,114,101,115,115,101,115]));
        let a = ((*crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_COMPRESSOR).clone().compress(&vec![
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(UStr::new(&[8221]), 0).unwrap()).clone(),
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(UStr::new(&[65281]), 1).unwrap()).clone(),
], PunctuationSpacingRuleTestSupport::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM).unwrap().adjustments[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.natural_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, a.adjusted_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.reduction, None).unwrap();
    });
}

#[test]
fn non_adjacent_punctuation_atoms_are_not_compressed() {
    testlib::run("org.tiqian.layout.PunctuationSpacingRuleTest.nonAdjacentPunctuationAtomsAreNotCompressed", "org.tiqian.layout.PunctuationSpacingRuleTest.nonAdjacentPunctuationAtomsAreNotCompressed", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,83,112,97,99,105,110,103,82,117,108,101,84,101,115,116])));
        test_trace.section(UStr::new(&[110,111,110,65,100,106,97,99,101,110,116,80,117,110,99,116,117,97,116,105,111,110,65,116,111,109,115,65,114,101,78,111,116,67,111,109,112,114,101,115,115,101,100]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::try_from(((*crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_COMPRESSOR).clone().compress(&vec![
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(UStr::new(&[65292]), 0).unwrap()).clone(),
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(UStr::new(&[12290]), 5).unwrap()).clone(),
], PunctuationSpacingRuleTestSupport::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM).unwrap().adjustments.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn cjk_closing_before_ascii_point_mark_consumes_only_closing_glue() {
    testlib::run("org.tiqian.layout.PunctuationSpacingRuleTest.cjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlue", "org.tiqian.layout.PunctuationSpacingRuleTest.cjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlue", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,83,112,97,99,105,110,103,82,117,108,101,84,101,115,116])));
        test_trace.section(UStr::new(&[99,106,107,67,108,111,115,105,110,103,66,101,102,111,114,101,65,115,99,105,105,80,111,105,110,116,77,97,114,107,67,111,110,115,117,109,101,115,79,110,108,121,67,108,111,115,105,110,103,71,108,117,101]));
        let a = ((*crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_COMPRESSOR).clone().compress_cjk_closing_before_ascii_point_mark(&vec![
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(UStr::new(&[12301]), 0).unwrap()).clone(),
], UStr::new(&[12301,44]), PunctuationSpacingRuleTestSupport::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM).unwrap().adjustments[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.natural_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, a.adjusted_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.reduction, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,48,44,32,101,110,100,61,49,41]), UString::from(format!("{}", (a.reduction_target_range).clone().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[99,111,108,108,97,112,115,101,45,99,106,107,45,99,108,111,115,105,110,103,45,98,101,102,111,114,101,45,97,115,99,105,105,45,112,111,105,110,116,45,109,97,114,107]), (a.reason).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn cjk_closing_does_not_compress_across_whitespace_before_ascii_point_mark() {
    testlib::run("org.tiqian.layout.PunctuationSpacingRuleTest.cjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMark", "org.tiqian.layout.PunctuationSpacingRuleTest.cjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMark", || {
        let mut test_trace = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,83,112,97,99,105,110,103,82,117,108,101,84,101,115,116])));
        test_trace.section(UStr::new(&[99,106,107,67,108,111,115,105,110,103,68,111,101,115,78,111,116,67,111,109,112,114,101,115,115,65,99,114,111,115,115,87,104,105,116,101,115,112,97,99,101,66,101,102,111,114,101,65,115,99,105,105,80,111,105,110,116,77,97,114,107]));
        let plan = (*crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_COMPRESSOR).clone().compress_cjk_closing_before_ascii_point_mark(&vec![
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(UStr::new(&[12301]), 0).unwrap()).clone(),
], UStr::new(&[12301,32,44]), PunctuationSpacingRuleTestSupport::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,93]), UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = plan.adjustments;
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(), None).unwrap();
    });
}
