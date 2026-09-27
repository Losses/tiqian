#![cfg(test)]

use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_object_boundary_adjustment::InlineObjectBoundaryAdjustment;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::line_break_policy::LineBreakPolicy;
use crate::org::tiqian::core::line_break_span::LineBreakSpan;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::layout::line_adjustment_stage_coverage_test_support::LineAdjustmentStageCoverageTestSupport;
use crate::org::tiqian::layout::line_adjustment_stage_coverage_test_support::ZeroSpaceShaper;
use crate::org::tiqian::test::test_helpers::TestHelpers;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestZeroAdvanceEdgeSpaceIsNeverCollapsedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestTrailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphenFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsIntRangeFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault {
    fn from(value: LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault> for LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault) -> Self {
        LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestTinyTechnicalTrackingStaysBelowTheRejectionThresholdFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestTechnicalLineBodyStretchRejectsTheCleanTierAndReplaysFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsAssertEqualsIntRangeFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault {
    fn from(value: LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault> for LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault) -> Self {
        LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestMandatoryBreakMiddleLineSkipsItsJustificationPlanFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestLoneMandatoryBreakEmitsTwoZeroWidthLinesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsIntRangeFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault),
    TracedAssertionsAssertEqualsStringArrayFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringArrayFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault {
    fn from(value: LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringArrayFault {
    fn from(value: LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault::TracedAssertionsAssertEqualsStringArrayFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault> for LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault) -> Self {
        LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringArrayFault> for LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringArrayFault) -> Self {
        LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault::TracedAssertionsAssertEqualsStringArrayFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestLoneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKeyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsIntRangeFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault> for LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFitsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsAssertEqualsFloatToleranceFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatToleranceFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatToleranceFault {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault::TracedAssertionsAssertEqualsFloatToleranceFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatToleranceFault> for LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatToleranceFault) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault::TracedAssertionsAssertEqualsFloatToleranceFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheWordSpaceRawAdvanceChannelFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault {
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeConsumesTheInterpunctPairedChannelFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault {
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestHyphenSqueezeConsumesOpeningAndClosingBracketGlueChannelsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsIntRangeFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault {
    fn from(value: LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault> for LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault) -> Self {
        LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestFormulaObjectWithoutBoundaryDiscardsNothingAtLineEndFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsIntRangeFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault {
    fn from(value: LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault> for LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault) -> Self {
        LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestFormulaLineEndDiscardsTheTrailingBoundaryAdvanceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestEmptyTextYieldsZeroHeightWithoutLinesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestEmergencySelectedBreakOpensThePreferredTrackingSpanFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestDashRunWithoutInkBoundsKeepsSyntheticGlyphsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsAssertEqualsIntRangeFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault {
    fn from(value: LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault> for LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault) -> Self {
        LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestBlankMiddleLineSkipsEveryEdgePassFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestBaselineShiftSpanRaisesTheFinalClusterShiftFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsIntRangeFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault {
    fn from(value: LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault> for LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault) -> Self {
        LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestAttachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdgeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertEqualsIntRangeFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault),
    TracedAssertionsAssertEqualsRenderedFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault),
    TracedAssertionsAssertEqualsFloatFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault {
    fn from(value: LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault {
    fn from(value: LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault::TracedAssertionsAssertEqualsRenderedFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault {
    fn from(value: LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault::TracedAssertionsAssertEqualsFloatFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault) -> Self {
        match value {
            LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault> for LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntRangeFault) -> Self {
        LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault::TracedAssertionsAssertEqualsIntRangeFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault> for LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsRenderedFault) -> Self {
        LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault::TracedAssertionsAssertEqualsRenderedFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault> for LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatFault) -> Self {
        LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault::TracedAssertionsAssertEqualsFloatFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageCoverageTestAttachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRunFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn attached_footnote_trailing_glue_trims_when_the_line_ends_at_the_run() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.attachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRun", "org.tiqian.layout.LineAdjustmentStageCoverageTest.attachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRun", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"attachedFootnoteTrailingGlueTrimsWhenTheLineEndsAtTheRun");
        let text = "正文：“内容。”[1]后文".to_string();
        let r = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(text.as_str(), 164.0f64, Some(vec![
    (TextSpan::new(TextRange::new(8u32, 11u32).unwrap(), TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::Previous)))).clone(),
]), None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 8u32), ((r.lines[0usize]).clone().cluster_range).clone(),
Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
        let trim = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_trim_by_reason((r).clone(), &"AttachedInlineVirtualBoundaryLineEndTrim").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(8u32, 11u32).unwrap().to_string().as_str(), (trim.cluster_range).clone().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8.0f64, trim.trim_amount, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"trailing", (trim.side).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn attached_object_mark_hangs_instead_of_leaving_the_separator_at_an_edge() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.attachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdge", "org.tiqian.layout.LineAdjustmentStageCoverageTest.attachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdge", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"attachedObjectMarkHangsInsteadOfLeavingTheSeparatorAtAnEdge");
        let text = format!("{}{}{}",
            "中",
            InlineObjectSpan::INLINE_OBJECT_SPAN_INLINE_OBJECT_REPLACEMENT_CHAR.to_string(),
            " ，中"
        );
        let r = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(text.as_str(), 48.0f64, None, Some(vec![
    (InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 100.0f64, 12.0f64, 12.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
]), None, None, None).unwrap();
        let mut hung = (r.lines[0usize]).clone();
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.lines[usize::try_from(i).unwrap_or(0)].hanging_punctuation_advance > (0.0f64) {
                hung = (r.lines[usize::try_from(i).unwrap_or(0)]).clone();
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(1u32, 3u32), (hung.cluster_range).clone(), Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
        let mut none_collapse = true;
        for i in 0..match u32::try_from((r.debug).clone().line_edge_trim_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().line_edge_trim_decisions[usize::try_from(i).unwrap_or(0)].clone().reason.to_string() == "LineEdgeWordSpaceCollapse" {
                none_collapse = false;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none_collapse, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_trims(&(r.debug).clone().line_edge_trim_decisions)).to_string())).unwrap();
    });
}

#[test]
fn baseline_shift_span_raises_the_final_cluster_shift() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.baselineShiftSpanRaisesTheFinalClusterShift", "org.tiqian.layout.LineAdjustmentStageCoverageTest.baselineShiftSpanRaisesTheFinalClusterShift", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"baselineShiftSpanRaisesTheFinalClusterShift");
        let r = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(&"中文正文", 200.0f64, Some(vec![
    (TextSpan::new(TextRange::new(0u32, 2u32).unwrap(), TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(4.0f64), Some(InlineAttachment::None)))).clone(),
]), None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4.0f64, r.clusters[0usize].baseline_shift, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4.0f64, r.clusters[1usize].baseline_shift, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, r.clusters[2usize].baseline_shift, None).unwrap();
    });
}

#[test]
fn blank_middle_line_skips_every_edge_pass() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.blankMiddleLineSkipsEveryEdgePass", "org.tiqian.layout.LineAdjustmentStageCoverageTest.blankMiddleLineSkipsEveryEdgePass", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"blankMiddleLineSkipsEveryEdgePass");
        let r = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(&concat!("中文\n",
"\n",
"中文"), 80.0f64, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(3, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(3u32, 3u32), ((r.lines[1usize]).clone().cluster_range).clone(),
Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, r.lines[1usize].natural_width, None).unwrap();
        let mut none_just = true;
        for i in 0..match u32::try_from((r.debug).clone().justification_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().justification_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if d.line_range.clone().start == ((r.lines[1usize]).clone().range).clone().start && (d.line_range).clone().end == ((r.lines[1usize]).clone().range).clone().end {
                none_just = false;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none_just, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_justifications(&(r.debug).clone().justification_decisions)).to_string())).unwrap();
    });
}

#[test]
fn dash_run_without_ink_bounds_keeps_synthetic_glyphs() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.dashRunWithoutInkBoundsKeepsSyntheticGlyphs", "org.tiqian.layout.LineAdjustmentStageCoverageTest.dashRunWithoutInkBoundsKeepsSyntheticGlyphs", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"dashRunWithoutInkBoundsKeepsSyntheticGlyphs");
        let r = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(&"中——中", 200.0f64, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((r.glyph_runs.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(3, u32::try_from(((r.glyph_runs[0usize]).clone().glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mut all_null = true;
        for i in 0..match u32::try_from((r.glyph_runs[0usize]).clone().glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.glyph_runs[0usize].clone().glyphs[usize::try_from(i).unwrap_or(0)].clone().bounds.is_some() {
                all_null = false;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all_null, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_glyphs(&(r.glyph_runs[0usize]).clone().glyphs)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(64.0f64, r.glyph_runs[0usize].advance, None).unwrap();
    });
}

#[test]
fn emergency_selected_break_opens_the_preferred_tracking_span() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.emergencySelectedBreakOpensThePreferredTrackingSpan", "org.tiqian.layout.LineAdjustmentStageCoverageTest.emergencySelectedBreakOpensThePreferredTrackingSpan", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"emergencySelectedBreakOpensThePreferredTrackingSpan");
        let text = "deadbeefcafebabefeedfaceabcdefabcdef".to_string();
        let r = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(text.as_str(), 101.0f64, None, None, Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_text_length(text.as_str())).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
]), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (1),
Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
        let tracking = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_allocation_deltas((r).clone(), &"EmergencyGraphemeTracking");
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((tracking.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0),
Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_justifications(&(r.debug).clone().justification_decisions)).to_string())).unwrap();
    });
}

#[test]
fn empty_text_yields_zero_height_without_lines() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.emptyTextYieldsZeroHeightWithoutLines", "org.tiqian.layout.LineAdjustmentStageCoverageTest.emptyTextYieldsZeroHeightWithoutLines", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"emptyTextYieldsZeroHeightWithoutLines");
        let r = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(&"", 100.0f64, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, (r.size).clone().height, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, (r.size).clone().width, None).unwrap();
    });
}

#[test]
fn formula_line_end_discards_the_trailing_boundary_advance() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.formulaLineEndDiscardsTheTrailingBoundaryAdvance", "org.tiqian.layout.LineAdjustmentStageCoverageTest.formulaLineEndDiscardsTheTrailingBoundaryAdvance", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"formulaLineEndDiscardsTheTrailingBoundaryAdvance");
        let text = format!("{}{}{}",
            "甲",
            InlineObjectSpan::INLINE_OBJECT_SPAN_INLINE_OBJECT_REPLACEMENT_CHAR.to_string(),
            "乙丙丁戊"
        );
        let r = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(text.as_str(), 48.0f64, None, Some(vec![
    (InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 24.0f64, 12.0f64, 12.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0), Some(6.0f64),
Some(false)).unwrap())).unwrap()).clone(),
]), None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((r.lines[0usize]).clone().cluster_range).clone(),
Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
        let discard = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_trim_by_reason((r).clone(), &"InlineObjectLineEndDiscardableGlue").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(6.0f64, discard.trim_amount, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, discard.consumed_before, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"trailing", (discard.side).to_string().as_str(), None).unwrap();
    });
}

