#![cfg(test)]

use crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PunctuationSpacingRuleTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationSpacingRuleTestPauseStopPlusOpeningCollapsesByHalfEmFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
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
        let mut test_trace = TestTraceRecorder::new("PunctuationSpacingRuleTest");
        test_trace.section(&"closingPlusClosingCollapsesInnerToZero");
        let a = ((*crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_COMPRESSOR).clone().compress(&vec![
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(&"」", 0).unwrap()).clone(),
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(&"。", 1).unwrap()).clone(),
], PunctuationSpacingRuleTestSupport::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM).unwrap().adjustments[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.natural_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, a.adjusted_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.reduction, None).unwrap();
    });
}

#[test]
fn opening_plus_opening_collapses_inner_to_zero() {
    testlib::run("org.tiqian.layout.PunctuationSpacingRuleTest.openingPlusOpeningCollapsesInnerToZero", "org.tiqian.layout.PunctuationSpacingRuleTest.openingPlusOpeningCollapsesInnerToZero", || {
        let mut test_trace = TestTraceRecorder::new("PunctuationSpacingRuleTest");
        test_trace.section(&"openingPlusOpeningCollapsesInnerToZero");
        let a = ((*crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_COMPRESSOR).clone().compress(&vec![
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(&"「", 0).unwrap()).clone(),
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(&"（", 1).unwrap()).clone(),
], PunctuationSpacingRuleTestSupport::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM).unwrap().adjustments[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.natural_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, a.adjusted_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.reduction, None).unwrap();
    });
}

#[test]
fn closing_plus_opening_keeps_half_em_gap() {
    testlib::run("org.tiqian.layout.PunctuationSpacingRuleTest.closingPlusOpeningKeepsHalfEmGap", "org.tiqian.layout.PunctuationSpacingRuleTest.closingPlusOpeningKeepsHalfEmGap", || {
        let mut test_trace = TestTraceRecorder::new("PunctuationSpacingRuleTest");
        test_trace.section(&"closingPlusOpeningKeepsHalfEmGap");
        let a = ((*crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_COMPRESSOR).clone().compress(&vec![
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(&"。", 0).unwrap()).clone(),
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(&"「", 1).unwrap()).clone(),
], PunctuationSpacingRuleTestSupport::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM).unwrap().adjustments[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, a.natural_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.adjusted_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.reduction, None).unwrap();
    });
}

#[test]
fn pause_stop_plus_opening_collapses_by_half_em() {
    testlib::run("org.tiqian.layout.PunctuationSpacingRuleTest.pauseStopPlusOpeningCollapsesByHalfEm", "org.tiqian.layout.PunctuationSpacingRuleTest.pauseStopPlusOpeningCollapsesByHalfEm", || {
        let mut test_trace = TestTraceRecorder::new("PunctuationSpacingRuleTest");
        test_trace.section(&"pauseStopPlusOpeningCollapsesByHalfEm");
        let a = ((*crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_COMPRESSOR).clone().compress(&vec![
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(&"，", 0).unwrap()).clone(),
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(&"「", 1).unwrap()).clone(),
], PunctuationSpacingRuleTestSupport::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM).unwrap().adjustments[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16 as f64, a.natural_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.adjusted_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.reduction, None).unwrap();
    });
}

#[test]
fn consecutive_pause_or_stop_marks_compress_like_any_adjacent_pair() {
    testlib::run("org.tiqian.layout.PunctuationSpacingRuleTest.consecutivePauseOrStopMarksCompressLikeAnyAdjacentPair", "org.tiqian.layout.PunctuationSpacingRuleTest.consecutivePauseOrStopMarksCompressLikeAnyAdjacentPair", || {
        let mut test_trace = TestTraceRecorder::new("PunctuationSpacingRuleTest");
        test_trace.section(&"consecutivePauseOrStopMarksCompressLikeAnyAdjacentPair");
        let a = ((*crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_COMPRESSOR).clone().compress(&vec![
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(&"！", 0).unwrap()).clone(),
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(&"！", 1).unwrap()).clone(),
], PunctuationSpacingRuleTestSupport::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM).unwrap().adjustments[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.natural_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, a.adjusted_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.reduction, None).unwrap();
    });
}

