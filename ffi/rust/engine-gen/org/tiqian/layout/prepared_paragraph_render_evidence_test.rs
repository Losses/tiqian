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
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


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
impl std::fmt::Display for PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::SupportLayoutFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestStyleDeltaEmitsPerCellStyleBlockFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault::SupportLayoutFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestPlainParagraphEvidenceIsAppendOnlyFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::SupportLayoutFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestPinyinRubyEmitsRubyDecisionsFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::SupportLayoutFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestInlineBoxesEmitInlineEdgesFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::SupportLayoutFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestDecorationsEmitSegmentsDotsAndRangesFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::SupportLayoutFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphRenderEvidenceTestBopomofoRubyEmitsBopomofoDecisionsFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,114,101,112,97,114,101,100,80,97,114,97,103,114,97,112,104,82,101,110,100,101,114,69,118,105,100,101,110,99,101,84,101,115,116])));
        t.section(UStr::new(&[112,108,97,105,110,80,97,114,97,103,114,97,112,104,69,118,105,100,101,110,99,101,73,115,65,112,112,101,110,100,79,110,108,121]));
        let a = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_layout(LayoutInput::new(TiqianTextContent::new(&(UStr::new(&[20013,25991,27573,33853,32431,25991,26412,27979,35797])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(200 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let ap = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_plain((a).clone()).unwrap();
        let ae = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_evidence((a).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((ae).starts_with(&u_string::substr(&ap, 0i32, Some(i32::from_ne_bytes(((u32::wrapping_sub(u_string::unit_count(&(ap)), 1)) as i32).to_ne_bytes())))), None).unwrap();
        let r = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_layout(LayoutInput::new(TiqianTextContent::new(&(UStr::new(&[20013,25991,27573,33853,65292,21547,26631,28857,19982,26367,25442,30772,25240,21495,8212,8212,27979,35797,12290])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(200 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let p = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_plain((r).clone()).unwrap();
        let e = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_evidence((r).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes(((u_string::find_from(&(p), UString::from("\"fontSize\"").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes(((u_string::find_from(&(p), UString::from("\"rubyDecisions\"").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes(((u_string::find_from(&(p), UString::from("\"dashStrategy\"").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(e), UString::from("\"schema\":1,").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(e), UString::from("\"fontSize\":").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(e), UString::from("\"overlayWidth\":").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[test]
fn pinyin_ruby_emits_ruby_decisions() {
    testlib::run("org.tiqian.layout.PreparedParagraphRenderEvidenceTest.pinyinRubyEmitsRubyDecisions", "org.tiqian.layout.PreparedParagraphRenderEvidenceTest.pinyinRubyEmitsRubyDecisions", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,114,101,112,97,114,101,100,80,97,114,97,103,114,97,112,104,82,101,110,100,101,114,69,118,105,100,101,110,99,101,84,101,115,116])));
        t.section(UStr::new(&[112,105,110,121,105,110,82,117,98,121,69,109,105,116,115,82,117,98,121,68,101,99,105,115,105,111,110,115]));
        let r = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_layout(LayoutInput::new(TiqianTextContent::new(&(UStr::new(&[21271,20140,26159,39318,37117,12290])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(200 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![
    (RubySpan::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[66,283,105,106,299,110,103])), Some(vec![]), RubyKind::Pinyin, None)).clone(),
]), Some(vec![]), Some(vec![]))).unwrap();
        let p = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_plain((r).clone()).unwrap();
        let e = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_evidence((r).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes(((u_string::find_from(&(p), UString::from("rubyDecisions").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,114,117,98,121,68,101,99,105,115,105,111,110,115,34,58,91]), UStr::new(&[109,105,115,115,105,110,103,32,114,117,98,121,68,101,99,105,115,105,111,110,115])).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,98,97,115,101,82,97,110,103,101,83,116,97,114,116,34,58,48]), UStr::new(&[98,97,115,101,82,97,110,103,101,83,116,97,114,116])).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,98,97,115,101,82,97,110,103,101,69,110,100,34,58,50]), UStr::new(&[98,97,115,101,82,97,110,103,101,69,110,100])).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,116,101,120,116,34,58,34,66,283,105,106,299,110,103,34]), UStr::new(&[116,101,120,116])).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,99,101,110,116,101,114,88,34,58]), UStr::new(&[99,101,110,116,101,114,88])).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,98,97,115,101,108,105,110,101,89,34,58]), UStr::new(&[98,97,115,101,108,105,110,101,89])).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,102,111,110,116,83,105,122,101,34,58]), UStr::new(&[102,111,110,116,83,105,122,101])).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,97,115,99,101,110,116,34,58]), UStr::new(&[97,115,99,101,110,116])).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,102,111,110,116,87,101,105,103,104,116,34,58,53,48,48]), UStr::new(&[102,111,110,116,87,101,105,103,104,116])).unwrap();
    });
}