#[test]
fn formula_object_without_boundary_discards_nothing_at_line_end() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.formulaObjectWithoutBoundaryDiscardsNothingAtLineEnd", "org.tiqian.layout.LineAdjustmentStageCoverageTest.formulaObjectWithoutBoundaryDiscardsNothingAtLineEnd", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"formulaObjectWithoutBoundaryDiscardsNothingAtLineEnd");
        let text = format!("{}{}{}",
            "甲",
            InlineObjectSpan::INLINE_OBJECT_SPAN_INLINE_OBJECT_REPLACEMENT_CHAR.to_string(),
            "乙丙丁戊"
        );
        let r = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(text.as_str(), 48.0f64, None, Some(vec![
    (InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 24.0f64, 12.0f64, 12.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
]), None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((r.lines[0usize]).clone().cluster_range).clone(),
Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
        let mut none_discard = true;
        for i in 0..match u32::try_from((r.debug).clone().line_edge_trim_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().line_edge_trim_decisions[usize::try_from(i).unwrap_or(0)].clone().reason.to_string() == "InlineObjectLineEndDiscardableGlue" {
                none_discard = false;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none_discard, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_trims(&(r.debug).clone().line_edge_trim_decisions)).to_string())).unwrap();
    });
}

#[test]
fn hyphen_squeeze_consumes_opening_and_closing_bracket_glue_channels() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.hyphenSqueezeConsumesOpeningAndClosingBracketGlueChannels", "org.tiqian.layout.LineAdjustmentStageCoverageTest.hyphenSqueezeConsumesOpeningAndClosingBracketGlueChannels", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"hyphenSqueezeConsumesOpeningAndClosingBracketGlueChannels");
        let r = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(&"（中·文，internationalization", 112.0f64, None, None, None, Some(true), None).unwrap();
        let opening = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_cluster_by_text((r).clone(), &"（").unwrap();
        let comma = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_cluster_by_text((r).clone(), &"，").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8.0f64, opening.advance, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_cluster_text_advance(&r.clusters)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8.0f64, comma.advance, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_cluster_text_advance(&r.clusters)).to_string())).unwrap();
    });
}

