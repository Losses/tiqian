#![cfg(test)]

use crate::org::tiqian::clreq::line_adjustment_strategy::LineAdjustmentStrategy;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::layout::line_adjustment_push_in_test_support::LineAdjustmentPushInTestSupport;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkChannel;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault) -> Self {
        match value {
            LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault) -> Self {
        match value {
            LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault) -> Self {
        match value {
            LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault) -> Self {
        match value {
            LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentPushInTestPushInFirstDoesNotCompressEveryLineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault) -> Self {
        match value {
            LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault) -> Self {
        match value {
            LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault) -> Self {
        match value {
            LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault) -> Self {
        match value {
            LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentPushInTestPushInFirstCompressesSomeBoundariesPushOutOnlyNoneFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault) -> Self {
        match value {
            LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault) -> Self {
        match value {
            LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault) -> Self {
        match value {
            LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentPushInTestNoShrinkFillPushInCanContinueUntilTheLineIsNoLongerLooseFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault) -> Self {
        match value {
            LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault) -> Self {
        match value {
            LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault) -> Self {
        match value {
            LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentPushInTestFillPushInPullsMinimalGroupToAvoidForbiddenNextHeadFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault) -> Self {
        match value {
            LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault) -> Self {
        match value {
            LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault) -> Self {
        match value {
            LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentPushInTestFillPushInExtendsPastForbiddenLineEndHeadFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault) -> Self {
        match value {
            LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault) -> Self {
        match value {
            LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault) -> Self {
        match value {
            LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentPushInTestFillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficitFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault) -> Self {
        match value {
            LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault) -> Self {
        match value {
            LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault) -> Self {
        match value {
            LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentPushInTestFillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTierFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault) -> Self {
        match value {
            LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault) -> Self {
        match value {
            LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault) -> Self {
        match value {
            LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentPushInTestFillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllableFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn fill_push_in_compresses_source_space_to_promote_emergency_break_to_syllable() {
    testlib::run("org.tiqian.layout.LineAdjustmentPushInTest.fillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllable", "org.tiqian.layout.LineAdjustmentPushInTest.fillPushInCompressesSourceSpaceToPromoteEmergencyBreakToSyllable", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,65,100,106,117,115,116,109,101,110,116,80,117,115,104,73,110,84,101,115,116])));
        t.section(UStr::new(&[102,105,108,108,80,117,115,104,73,110,67,111,109,112,114,101,115,115,101,115,83,111,117,114,99,101,83,112,97,99,101,84,111,80,114,111,109,111,116,101,69,109,101,114,103,101,110,99,121,66,114,101,97,107,84,111,83,121,108,108,97,98,108,101]));
        let c = LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_technical_clusters().unwrap();
        let tech_range = TextRange::new(0u32, 5u32).unwrap();
        let mut prog: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        prog.put(&(1), &(ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Structural, (tech_range).clone(), Some(0.0))));
        prog.put(&(3), &(ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Emergency, (tech_range).clone(), Some(0.0))));
        prog.put(&(4), &(ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Syllable, (tech_range).clone(), Some(0.0))));
        let s = LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_fill(&c, 80 as f64, Some(vec![
    (ShrinkOpportunity::new(1u32, 2u32, 10 as f64 as f64, ShrinkChannel::RawAdvance, Some(false))).clone(),
]), None, None, Some(prog.clone().build()), Some(2)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 3u32), ((s[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(80 as f64, s[0usize].adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(4u32, 4u32), ((s[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(5 as f64, LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_repair_total_shrink(((s[0usize]).clone().repair).clone()), None).unwrap();
        let allocs = LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_repair_allocations(((s[0usize]).clone().repair).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, allocs[0usize].cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(5 as f64, allocs[0usize].shrink, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_repair_reason(((s[0usize]).clone().repair).clone())).starts_with(&UString::from("ProgressiveTechnicalTierPromotion")), None).unwrap();
    });
}

#[test]
fn fill_push_in_crosses_intermediate_cleaner_boundary_to_refill_at_selected_tier() {
    testlib::run("org.tiqian.layout.LineAdjustmentPushInTest.fillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTier", "org.tiqian.layout.LineAdjustmentPushInTest.fillPushInCrossesIntermediateCleanerBoundaryToRefillAtSelectedTier", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,65,100,106,117,115,116,109,101,110,116,80,117,115,104,73,110,84,101,115,116])));
        t.section(UStr::new(&[102,105,108,108,80,117,115,104,73,110,67,114,111,115,115,101,115,73,110,116,101,114,109,101,100,105,97,116,101,67,108,101,97,110,101,114,66,111,117,110,100,97,114,121,84,111,82,101,102,105,108,108,65,116,83,101,108,101,99,116,101,100,84,105,101,114]));
        let c = LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_technical_clusters().unwrap();
        let tech_range = TextRange::new(0u32, 5u32).unwrap();
        let mut prog: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        prog.put(&(3), &(ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Emergency, (tech_range).clone(), Some(0.0))));
        prog.put(&(4), &(ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Syllable, (tech_range).clone(), Some(0.0))));
        prog.put(&(5), &(ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Emergency, (tech_range).clone(), Some(0.0))));
        let s = LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_fill(&c, 100 as f64, None, None, None, Some(prog.clone().build()), Some(2)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((s.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 4u32), ((s[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(100 as f64, s[0usize].adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_repair_reason(((s[0usize]).clone().repair).clone())).starts_with(&UString::from("LineAdjustmentPushIn")), None).unwrap();
    });
}

#[test]
fn fill_push_in_does_not_promote_emergency_break_when_cleaner_boundary_still_leaves_deficit() {
    testlib::run("org.tiqian.layout.LineAdjustmentPushInTest.fillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficit", "org.tiqian.layout.LineAdjustmentPushInTest.fillPushInDoesNotPromoteEmergencyBreakWhenCleanerBoundaryStillLeavesDeficit", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,65,100,106,117,115,116,109,101,110,116,80,117,115,104,73,110,84,101,115,116])));
        t.section(UStr::new(&[102,105,108,108,80,117,115,104,73,110,68,111,101,115,78,111,116,80,114,111,109,111,116,101,69,109,101,114,103,101,110,99,121,66,114,101,97,107,87,104,101,110,67,108,101,97,110,101,114,66,111,117,110,100,97,114,121,83,116,105,108,108,76,101,97,118,101,115,68,101,102,105,99,105,116]));
        let c = LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_technical_clusters().unwrap();
        let tech_range = TextRange::new(0u32, 5u32).unwrap();
        let mut prog: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        prog.put(&(3), &(ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Emergency, (tech_range).clone(), Some(0.0))));
        prog.put(&(4), &(ProgressiveBreakOpportunity::new(ProgressiveBreakTier::Syllable, (tech_range).clone(), Some(0.0))));
        let s = LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_fill(&c, 100 as f64, None, None, None, Some(prog.clone().build()), Some(2)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 2u32), ((s[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(3u32, 4u32), ((s[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_repair_option(None, ((s[0usize]).clone().repair).clone(), None).unwrap();
    });
}

#[test]
fn fill_push_in_extends_past_forbidden_line_end_head() {
    testlib::run("org.tiqian.layout.LineAdjustmentPushInTest.fillPushInExtendsPastForbiddenLineEndHead", "org.tiqian.layout.LineAdjustmentPushInTest.fillPushInExtendsPastForbiddenLineEndHead", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,65,100,106,117,115,116,109,101,110,116,80,117,115,104,73,110,84,101,115,116])));
        t.section(UStr::new(&[102,105,108,108,80,117,115,104,73,110,69,120,116,101,110,100,115,80,97,115,116,70,111,114,98,105,100,100,101,110,76,105,110,101,69,110,100,72,101,97,100]));
        let c = LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_forbidden_head_end_clusters().unwrap();
        let s = LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_fill(&c, 100 as f64, None, None, Some(vec![2]), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 3u32), ((s[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(90 as f64, s[0usize].adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(4u32, 4u32), ((s[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_repair_offender_index(((s[0usize]).clone().repair).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_repair_total_shrink(((s[0usize]).clone().repair).clone()), None).unwrap();
    });
}

#[test]
fn fill_push_in_pulls_minimal_group_to_avoid_forbidden_next_head() {
    testlib::run("org.tiqian.layout.LineAdjustmentPushInTest.fillPushInPullsMinimalGroupToAvoidForbiddenNextHead", "org.tiqian.layout.LineAdjustmentPushInTest.fillPushInPullsMinimalGroupToAvoidForbiddenNextHead", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,65,100,106,117,115,116,109,101,110,116,80,117,115,104,73,110,84,101,115,116])));
        t.section(UStr::new(&[102,105,108,108,80,117,115,104,73,110,80,117,108,108,115,77,105,110,105,109,97,108,71,114,111,117,112,84,111,65,118,111,105,100,70,111,114,98,105,100,100,101,110,78,101,120,116,72,101,97,100]));
        let c = LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_forbidden_head_start_clusters().unwrap();
        let s = LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_fill(&c, 100 as f64, None, Some(vec![3]), None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 3u32), ((s[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(90 as f64, s[0usize].adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(4u32, 4u32), ((s[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_repair_offender_index(((s[0usize]).clone().repair).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_repair_total_shrink(((s[0usize]).clone().repair).clone()), None).unwrap();
    });
}

#[test]
fn no_shrink_fill_push_in_can_continue_until_the_line_is_no_longer_loose() {
    testlib::run("org.tiqian.layout.LineAdjustmentPushInTest.noShrinkFillPushInCanContinueUntilTheLineIsNoLongerLoose", "org.tiqian.layout.LineAdjustmentPushInTest.noShrinkFillPushInCanContinueUntilTheLineIsNoLongerLoose", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,65,100,106,117,115,116,109,101,110,116,80,117,115,104,73,110,84,101,115,116])));
        t.section(UStr::new(&[110,111,83,104,114,105,110,107,70,105,108,108,80,117,115,104,73,110,67,97,110,67,111,110,116,105,110,117,101,85,110,116,105,108,84,104,101,76,105,110,101,73,115,78,111,76,111,110,103,101,114,76,111,111,115,101]));
        let c = LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_base_clusters().unwrap();
        let s = LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_fill(&c, 100 as f64, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 3u32), ((s[0usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(100 as f64, s[0usize].adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(4u32, 5u32), ((s[1usize]).clone().cluster_range).clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_repair_total_shrink(((s[0usize]).clone().repair).clone()), None).unwrap();
    });
}

#[test]
fn push_in_first_compresses_some_boundaries_push_out_only_none() {
    testlib::run("org.tiqian.layout.LineAdjustmentPushInTest.pushInFirstCompressesSomeBoundariesPushOutOnlyNone", "org.tiqian.layout.LineAdjustmentPushInTest.pushInFirstCompressesSomeBoundariesPushOutOnlyNone", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,65,100,106,117,115,116,109,101,110,116,80,117,115,104,73,110,84,101,115,116])));
        t.section(UStr::new(&[112,117,115,104,73,110,70,105,114,115,116,67,111,109,112,114,101,115,115,101,115,83,111,109,101,66,111,117,110,100,97,114,105,101,115,80,117,115,104,79,117,116,79,110,108,121,78,111,110,101]));
        let auto = LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_layout(LineAdjustmentStrategy::PushInFirst).unwrap();
        let push_out = LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_layout(LineAdjustmentStrategy::PushOutOnly).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_fill_push_in_count((push_out).clone()), Some(UString::from("PushOutOnly must never fill-push-in"))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_fill_push_in_count((auto).clone())) as i32).to_ne_bytes())) > (0), Some(UString::from("PushInFirst should compress at least one boundary"))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((auto.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((u32::try_from((push_out.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("PushInFirst (")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(u32::try_from((auto.lines.len()) & 0xFFFF_FFFF).unwrap_or(0))).as_str())); __s += &(UString::from(") should not need more lines than PushOutOnly (")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(u32::try_from((push_out.lines.len()) & 0xFFFF_FFFF).unwrap_or(0))).as_str())); __s += &(UString::from(")")); __s }).as_str()))).unwrap();
    });
}

#[test]
fn push_in_first_does_not_compress_every_line() {
    testlib::run("org.tiqian.layout.LineAdjustmentPushInTest.pushInFirstDoesNotCompressEveryLine", "org.tiqian.layout.LineAdjustmentPushInTest.pushInFirstDoesNotCompressEveryLine", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,65,100,106,117,115,116,109,101,110,116,80,117,115,104,73,110,84,101,115,116])));
        t.section(UStr::new(&[112,117,115,104,73,110,70,105,114,115,116,68,111,101,115,78,111,116,67,111,109,112,114,101,115,115,69,118,101,114,121,76,105,110,101]));
        let auto = LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_layout(LineAdjustmentStrategy::PushInFirst).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_fill_push_in_count((auto).clone())) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((auto.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("not every line should be a fill-push-in (")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(LineAdjustmentPushInTestSupport::line_adjustment_push_in_test_support_fill_push_in_count((auto).clone()))).as_str())); __s += &(UString::from("/")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(u32::try_from((auto.lines.len()) & 0xFFFF_FFFF).unwrap_or(0))).as_str())); __s += &(UString::from(")")); __s }).as_str()))).unwrap();
    });
}
