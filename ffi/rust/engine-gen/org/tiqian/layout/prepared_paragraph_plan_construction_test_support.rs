use crate::org::tiqian::core::bopomofo_decision_info::BopomofoDecisionInfo;
use crate::org::tiqian::core::bopomofo_glyph_placement::BopomofoGlyphPlacement;
use crate::org::tiqian::core::bopomofo_glyph_role::BopomofoGlyphRole;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::decoration_decision_info::DecorationDecisionInfo;
use crate::org::tiqian::core::decoration_kind::DecorationKind;
use crate::org::tiqian::core::decoration_segment_info::DecorationSegmentInfo;
use crate::org::tiqian::core::decoration_span::DecorationSpan;
use crate::org::tiqian::core::font_decision_info::FontDecisionInfo;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::glyph_run::GlyphRun;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_box_outer_spacing::InlineBoxOuterSpacing;
use crate::org::tiqian::core::inline_box_span::InlineBoxSpan;
use crate::org::tiqian::core::inline_object_boundary_adjustment::InlineObjectBoundaryAdjustment;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_debug_info::LayoutDebugInfo;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_box::LineBox;
use crate::org::tiqian::core::line_debug_info::LineDebugInfo;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::punctuation_decision_info::PunctuationDecisionInfo;
use crate::org::tiqian::core::ruby_decision_info::RubyDecisionInfo;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::shaping_decision_info::ShapingDecisionInfo;
use crate::org::tiqian::core::size::Size;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::core::zero_width_break_decision_info::ZeroWidthBreakDecisionInfo;
use crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphFns;
use crate::org::tiqian::test::test_helpers::TestHelpers;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsAssertFalseFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsAssertEqualsFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsAssertFalseFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsAssertFalseFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault> for PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsAssertFalseFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphPlanConstructionTestSupportRunPunctuationFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PreparedParagraphPlanConstructionTestSupportRunPunctuationFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TracedAssertionsAssertEqualsFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunPunctuationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunPunctuationFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunPunctuationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunPunctuationFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunPunctuationFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunPunctuationFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunPunctuationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunPunctuationFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunPunctuationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunPunctuationFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunPunctuationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunPunctuationFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphPlanConstructionTestSupportRunPunctuationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphPlanConstructionTestSupportRunPunctuationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphPlanConstructionTestSupportRunPunctuationFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for PreparedParagraphPlanConstructionTestSupportRunPunctuationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for PreparedParagraphPlanConstructionTestSupportRunPunctuationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphPlanConstructionTestSupportRunPunctuationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertFalseFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsAssertFalseFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsAssertFalseFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault> for PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsAssertFalseFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphPlanConstructionTestSupportRunEscapesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PreparedParagraphPlanConstructionTestSupportRunEscapesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreparedParagraphPlanConstructionTestSupportRunEscapesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunEscapesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunEscapesFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunEscapesFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunEscapesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunEscapesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunEscapesFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunEscapesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunEscapesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunEscapesFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunEscapesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunEscapesFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunEscapesFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunEscapesFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunEscapesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunEscapesFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunEscapesFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunEscapesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunEscapesFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunEscapesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphPlanConstructionTestSupportRunEscapesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunEscapesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphPlanConstructionTestSupportRunEscapesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunEscapesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphPlanConstructionTestSupportRunEscapesFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunEscapesFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for PreparedParagraphPlanConstructionTestSupportRunEscapesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunEscapesFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphPlanConstructionTestSupportRunEscapesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunEscapesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertFalseFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    PreparedParagraphToPlanWithDiagnosticsJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPlanWithDiagnosticsJsonFault),
}
impl std::fmt::Display for PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsAssertFalseFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::PreparedParagraphToPlanWithDiagnosticsJsonFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsAssertFalseFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPlanWithDiagnosticsJsonFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::PreparedParagraphToPlanWithDiagnosticsJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault> for PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsAssertFalseFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPlanWithDiagnosticsJsonFault> for PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPlanWithDiagnosticsJsonFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::PreparedParagraphToPlanWithDiagnosticsJsonFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphPlanConstructionTestSupportRunEvidenceFault {
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PreparedParagraphPlanConstructionTestSupportRunEvidenceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunEvidenceFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunEvidenceFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunEvidenceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunEvidenceFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphPlanConstructionTestSupportRunEvidenceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunEvidenceFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphPlanConstructionTestSupportRunEvidenceFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::PreparedParagraphToPreparedParagraphJsonFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for PreparedParagraphPlanConstructionTestSupportRunEvidenceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PreparedParagraphPlanConstructionTestSupportRunEvidenceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct PreparedParagraphPlanConstructionTestSupport;

impl PreparedParagraphPlanConstructionTestSupport {
    pub fn prepared_paragraph_plan_construction_test_support_line(r: TextRange, cr: IntRange, w: Option<f64>) -> LineBox {
        let x = match &(w) { None => 26.0f64, Some(__option) => *__option };
        return LineBox::new((r).clone(), (cr).clone(), 20 as f64 as f64, 0 as f64 as f64, 24 as f64 as f64, x, x, x, Some(0.0), Some(0.0), Some(LineEndReason::ParagraphEnd), Some(0.0), Some(vec![]), LineDebugInfo::new(None.clone(), Some(vec![])));
    }