#[test]
fn hyphen_squeeze_consumes_the_interpunct_paired_channel() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.hyphenSqueezeConsumesTheInterpunctPairedChannel", "org.tiqian.layout.LineAdjustmentStageCoverageTest.hyphenSqueezeConsumesTheInterpunctPairedChannel", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"hyphenSqueezeConsumesTheInterpunctPairedChannel");
        let r = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(&"中文，文internationalization", 112.0f64, None, None, None, Some(true), None).unwrap();
        let comma = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_cluster_by_text((r).clone(), &"，").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(14.0f64, comma.advance, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_cluster_text_advance(&r.clusters)).to_string())).unwrap();
    });
}

#[test]
fn hyphen_squeeze_consumes_the_word_space_raw_advance_channel() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.hyphenSqueezeConsumesTheWordSpaceRawAdvanceChannel", "org.tiqian.layout.LineAdjustmentStageCoverageTest.hyphenSqueezeConsumesTheWordSpaceRawAdvanceChannel", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"hyphenSqueezeConsumesTheWordSpaceRawAdvanceChannel");
        let r = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(&"中文aa internationalization", 118.0f64, None, None, None, Some(true), None).unwrap();
        let mut space = (r.clusters[0usize]).clone();
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_string() == " " {
                space = (r.clusters[usize::try_from(i).unwrap_or(0)]).clone();
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4.0f64, space.advance, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_cluster_text_advance(&r.clusters)).to_string())).unwrap();
        let first = (r.lines[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.0f64, first.hyphen_advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(118.0f64, first.adjusted_width + first.hyphen_advance, 1e-9f64, None).unwrap();
    });
}