#[test]
fn bopomofo_ruby_emits_bopomofo_decisions() {
    testlib::run("org.tiqian.layout.PreparedParagraphRenderEvidenceTest.bopomofoRubyEmitsBopomofoDecisions", "org.tiqian.layout.PreparedParagraphRenderEvidenceTest.bopomofoRubyEmitsBopomofoDecisions", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,114,101,112,97,114,101,100,80,97,114,97,103,114,97,112,104,82,101,110,100,101,114,69,118,105,100,101,110,99,101,84,101,115,116])));
        t.section(UStr::new(&[98,111,112,111,109,111,102,111,82,117,98,121,69,109,105,116,115,66,111,112,111,109,111,102,111,68,101,99,105,115,105,111,110,115]));
        let r = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_layout(LayoutInput::new(TiqianTextContent::new(&(UStr::new(&[22909,25991,12290])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(200 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[12559,12576,711])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
]), Some(vec![]), Some(vec![]))).unwrap();
        let p = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_plain((r).clone()).unwrap();
        let e = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_evidence((r).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes(((u_string::find_from(&(p), UString::from("bopomofoDecisions").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,98,111,112,111,109,111,102,111,68,101,99,105,115,105,111,110,115,34,58,91]), UStr::new(&[98,111,112,111,109,111,102,111])).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,112,108,97,99,101,109,101,110,116,115,34,58,91]), UStr::new(&[112,108,97,99,101,109,101,110,116,115])).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,114,111,108,101,34,58,34]), UStr::new(&[114,111,108,101])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes(((u_string::find_from(&(e), UString::from("\"rubyDecisions\"").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[test]
fn decorations_emit_segments_dots_and_ranges() {
    testlib::run("org.tiqian.layout.PreparedParagraphRenderEvidenceTest.decorationsEmitSegmentsDotsAndRanges", "org.tiqian.layout.PreparedParagraphRenderEvidenceTest.decorationsEmitSegmentsDotsAndRanges", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,114,101,112,97,114,101,100,80,97,114,97,103,114,97,112,104,82,101,110,100,101,114,69,118,105,100,101,110,99,101,84,101,115,116])));
        t.section(UStr::new(&[100,101,99,111,114,97,116,105,111,110,115,69,109,105,116,83,101,103,109,101,110,116,115,68,111,116,115,65,110,100,82,97,110,103,101,115]));
        let r = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_layout(LayoutInput::new(TiqianTextContent::new(&(UStr::new(&[40065,36805,30340,23567,35828,22312,20013,22269,29616,20195,25991,23398,37324,24456,37325,35201,12290])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(200 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![
    (DecorationSpan::new(TextRange::new(0u32, 2u32).unwrap(), DecorationKind::ProperNoun)).clone(),
    (DecorationSpan::new(TextRange::new(3u32, 5u32).unwrap(), DecorationKind::BookTitle)).clone(),
    (DecorationSpan::new(TextRange::new(6u32, 9u32).unwrap(), DecorationKind::Emphasis)).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let p = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_plain((r).clone()).unwrap();
        let e = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_evidence((r).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes(((u_string::find_from(&(p), UString::from("decorationSegments").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes(((u_string::find_from(&(p), UString::from("emphasisDots").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes(((u_string::find_from(&(p), UString::from("emphasisRanges").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,100,101,99,111,114,97,116,105,111,110,83,101,103,109,101,110,116,115,34,58,91]), UStr::new(&[115,101,103,109,101,110,116,115])).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,107,105,110,100,34,58,34,80,114,111,112,101,114,78,111,117,110,34]), UStr::new(&[107,105,110,100])).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,107,105,110,100,34,58,34,66,111,111,107,84,105,116,108,101,34]), UStr::new(&[107,105,110,100])).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,115,111,117,114,99,101,82,97,110,103,101,83,116,97,114,116,34,58,48]), UStr::new(&[114,97,110,103,101])).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,101,109,112,104,97,115,105,115,82,97,110,103,101,115,34,58,91,91,54,44,57,93,93]), UStr::new(&[114,97,110,103,101,115])).unwrap();
        if u32::from_ne_bytes(((u_string::find_from(&(e), UString::from("\"emphasisDots\"").as_ustr(), 0)) as u32).to_ne_bytes()) <= 2147483647 {
            let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,97,110,99,104,111,114,88,34,58]), UStr::new(&[97,110,99,104,111,114])).unwrap();
            let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,100,111,116,68,105,97,109,101,116,101,114,34,58]), UStr::new(&[100,105,97,109,101,116,101,114])).unwrap();
        }
    });
}

