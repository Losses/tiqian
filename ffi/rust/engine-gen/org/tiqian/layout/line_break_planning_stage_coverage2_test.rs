#![cfg(test)]

use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::emergency_tracking_eligibility_decision_info::EmergencyTrackingEligibilityDecisionInfo;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::inline_object_boundary_adjustment::InlineObjectBoundaryAdjustment;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::font::font_policy::FontCandidate;
use crate::org::tiqian::font::font_policy::FontDecision;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    SupportPrepFault(crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault),
    SupportPlanFault(crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault),
}
impl std::fmt::Display for LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault::SupportPrepFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault::SupportPlanFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault> for crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault::SupportPrepFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault> for crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault::SupportPlanFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault> for LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault {
    fn from(value: crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault::SupportPrepFault(value)
    }
}

impl From<crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault> for LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault {
    fn from(value: crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestProgressiveBreakOffsetsUnmappedClusterIndexFault::SupportPlanFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestInlineObjectKinsokuLineStartFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    SupportPrepFault(crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault),
    SupportPlanFault(crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault),
}
impl std::fmt::Display for LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault::SupportPrepFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault::SupportPlanFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault> for crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault::SupportPrepFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault> for crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault::SupportPlanFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault> for LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault {
    fn from(value: crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault::SupportPrepFault(value)
    }
}

impl From<crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault> for LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault {
    fn from(value: crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestFontDecisionWithNoMatchingClustersUsesTextSubstringFault::SupportPlanFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    SupportPrepFault(crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault),
    SupportPlanFault(crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault),
}
impl std::fmt::Display for LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault::SupportPrepFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault::SupportPlanFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault> for crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault::SupportPrepFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault> for crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault::SupportPlanFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault> for LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault {
    fn from(value: crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault::SupportPrepFault(value)
    }
}

impl From<crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault> for LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault {
    fn from(value: crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestEmergencyTrackingEligibilityDecisionsBranchesFault::SupportPlanFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    SupportPrepFault(crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault),
    SupportPlanFault(crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault),
}
impl std::fmt::Display for LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault::SupportPrepFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault::SupportPlanFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault> for crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault::SupportPrepFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault> for crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault::SupportPlanFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault> for LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault {
    fn from(value: crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault::SupportPrepFault(value)
    }
}

impl From<crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault> for LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault {
    fn from(value: crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestEmergencyTrackingBoundaryWhitespaceAndEmptyFault::SupportPlanFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    SupportPrepFault(crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault),
    SupportPlanFault(crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault),
}
impl std::fmt::Display for LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::SupportPrepFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::SupportPlanFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault> for crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::SupportPrepFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault> for crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::SupportPlanFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault> for LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault {
    fn from(value: crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::SupportPrepFault(value)
    }
}

impl From<crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault> for LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault {
    fn from(value: crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestClusterCrossesFontDecisionThrowsFault::SupportPlanFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestAsciiPointMarkKinsokuLineStartFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    SupportPrepFault(crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault),
    SupportPlanFault(crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault),
}
impl std::fmt::Display for LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault::SupportPrepFault(value) => write!(formatter, "{}", value),
            LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault::SupportPlanFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault> for crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault::SupportPrepFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault> for crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault {
    fn from(value: LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault) -> Self {
        match value {
            LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault::SupportPlanFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault> for LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault {
    fn from(value: crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPrepFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault::SupportPrepFault(value)
    }
}

impl From<crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault> for LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault {
    fn from(value: crate::org::tiqian::layout::line_break_planning_stage_coverage2_test_support::LineBreakPlanningStageCoverage2TestSupportPlanFault) -> Self {
        LineBreakPlanningStageCoverage2TestTestAdjustableInlineBoundaryRightClustersNoStretchBoundariesFault::SupportPlanFault(value)
    }
}

#[test]
fn test_adjustable_inline_boundary_right_clusters_no_stretch_boundaries() {
    testlib::run("org.tiqian.layout.LineBreakPlanningStageCoverage2Test.testAdjustableInlineBoundaryRightClustersNoStretchBoundaries", "org.tiqian.layout.LineBreakPlanningStageCoverage2Test.testAdjustableInlineBoundaryRightClustersNoStretchBoundaries", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,80,108,97,110,110,105,110,103,83,116,97,103,101,67,111,118,101,114,97,103,101,50,84,101,115,116])));
        t.section(UStr::new(&[116,101,115,116,65,100,106,117,115,116,97,98,108,101,73,110,108,105,110,101,66,111,117,110,100,97,114,121,82,105,103,104,116,67,108,117,115,116,101,114,115,78,111,83,116,114,101,116,99,104,66,111,117,110,100,97,114,105,101,115]));
        let p = LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_prep(UStr::new(&[20013,25991,23383,31526,25490,29256])).unwrap();
        let r = LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_plan(LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_with_prep((p).clone(), None, None, None, None, None, Some(LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_set_uniform()), Some(LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_map_atom().unwrap()))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from(((r.line_solution).clone().lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn test_ascii_point_mark_kinsoku_line_start() {
    testlib::run("org.tiqian.layout.LineBreakPlanningStageCoverage2Test.testAsciiPointMarkKinsokuLineStart", "org.tiqian.layout.LineBreakPlanningStageCoverage2Test.testAsciiPointMarkKinsokuLineStart", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,80,108,97,110,110,105,110,103,83,116,97,103,101,67,111,118,101,114,97,103,101,50,84,101,115,116])));
        t.section(UStr::new(&[116,101,115,116,65,115,99,105,105,80,111,105,110,116,77,97,114,107,75,105,110,115,111,107,117,76,105,110,101,83,116,97,114,116]));
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_layout(UStr::new(&[104,101,108,108,111,44,32,119,111,114,108,100]), 50 as f64, None).unwrap().lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn test_cluster_crosses_font_decision_throws() {
    testlib::run("org.tiqian.layout.LineBreakPlanningStageCoverage2Test.testClusterCrossesFontDecisionThrows", "org.tiqian.layout.LineBreakPlanningStageCoverage2Test.testClusterCrossesFontDecisionThrows", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,80,108,97,110,110,105,110,103,83,116,97,103,101,67,111,118,101,114,97,103,101,50,84,101,115,116])));
        t.section(UStr::new(&[116,101,115,116,67,108,117,115,116,101,114,67,114,111,115,115,101,115,70,111,110,116,68,101,99,105,115,105,111,110,84,104,114,111,119,115]));
        let p = LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_prep(UStr::new(&[97,98,99,100,101,102])).unwrap();
        let c = Cluster::new(TextRange::new(0u32, 5u32).unwrap(), &(UStr::new(&[97,98,99,100,101])), &(UStr::new(&[116,101,115,116])), 50 as f64 as f64, Some(UString::from("abcde")), Some(0.0), Some(0.0), Some(0.0));
        let d = FontDecision::new(TextRange::new(0u32, 3u32).unwrap(), FontCandidate::new(&(UStr::new(&[116,101,115,116])), &(UStr::new(&[116,101,115,116])), FontRole::LatinText), FontRole::LatinText, &(UStr::new(&[116,101,115,116])));
        let e = TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let p = (p).clone(); let c = (c).clone(); let d = (d).clone(); Arc::new(move || {
        LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_plan(LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_with_prep((p).clone(), Some(vec![(c).clone()]), Some(vec![(c).clone()]), Some(vec![(d).clone()]), None, None, None, None)).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&UString::from(format!("{}", format!("{}", e)).as_str()), UString::from("crosses font decision").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, Some(UString::from(format!("{}", format!("{}", e)).as_str()))).unwrap();
    });
}

#[test]
fn test_emergency_tracking_boundary_whitespace_and_empty() {
    testlib::run("org.tiqian.layout.LineBreakPlanningStageCoverage2Test.testEmergencyTrackingBoundaryWhitespaceAndEmpty", "org.tiqian.layout.LineBreakPlanningStageCoverage2Test.testEmergencyTrackingBoundaryWhitespaceAndEmpty", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,80,108,97,110,110,105,110,103,83,116,97,103,101,67,111,118,101,114,97,103,101,50,84,101,115,116])));
        t.section(UStr::new(&[116,101,115,116,69,109,101,114,103,101,110,99,121,84,114,97,99,107,105,110,103,66,111,117,110,100,97,114,121,87,104,105,116,101,115,112,97,99,101,65,110,100,69,109,112,116,121]));
        let p = LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_prep(UStr::new(&[97,98])).unwrap();
        let cs = vec![
    (Cluster::new(TextRange::new(0u32, 0u32).unwrap(), &(UStr::new(&[])), &(UStr::new(&[116,101,115,116])), 0 as f64 as f64, Some(UString::from("")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[97])), &(UStr::new(&[116,101,115,116])), 10 as f64 as f64, Some(UString::from("a")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 1u32).unwrap(), &(UStr::new(&[])), &(UStr::new(&[116,101,115,116])), 0 as f64 as f64, Some(UString::from("")), Some(0.0), Some(0.0), Some(0.0))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32).unwrap(), &(UStr::new(&[98])), &(UStr::new(&[116,101,115,116])), 10 as f64 as f64, Some(UString::from("b")), Some(0.0), Some(0.0), Some(0.0))).clone(),
];
        let e = EmergencyTrackingEligibilityDecisionInfo::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[97,98])), &(UStr::new(&[114,101,97,115,111,110])));
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from(((LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_plan(LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_with_prep((p).clone(), Some((cs).clone()), Some((cs).clone()), None, None, Some(vec![(e).clone()]), None, None)).unwrap().line_solution).clone().lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn test_emergency_tracking_eligibility_decisions_branches() {
    testlib::run("org.tiqian.layout.LineBreakPlanningStageCoverage2Test.testEmergencyTrackingEligibilityDecisionsBranches", "org.tiqian.layout.LineBreakPlanningStageCoverage2Test.testEmergencyTrackingEligibilityDecisionsBranches", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,80,108,97,110,110,105,110,103,83,116,97,103,101,67,111,118,101,114,97,103,101,50,84,101,115,116])));
        t.section(UStr::new(&[116,101,115,116,69,109,101,114,103,101,110,99,121,84,114,97,99,107,105,110,103,69,108,105,103,105,98,105,108,105,116,121,68,101,99,105,115,105,111,110,115,66,114,97,110,99,104,101,115]));
        let p = LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_prep(UStr::new(&[20013,25991,23383,31526])).unwrap();
        let es = vec![
    (EmergencyTrackingEligibilityDecisionInfo::new(TextRange::new(100u32, 200u32).unwrap(), &(UStr::new(&[117,110,109,97,112,112,101,100])), &(UStr::new(&[114,101,97,115,111,110])))).clone(),
    (EmergencyTrackingEligibilityDecisionInfo::new(TextRange::new(0u32, 4u32).unwrap(), &(UStr::new(&[20013,25991,23383,31526])), &(UStr::new(&[118,97,108,105,100,82,101,97,115,111,110])))).clone(),
    (EmergencyTrackingEligibilityDecisionInfo::new(TextRange::new(0u32, 4u32).unwrap(), &(UStr::new(&[20013,25991,23383,31526])), &(UStr::new(&[100,117,112,108,105,99,97,116,101,82,101,97,115,111,110])))).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from(((LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_plan(LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_with_prep((p).clone(), None, None, None, None, Some((es).clone()), None, None)).unwrap().line_solution).clone().lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn test_font_decision_with_no_matching_clusters_uses_text_substring() {
    testlib::run("org.tiqian.layout.LineBreakPlanningStageCoverage2Test.testFontDecisionWithNoMatchingClustersUsesTextSubstring", "org.tiqian.layout.LineBreakPlanningStageCoverage2Test.testFontDecisionWithNoMatchingClustersUsesTextSubstring", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,80,108,97,110,110,105,110,103,83,116,97,103,101,67,111,118,101,114,97,103,101,50,84,101,115,116])));
        t.section(UStr::new(&[116,101,115,116,70,111,110,116,68,101,99,105,115,105,111,110,87,105,116,104,78,111,77,97,116,99,104,105,110,103,67,108,117,115,116,101,114,115,85,115,101,115,84,101,120,116,83,117,98,115,116,114,105,110,103]));
        let p = LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_prep(UStr::new(&[97,98,99,100,101,102])).unwrap();
        let d = FontDecision::new(TextRange::new(4u32, 6u32).unwrap(), FontCandidate::new(&(UStr::new(&[116,101,115,116])), &(UStr::new(&[116,101,115,116])), FontRole::LatinText), FontRole::LatinText, &(UStr::new(&[116,101,115,116])));
        let c = Cluster::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[97,98])), &(UStr::new(&[116,101,115,116])), 20 as f64 as f64, Some(UString::from("ab")), Some(0.0), Some(0.0), Some(0.0));
        let r = LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_plan(LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_with_prep((p).clone(), Some(vec![(c).clone()]), Some(vec![(c).clone()]), Some(vec![(d).clone()]), None, None, None, None)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((r.metric_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[101,102]), (((r.metric_decisions[0usize]).clone().request).clone().face_selection_text).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn test_inline_object_kinsoku_line_start() {
    testlib::run("org.tiqian.layout.LineBreakPlanningStageCoverage2Test.testInlineObjectKinsokuLineStart", "org.tiqian.layout.LineBreakPlanningStageCoverage2Test.testInlineObjectKinsokuLineStart", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,80,108,97,110,110,105,110,103,83,116,97,103,101,67,111,118,101,114,97,103,101,50,84,101,115,116])));
        t.section(UStr::new(&[116,101,115,116,73,110,108,105,110,101,79,98,106,101,99,116,75,105,110,115,111,107,117,76,105,110,101,83,116,97,114,116]));
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_layout(UStr::new(&[65532,104,101,108,108,111]), 50 as f64, Some(vec![
    (InlineObjectSpan::new(TextRange::new(0u32, 1u32).unwrap(), 16 as f64 as f64, 8 as f64 as f64, 8 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
])).unwrap().lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn test_progressive_break_offsets_unmapped_cluster_index() {
    testlib::run("org.tiqian.layout.LineBreakPlanningStageCoverage2Test.testProgressiveBreakOffsetsUnmappedClusterIndex", "org.tiqian.layout.LineBreakPlanningStageCoverage2Test.testProgressiveBreakOffsetsUnmappedClusterIndex", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,66,114,101,97,107,80,108,97,110,110,105,110,103,83,116,97,103,101,67,111,118,101,114,97,103,101,50,84,101,115,116])));
        t.section(UStr::new(&[116,101,115,116,80,114,111,103,114,101,115,115,105,118,101,66,114,101,97,107,79,102,102,115,101,116,115,85,110,109,97,112,112,101,100,67,108,117,115,116,101,114,73,110,100,101,120]));
        let p = LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_prep(UStr::new(&[97,98,99])).unwrap();
        let r = LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_plan(LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_with_prep((p).clone(), None, None, None, Some(LineBreakPlanningStageCoverage2TestSupport::line_break_planning_stage_coverage2_test_support_map_opp().unwrap()), None, None, None)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::from_ne_bytes(((r.progressive_break_opportunities.size()) as u32).to_ne_bytes()) == 0, None).unwrap();
    });
}