#[test]
fn hyphen_squeeze_falls_back_to_zero_used_glue_when_the_line_already_fits() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.hyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFits", "org.tiqian.layout.LineAdjustmentStageCoverageTest.hyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFits", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"hyphenSqueezeFallsBackToZeroUsedGlueWhenTheLineAlreadyFits");
        let comma = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(&"中文，internationalization", 88.0f64, None, None, None, Some(true), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 3u32), ((comma.lines[0usize]).clone().cluster_range).clone(),
Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_lines(&comma.lines)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.0f64, comma.lines[0usize].hyphen_advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8.0f64, comma.clusters[2usize].advance, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_advances(&comma.clusters)).to_string())).unwrap();
        let bracket = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(&"（中文internationalization", 84.0f64, None, None, None, Some(true), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 3u32), ((bracket.lines[0usize]).clone().cluster_range).clone(),
Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_lines(&bracket.lines)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.0f64, bracket.lines[0usize].hyphen_advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((bracket.clusters[0usize].advance) <= 16.0f64, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_advances(&bracket.clusters)).to_string())).unwrap();
    });
}

#[test]
fn lone_latin_cluster_merges_both_auto_space_edge_trims_into_one_key() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.loneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKey", "org.tiqian.layout.LineAdjustmentStageCoverageTest.loneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKey", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"loneLatinClusterMergesBothAutoSpaceEdgeTrimsIntoOneKey");
        let r = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(&"中A中", 24.0f64, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(1u32, 1u32), ((r.lines[1usize]).clone().cluster_range).clone(),
Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
        let trims = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_trims_by_reason((r).clone(), &"TextAutoSpaceLineEdgeTrim");
        let mut sides: Vec<String> = vec![];
        for i in 0..match u32::try_from(trims.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            sides.push(((trims[usize::try_from(i).unwrap_or(0)]).clone().side).to_string());
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["trailing".to_string(), "leading".to_string()], &sides, None).unwrap();
        let mut all_match = true;
        for d in &trims {
            if !((d.cluster_range).clone().start == 1 && (d.cluster_range).clone().end == 2 && d.trim_amount == 2.0f64) {
                all_match = false;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all_match, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_trims(&trims)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.0f64, r.lines[1usize].adjusted_width, None).unwrap();
    });
}

