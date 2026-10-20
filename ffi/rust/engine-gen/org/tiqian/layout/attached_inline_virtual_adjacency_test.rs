#![cfg(test)]

use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::layout::attached_inline_virtual_adjacency_test_support::AttachedInlineVirtualAdjacencyTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportLayoutAttachedReferenceFault(crate::org::tiqian::layout::attached_inline_virtual_adjacency_test_support::AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault::SupportLayoutAttachedReferenceFault(value) => write!(formatter, "{}", value),
            AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault> for crate::org::tiqian::layout::attached_inline_virtual_adjacency_test_support::AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault {
    fn from(value: AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault::SupportLayoutAttachedReferenceFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::attached_inline_virtual_adjacency_test_support::AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault> for AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault {
    fn from(value: crate::org::tiqian::layout::attached_inline_virtual_adjacency_test_support::AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault) -> Self {
        AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault::SupportLayoutAttachedReferenceFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportLayoutAttachedReferenceFault(crate::org::tiqian::layout::attached_inline_virtual_adjacency_test_support::AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault::SupportLayoutAttachedReferenceFault(value) => write!(formatter, "{}", value),
            AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault> for crate::org::tiqian::layout::attached_inline_virtual_adjacency_test_support::AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault {
    fn from(value: AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault::SupportLayoutAttachedReferenceFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::attached_inline_virtual_adjacency_test_support::AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault> for AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault {
    fn from(value: crate::org::tiqian::layout::attached_inline_virtual_adjacency_test_support::AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault) -> Self {
        AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault::SupportLayoutAttachedReferenceFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlueFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportLayoutAttachedReferenceFault(crate::org::tiqian::layout::attached_inline_virtual_adjacency_test_support::AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault::SupportLayoutAttachedReferenceFault(value) => write!(formatter, "{}", value),
            AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault> for crate::org::tiqian::layout::attached_inline_virtual_adjacency_test_support::AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault {
    fn from(value: AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault::SupportLayoutAttachedReferenceFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::attached_inline_virtual_adjacency_test_support::AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault> for AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault {
    fn from(value: crate::org::tiqian::layout::attached_inline_virtual_adjacency_test_support::AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault) -> Self {
        AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault::SupportLayoutAttachedReferenceFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AttachedInlineVirtualAdjacencyTestClosingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlueFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportLayoutWithBreakerFault(crate::org::tiqian::layout::attached_inline_virtual_adjacency_test_support::AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault::SupportLayoutWithBreakerFault(value) => write!(formatter, "{}", value),
            AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault> for crate::org::tiqian::layout::attached_inline_virtual_adjacency_test_support::AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault {
    fn from(value: AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault::SupportLayoutWithBreakerFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault) -> Self {
        match value {
            AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::attached_inline_virtual_adjacency_test_support::AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault> for AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault {
    fn from(value: crate::org::tiqian::layout::attached_inline_virtual_adjacency_test_support::AttachedInlineVirtualAdjacencyTestSupportLayoutWithBreakerFault) -> Self {
        AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault::SupportLayoutWithBreakerFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        AttachedInlineVirtualAdjacencyTestAttachedReferenceNeverStartsAWrappedLineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn attached_run_exposes_the_prose_clusters_on_its_two_sides() {
    testlib::run("org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.attachedRunExposesTheProseClustersOnItsTwoSides", "org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.attachedRunExposesTheProseClustersOnItsTwoSides", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,116,116,97,99,104,101,100,73,110,108,105,110,101,86,105,114,116,117,97,108,65,100,106,97,99,101,110,99,121,84,101,115,116])));
        t.section(UStr::new(&[97,116,116,97,99,104,101,100,82,117,110,69,120,112,111,115,101,115,84,104,101,80,114,111,115,101,67,108,117,115,116,101,114,115,79,110,73,116,115,84,119,111,83,105,100,101,115]));
        let result = AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_resolve(&vec![
    InlineAttachment::None,
    InlineAttachment::None,
    InlineAttachment::Previous,
    InlineAttachment::Previous,
    InlineAttachment::Previous,
    InlineAttachment::None,
]);
        let _ = TracedAssertions::traced_assertions_assert_equals(1, result[0usize].previous_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(2u32, 4u32), ((result[0usize]).clone().attached_cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(5, *(result[0usize].next_cluster_index).as_ref().unwrap(), None).unwrap();
    });
}

#[test]
fn attached_run_at_paragraph_end_has_no_virtual_right_neighbor() {
    testlib::run("org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.attachedRunAtParagraphEndHasNoVirtualRightNeighbor", "org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.attachedRunAtParagraphEndHasNoVirtualRightNeighbor", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,116,116,97,99,104,101,100,73,110,108,105,110,101,86,105,114,116,117,97,108,65,100,106,97,99,101,110,99,121,84,101,115,116])));
        t.section(UStr::new(&[97,116,116,97,99,104,101,100,82,117,110,65,116,80,97,114,97,103,114,97,112,104,69,110,100,72,97,115,78,111,86,105,114,116,117,97,108,82,105,103,104,116,78,101,105,103,104,98,111,114]));
        let result = AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_resolve(&vec![
    InlineAttachment::None,
    InlineAttachment::None,
    InlineAttachment::Previous,
    InlineAttachment::Previous,
    InlineAttachment::Previous,
]);
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(result[0usize].next_cluster_index.is_none(), if result[0usize].next_cluster_index.is_none() { UString::from("-") } else { UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("")); __s += &(match result[0usize].next_cluster_index { Some(v) => UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(v)).as_str()), None => UString::from("null") }); __s }).as_str()) }.as_ustr(), None).unwrap();
    });
}

#[test]
fn punctuation_after_footnote_is_judged_against_the_preceding_punctuation() {
    testlib::run("org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.punctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuation", "org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.punctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuation", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,116,116,97,99,104,101,100,73,110,108,105,110,101,86,105,114,116,117,97,108,65,100,106,97,99,101,110,99,121,84,101,115,116])));
        t.section(UStr::new(&[112,117,110,99,116,117,97,116,105,111,110,65,102,116,101,114,70,111,111,116,110,111,116,101,73,115,74,117,100,103,101,100,65,103,97,105,110,115,116,84,104,101,80,114,101,99,101,100,105,110,103,80,117,110,99,116,117,97,116,105,111,110]));
        let result = AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_layout_attached_reference(UStr::new(&[27491,25991,65306,8220,20869,23481,12290,8221,91,49,93,65292,21518,25991])).unwrap();
        let virtual_boundary = AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_virtual_boundary((result).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65,116,116,97,99,104,101,100,73,110,108,105,110,101,86,105,114,116,117,97,108,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,58,97,100,106,97,99,101,110,116,45,112,117,110,99,116,117,97,116,105,111,110]), (virtual_boundary.reason).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((virtual_boundary.natural_inner_glue) > (0 as f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, virtual_boundary.adjusted_inner_glue, None).unwrap();
    });
}

#[test]
fn closing_quote_before_footnote_and_body_keeps_its_natural_trailing_glue() {
    testlib::run("org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.closingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlue", "org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.closingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlue", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,116,116,97,99,104,101,100,73,110,108,105,110,101,86,105,114,116,117,97,108,65,100,106,97,99,101,110,99,121,84,101,115,116])));
        t.section(UStr::new(&[99,108,111,115,105,110,103,81,117,111,116,101,66,101,102,111,114,101,70,111,111,116,110,111,116,101,65,110,100,66,111,100,121,75,101,101,112,115,73,116,115,78,97,116,117,114,97,108,84,114,97,105,108,105,110,103,71,108,117,101]));
        let result = AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_layout_attached_reference(UStr::new(&[27491,25991,65306,8220,20869,23481,12290,8221,91,49,93,21518,25991])).unwrap();
        let virtual_boundary = AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_virtual_boundary((result).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65,116,116,97,99,104,101,100,73,110,108,105,110,101,86,105,114,116,117,97,108,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,58,110,97,116,117,114,97,108]), (virtual_boundary.reason).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(virtual_boundary.natural_inner_glue, virtual_boundary.adjusted_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((virtual_boundary.adjusted_inner_glue) > (0 as f64), None).unwrap();
    });
}

