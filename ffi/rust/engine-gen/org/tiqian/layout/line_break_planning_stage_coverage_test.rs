#![cfg(test)]

use crate::org::tiqian::clreq::line_adjustment_strategy::LineAdjustmentStrategy;
use crate::org::tiqian::core::inline_object_boundary_adjustment::InlineObjectBoundaryAdjustment;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::line_break_policy::LineBreakPolicy;
use crate::org::tiqian::core::line_break_span::LineBreakSpan;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::layout::line_break_planning_stage_coverage_test_support::LineBreakPlanningStageCoverageTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;


#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakPlanningStageCoverageTestPushOutFirstTakesFewerFillPushInsThanPushInFirstFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakPlanningStageCoverageTestOverlappingTechnicalSpansKeepTheFirstBoundaryReasonFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakPlanningStageCoverageTestExplicitZeroLineHeightKeepsTheControlParagraphAtZeroHeightFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControlsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakPlanningStageCoverageTestEmergencyBoundaryEligibilitySkipsInlineObjectBoundariesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault) -> Self {
        match value {
            LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakPlanningStageCoverageTestDashAndSolidusBoundariesInsideTechnicalSpansNeverStretchFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn push_out_first_takes_fewer_fill_push_ins_than_push_in_first() {
    testlib::run("org.tiqian.layout.LineBreakPlanningStageCoverageTest.pushOutFirstTakesFewerFillPushInsThanPushInFirst", "org.tiqian.layout.LineBreakPlanningStageCoverageTest.pushOutFirstTakesFewerFillPushInsThanPushInFirst", || {
        let mut t = TestTraceRecorder::new("LineBreakPlanningStageCoverageTest");
        t.section(&"pushOutFirstTakesFewerFillPushInsThanPushInFirst");
        let text = "咖啡（coffee）在十七世纪经威尼斯传入欧洲。最初它被当作药物出售，价格高得吓人，真正让它流行起来的是随后遍地开花的咖啡馆——读报、辩论、下棋、写作——城市生活忽然多出一个公共客厅。意大利人做出了 espresso，维也纳人往杯里加奶油，土耳其人坚持连渣同煮……每座城市都相信自己手里那一杯才是正统。有人说：「先有咖啡馆，后有启蒙运动」。这话说得夸张，但也不算太离谱。".to_string();
        let a = LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_layout_with_default_style(text.as_str(), 320 as f64, LineAdjustmentStrategy::PushInFirst).unwrap();
        let b = LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_layout_with_default_style(text.as_str(), 320 as f64, LineAdjustmentStrategy::PushOutFirst).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_fill_push_in_count((a).clone())).to_ne_bytes())) > (0),
Some((LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_render_decisions((a).clone())).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_fill_push_in_count((b).clone())).to_ne_bytes())) <=
i32::from_ne_bytes((LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_fill_push_in_count((a).clone())).to_ne_bytes()), Some((format!("{}{}{}{}",
            "PushOutFirst ",
            crate::runtime::int_text::IntText::int_text(LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_fill_push_in_count((b).clone())),
            " vs PushInFirst ",
            crate::runtime::int_text::IntText::int_text(LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_fill_push_in_count((a).clone()))
        )).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((b.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) >= i32::from_ne_bytes((u32::try_from((a.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()),
Some((format!("{}{}{}{}",
            "PushOutFirst ",
            crate::runtime::int_text::IntText::int_text(u32::try_from((b.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)),
            " vs PushInFirst ",
            crate::runtime::int_text::IntText::int_text(u32::try_from((a.lines.len()) & 0xFFFF_FFFF).unwrap_or(0))
        )).to_string())).unwrap();
    });
}

#[test]
fn explicit_zero_line_height_keeps_the_control_paragraph_at_zero_height() {
    testlib::run("org.tiqian.layout.LineBreakPlanningStageCoverageTest.explicitZeroLineHeightKeepsTheControlParagraphAtZeroHeight", "org.tiqian.layout.LineBreakPlanningStageCoverageTest.explicitZeroLineHeightKeepsTheControlParagraphAtZeroHeight", || {
        let mut t = TestTraceRecorder::new("LineBreakPlanningStageCoverageTest");
        t.section(&"explicitZeroLineHeightKeepsTheControlParagraphAtZeroHeight");
        let r = LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_layout(&concat!("\n",
""), 100 as f64, None, Some(0 as f64 as f64), None, None, Some(false)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), Some((LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, (r.size).clone().height, Some((crate::runtime::fp_helper::FPHelper::format_float((r.size).clone().height)).to_string())).unwrap();
    });
}

#[test]
fn emergency_boundary_eligibility_skips_zero_width_and_mandatory_controls() {
    testlib::run("org.tiqian.layout.LineBreakPlanningStageCoverageTest.emergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControls", "org.tiqian.layout.LineBreakPlanningStageCoverageTest.emergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControls", || {
        let mut t = TestTraceRecorder::new("LineBreakPlanningStageCoverageTest");
        t.section(&"emergencyBoundaryEligibilitySkipsZeroWidthAndMandatoryControls");
        let s = "ab​cd".to_string();
        let mut r = LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_layout(s.as_str(), 200 as f64, None, None, Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, u_string::count(s.as_str())).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
]), None, Some(false)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), Some((LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
        r = LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_layout(&concat!("aa\n",
"bb"), 200 as f64, None, None, Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, 5u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
]), None, Some(false)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(2, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), Some((LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
    });
}

#[test]
fn emergency_boundary_eligibility_skips_inline_object_boundaries() {
    testlib::run("org.tiqian.layout.LineBreakPlanningStageCoverageTest.emergencyBoundaryEligibilitySkipsInlineObjectBoundaries", "org.tiqian.layout.LineBreakPlanningStageCoverageTest.emergencyBoundaryEligibilitySkipsInlineObjectBoundaries", || {
        let mut t = TestTraceRecorder::new("LineBreakPlanningStageCoverageTest");
        t.section(&"emergencyBoundaryEligibilitySkipsInlineObjectBoundaries");
        let r = LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_layout(&"a￼b", 200 as f64, None, None, Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, 3u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
]), Some(vec![
    (InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 16 as f64 as f64, 8 as f64 as f64, 8 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
]), Some(false)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), Some((LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((r.lines[0usize]).clone().cluster_range.start == 0, None).unwrap();
    });
}

#[test]
fn dash_and_solidus_boundaries_inside_technical_spans_never_stretch() {
    testlib::run("org.tiqian.layout.LineBreakPlanningStageCoverageTest.dashAndSolidusBoundariesInsideTechnicalSpansNeverStretch", "org.tiqian.layout.LineBreakPlanningStageCoverageTest.dashAndSolidusBoundariesInsideTechnicalSpansNeverStretch", || {
        let mut t = TestTraceRecorder::new("LineBreakPlanningStageCoverageTest");
        t.section(&"dashAndSolidusBoundariesInsideTechnicalSpansNeverStretch");
        {
            let _g1 = vec!["a—b—c".to_string(), "a/b/c".to_string(), "a…b".to_string()];
            for s in &_g1 {
                let r = LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_layout(s.as_str(), 24 as f64, None, None, Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, u_string::count(s.as_str())).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
]), None, Some(false)).unwrap();
                let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0), Some((format!("{}{}{}",
            s,
            ": ",
            LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_render_lines(&r.lines)
        )).to_string())).unwrap();
                let mut ok = true;
                for i in 0..match u32::try_from((r.debug).clone().justification_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    for j in 0..match u32::try_from(((r.debug).clone().justification_decisions[usize::try_from(i).unwrap_or(0)]).clone().allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        let x = (((r.debug).clone().justification_decisions[usize::try_from(i).unwrap_or(0)]).clone().allocations[usize::try_from(j).unwrap_or(0)]).clone();
                        if x.kind.to_string() == "EmergencyGraphemeTracking" && (x.delta) > (0 as f64) {
                            ok = false;
                        }
                    }
                }
                let _ = TracedAssertions::traced_assertions_assert_true(ok, Some((format!("{}{}{}",
            s,
            ": ",
            LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_render_justification(&(r.debug).clone().justification_decisions)
        )).to_string())).unwrap();
            }
        }
    });
}

#[test]
fn overlapping_technical_spans_keep_the_first_boundary_reason() {
    testlib::run("org.tiqian.layout.LineBreakPlanningStageCoverageTest.overlappingTechnicalSpansKeepTheFirstBoundaryReason", "org.tiqian.layout.LineBreakPlanningStageCoverageTest.overlappingTechnicalSpansKeepTheFirstBoundaryReason", || {
        let mut t = TestTraceRecorder::new("LineBreakPlanningStageCoverageTest");
        t.section(&"overlappingTechnicalSpansKeepTheFirstBoundaryReason");
        let r = LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_layout(&"aabbcc", 200 as f64, None, None, Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, 4u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
    (LineBreakSpan::new(TextRange::new(2u32, 6u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
]), None, Some(false)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), Some((LineBreakPlanningStageCoverageTestSupport::line_break_planning_stage_coverage_test_support_render_lines(&r.lines)).to_string())).unwrap();
    });
}