#[test]
fn closing_plus_pause_or_stop_still_compresses() {
    testlib::run("org.tiqian.layout.PunctuationSpacingRuleTest.closingPlusPauseOrStopStillCompresses", "org.tiqian.layout.PunctuationSpacingRuleTest.closingPlusPauseOrStopStillCompresses", || {
        let mut test_trace = TestTraceRecorder::new("PunctuationSpacingRuleTest");
        test_trace.section(&"closingPlusPauseOrStopStillCompresses");
        let a = ((*crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_COMPRESSOR).clone().compress(&vec![
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(&"”", 0).unwrap()).clone(),
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(&"！", 1).unwrap()).clone(),
], PunctuationSpacingRuleTestSupport::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM).unwrap().adjustments[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.natural_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, a.adjusted_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.reduction, None).unwrap();
    });
}

#[test]
fn non_adjacent_punctuation_atoms_are_not_compressed() {
    testlib::run("org.tiqian.layout.PunctuationSpacingRuleTest.nonAdjacentPunctuationAtomsAreNotCompressed", "org.tiqian.layout.PunctuationSpacingRuleTest.nonAdjacentPunctuationAtomsAreNotCompressed", || {
        let mut test_trace = TestTraceRecorder::new("PunctuationSpacingRuleTest");
        test_trace.section(&"nonAdjacentPunctuationAtomsAreNotCompressed");
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::try_from(((*crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_COMPRESSOR).clone().compress(&vec![
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(&"，", 0).unwrap()).clone(),
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(&"。", 5).unwrap()).clone(),
], PunctuationSpacingRuleTestSupport::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM).unwrap().adjustments.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn cjk_closing_before_ascii_point_mark_consumes_only_closing_glue() {
    testlib::run("org.tiqian.layout.PunctuationSpacingRuleTest.cjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlue", "org.tiqian.layout.PunctuationSpacingRuleTest.cjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlue", || {
        let mut test_trace = TestTraceRecorder::new("PunctuationSpacingRuleTest");
        test_trace.section(&"cjkClosingBeforeAsciiPointMarkConsumesOnlyClosingGlue");
        let a = ((*crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_COMPRESSOR).clone().compress_cjk_closing_before_ascii_point_mark(&vec![
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(&"」", 0).unwrap()).clone(),
], &"」,", PunctuationSpacingRuleTestSupport::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM).unwrap().adjustments[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.natural_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, a.adjusted_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, a.reduction, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"TextRange(start=0, end=1)", (a.reduction_target_range).clone().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"collapse-cjk-closing-before-ascii-point-mark", (a.reason).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn cjk_closing_does_not_compress_across_whitespace_before_ascii_point_mark() {
    testlib::run("org.tiqian.layout.PunctuationSpacingRuleTest.cjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMark", "org.tiqian.layout.PunctuationSpacingRuleTest.cjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMark", || {
        let mut test_trace = TestTraceRecorder::new("PunctuationSpacingRuleTest");
        test_trace.section(&"cjkClosingDoesNotCompressAcrossWhitespaceBeforeAsciiPointMark");
        let plan = (*crate::org::tiqian::layout::punctuation_spacing_rule_test_support::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_COMPRESSOR).clone().compress_cjk_closing_before_ascii_point_mark(&vec![
    (PunctuationSpacingRuleTestSupport::punctuation_spacing_rule_test_support_atom(&"」", 0).unwrap()).clone(),
], &"」 ,", PunctuationSpacingRuleTestSupport::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[]", {
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
    }.as_str(), None).unwrap();
    });
}