    pub(crate) fn prepared_paragraph_plan_construction_test_support_result(content: TiqianTextContent, clusters: &Vec<Cluster>, runs: &Vec<GlyphRun>, lines: &Vec<LineBox>, decorations: Option<Vec<DecorationSpan>>, boxes: Option<Vec<InlineBoxSpan>>, objects: Option<Vec<InlineObjectSpan>>, debug: Option<LayoutDebugInfo>, width: Option<f64>, height: Option<f64>) -> Result<LayoutResult, TextRangeError> {
        let w = match &(width) { None => 480.0f64, Some(__option1) => *__option1 };
        let h = match &(height) { None => 24.0f64, Some(__option2) => *__option2 };
        return Ok(LayoutResult::new(LayoutInput::new((content).clone(), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(w, Some(f64::INFINITY), Some(2147483647))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), (decorations).clone(), Some(vec![]), (boxes).clone(), (objects).clone()), Size::new(w, h), (clusters).clone(), (runs).clone(), (lines).clone(), match &(debug) { None => LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(__option3) => (*__option3).clone() }));
    }

    pub(crate) fn prepared_paragraph_plan_construction_test_support_one(text: &UStr, font: Option<UString>, advance: Option<f64>) -> Result<LayoutResult, TextRangeError> {
        let a = match &(advance) { None => 16.0f64, Some(__option4) => *__option4 };
        let f = match &(font) { None => UString::from("cjk"), Some(__option5) => __option5.to_ustring() };
        let r = TextRange::new(0u32, u_string::unit_count(&(text)))?;
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((r).clone(), text, f.as_ustr(), a, Some(text.to_ustring()), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((r).clone(), f.as_ustr(), vec![(Glyph::new(1u32, (r).clone(), a, Some(0.0), Some(0.0), None, None, None, None)).clone()].to_vec(), a, Some(vec![]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line((r).clone(), IntRange::new(0u32, 0u32), Some(a))).clone(),
], None, None, None, None, None, None)?);
    }

    pub fn prepared_paragraph_plan_construction_test_support_open_type_features_and_render_font_family_attach_per_cluster() -> Result<LayoutResult, TextRangeError> {
        let r = TextRange::new(0u32, 1u32)?;
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new(&(UStr::new(&[27721])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((r).clone(), &(UStr::new(&[27721])), &(UStr::new(&[99,106,107])), 16 as f64 as f64, Some(UString::from("汉")), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((r).clone(), &(UStr::new(&[99,106,107])), vec![
    (Glyph::new(7u32, (r).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), Some(UString::from("Noto Serif CJK")), None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![UString::from("kern").to_ustring(), UString::from("liga").to_ustring()]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line((r).clone(), IntRange::new(0u32, 0u32), None)).clone(),
], None, None, None, None, None, None)?);
    }

    pub fn prepared_paragraph_plan_construction_test_support_multi_unit_cluster_marks_shaping_boundary() -> Result<LayoutResult, TextRangeError> {
        let r = TextRange::new(0u32, 2u32)?;
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new(&(UStr::new(&[65,66])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((r).clone(), &(UStr::new(&[65,66])), &(UStr::new(&[108,97,116,105,110])), 18 as f64 as f64, Some(UString::from("AB")), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((r).clone(), &(UStr::new(&[108,97,116,105,110])), vec![
    (Glyph::new(1u32, (r).clone(), 18 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 18 as f64 as f64, Some(vec![]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line((r).clone(), IntRange::new(0u32, 0u32), None)).clone(),
], None, None, None, None, None, None)?);
    }

    pub fn prepared_paragraph_plan_construction_test_support_inline_object_cell_emits_advance_override() -> Result<LayoutResult, TextRangeError> {
        let a = TextRange::new(0u32, 1u32)?;
        let b = TextRange::new(1u32, 2u32)?;
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new(&(UStr::new(&[27721,22270])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((a).clone(), &(UStr::new(&[27721])), &(UStr::new(&[99,106,107])), 16 as f64 as f64, Some(UString::from("汉")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((b).clone(), &(UStr::new(&[22270])), &(UStr::new(&[105,110,108,105,110,101])), 10 as f64 as f64, Some(UString::from("图")), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((a).clone(), &(UStr::new(&[99,106,107])), vec![
    (Glyph::new(1u32, (a).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
    (GlyphRun::new((b).clone(), &(UStr::new(&[105,110,108,105,110,101])), vec![
    (Glyph::new(2u32, (b).clone(), 24 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 24 as f64 as f64, Some(vec![]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line(TextRange::new(0u32, 2u32)?, IntRange::new(0u32, 1u32), Some(40 as f64 as f64))).clone(),
], None, None, Some(vec![
    (InlineObjectSpan::new((b).clone(), 24 as f64 as f64, 12 as f64 as f64, 4 as f64 as f64, Some((InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed()?).clone()), Some((InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed()?).clone()))?).clone(),
]), None, None, None)?);
    }

    pub fn prepared_paragraph_plan_construction_test_support_style_delta_lists_only_paint_fields() -> Result<LayoutResult, TextRangeError> {
        let a = TextRange::new(0u32, 1u32)?;
        let b = TextRange::new(1u32, 2u32)?;
        let c = TextRange::new(2u32, 3u32)?;
        let content = TiqianTextContent::new(&(UStr::new(&[27721,65,23383])), Some(vec![
    (TextSpan::new((a).clone(), TextStyle::new(Some(vec![]), Some(20 as f64), Some(UString::from("zh-Hans")), Some(700), Some(true), Some(0.0), Some(InlineAttachment::None)))).clone(),
    (TextSpan::new((b).clone(), TextStyle::new(Some(vec![UString::from("Kai").to_ustring()]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![]));
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result((content).clone(), &vec![
    (Cluster::new((a).clone(), &(UStr::new(&[27721])), &(UStr::new(&[99,106,107])), 16 as f64 as f64, Some(UString::from("汉")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((b).clone(), &(UStr::new(&[65])), &(UStr::new(&[108,97,116,105,110])), 10 as f64 as f64, Some(UString::from("A")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((c).clone(), &(UStr::new(&[23383])), &(UStr::new(&[99,106,107])), 16 as f64 as f64, Some(UString::from("字")), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((a).clone(), &(UStr::new(&[99,106,107])), vec![
    (Glyph::new(1u32, (a).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
    (GlyphRun::new((b).clone(), &(UStr::new(&[108,97,116,105,110])), vec![
    (Glyph::new(2u32, (b).clone(), 10 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 10 as f64 as f64, Some(vec![]))).clone(),
    (GlyphRun::new((c).clone(), &(UStr::new(&[99,106,107])), vec![
    (Glyph::new(3u32, (c).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line(TextRange::new(0u32, 3u32)?, IntRange::new(0u32, 2u32), Some(42 as f64 as f64))).clone(),
], None, None, None, None, None, None)?);
    }

    pub fn prepared_paragraph_plan_construction_test_support_dash_cluster_emits_shaping_evidence_block() -> Result<LayoutResult, TextRangeError> {
        let a = TextRange::new(0u32, 1u32)?;
        let b = TextRange::new(1u32, 3u32)?;
        let d = ShapingDecisionInfo::new((b).clone(), &(UStr::new(&[8212,8212])), &(UStr::new(&[8212,8212])), &(UStr::new(&[99,106,107])), 2u32, 32 as f64 as f64, &(UStr::new(&[83,104,97,112,105,110,103,83,116,97,103,101])), &(UStr::new(&[100,97,115,104,45,114,101,97,115,111,110])), Some(0), Some(0), Some(UString::from("NotoSansCJK")), None, Some(UString::from("zh-Hans")), Some(UString::from("PairedEmDash")), None, None);
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new(&(UStr::new(&[27721,8212,8212])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((a).clone(), &(UStr::new(&[27721])), &(UStr::new(&[99,106,107])), 16 as f64 as f64, Some(UString::from("汉")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((b).clone(), &(UStr::new(&[8212,8212])), &(UStr::new(&[99,106,107])), 32 as f64 as f64, Some(UString::from("——")), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((a).clone(), &(UStr::new(&[99,106,107])), vec![
    (Glyph::new(1u32, (a).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
    (GlyphRun::new((b).clone(), &(UStr::new(&[99,106,107])), vec![
    (Glyph::new(9u32, (b).clone(), 32 as f64 as f64, Some(0.0), Some(0.0), Some(UString::from("Noto Sans CJK")), None, None, None)).clone(),
    (Glyph::new(10u32, (b).clone(), 0 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 32 as f64 as f64, Some(vec![]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line(TextRange::new(0u32, 3u32)?, IntRange::new(0u32, 1u32), Some(48 as f64 as f64))).clone(),
], None, None, None,
Some(LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(d).clone()]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))), None, None)?);
    }

    pub fn prepared_paragraph_plan_construction_test_support_punctuation_ink_floor_and_latin_role_mark_cells() -> Result<LayoutResult, TextRangeError> {
        let a = TextRange::new(0u32, 1u32)?;
        let b = TextRange::new(1u32, 2u32)?;
        let c = TextRange::new(2u32, 3u32)?;
        let ps = vec![
    (PunctuationDecisionInfo::new((a).clone(), &(UStr::new(&[12290])), &(UStr::new(&[80,97,117,115,101,79,114,83,116,111,112])), 16 as f64 as f64, 16 as f64 as f64, 0 as f64 as f64, 0 as f64 as f64, &(UStr::new(&[99,101,110,116,114,101])), None, Some(UString::from("PolicyDerived")), 16 as f64 as f64, None, None, Some(6 as f64), Some(true), None, None, None, Some(0.0), Some(0.0), None, Some(0.0), Some(0.0))).clone(),
    (PunctuationDecisionInfo::new((b).clone(), &(UStr::new(&[65])), &(UStr::new(&[79,116,104,101,114])), 10 as f64 as f64, 10 as f64 as f64, 0 as f64 as f64, 0 as f64 as f64, &(UStr::new(&[99,101,110,116,114,101])), None, Some(UString::from("PolicyDerived")), 10 as f64 as f64, None, None, None, Some(false), None, None, None, Some(0.0), Some(0.0), None, Some(0.0), Some(0.0))).clone(),
    (PunctuationDecisionInfo::new((c).clone(), &(UStr::new(&[20013])), &(UStr::new(&[79,116,104,101,114])), 16 as f64 as f64, 16 as f64 as f64, 0 as f64 as f64, 0 as f64 as f64, &(UStr::new(&[99,101,110,116,114,101])), None, Some(UString::from("PolicyDerived")), 16 as f64 as f64, None, None, None, Some(false), None, None, None, Some(0.0), Some(0.0), None, Some(0.0), Some(0.0))).clone(),
];
        let fd = vec![
    (FontDecisionInfo::new((b).clone(), &(UStr::new(&[65])), &(UStr::new(&[65])), &(UStr::new(&[76,97,116,105,110,84,101,120,116])), &(UStr::new(&[108,97,116,105,110])), &(UStr::new(&[108,97,116,105,110,45,114,117,110])), &(UStr::new(&[110,111,110,101])))).clone(),
];
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new(&(UStr::new(&[12290,65,20013])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((a).clone(), &(UStr::new(&[12290])), &(UStr::new(&[99,106,107])), 16 as f64 as f64, Some(UString::from("。")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((b).clone(), &(UStr::new(&[65])), &(UStr::new(&[108,97,116,105,110])), 10 as f64 as f64, Some(UString::from("A")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((c).clone(), &(UStr::new(&[20013])), &(UStr::new(&[99,106,107])), 16 as f64 as f64, Some(UString::from("中")), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((a).clone(), &(UStr::new(&[99,106,107])), vec![
    (Glyph::new(1u32, (a).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
    (GlyphRun::new((b).clone(), &(UStr::new(&[108,97,116,105,110])), vec![
    (Glyph::new(2u32, (b).clone(), 10 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 10 as f64 as f64, Some(vec![]))).clone(),
    (GlyphRun::new((c).clone(), &(UStr::new(&[99,106,107])), vec![
    (Glyph::new(3u32, (c).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line(TextRange::new(0u32, 3u32)?, IntRange::new(0u32, 2u32), Some(42 as f64 as f64))).clone(),
], None, None, None,
Some(LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some((fd).clone()), Some(vec![]), Some((ps).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))), None, None)?);
    }

    pub fn prepared_paragraph_plan_construction_test_support_zero_width_break_cluster_survives_empty_display_text() -> Result<LayoutResult, TextRangeError> {
        let a = TextRange::new(0u32, 1u32)?;
        let b = TextRange::new(1u32, 2u32)?;
        let c = TextRange::new(2u32, 3u32)?;
        let z = ShapingDecisionInfo::new((b).clone(), &(UStr::new(&[8203])), &(UStr::new(&[])), &(UStr::new(&[99,106,107])), 0u32, 0 as f64 as f64, &(UStr::new(&[83,104,97,112,105,110,103,83,116,97,103,101])), &(UStr::new(&[110,111,45,115,104,97,112,101])), Some(0), Some(0), None, None, None, Some(UString::from("ZeroWidthNoShape")), None, None);
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new(&(UStr::new(&[27721,8203,23383])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((a).clone(), &(UStr::new(&[27721])), &(UStr::new(&[99,106,107])), 16 as f64 as f64, Some(UString::from("汉")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((b).clone(), &(UStr::new(&[8203])), &(UStr::new(&[99,106,107])), 0 as f64 as f64, Some(UString::from("")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((c).clone(), &(UStr::new(&[23383])), &(UStr::new(&[99,106,107])), 16 as f64 as f64, Some(UString::from("字")), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((a).clone(), &(UStr::new(&[99,106,107])), vec![
    (Glyph::new(1u32, (a).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
    (GlyphRun::new((c).clone(), &(UStr::new(&[99,106,107])), vec![
    (Glyph::new(3u32, (c).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line(TextRange::new(0u32, 3u32)?, IntRange::new(0u32, 2u32), Some(32 as f64 as f64))).clone(),
], None, None, None,
Some(LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(z).clone()]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![
    (ZeroWidthBreakDecisionInfo::new((b).clone(), &(UStr::new(&[8203])), 1u32, Some(UString::from("ZeroWidthSpaceSoftBreakNoShape")))).clone(),
]), Some(vec![]), Some(vec![]))), None, None)?);
    }

    pub fn prepared_paragraph_plan_construction_test_support_paragraph_evidence_emits_every_section() -> Result<LayoutResult, TextRangeError> {
        let a = TextRange::new(0u32, 1u32)?;
        let b = TextRange::new(1u32, 2u32)?;
        let ds = vec![
    (DecorationSpan::new((a).clone(), DecorationKind::Emphasis)).clone(),
    (DecorationSpan::new((b).clone(), DecorationKind::Emphasis)).clone(),
];
        let dec = vec![
    (DecorationDecisionInfo::new((a).clone(), &(UStr::new(&[27721])), &(UStr::new(&[69,109,112,104,97,115,105,115])), true, &(UStr::new(&[100,111,116,45,97,112,112,108,105,101,100])), Some(8 as f64), Some(22 as f64), Some(2 as f64))).clone(),
    (DecorationDecisionInfo::new((b).clone(), &(UStr::new(&[27880])), &(UStr::new(&[69,109,112,104,97,115,105,115])), true, &(UStr::new(&[100,111,116,45,119,105,116,104,111,117,116,45,115,105,122,101])), Some(24 as f64), Some(22 as f64), Some(0 as f64))).clone(),
    (DecorationDecisionInfo::new((b).clone(), &(UStr::new(&[27880])), &(UStr::new(&[69,109,112,104,97,115,105,115])), false, &(UStr::new(&[100,111,116,45,115,107,105,112,112,101,100])), Some(24 as f64), Some(22 as f64), Some(2 as f64))).clone(),
];
        let seg = vec![
    (DecorationSegmentInfo::new((a).clone(), &(UStr::new(&[80,114,111,112,101,114,78,111,117,110])), 0u32, 0 as f64 as f64, 20 as f64 as f64, 16 as f64 as f64, 22 as f64 as f64, false, false, &(UStr::new(&[112,114,111,112,101,114,45,110,111,117,110])))).clone(),
    (DecorationSegmentInfo::new((b).clone(), &(UStr::new(&[66,111,111,107,84,105,116,108,101])), 0u32, 16 as f64 as f64, 20 as f64 as f64, 32 as f64 as f64, 22 as f64 as f64, false, false, &(UStr::new(&[98,111,111,107,45,116,105,116,108,101])))).clone(),
];
        let ruby = vec![
    (RubyDecisionInfo::new((a).clone(), &(UStr::new(&[104,224,110])), 0u32, 8 as f64 as f64, 2 as f64 as f64, 8 as f64 as f64, 0.5f64, Some(6 as f64), Some(0.0), Some(0.0), Some(vec![UString::from("RubyKai").to_ustring(), UString::from("RubyLatin").to_ustring()]), Some(400), Some(UString::from("zh-Hans")), Some(vec![]))).clone(),
    (RubyDecisionInfo::new((b).clone(), &(UStr::new(&[122,104,249])), 0u32, 24 as f64 as f64, 2 as f64 as f64, 8 as f64 as f64, 0 as f64 as f64, Some(0.0), Some(0.0), Some(0.0), Some(vec![]), Some(400), Some(UString::from("zh-Hans")), Some(vec![]))).clone(),
];
        let bop = vec![
    (BopomofoDecisionInfo::new((b).clone(), &(UStr::new(&[12563,12584,715])), 0u32, vec![
    (BopomofoGlyphPlacement::new(&(UStr::new(&[12563])), 1 as f64 as f64, 2 as f64 as f64, 4 as f64 as f64, 4 as f64 as f64, BopomofoGlyphRole::Symbol, Some(vec![]), 1 as f64 as f64, 6 as f64 as f64, 4 as f64 as f64)).clone(),
    (BopomofoGlyphPlacement::new(&(UStr::new(&[715])), 2 as f64 as f64, 0 as f64 as f64, 2 as f64 as f64, 2 as f64 as f64, BopomofoGlyphRole::Tone, Some(vec![]), 2 as f64 as f64, 2 as f64 as f64, 2 as f64 as f64)).clone(),
].to_vec(), Some(vec![UString::from("BopomofoKai").to_ustring(), UString::from("BopomofoLatin").to_ustring()]), Some(400), Some(UString::from("zh-Hans")))).clone(),
    (BopomofoDecisionInfo::new((a).clone(), &(UStr::new(&[12559,12578,715])), 0u32, vec![
    (BopomofoGlyphPlacement::new(&(UStr::new(&[12559])), 0 as f64 as f64, 2 as f64 as f64, 4 as f64 as f64, 4 as f64 as f64, BopomofoGlyphRole::Symbol, Some(vec![]), 0 as f64 as f64, 6 as f64 as f64, 4 as f64 as f64)).clone(),
].to_vec(), Some(vec![]), Some(400), Some(UString::from("zh-Hans")))).clone(),
];
        let debug = LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some((ruby).clone()), Some((bop).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some((dec).clone()), Some((seg).clone()), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new(&(UStr::new(&[27721,27880])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((a).clone(), &(UStr::new(&[27721])), &(UStr::new(&[99,106,107])), 16 as f64 as f64, Some(UString::from("汉")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((b).clone(), &(UStr::new(&[27880])), &(UStr::new(&[99,106,107])), 16 as f64 as f64, Some(UString::from("注")), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new(TextRange::new(0u32, 2u32)?, &(UStr::new(&[99,106,107])), vec![
    (Glyph::new(1u32, (a).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
    (Glyph::new(2u32, (b).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 32 as f64 as f64, Some(vec![]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line(TextRange::new(0u32, 2u32)?, IntRange::new(0u32, 1u32), Some(32 as f64 as f64))).clone(),
], Some((ds).clone()), Some(vec![
    (InlineBoxSpan::new((a).clone(), Some(2 as f64), Some(0.0), Some(InlineBoxOuterSpacing::Narrow))).clone(),
    (InlineBoxSpan::new((a).clone(), Some(0.5f64), Some(0.0), Some(InlineBoxOuterSpacing::Narrow))).clone(),
    (InlineBoxSpan::new((b).clone(), Some(0.0), Some(3 as f64), Some(InlineBoxOuterSpacing::Narrow))).clone(),
    (InlineBoxSpan::new(TextRange::new(0u32, 2u32)?, Some(0.0), Some(1.5f64), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]), None, Some((debug).clone()), None, None)?);
    }

    pub fn prepared_paragraph_plan_construction_test_support_negative_zero_and_exponent_widths_normalize() -> Result<LayoutResult, TextRangeError> {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_one(UStr::new(&[27721]), None, None)?;
        let l = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line(TextRange::new(0u32, 1u32)?, IntRange::new(0u32, 0u32), None);
        let nl = LineBox::new((l.range).clone(), (l.cluster_range).clone(), l.baseline, l.top, l.bottom, l.natural_width, l.adjusted_width, l.visual_width, Some(l.hanging_punctuation_advance), Some(-0.0f64), Some(l.end_reason), Some(-0.0f64), Some(vec![]), (l.debug).clone());
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new(&(UStr::new(&[27721])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new(TextRange::new(0u32, 1u32)?, &(UStr::new(&[27721])), &(UStr::new(&[99,106,107])), 16 as f64 as f64, Some(UString::from("汉")), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new(TextRange::new(0u32, 1u32)?, &(UStr::new(&[99,106,107])), vec![
    (Glyph::new(1u32, TextRange::new(0u32, 1u32)?, 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
], &vec![(nl).clone()], None, None, None, None, Some(TestHelpers::test_helpers_f32_literal(1.0e21f64)), Some(-0.0f64))?);
    }

    pub fn prepared_paragraph_plan_construction_test_support_escapes() -> Result<LayoutResult, TextRangeError> {
        let s = TestHelpers::test_helpers_surrogate_text(&vec![34, 92, 8, 12, 10, 13, 9, 1]);
        let r = TextRange::new(0u32, 8u32)?;
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new(s.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((r).clone(), s.as_ustr(), &(UStr::new(&[99,106,107])), 8 as f64 as f64, Some(s.to_ustring()), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((r).clone(), &(UStr::new(&[99,106,107])), vec![
    (Glyph::new(1u32, (r).clone(), 8 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 8 as f64 as f64, Some(vec![]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line((r).clone(), IntRange::new(0u32, 0u32), Some(8 as f64 as f64))).clone(),
], None, None, None, None, None, None)?);
    }

    pub fn prepared_paragraph_plan_construction_test_support_diagnostics() -> Result<LayoutResult, TextRangeError> {
        let a = TextRange::new(0u32, 1u32)?;
        let b = TextRange::new(1u32, 2u32)?;
        let c = TextRange::new(2u32, 3u32)?;
        let ds = vec![
    (ShapingDecisionInfo::new((a).clone(), &(UStr::new(&[27721])), &(UStr::new(&[27721])), &(UStr::new(&[99,106,107])), 1u32, 32 as f64 as f64, &(UStr::new(&[83,104,97,112,105,110,103,83,116,97,103,101])), &(UStr::new(&[99,97,112,97,98,105,108,105,116,121,45,114,101,97,115,111,110])), Some(0), Some(0), None, None, None, None, None, Some(UString::from("InvalidWebShapingAdvance")))).clone(),
    (ShapingDecisionInfo::new((c).clone(), &(UStr::new(&[33251])), &(UStr::new(&[33251])), &(UStr::new(&[99,106,107])), 1u32, f64::INFINITY, &(UStr::new(&[83,104,97,112,105,110,103,83,116,97,103,101])), &(UStr::new(&[105,110,102,105,110,105,116,101,45,99,97,112,97,98,105,108,105,116,121])), Some(0), Some(0), None, None, None, None, None, Some(UString::from("MissingInkBoundsFallback")))).clone(),
    (ShapingDecisionInfo::new((b).clone(), &(UStr::new(&[38646])), &(UStr::new(&[38646])), &(UStr::new(&[99,106,107])), 1u32, 0 as f64 as f64, &(UStr::new(&[83,104,97,112,105,110,103,83,116,97,103,101])), &(UStr::new(&[122,101,114,111,45,97,100,118,97,110,99,101])), Some(0), Some(0), None, None, None, None, None, None)).clone(),
    (ShapingDecisionInfo::new((c).clone(), &(UStr::new(&[33251])), &(UStr::new(&[33251])), &(UStr::new(&[99,106,107])), 1u32, f64::NAN, &(UStr::new(&[83,104,97,112,105,110,103,83,116,97,103,101])), &(UStr::new(&[110,97,110,45,97,100,118,97,110,99,101])), Some(0), Some(0), None, None, None, None, None, None)).clone(),
    (ShapingDecisionInfo::new((c).clone(), &(UStr::new(&[33251])), &(UStr::new(&[33251])), &(UStr::new(&[99,106,107])), 1u32, f64::INFINITY, &(UStr::new(&[83,104,97,112,105,110,103,83,116,97,103,101])), &(UStr::new(&[105,110,102,105,110,105,116,101,45,97,100,118,97,110,99,101])), Some(0), Some(0), None, None, None, None, None, None)).clone(),
];
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new(&(UStr::new(&[27721,38646,33251])), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((a).clone(), &(UStr::new(&[27721])), &(UStr::new(&[99,106,107])), 32 as f64 as f64, Some(UString::from("汉")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((b).clone(), &(UStr::new(&[38646])), &(UStr::new(&[99,106,107])), 0 as f64 as f64, Some(UString::from("零")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((c).clone(), &(UStr::new(&[33251])), &(UStr::new(&[99,106,107])), 16 as f64 as f64, Some(UString::from("臣")), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((a).clone(), &(UStr::new(&[99,106,107])), vec![
    (Glyph::new(1u32, (a).clone(), 32 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 32 as f64 as f64, Some(vec![]))).clone(),
    (GlyphRun::new((b).clone(), &(UStr::new(&[99,106,107])), vec![
    (Glyph::new(2u32, (b).clone(), 0 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 0 as f64 as f64, Some(vec![]))).clone(),
    (GlyphRun::new((c).clone(), &(UStr::new(&[99,106,107])), vec![
    (Glyph::new(3u32, (c).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line(TextRange::new(0u32, 3u32)?, IntRange::new(0u32, 2u32), Some(48 as f64 as f64))).clone(),
], None, None, None,
Some(LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some((ds).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))), None, None)?);
    }

    pub(crate) fn prepared_paragraph_plan_construction_test_support_begin(name: &UStr) -> TestTraceRecorder {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,114,101,112,97,114,101,100,80,97,114,97,103,114,97,112,104,80,108,97,110,67,111,110,115,116,114,117,99,116,105,111,110,84,101,115,116])));
        t.section(name);
        return t;
    }

    pub fn prepared_paragraph_plan_construction_test_support_run_evidence(name: &UStr, r: LayoutResult) -> Result<(), PreparedParagraphPlanConstructionTestSupportRunEvidenceFault> {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_begin(name);
        let j = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json((r).clone(), true).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::PreparedParagraphToPreparedParagraphJsonFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"dashStrategy\":\"PairedEmDash\"").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"shapingLanguage\":\"zh-Hans\"").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"resolvedFace\":\"NotoSansCJK\"").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"glyphIds\":\"9,10\"").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"shapingEvidence\":\"dash-reason\"").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"naturalWidth\":32").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn prepared_paragraph_plan_construction_test_support_run_punctuation() -> Result<(), PreparedParagraphPlanConstructionTestSupportRunPunctuationFault> {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_begin(UStr::new(&[112,117,110,99,116,117,97,116,105,111,110,73,110,107,70,108,111,111,114,65,110,100,76,97,116,105,110,82,111,108,101,77,97,114,107,67,101,108,108,115]));
        let j = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_punctuation_ink_floor_and_latin_role_mark_cells().map_err(|e| PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TextRangeErrorFault(e))?, true).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::PreparedParagraphToPreparedParagraphJsonFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"punctuationInkFloor\":6").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"punctuationBodyWidth\":16").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::wrapping_sub(u32::try_from((u_string::split(&j, &UString::from("\"punctuationInkFloor\":")).len()) & 0xFFFF_FFFF).unwrap_or(0), 1), None).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"latin\":true").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn prepared_paragraph_plan_construction_test_support_run_zero_width() -> Result<(), PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault> {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_begin(UStr::new(&[122,101,114,111,87,105,100,116,104,66,114,101,97,107,67,108,117,115,116,101,114,83,117,114,118,105,118,101,115,69,109,112,116,121,68,105,115,112,108,97,121,84,101,120,116]));
        let r = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_zero_width_break_cluster_survives_empty_display_text().map_err(|e| PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TextRangeErrorFault(e))?;
        let j = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json((r).clone(), false).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::PreparedParagraphToPreparedParagraphJsonFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"display\":\"\",\"drawX\":16").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_equals(3, u32::wrapping_sub(u32::try_from((u_string::split(&j, &UString::from("\"source\":")).len()) & 0xFFFF_FFFF).unwrap_or(0), 1), None).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsFailFaultFault(e))?;
        let e = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json((r).clone(), true).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::PreparedParagraphToPreparedParagraphJsonFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(e), UString::from("\"dashStrategy\":\"ZeroWidthNoShape\"").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((e).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(e), UString::from("\"shapingEvidence\":\"no-shape\"").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((e).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes(((u_string::find_from(&(e), UString::from("shapingLanguage").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((e).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes(((u_string::find_from(&(e), UString::from("resolvedFace").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((e).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes(((u_string::find_from(&(e), UString::from("glyphIds").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((e).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn prepared_paragraph_plan_construction_test_support_run_paragraph_evidence() -> Result<(), PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault> {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_begin(UStr::new(&[112,97,114,97,103,114,97,112,104,69,118,105,100,101,110,99,101,69,109,105,116,115,69,118,101,114,121,83,101,99,116,105,111,110]));
        let j = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_paragraph_evidence_emits_every_section().map_err(|e| PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TextRangeErrorFault(e))?, true).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::PreparedParagraphToPreparedParagraphJsonFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"emphasisRanges\":[[0,1],[1,2]]").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"inlineEdges\":[{\"offset\":0,\"inlineStart\":2.5},{\"offset\":2,\"inlineEnd\":4.5}]").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"rubyDecisions\":[{\"baseRangeStart\":0").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"ascent\":6").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"fontFamilies\":[\"RubyKai\",\"RubyLatin\"]").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"bopomofoDecisions\":[{\"baseRangeStart\":1").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"role\":\"Symbol\"").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"role\":\"Tone\"").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"fontFamilies\":[\"BopomofoKai\",\"BopomofoLatin\"]").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"decorationSegments\":[{\"kind\":\"ProperNoun\"").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"kind\":\"BookTitle\"").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("Emphasis").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"emphasisDots\":[{\"clusterRangeStart\":0,\"anchorX\":8,\"anchorY\":22,\"dotDiameter\":2}]").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"fontSize\":16").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"overlayWidth\":480").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn prepared_paragraph_plan_construction_test_support_run_negative_zero() -> Result<(), PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault> {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_begin(UStr::new(&[110,101,103,97,116,105,118,101,90,101,114,111,65,110,100,69,120,112,111,110,101,110,116,87,105,100,116,104,115,78,111,114,109,97,108,105,122,101]));
        let j = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_negative_zero_and_exponent_widths_normalize().map_err(|e| PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TextRangeErrorFault(e))?, false).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::PreparedParagraphToPreparedParagraphJsonFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"width\":1.0000000200408773e+21").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"height\":0").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"indent\":0").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"hyphenAdvance\":0").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn prepared_paragraph_plan_construction_test_support_run_escapes() -> Result<(), PreparedParagraphPlanConstructionTestSupportRunEscapesFault> {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_begin(UStr::new(&[106,115,111,110,83,116,114,105,110,103,69,115,99,97,112,101,115,81,117,111,116,101,115,66,97,99,107,115,108,97,115,104,101,115,65,110,100,67,111,110,116,114,111,108,67,104,97,114,97,99,116,101,114,115]));
        let j = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_escapes().map_err(|e| PreparedParagraphPlanConstructionTestSupportRunEscapesFault::TextRangeErrorFault(e))?, false).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunEscapesFault::PreparedParagraphToPreparedParagraphJsonFaultFault(e))?;
        let ss = vec![
    UString::from("\\\"").to_ustring(),
    UString::from("\\\\").to_ustring(),
    UString::from("\\b").to_ustring(),
    UString::from("\\f").to_ustring(),
    UString::from("\\n").to_ustring(),
    UString::from("\\r").to_ustring(),
    UString::from("\\t").to_ustring(),
    UString::from("\\u0001").to_ustring(),
];
        for s in &ss {
            let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(j), (s).as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunEscapesFault::TracedAssertionsFailFaultFault(e))?;
        }
        Ok(())
    }

    pub fn prepared_paragraph_plan_construction_test_support_run_diagnostics() -> Result<(), PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault> {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_begin(UStr::new(&[112,108,97,110,87,105,116,104,68,105,97,103,110,111,115,116,105,99,115,76,105,115,116,115,67,97,112,97,98,105,108,105,116,121,73,115,115,117,101,115,65,110,100,65,100,118,97,110,99,101,83,117,115,112,101,99,116,115]));
        let j = PreparedParagraphFns::prepared_paragraph_fns_to_plan_with_diagnostics_json(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_diagnostics().map_err(|e| PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TextRangeErrorFault(e))?, false, 0.5f64).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::PreparedParagraphToPlanWithDiagnosticsJsonFaultFault(e))?;
        let d = u_string::substr(&j, i32::from_ne_bytes(((u_string::find_from(&(j), UString::from("\"diagnostics\":").as_ustr(), 0)) as i32).to_ne_bytes()), None);
        {
            {
                let s = UString::from("\"name\":\"InvalidWebShapingAdvance\"").to_ustring();
                let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(d), (s).as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
            }
            {
                let s = UString::from("\"reason\":\"capability-reason\"").to_ustring();
                let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(d), (s).as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
            }
            {
                let s = UString::from("\"rangeStart\":0").to_ustring();
                let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(d), (s).as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
            }
            {
                let s = UString::from("\"rangeEnd\":1").to_ustring();
                let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(d), (s).as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
            }
            {
                let s = UString::from("\"displayText\":\"零\"").to_ustring();
                let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(d), (s).as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
            }
            {
                let s = UString::from("\"advance\":\"0\"").to_ustring();
                let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(d), (s).as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
            }
            {
                let s = UString::from("\"advance\":\"NaN\"").to_ustring();
                let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(d), (s).as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
            }
            {
                let s = UString::from("\"advance\":\"Infinity\"").to_ustring();
                let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&(d), (s).as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes(((u_string::find_from(&(d), UString::from("\"advance\":\"32\"").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some((j).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes(((u_string::find_from(&(j), UString::from("{\"plan\":\"").as_ustr(), 0)) as u32).to_ne_bytes()) == 0, Some((u_string::substr(&j, 0i32, Some(20i32))).to_ustring())).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }
}