#[test]
fn closing_quote_before_paragraph_end_footnote_has_no_trailing_glue() {
    testlib::run("org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.closingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlue", "org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.closingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlue", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,116,116,97,99,104,101,100,73,110,108,105,110,101,86,105,114,116,117,97,108,65,100,106,97,99,101,110,99,121,84,101,115,116])));
        t.section(UStr::new(&[99,108,111,115,105,110,103,81,117,111,116,101,66,101,102,111,114,101,80,97,114,97,103,114,97,112,104,69,110,100,70,111,111,116,110,111,116,101,72,97,115,78,111,84,114,97,105,108,105,110,103,71,108,117,101]));
        let result = AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_layout_attached_reference(UStr::new(&[27491,25991,65306,8220,20869,23481,12290,8221,91,49,93])).unwrap();
        let virtual_boundary = AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_virtual_boundary((result).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[65,116,116,97,99,104,101,100,73,110,108,105,110,101,86,105,114,116,117,97,108,80,117,110,99,116,117,97,116,105,111,110,66,111,117,110,100,97,114,121,58,108,105,110,101,45,101,110,100]), (virtual_boundary.reason).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, virtual_boundary.adjusted_inner_glue, None).unwrap();
    });
}

#[test]
fn attached_reference_never_starts_a_wrapped_line() {
    testlib::run("org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.attachedReferenceNeverStartsAWrappedLine", "org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.attachedReferenceNeverStartsAWrappedLine", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[65,116,116,97,99,104,101,100,73,110,108,105,110,101,86,105,114,116,117,97,108,65,100,106,97,99,101,110,99,121,84,101,115,116])));
        t.section(UStr::new(&[97,116,116,97,99,104,101,100,82,101,102,101,114,101,110,99,101,78,101,118,101,114,83,116,97,114,116,115,65,87,114,97,112,112,101,100,76,105,110,101]));
        let text = UString::from("甲乙1丙").to_ustring();
        let reference_range = TextRange::new(2u32, 3u32).unwrap();
        let breakers = AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_breakers().unwrap();
        for choice in &breakers {
            let result = AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_layout_with_breaker(text.as_ustr(), (choice.breaker).clone()).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (1), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (choice.breaker).clone().get_strategy_name().as_ustr(); __s += &(UString::from(": test must wrap: ")); __s += AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_render_lines(&result.lines).as_ustr(); __s }).as_str()))).unwrap();
            let mut started = false;
            for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let s = ((result.lines[usize::try_from(i).unwrap_or(0)]).clone().range).clone().start;
                if i32::from_ne_bytes(((s) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((reference_range.start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((s) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((reference_range.end) as i32).to_ne_bytes())) {
                    started = true;
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(!started, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (choice.breaker).clone().get_strategy_name().as_ustr(); __s += &(UString::from(": attached reference started a line: ")); __s += AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_render_ranges(&result.lines).as_ustr(); __s }).as_str()))).unwrap();
            let mut attached = false;
            for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let line = (result.lines[usize::try_from(i).unwrap_or(0)]).clone();
                if i32::from_ne_bytes((((line.range).clone().start) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((reference_range.start) as i32).to_ne_bytes())) && (i32::from_ne_bytes((((line.range).clone().end) as i32).to_ne_bytes())) >= i32::from_ne_bytes(((reference_range.end) as i32).to_ne_bytes()) {
                    attached = true;
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(attached, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += (choice.breaker).clone().get_strategy_name().as_ustr(); __s += &(UString::from(": reference detached from prose: ")); __s += AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_render_ranges(&result.lines).as_ustr(); __s }).as_str()))).unwrap();
        }
    });
}