#[test]
fn style_delta_emits_per_cell_style_block() {
    testlib::run("org.tiqian.layout.PreparedParagraphRenderEvidenceTest.styleDeltaEmitsPerCellStyleBlock", "org.tiqian.layout.PreparedParagraphRenderEvidenceTest.styleDeltaEmitsPerCellStyleBlock", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,114,101,112,97,114,101,100,80,97,114,97,103,114,97,112,104,82,101,110,100,101,114,69,118,105,100,101,110,99,101,84,101,115,116])));
        t.section(UStr::new(&[115,116,121,108,101,68,101,108,116,97,69,109,105,116,115,80,101,114,67,101,108,108,83,116,121,108,101,66,108,111,99,107]));
        let r = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_layout(LayoutInput::new(TiqianTextContent::new(&(UStr::new(&[26222,36890,23383,19982,23567,23383,28151,25490,30340,27573,33853,12290])), Some(vec![
    (TextSpan::new(TextRange::new(4u32, 6u32).unwrap(), TextStyle::new(Some(vec![]), Some(12 as f64), Some(UString::from("zh-Hans")), Some(700), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(200 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let p = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_plain((r).clone()).unwrap();
        let e = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_evidence((r).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes(((u_string::find_from(&(p), UString::from("\"style\":{").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,115,116,121,108,101,34,58,123,34,102,111,110,116,83,105,122,101,34,58]), UStr::new(&[115,116,121,108,101])).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,102,111,110,116,87,101,105,103,104,116,34,58,55,48,48]), UStr::new(&[119,101,105,103,104,116])).unwrap();
    });
}

#[test]
fn inline_boxes_emit_inline_edges() {
    testlib::run("org.tiqian.layout.PreparedParagraphRenderEvidenceTest.inlineBoxesEmitInlineEdges", "org.tiqian.layout.PreparedParagraphRenderEvidenceTest.inlineBoxesEmitInlineEdges", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,114,101,112,97,114,101,100,80,97,114,97,103,114,97,112,104,82,101,110,100,101,114,69,118,105,100,101,110,99,101,84,101,115,116])));
        t.section(UStr::new(&[105,110,108,105,110,101,66,111,120,101,115,69,109,105,116,73,110,108,105,110,101,69,100,103,101,115]));
        let r = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_layout(LayoutInput::new(TiqianTextContent::new(&(UStr::new(&[25991,23383,19982,36793,36317,12290])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(200 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![
    (InlineBoxSpan::new(TextRange::new(0u32, 1u32).unwrap(), Some(2 as f64), Some(3 as f64), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]), Some(vec![]))).unwrap();
        let p = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_plain((r).clone()).unwrap();
        let e = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_evidence((r).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes(((u_string::find_from(&(p), UString::from("inlineEdges").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,105,110,108,105,110,101,69,100,103,101,115,34,58,91]), UStr::new(&[101,100,103,101,115])).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,111,102,102,115,101,116,34,58,48]), UStr::new(&[111,102,102,115,101,116])).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,111,102,102,115,101,116,34,58,49]), UStr::new(&[111,102,102,115,101,116])).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,105,110,108,105,110,101,83,116,97,114,116,34,58,50]), UStr::new(&[115,116,97,114,116])).unwrap();
        let _ = PreparedParagraphRenderEvidenceTestSupport::prepared_paragraph_render_evidence_test_support_contains(e.as_ustr(), UStr::new(&[34,105,110,108,105,110,101,69,110,100,34,58,51]), UStr::new(&[101,110,100])).unwrap();
    });
}
