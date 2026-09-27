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
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PreparedParagraphPlanConstructionTestSupportRunPunctuationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunPunctuationFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
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

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphPlanConstructionTestSupportRunPunctuationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphPlanConstructionTestSupportRunPunctuationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TextRangeErrorFault(value)
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
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertFalseFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
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

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TextRangeErrorFault(value)
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
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
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

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TextRangeErrorFault(value)
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
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    PreparedParagraphToPreparedParagraphJsonFaultFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PreparedParagraphPlanConstructionTestSupportRunEscapesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphPlanConstructionTestSupportRunEscapesFault) -> Self {
        match value {
            PreparedParagraphPlanConstructionTestSupportRunEscapesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
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

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphPlanConstructionTestSupportRunEscapesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunEscapesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphPlanConstructionTestSupportRunEscapesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphPlanConstructionTestSupportRunEscapesFault::TextRangeErrorFault(value)
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

    pub(crate) fn prepared_paragraph_plan_construction_test_support_result(content: TiqianTextContent, clusters: &Vec<Cluster>, runs: &Vec<GlyphRun>, lines: &Vec<LineBox>, decorations: Option<Vec<DecorationSpan>>, boxes: Option<Vec<InlineBoxSpan>>, objects:
Option<Vec<InlineObjectSpan>>, debug: Option<LayoutDebugInfo>, width: Option<f64>, height: Option<f64>) -> Result<LayoutResult, TextRangeError> {
        let w = match &(width) { None => 480.0f64, Some(__option1) => *__option1 };
        let h = match &(height) { None => 24.0f64, Some(__option2) => *__option2 };
        return Ok(LayoutResult::new(LayoutInput::new((content).clone(), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start),
Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(w, Some(f64::INFINITY), Some(2147483647))?,
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), (decorations).clone(), Some(vec![]), (boxes).clone(), (objects).clone()), Size::new(w, h), (clusters).clone(), (runs).clone(), (lines).clone(), match &(debug) { None =>
LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None,
Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(__option3) => (*__option3).clone() }));
    }

    pub(crate) fn prepared_paragraph_plan_construction_test_support_one(text: &str, font: Option<String>, advance: Option<f64>) -> Result<LayoutResult, TextRangeError> {
        let a = match &(advance) { None => 16.0f64, Some(__option4) => *__option4 };
        let f = match &(font) { None => "cjk".to_string(), Some(__option5) => __option5.to_string() };
        let r = TextRange::new(0u32, u_string::unit_count(&(text)))?;
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((r).clone(), text, f.as_str(), a, Some(text.to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((r).clone(), f.as_str(), vec![(Glyph::new(1u32, (r).clone(), a, Some(0.0), Some(0.0), None, None, None, None)).clone()].to_vec(), a, Some(vec![]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line((r).clone(), IntRange::new(0u32, 0u32), Some(a))).clone(),
], None, None, None, None, None, None)?);
    }

    pub fn prepared_paragraph_plan_construction_test_support_open_type_features_and_render_font_family_attach_per_cluster() -> Result<LayoutResult, TextRangeError> {
        let r = TextRange::new(0u32, 1u32)?;
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new("汉", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((r).clone(), "汉", "cjk", 16 as f64 as f64, Some("汉".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((r).clone(), "cjk", vec![
    (Glyph::new(7u32, (r).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), Some("Noto Serif CJK".to_string()), None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec!["kern".to_string(), "liga".to_string()]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line((r).clone(), IntRange::new(0u32, 0u32), None)).clone(),
], None, None, None, None, None, None)?);
    }

    pub fn prepared_paragraph_plan_construction_test_support_multi_unit_cluster_marks_shaping_boundary() -> Result<LayoutResult, TextRangeError> {
        let r = TextRange::new(0u32, 2u32)?;
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new("AB", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((r).clone(), "AB", "latin", 18 as f64 as f64, Some("AB".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((r).clone(), "latin", vec![
    (Glyph::new(1u32, (r).clone(), 18 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 18 as f64 as f64, Some(vec![]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line((r).clone(), IntRange::new(0u32, 0u32), None)).clone(),
], None, None, None, None, None, None)?);
    }

    pub fn prepared_paragraph_plan_construction_test_support_inline_object_cell_emits_advance_override() -> Result<LayoutResult, TextRangeError> {
        let a = TextRange::new(0u32, 1u32)?;
        let b = TextRange::new(1u32, 2u32)?;
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new("汉图", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((a).clone(), "汉", "cjk", 16 as f64 as f64, Some("汉".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((b).clone(), "图", "inline", 10 as f64 as f64, Some("图".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((a).clone(), "cjk", vec![
    (Glyph::new(1u32, (a).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
    (GlyphRun::new((b).clone(), "inline", vec![
    (Glyph::new(2u32, (b).clone(), 24 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 24 as f64 as f64, Some(vec![]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line(TextRange::new(0u32, 2u32)?, IntRange::new(0u32, 1u32), Some(40 as f64 as f64))).clone(),
], None, None, Some(vec![
    (InlineObjectSpan::new((b).clone(), 24 as f64 as f64, 12 as f64 as f64, 4 as f64 as f64, Some((InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed()?).clone()),
Some((InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed()?).clone()))?).clone(),
]), None, None, None)?);
    }

    pub fn prepared_paragraph_plan_construction_test_support_style_delta_lists_only_paint_fields() -> Result<LayoutResult, TextRangeError> {
        let a = TextRange::new(0u32, 1u32)?;
        let b = TextRange::new(1u32, 2u32)?;
        let c = TextRange::new(2u32, 3u32)?;
        let content = TiqianTextContent::new("汉A字", Some(vec![
    (TextSpan::new((a).clone(), TextStyle::new(Some(vec![]), Some(20 as f64), Some("zh-Hans".to_string()), Some(700), Some(true), Some(0.0), Some(InlineAttachment::None)))).clone(),
    (TextSpan::new((b).clone(), TextStyle::new(Some(vec!["Kai".to_string()]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![]));
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result((content).clone(), &vec![
    (Cluster::new((a).clone(), "汉", "cjk", 16 as f64 as f64, Some("汉".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((b).clone(), "A", "latin", 10 as f64 as f64, Some("A".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((c).clone(), "字", "cjk", 16 as f64 as f64, Some("字".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((a).clone(), "cjk", vec![
    (Glyph::new(1u32, (a).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
    (GlyphRun::new((b).clone(), "latin", vec![
    (Glyph::new(2u32, (b).clone(), 10 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 10 as f64 as f64, Some(vec![]))).clone(),
    (GlyphRun::new((c).clone(), "cjk", vec![
    (Glyph::new(3u32, (c).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line(TextRange::new(0u32, 3u32)?, IntRange::new(0u32, 2u32), Some(42 as f64 as f64))).clone(),
], None, None, None, None, None, None)?);
    }

    pub fn prepared_paragraph_plan_construction_test_support_dash_cluster_emits_shaping_evidence_block() -> Result<LayoutResult, TextRangeError> {
        let a = TextRange::new(0u32, 1u32)?;
        let b = TextRange::new(1u32, 3u32)?;
        let d = ShapingDecisionInfo::new((b).clone(), "——", "——", "cjk", 2u32, 32 as f64 as f64, "ShapingStage", "dash-reason", Some(0), Some(0), Some("NotoSansCJK".to_string()), None, Some("zh-Hans".to_string()), Some("PairedEmDash".to_string()), None, None);
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new("汉——", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((a).clone(), "汉", "cjk", 16 as f64 as f64, Some("汉".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((b).clone(), "——", "cjk", 32 as f64 as f64, Some("——".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((a).clone(), "cjk", vec![
    (Glyph::new(1u32, (a).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
    (GlyphRun::new((b).clone(), "cjk", vec![
    (Glyph::new(9u32, (b).clone(), 32 as f64 as f64, Some(0.0), Some(0.0), Some("Noto Sans CJK".to_string()), None, None, None)).clone(),
    (Glyph::new(10u32, (b).clone(), 0 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 32 as f64 as f64, Some(vec![]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line(TextRange::new(0u32, 3u32)?, IntRange::new(0u32, 1u32), Some(48 as f64 as f64))).clone(),
], None, None, None, Some(LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(d).clone()]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))), None, None)?);
    }

    pub fn prepared_paragraph_plan_construction_test_support_punctuation_ink_floor_and_latin_role_mark_cells() -> Result<LayoutResult, TextRangeError> {
        let a = TextRange::new(0u32, 1u32)?;
        let b = TextRange::new(1u32, 2u32)?;
        let c = TextRange::new(2u32, 3u32)?;
        let ps = vec![
    (PunctuationDecisionInfo::new((a).clone(), "。", "PauseOrStop", 16 as f64 as f64, 16 as f64 as f64, 0 as f64 as f64, 0 as f64 as f64, "centre", None, Some("PolicyDerived".to_string()), 16 as f64 as f64, None, None, Some(6 as f64), Some(true), None, None, None, Some(0.0),
Some(0.0), None, Some(0.0), Some(0.0))).clone(),
    (PunctuationDecisionInfo::new((b).clone(), "A", "Other", 10 as f64 as f64, 10 as f64 as f64, 0 as f64 as f64, 0 as f64 as f64, "centre", None, Some("PolicyDerived".to_string()), 10 as f64 as f64, None, None, None, Some(false), None, None, None, Some(0.0), Some(0.0), None,
Some(0.0), Some(0.0))).clone(),
    (PunctuationDecisionInfo::new((c).clone(), "中", "Other", 16 as f64 as f64, 16 as f64 as f64, 0 as f64 as f64, 0 as f64 as f64, "centre", None, Some("PolicyDerived".to_string()), 16 as f64 as f64, None, None, None, Some(false), None, None, None, Some(0.0), Some(0.0), None,
Some(0.0), Some(0.0))).clone(),
];
        let fd = vec![
    (FontDecisionInfo::new((b).clone(), "A", "A", "LatinText", "latin", "latin-run", "none")).clone(),
];
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new("。A中", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((a).clone(), "。", "cjk", 16 as f64 as f64, Some("。".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((b).clone(), "A", "latin", 10 as f64 as f64, Some("A".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((c).clone(), "中", "cjk", 16 as f64 as f64, Some("中".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((a).clone(), "cjk", vec![
    (Glyph::new(1u32, (a).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
    (GlyphRun::new((b).clone(), "latin", vec![
    (Glyph::new(2u32, (b).clone(), 10 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 10 as f64 as f64, Some(vec![]))).clone(),
    (GlyphRun::new((c).clone(), "cjk", vec![
    (Glyph::new(3u32, (c).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line(TextRange::new(0u32, 3u32)?, IntRange::new(0u32, 2u32), Some(42 as f64 as f64))).clone(),
], None, None, None, Some(LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some((fd).clone()), Some(vec![]), Some((ps).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))), None, None)?);
    }

    pub fn prepared_paragraph_plan_construction_test_support_zero_width_break_cluster_survives_empty_display_text() -> Result<LayoutResult, TextRangeError> {
        let a = TextRange::new(0u32, 1u32)?;
        let b = TextRange::new(1u32, 2u32)?;
        let c = TextRange::new(2u32, 3u32)?;
        let z = ShapingDecisionInfo::new((b).clone(), "​", "", "cjk", 0u32, 0 as f64 as f64, "ShapingStage", "no-shape", Some(0), Some(0), None, None, None, Some("ZeroWidthNoShape".to_string()), None, None);
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new("汉​字", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((a).clone(), "汉", "cjk", 16 as f64 as f64, Some("汉".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((b).clone(), "​", "cjk", 0 as f64 as f64, Some("".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((c).clone(), "字", "cjk", 16 as f64 as f64, Some("字".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((a).clone(), "cjk", vec![
    (Glyph::new(1u32, (a).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
    (GlyphRun::new((c).clone(), "cjk", vec![
    (Glyph::new(3u32, (c).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line(TextRange::new(0u32, 3u32)?, IntRange::new(0u32, 2u32), Some(32 as f64 as f64))).clone(),
], None, None, None, Some(LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(z).clone()]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![
    (ZeroWidthBreakDecisionInfo::new((b).clone(), "​", 1u32, Some("ZeroWidthSpaceSoftBreakNoShape".to_string()))).clone(),
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
    (DecorationDecisionInfo::new((a).clone(), "汉", "Emphasis", true, "dot-applied", Some(8 as f64), Some(22 as f64), Some(2 as f64))).clone(),
    (DecorationDecisionInfo::new((b).clone(), "注", "Emphasis", true, "dot-without-size", Some(24 as f64), Some(22 as f64), Some(0 as f64))).clone(),
    (DecorationDecisionInfo::new((b).clone(), "注", "Emphasis", false, "dot-skipped", Some(24 as f64), Some(22 as f64), Some(2 as f64))).clone(),
];
        let seg = vec![
    (DecorationSegmentInfo::new((a).clone(), "ProperNoun", 0u32, 0 as f64 as f64, 20 as f64 as f64, 16 as f64 as f64, 22 as f64 as f64, false, false, "proper-noun")).clone(),
    (DecorationSegmentInfo::new((b).clone(), "BookTitle", 0u32, 16 as f64 as f64, 20 as f64 as f64, 32 as f64 as f64, 22 as f64 as f64, false, false, "book-title")).clone(),
];
        let ruby = vec![
    (RubyDecisionInfo::new((a).clone(), "hàn", 0u32, 8 as f64 as f64, 2 as f64 as f64, 8 as f64 as f64, 0.5f64, Some(6 as f64), Some(0.0), Some(0.0), Some(vec!["RubyKai".to_string(), "RubyLatin".to_string()]), Some(400), Some("zh-Hans".to_string()), Some(vec![]))).clone(),
    (RubyDecisionInfo::new((b).clone(), "zhù", 0u32, 24 as f64 as f64, 2 as f64 as f64, 8 as f64 as f64, 0 as f64 as f64, Some(0.0), Some(0.0), Some(0.0), Some(vec![]), Some(400), Some("zh-Hans".to_string()), Some(vec![]))).clone(),
];
        let bop = vec![
    (BopomofoDecisionInfo::new((b).clone(), "ㄓㄨˋ", 0u32, vec![
    (BopomofoGlyphPlacement::new("ㄓ", 1 as f64 as f64, 2 as f64 as f64, 4 as f64 as f64, 4 as f64 as f64, BopomofoGlyphRole::Symbol, Some(vec![]), 1 as f64 as f64, 6 as f64 as f64, 4 as f64 as f64)).clone(),
    (BopomofoGlyphPlacement::new("ˋ", 2 as f64 as f64, 0 as f64 as f64, 2 as f64 as f64, 2 as f64 as f64, BopomofoGlyphRole::Tone, Some(vec![]), 2 as f64 as f64, 2 as f64 as f64, 2 as f64 as f64)).clone(),
].to_vec(), Some(vec!["BopomofoKai".to_string(), "BopomofoLatin".to_string()]), Some(400), Some("zh-Hans".to_string()))).clone(),
    (BopomofoDecisionInfo::new((a).clone(), "ㄏㄢˋ", 0u32, vec![
    (BopomofoGlyphPlacement::new("ㄏ", 0 as f64 as f64, 2 as f64 as f64, 4 as f64 as f64, 4 as f64 as f64, BopomofoGlyphRole::Symbol, Some(vec![]), 0 as f64 as f64, 6 as f64 as f64, 4 as f64 as f64)).clone(),
].to_vec(), Some(vec![]), Some(400), Some("zh-Hans".to_string()))).clone(),
];
        let debug = LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some((ruby).clone()), Some((bop).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some((dec).clone()),
Some((seg).clone()), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new("汉注", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((a).clone(), "汉", "cjk", 16 as f64 as f64, Some("汉".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((b).clone(), "注", "cjk", 16 as f64 as f64, Some("注".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new(TextRange::new(0u32, 2u32)?, "cjk", vec![
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
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_one(&"汉", None, None)?;
        let l = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line(TextRange::new(0u32, 1u32)?, IntRange::new(0u32, 0u32), None);
        let nl = LineBox::new((l.range).clone(), (l.cluster_range).clone(), l.baseline, l.top, l.bottom, l.natural_width, l.adjusted_width, l.visual_width, Some(l.hanging_punctuation_advance), Some(-0.0f64), Some(l.end_reason), Some(-0.0f64), Some(vec![]), (l.debug).clone());
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new("汉", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new(TextRange::new(0u32, 1u32)?, "汉", "cjk", 16 as f64 as f64, Some("汉".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new(TextRange::new(0u32, 1u32)?, "cjk", vec![
    (Glyph::new(1u32, TextRange::new(0u32, 1u32)?, 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
], &vec![(nl).clone()], None, None, None, None, Some(TestHelpers::test_helpers_f32_literal(1.0e21f64)), Some(-0.0f64))?);
    }

    pub fn prepared_paragraph_plan_construction_test_support_escapes() -> Result<LayoutResult, TextRangeError> {
        let s = TestHelpers::test_helpers_surrogate_text(&vec![34, 92, 8, 12, 10, 13, 9, 1]);
        let r = TextRange::new(0u32, 8u32)?;
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new(s.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((r).clone(), s.as_str(), "cjk", 8 as f64 as f64, Some(s.to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((r).clone(), "cjk", vec![
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
    (ShapingDecisionInfo::new((a).clone(), "汉", "汉", "cjk", 1u32, 32 as f64 as f64, "ShapingStage", "capability-reason", Some(0), Some(0), None, None, None, None, None, Some("InvalidWebShapingAdvance".to_string()))).clone(),
    (ShapingDecisionInfo::new((c).clone(), "臣", "臣", "cjk", 1u32, f64::INFINITY, "ShapingStage", "infinite-capability", Some(0), Some(0), None, None, None, None, None, Some("MissingInkBoundsFallback".to_string()))).clone(),
    (ShapingDecisionInfo::new((b).clone(), "零", "零", "cjk", 1u32, 0 as f64 as f64, "ShapingStage", "zero-advance", Some(0), Some(0), None, None, None, None, None, None)).clone(),
    (ShapingDecisionInfo::new((c).clone(), "臣", "臣", "cjk", 1u32, f64::NAN, "ShapingStage", "nan-advance", Some(0), Some(0), None, None, None, None, None, None)).clone(),
    (ShapingDecisionInfo::new((c).clone(), "臣", "臣", "cjk", 1u32, f64::INFINITY, "ShapingStage", "infinite-advance", Some(0), Some(0), None, None, None, None, None, None)).clone(),
];
        return Ok(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_result(TiqianTextContent::new("汉零臣", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), &vec![
    (Cluster::new((a).clone(), "汉", "cjk", 32 as f64 as f64, Some("汉".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((b).clone(), "零", "cjk", 0 as f64 as f64, Some("零".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new((c).clone(), "臣", "cjk", 16 as f64 as f64, Some("臣".to_string()), Some(0.0), Some(0.0), Some(0.0))).clone(),
], &vec![
    (GlyphRun::new((a).clone(), "cjk", vec![
    (Glyph::new(1u32, (a).clone(), 32 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 32 as f64 as f64, Some(vec![]))).clone(),
    (GlyphRun::new((b).clone(), "cjk", vec![
    (Glyph::new(2u32, (b).clone(), 0 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 0 as f64 as f64, Some(vec![]))).clone(),
    (GlyphRun::new((c).clone(), "cjk", vec![
    (Glyph::new(3u32, (c).clone(), 16 as f64 as f64, Some(0.0), Some(0.0), None, None, None, None)).clone(),
].to_vec(), 16 as f64 as f64, Some(vec![]))).clone(),
], &vec![
    (PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_line(TextRange::new(0u32, 3u32)?, IntRange::new(0u32, 2u32), Some(48 as f64 as f64))).clone(),
], None, None, None, Some(LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some((ds).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))), None, None)?);
    }

    pub(crate) fn prepared_paragraph_plan_construction_test_support_begin(name: &str) -> TestTraceRecorder {
        let mut t = TestTraceRecorder::new("PreparedParagraphPlanConstructionTest");
        t.section(name);
        return t;
    }

    pub fn prepared_paragraph_plan_construction_test_support_run_evidence(name: &str, r: LayoutResult) -> Result<(), PreparedParagraphPlanConstructionTestSupportRunEvidenceFault> {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_begin(name);
        let j = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json((r).clone(), true).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::PreparedParagraphToPreparedParagraphJsonFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"dashStrategy\":\"PairedEmDash\"", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"shapingLanguage\":\"zh-Hans\"", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"resolvedFace\":\"NotoSansCJK\"", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"glyphIds\":\"9,10\"", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"shapingEvidence\":\"dash-reason\"", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"naturalWidth\":32", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn prepared_paragraph_plan_construction_test_support_run_punctuation() -> Result<(), PreparedParagraphPlanConstructionTestSupportRunPunctuationFault> {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_begin(&"punctuationInkFloorAndLatinRoleMarkCells");
        let j = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_punctuation_ink_floor_and_latin_role_mark_cells().map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TextRangeErrorFault(e))?, true).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::PreparedParagraphToPreparedParagraphJsonFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"punctuationInkFloor\":6", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"punctuationBodyWidth\":16", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::wrapping_sub(u32::try_from((u_string::split(&j, &"\"punctuationInkFloor\":").len()) & 0xFFFF_FFFF).unwrap_or(0), 1), None).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"latin\":true", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunPunctuationFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn prepared_paragraph_plan_construction_test_support_run_zero_width() -> Result<(), PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault> {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_begin(&"zeroWidthBreakClusterSurvivesEmptyDisplayText");
        let r = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_zero_width_break_cluster_survives_empty_display_text().map_err(|e| PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TextRangeErrorFault(e))?;
        let j = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json((r).clone(), false).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::PreparedParagraphToPreparedParagraphJsonFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"display\":\"\",\"drawX\":16", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_equals(3, u32::wrapping_sub(u32::try_from((u_string::split(&j, &"\"source\":").len()) & 0xFFFF_FFFF).unwrap_or(0), 1), None).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsFailFaultFault(e))?;
        let e = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json((r).clone(), true).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::PreparedParagraphToPreparedParagraphJsonFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&e, "\"dashStrategy\":\"ZeroWidthNoShape\"", 0)).to_ne_bytes())) <= 2147483647, Some((e).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&e, "\"shapingEvidence\":\"no-shape\"", 0)).to_ne_bytes())) <= 2147483647, Some((e).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&e, "shapingLanguage", 0)).to_ne_bytes())) <= 2147483647, Some((e).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&e, "resolvedFace", 0)).to_ne_bytes())) <= 2147483647, Some((e).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&e, "glyphIds", 0)).to_ne_bytes())) <= 2147483647, Some((e).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunZeroWidthFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn prepared_paragraph_plan_construction_test_support_run_paragraph_evidence() -> Result<(), PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault> {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_begin(&"paragraphEvidenceEmitsEverySection");
        let j = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_paragraph_evidence_emits_every_section().map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TextRangeErrorFault(e))?, true).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::PreparedParagraphToPreparedParagraphJsonFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"emphasisRanges\":[[0,1],[1,2]]", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"inlineEdges\":[{\"offset\":0,\"inlineStart\":2.5},{\"offset\":2,\"inlineEnd\":4.5}]", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"rubyDecisions\":[{\"baseRangeStart\":0", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"ascent\":6", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"fontFamilies\":[\"RubyKai\",\"RubyLatin\"]", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"bopomofoDecisions\":[{\"baseRangeStart\":1", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"role\":\"Symbol\"", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"role\":\"Tone\"", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"fontFamilies\":[\"BopomofoKai\",\"BopomofoLatin\"]", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"decorationSegments\":[{\"kind\":\"ProperNoun\"", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"kind\":\"BookTitle\"", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&j, "Emphasis", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"emphasisDots\":[{\"clusterRangeStart\":0,\"anchorX\":8,\"anchorY\":22,\"dotDiameter\":2}]", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"fontSize\":16", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"overlayWidth\":480", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunParagraphEvidenceFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn prepared_paragraph_plan_construction_test_support_run_negative_zero() -> Result<(), PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault> {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_begin(&"negativeZeroAndExponentWidthsNormalize");
        let j = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_negative_zero_and_exponent_widths_normalize().map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TextRangeErrorFault(e))?, false).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::PreparedParagraphToPreparedParagraphJsonFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"width\":1.0000000200408773e+21", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"height\":0", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"indent\":0", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, "\"hyphenAdvance\":0", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunNegativeZeroFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }

    pub fn prepared_paragraph_plan_construction_test_support_run_escapes() -> Result<(), PreparedParagraphPlanConstructionTestSupportRunEscapesFault> {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_begin(&"jsonStringEscapesQuotesBackslashesAndControlCharacters");
        let j = PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_escapes().map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunEscapesFault::TextRangeErrorFault(e))?, false).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunEscapesFault::PreparedParagraphToPreparedParagraphJsonFaultFault(e))?;
        let ss = vec![
    "\\\"".to_string(),
    "\\\\".to_string(),
    "\\b".to_string(),
    "\\f".to_string(),
    "\\n".to_string(),
    "\\r".to_string(),
    "\\t".to_string(),
    "\\u0001".to_string(),
];
        for s in &ss {
            let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&j, (s).as_str(), 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunEscapesFault::TracedAssertionsFailFaultFault(e))?;
        }
        Ok(())
    }

    pub fn prepared_paragraph_plan_construction_test_support_run_diagnostics() -> Result<(), PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault> {
        let _ = PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_begin(&"planWithDiagnosticsListsCapabilityIssuesAndAdvanceSuspects");
        let j = PreparedParagraphFns::prepared_paragraph_fns_to_plan_with_diagnostics_json(PreparedParagraphPlanConstructionTestSupport::prepared_paragraph_plan_construction_test_support_diagnostics().map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TextRangeErrorFault(e))?, false, 0.5f64).map_err(|e| PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::PreparedParagraphToPlanWithDiagnosticsJsonFaultFault(e))?;
        let d = u_string::substr(&j, i32::from_ne_bytes((u_string::find_from(&j, "\"diagnostics\":", 0)).to_ne_bytes()), None);
        {
            {
                let s = "\"name\":\"InvalidWebShapingAdvance\"".to_string();
                let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&d, (s).as_str(), 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
            }
            {
                let s = "\"reason\":\"capability-reason\"".to_string();
                let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&d, (s).as_str(), 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
            }
            {
                let s = "\"rangeStart\":0".to_string();
                let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&d, (s).as_str(), 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
            }
            {
                let s = "\"rangeEnd\":1".to_string();
                let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&d, (s).as_str(), 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
            }
            {
                let s = "\"displayText\":\"零\"".to_string();
                let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&d, (s).as_str(), 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
            }
            {
                let s = "\"advance\":\"0\"".to_string();
                let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&d, (s).as_str(), 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
            }
            {
                let s = "\"advance\":\"NaN\"".to_string();
                let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&d, (s).as_str(), 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
            }
            {
                let s = "\"advance\":\"Infinity\"".to_string();
                let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&d, (s).as_str(), 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_false((u32::from_ne_bytes((u_string::find_from(&d, "\"advance\":\"32\"", 0)).to_ne_bytes())) <= 2147483647, Some((j).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes((u_string::find_from(&j, "{\"plan\":\"", 0)).to_ne_bytes()) == 0, Some((u_string::substr(&j, 0i32, Some(20i32))).to_string())).map_err(|e|
PreparedParagraphPlanConstructionTestSupportRunDiagnosticsFault::TracedAssertionsFailFaultFault(e))?;
        Ok(())
    }
}