#[test]
fn lone_mandatory_break_emits_two_zero_width_lines() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.loneMandatoryBreakEmitsTwoZeroWidthLines", "org.tiqian.layout.LineAdjustmentStageCoverageTest.loneMandatoryBreakEmitsTwoZeroWidthLines", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"loneMandatoryBreakEmitsTwoZeroWidthLines");
        let r = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(&concat!("\n",
""), 100.0f64, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
        let mut all_zero = true;
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if !(r.lines[usize::try_from(i).unwrap_or(0)].natural_width == 0.0f64 && r.lines[usize::try_from(i).unwrap_or(0)].visual_width == 0.0f64) {
                all_zero = false;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all_zero, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(((r.size).clone().height) > (0.0f64), Some((crate::runtime::fp_helper::FPHelper::format_float((r.size).clone().height)).to_string())).unwrap();
    });
}

#[test]
fn mandatory_break_middle_line_skips_its_justification_plan() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.mandatoryBreakMiddleLineSkipsItsJustificationPlan", "org.tiqian.layout.LineAdjustmentStageCoverageTest.mandatoryBreakMiddleLineSkipsItsJustificationPlan", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"mandatoryBreakMiddleLineSkipsItsJustificationPlan");
        let r = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(&concat!("中文中文\n",
"中文中文"), 80.0f64, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 4u32), ((r.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(5u32, 8u32), ((r.lines[1usize]).clone().cluster_range).clone(), None).unwrap();
        let mut all_adjusted = true;
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.lines[usize::try_from(i).unwrap_or(0)].adjusted_width != r.lines[usize::try_from(i).unwrap_or(0)].natural_width {
                all_adjusted = false;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all_adjusted, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_line_range_widths(&r.lines)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from(((r.debug).clone().justification_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn technical_line_body_stretch_rejects_the_clean_tier_and_replays() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.technicalLineBodyStretchRejectsTheCleanTierAndReplays", "org.tiqian.layout.LineAdjustmentStageCoverageTest.technicalLineBodyStretchRejectsTheCleanTierAndReplays", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"technicalLineBodyStretchRejectsTheCleanTierAndReplays");
        let text = "中文中 aa bb 中文中文中文中文中文中文".to_string();
        let r = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(text.as_str(), 96.0f64, None, None, Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_text_length(text.as_str())).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
]), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (1),
Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
        let reasons = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_emergency_reasons((r).clone());
        let mut has_rejection = false;
        for i in 0..match u32::try_from(reasons.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if reasons[usize::try_from(i).unwrap_or(0)].clone().starts_with(&"CurrentLineTechnicalTierRejection:") {
                has_rejection = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has_rejection, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_strings(&reasons)).to_string())).unwrap();
        let b_reasons = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_break_reasons((r).clone());
        let mut has_emergency = false;
        for i in 0..match u32::try_from(b_reasons.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if b_reasons[usize::try_from(i).unwrap_or(0)].clone() == "CurrentLineTechnicalEmergencyBreak" {
                has_emergency = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has_emergency, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_strings(&b_reasons)).to_string())).unwrap();
    });
}

