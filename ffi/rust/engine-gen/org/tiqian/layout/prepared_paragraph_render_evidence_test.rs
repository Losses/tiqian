#![cfg(test)]

use crate::org::tiqian::core::decoration_kind::DecorationKind;
use crate::org::tiqian::core::decoration_span::DecorationSpan;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_box_outer_spacing::InlineBoxOuterSpacing;
use crate::org::tiqian::core::inline_box_span::InlineBoxSpan;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_kind::RubyKind;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;


#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    SupportLayoutFault(crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
}

impl From<PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault> for crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault {
    fn from(value: PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::SupportLayoutFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault> for PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault) -> Self {
        PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::SupportLayoutFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    SupportLayoutFault(crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
}

impl From<PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault> for crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault {
    fn from(value: PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault::SupportLayoutFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault> for PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault) -> Self {
        PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault::SupportLayoutFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    SupportLayoutFault(crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
}

impl From<PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault> for crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault {
    fn from(value: PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::SupportLayoutFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault> for PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault) -> Self {
        PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::SupportLayoutFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    SupportLayoutFault(crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
}

impl From<PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault> for crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault {
    fn from(value: PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::SupportLayoutFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault> for PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault) -> Self {
        PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::SupportLayoutFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    SupportLayoutFault(crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
}

impl From<PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault> for crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault {
    fn from(value: PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::SupportLayoutFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault> for PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault) -> Self {
        PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::SupportLayoutFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    SupportLayoutFault(crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
}

impl From<PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault> for crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault {
    fn from(value: PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::SupportLayoutFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault) -> Self {
        match value {
            PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault> for PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph_render_evidence_test_support::PreparedParagraphRenderEvidenceTestSupportLayoutFault) -> Self {
        PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::SupportLayoutFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

#[test]
fn plain_paragraph_evidence_is_append_only() {
    testlib::run("org.tiqian.layout.PreparedParagraphRenderEvidenceTest.plainParagraphEvidenceIsAppendOnly", "org.tiqian.layout.PreparedParagraphRenderEvidenceTest.plainParagraphEvidenceIsAppendOnly", || {
        let mut t = TestTraceRecorder::new("PreparedParagraphRenderEvidenceTest");
        t.section(&"plainParagraphEvidenceIsAppendOnly");
        let a = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_layout(LayoutInput::new(TiqianTextContent::new("中文段落纯文本测试", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0),
Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0),
Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(200 as f64 as
f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let ap = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_plain((a).clone()).unwrap();
        let ae = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_evidence((a).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((ae).starts_with(&u_string::substr(&ap, 0i32, Some(i32::from_ne_bytes((u32::wrapping_sub(u_string::unit_count(&(ap)), 1)).to_ne_bytes())))), None).unwrap();
        let r = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_layout(LayoutInput::new(TiqianTextContent::new("中文段落，含标点与替换破折号——测试。", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0),
Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0),
Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(200 as f64 as
f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let p = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_plain((r).clone()).unwrap();
        let e = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_evidence((r).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&p, "\"fontSize\"", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&p, "\"rubyDecisions\"", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&p, "\"dashStrategy\"", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&e, "\"schema\":1,", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&e, "\"fontSize\":", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&e, "\"overlayWidth\":", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[test]
fn pinyin_ruby_emits_ruby_decisions() {
    testlib::run("org.tiqian.layout.PreparedParagraphRenderEvidenceTest.pinyinRubyEmitsRubyDecisions", "org.tiqian.layout.PreparedParagraphRenderEvidenceTest.pinyinRubyEmitsRubyDecisions", || {
        let mut t = TestTraceRecorder::new("PreparedParagraphRenderEvidenceTest");
        t.section(&"pinyinRubyEmitsRubyDecisions");
        let r = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_layout(LayoutInput::new(TiqianTextContent::new("北京是首都。", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0),
Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0),
Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(200 as f64 as
f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![
    (RubySpan::new(TextRange::new(0u32, 2u32).unwrap(), "Běijīng", Some(vec![]), RubyKind::Pinyin, None)).clone(),
]), Some(vec![]), Some(vec![]))).unwrap();
        let p = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_plain((r).clone()).unwrap();
        let e = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_evidence((r).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&p, "rubyDecisions", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"rubyDecisions\":[", &"missing rubyDecisions").unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"baseRangeStart\":0", &"baseRangeStart").unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"baseRangeEnd\":2", &"baseRangeEnd").unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"text\":\"Běijīng\"", &"text").unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"centerX\":", &"centerX").unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"baselineY\":", &"baselineY").unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"fontSize\":", &"fontSize").unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"ascent\":", &"ascent").unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"fontWeight\":500", &"fontWeight").unwrap();
    });
}

#[test]
fn bopomofo_ruby_emits_bopomofo_decisions() {
    testlib::run("org.tiqian.layout.PreparedParagraphRenderEvidenceTest.bopomofoRubyEmitsBopomofoDecisions", "org.tiqian.layout.PreparedParagraphRenderEvidenceTest.bopomofoRubyEmitsBopomofoDecisions", || {
        let mut t = TestTraceRecorder::new("PreparedParagraphRenderEvidenceTest");
        t.section(&"bopomofoRubyEmitsBopomofoDecisions");
        let r = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_layout(LayoutInput::new(TiqianTextContent::new("好文。", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0),
Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0),
Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(200 as f64 as
f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), "ㄏㄠˇ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
]), Some(vec![]), Some(vec![]))).unwrap();
        let p = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_plain((r).clone()).unwrap();
        let e = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_evidence((r).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&p, "bopomofoDecisions", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"bopomofoDecisions\":[", &"bopomofo").unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"placements\":[", &"placements").unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"role\":\"", &"role").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&e, "\"rubyDecisions\"", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[test]
fn decorations_emit_segments_dots_and_ranges() {
    testlib::run("org.tiqian.layout.PreparedParagraphRenderEvidenceTest.decorationsEmitSegmentsDotsAndRanges", "org.tiqian.layout.PreparedParagraphRenderEvidenceTest.decorationsEmitSegmentsDotsAndRanges", || {
        let mut t = TestTraceRecorder::new("PreparedParagraphRenderEvidenceTest");
        t.section(&"decorationsEmitSegmentsDotsAndRanges");
        let r = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_layout(LayoutInput::new(TiqianTextContent::new("鲁迅的小说在中国现代文学里很重要。", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0),
Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0),
Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(200 as f64 as
f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![
    (DecorationSpan::new(TextRange::new(0u32, 2u32).unwrap(), DecorationKind::ProperNoun)).clone(),
    (DecorationSpan::new(TextRange::new(3u32, 5u32).unwrap(), DecorationKind::BookTitle)).clone(),
    (DecorationSpan::new(TextRange::new(6u32, 9u32).unwrap(), DecorationKind::Emphasis)).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let p = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_plain((r).clone()).unwrap();
        let e = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_evidence((r).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&p, "decorationSegments", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&p, "emphasisDots", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&p, "emphasisRanges", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"decorationSegments\":[", &"segments").unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"kind\":\"ProperNoun\"", &"kind").unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"kind\":\"BookTitle\"", &"kind").unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"sourceRangeStart\":0", &"range").unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"emphasisRanges\":[[6,9]]", &"ranges").unwrap();
        if u32::from_ne_bytes((u_string::find_from(&e, "\"emphasisDots\"", 0)).to_ne_bytes()) <= 2147483647 {
            let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"anchorX\":", &"anchor").unwrap();
            let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"dotDiameter\":", &"diameter").unwrap();
        }
    });
}

#[test]
fn style_delta_emits_per_cell_style_block() {
    testlib::run("org.tiqian.layout.PreparedParagraphRenderEvidenceTest.styleDeltaEmitsPerCellStyleBlock", "org.tiqian.layout.PreparedParagraphRenderEvidenceTest.styleDeltaEmitsPerCellStyleBlock", || {
        let mut t = TestTraceRecorder::new("PreparedParagraphRenderEvidenceTest");
        t.section(&"styleDeltaEmitsPerCellStyleBlock");
        let r = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_layout(LayoutInput::new(TiqianTextContent::new("普通字与小字混排的段落。", Some(vec![
    (TextSpan::new(TextRange::new(4u32, 6u32).unwrap(), TextStyle::new(Some(vec![]), Some(12 as f64), Some("zh-Hans".to_string()), Some(700), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb),
None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(200 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let p = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_plain((r).clone()).unwrap();
        let e = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_evidence((r).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&p, "\"style\":{", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"style\":{\"fontSize\":", &"style").unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"fontWeight\":700", &"weight").unwrap();
    });
}

#[test]
fn inline_boxes_emit_inline_edges() {
    testlib::run("org.tiqian.layout.PreparedParagraphRenderEvidenceTest.inlineBoxesEmitInlineEdges", "org.tiqian.layout.PreparedParagraphRenderEvidenceTest.inlineBoxesEmitInlineEdges", || {
        let mut t = TestTraceRecorder::new("PreparedParagraphRenderEvidenceTest");
        t.section(&"inlineBoxesEmitInlineEdges");
        let r = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_layout(LayoutInput::new(TiqianTextContent::new("文字与边距。", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0),
Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0),
Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(200 as f64 as
f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![
    (InlineBoxSpan::new(TextRange::new(0u32, 1u32).unwrap(), Some(2 as f64), Some(3 as f64), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]), Some(vec![]))).unwrap();
        let p = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_plain((r).clone()).unwrap();
        let e = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_evidence((r).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&p, "inlineEdges", 0)).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"inlineEdges\":[", &"edges").unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"offset\":0", &"offset").unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"offset\":1", &"offset").unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"inlineStart\":2", &"start").unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_str(), &"\"inlineEnd\":3", &"end").unwrap();
    });
}
