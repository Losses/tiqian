#![cfg(test)]

use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::layout::attached_inline_virtual_adjacency_test_support::AttachedInlineVirtualAdjacencyTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum AttachedInlineVirtualAdjacencyTestPunctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuationFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportLayoutAttachedReferenceFault(crate::org::tiqian::layout::attached_inline_virtual_adjacency_test_support::AttachedInlineVirtualAdjacencyTestSupportLayoutAttachedReferenceFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
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
        let mut t = TestTraceRecorder::new("AttachedInlineVirtualAdjacencyTest");
        t.section(&"attachedRunExposesTheProseClustersOnItsTwoSides");
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
        let mut t = TestTraceRecorder::new("AttachedInlineVirtualAdjacencyTest");
        t.section(&"attachedRunAtParagraphEndHasNoVirtualRightNeighbor");
        let result = AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_resolve(&vec![
    InlineAttachment::None,
    InlineAttachment::None,
    InlineAttachment::Previous,
    InlineAttachment::Previous,
    InlineAttachment::Previous,
]);
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(result[0usize].next_cluster_index.is_none(), if result[0usize].next_cluster_index.is_none() { "-".to_string() } else { format!("{}{}",
            "",
            match result[0usize].next_cluster_index { Some(v) => crate::runtime::int_text::IntText::int_text(v), None => "null".to_string() }
        ).to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn punctuation_after_footnote_is_judged_against_the_preceding_punctuation() {
    testlib::run("org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.punctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuation", "org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.punctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuation", || {
        let mut t = TestTraceRecorder::new("AttachedInlineVirtualAdjacencyTest");
        t.section(&"punctuationAfterFootnoteIsJudgedAgainstThePrecedingPunctuation");
        let result = AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_layout_attached_reference(&"正文：“内容。”[1]，后文").unwrap();
        let virtual_boundary = AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_virtual_boundary((result).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"AttachedInlineVirtualPunctuationBoundary:adjacent-punctuation", (virtual_boundary.reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((virtual_boundary.natural_inner_glue) > (0 as f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, virtual_boundary.adjusted_inner_glue, None).unwrap();
    });
}

#[test]
fn closing_quote_before_footnote_and_body_keeps_its_natural_trailing_glue() {
    testlib::run("org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.closingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlue", "org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.closingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlue", || {
        let mut t = TestTraceRecorder::new("AttachedInlineVirtualAdjacencyTest");
        t.section(&"closingQuoteBeforeFootnoteAndBodyKeepsItsNaturalTrailingGlue");
        let result = AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_layout_attached_reference(&"正文：“内容。”[1]后文").unwrap();
        let virtual_boundary = AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_virtual_boundary((result).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"AttachedInlineVirtualPunctuationBoundary:natural", (virtual_boundary.reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(virtual_boundary.natural_inner_glue, virtual_boundary.adjusted_inner_glue, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((virtual_boundary.adjusted_inner_glue) > (0 as f64), None).unwrap();
    });
}

#[test]
fn closing_quote_before_paragraph_end_footnote_has_no_trailing_glue() {
    testlib::run("org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.closingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlue", "org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.closingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlue", || {
        let mut t = TestTraceRecorder::new("AttachedInlineVirtualAdjacencyTest");
        t.section(&"closingQuoteBeforeParagraphEndFootnoteHasNoTrailingGlue");
        let result = AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_layout_attached_reference(&"正文：“内容。”[1]").unwrap();
        let virtual_boundary = AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_virtual_boundary((result).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"AttachedInlineVirtualPunctuationBoundary:line-end", (virtual_boundary.reason).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, virtual_boundary.adjusted_inner_glue, None).unwrap();
    });
}

#[test]
fn attached_reference_never_starts_a_wrapped_line() {
    testlib::run("org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.attachedReferenceNeverStartsAWrappedLine", "org.tiqian.layout.AttachedInlineVirtualAdjacencyTest.attachedReferenceNeverStartsAWrappedLine", || {
        let mut t = TestTraceRecorder::new("AttachedInlineVirtualAdjacencyTest");
        t.section(&"attachedReferenceNeverStartsAWrappedLine");
        let text = "甲乙1丙".to_string();
        let reference_range = TextRange::new(2u32, 3u32).unwrap();
        let breakers = AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_breakers().unwrap();
        for choice in &breakers {
            let result = AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_layout_with_breaker(text.as_str(), (choice.breaker).clone()).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (1), Some((format!("{}{}{}",
            (choice.breaker).clone().get_strategy_name(),
            ": test must wrap: ",
            AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_render_lines(&result.lines)
        )).to_string())).unwrap();
            let mut started = false;
            for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let s = ((result.lines[usize::try_from(i).unwrap_or(0)]).clone().range).clone().start;
                if i32::from_ne_bytes((s).to_ne_bytes()) >= i32::from_ne_bytes((reference_range.start).to_ne_bytes()) && (i32::from_ne_bytes((s).to_ne_bytes())) < (i32::from_ne_bytes((reference_range.end).to_ne_bytes())) {
                    started = true;
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(!started, Some((format!("{}{}{}",
            (choice.breaker).clone().get_strategy_name(),
            ": attached reference started a line: ",
            AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_render_ranges(&result.lines)
        )).to_string())).unwrap();
            let mut attached = false;
            for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let line = (result.lines[usize::try_from(i).unwrap_or(0)]).clone();
                if i32::from_ne_bytes(((line.range).clone().start).to_ne_bytes()) < (i32::from_ne_bytes((reference_range.start).to_ne_bytes())) && (i32::from_ne_bytes(((line.range).clone().end).to_ne_bytes())) >= i32::from_ne_bytes((reference_range.end).to_ne_bytes()) {
                    attached = true;
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true(attached, Some((format!("{}{}{}",
            (choice.breaker).clone().get_strategy_name(),
            ": reference detached from prose: ",
            AttachedInlineVirtualAdjacencyTestSupport::attached_inline_virtual_adjacency_test_support_render_ranges(&result.lines)
        )).to_string())).unwrap();
        }
    });
}