#[test]
fn tiny_technical_tracking_stays_below_the_rejection_threshold() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.tinyTechnicalTrackingStaysBelowTheRejectionThreshold", "org.tiqian.layout.LineAdjustmentStageCoverageTest.tinyTechnicalTrackingStaysBelowTheRejectionThreshold", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"tinyTechnicalTrackingStaysBelowTheRejectionThreshold");
        let text = "中中中中中中 aaaa".to_string();
        let span = vec![
    (LineBreakSpan::new(TextRange::new(0u32, LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_text_length(text.as_str())).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
];
        let tiny = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(text.as_str(), TestHelpers::test_helpers_f32_literal(96.004f64), None, None, Some((span).clone()), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 5u32), ((tiny.lines[0usize]).clone().cluster_range).clone(),
Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_lines(&tiny.lines)).to_string())).unwrap();
        let deltas = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_allocation_deltas((tiny).clone(), &"CjkInterChar");
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((deltas.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0),
Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_justifications(&(tiny.debug).clone().justification_decisions)).to_string())).unwrap();
        let mut all_small = true;
        for i in 0..match u32::try_from(deltas.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if !((deltas[usize::try_from(i).unwrap_or(0)]) <= 0.001f64) {
                all_small = false;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all_small, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_floats(&deltas)).to_string())).unwrap();
        let tiny_reasons = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_emergency_reasons((tiny).clone());
        let mut none_rejection = true;
        for i in 0..match u32::try_from(tiny_reasons.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if tiny_reasons[usize::try_from(i).unwrap_or(0)].clone().starts_with(&"CurrentLineTechnicalTierRejection:") {
                none_rejection = false;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none_rejection, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_strings(&tiny_reasons)).to_string())).unwrap();
        let rejected = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(text.as_str(), 96.4f64, None, None, Some((span).clone()), None, None).unwrap();
        let rej_reasons = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_emergency_reasons((rejected).clone());
        let mut has_whole_token = false;
        for i in 0..match u32::try_from(rej_reasons.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if rej_reasons[usize::try_from(i).unwrap_or(0)].clone() == "CurrentLineTechnicalTierRejection:WholeToken" {
                has_whole_token = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has_whole_token, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_strings(&rej_reasons)).to_string())).unwrap();
    });
}

#[test]
fn trailing_mandatory_break_emits_terminal_empty_line_without_hyphen() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.trailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphen", "org.tiqian.layout.LineAdjustmentStageCoverageTest.trailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphen", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"trailingMandatoryBreakEmitsTerminalEmptyLineWithoutHyphen");
        let r = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(&concat!("中文aa internationalization\n",
""), 118.0f64, None, None, None, Some(true), None).unwrap();
        let last = (r.lines[usize::try_from(u32::wrapping_sub(u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone();
        let _ = TracedAssertions::traced_assertions_assert_true(last.cluster_range.get_is_empty(), Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, last.hyphen_advance, None).unwrap();
        let before = (r.lines[usize::try_from(u32::wrapping_sub(u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), 2)).unwrap_or(0)]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, before.hyphen_advance, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((before.hyphen_glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn zero_advance_edge_space_is_never_collapsed() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageCoverageTest.zeroAdvanceEdgeSpaceIsNeverCollapsed", "org.tiqian.layout.LineAdjustmentStageCoverageTest.zeroAdvanceEdgeSpaceIsNeverCollapsed", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageCoverageTest");
        t.section(&"zeroAdvanceEdgeSpaceIsNeverCollapsed");
        let r = LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_layout(&"中中中中 aaa bbb", 114.0f64, None, None, None, None, Some(Box::new(ZeroSpaceShaper::new()))).unwrap();
        let first = (r.lines[0usize]).clone();
        let edge = (r.clusters[usize::try_from(first.cluster_range.end).unwrap_or(0)]).clone();
        let _ = TracedAssertions::traced_assertions_assert_true(LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_is_all_spaces((edge.text).to_string().as_str()),
Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_cluster_texts(&r.clusters)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, edge.advance, None).unwrap();
        let mut none_collapse = true;
        for i in 0..match u32::try_from((r.debug).clone().line_edge_trim_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().line_edge_trim_decisions[usize::try_from(i).unwrap_or(0)].clone().reason.to_string() == "LineEdgeWordSpaceCollapse" {
                none_collapse = false;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none_collapse, Some((LineAdjustmentStageCoverageTestSupport::line_adjustment_stage_coverage_test_support_render_trims(&(r.debug).clone().line_edge_trim_decisions)).to_string())).unwrap();
    });
}
