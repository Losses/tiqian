#![cfg(test)]

use crate::org::tiqian::core::inline_object_boundary_adjustment::InlineObjectBoundaryAdjustment;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::layout::line_adjustment_stage_jf_test_support::DashBoundsShaper;
use crate::org::tiqian::layout::line_adjustment_stage_jf_test_support::LineAdjustmentStageJfTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault) -> Self {
        match value {
            LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault) -> Self {
        match value {
            LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault) -> Self {
        match value {
            LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault) -> Self {
        match value {
            LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault) -> Self {
        match value {
            LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault) -> Self {
        match value {
            LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault) -> Self {
        match value {
            LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault) -> Self {
        match value {
            LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageJfTestInlineObjectSeparatorSpaceTrimEdgeFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault) -> Self {
        match value {
            LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault) -> Self {
        match value {
            LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault) -> Self {
        match value {
            LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault) -> Self {
        match value {
            LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageJfTestHyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfileFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault) -> Self {
        match value {
            LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault) -> Self {
        match value {
            LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault) -> Self {
        match value {
            LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault) -> Self {
        match value {
            LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageJfTestDashInkCenteringWithWideBoundsReturnsSameGlyphFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}
impl std::fmt::Display for LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault) -> Self {
        match value {
            LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault) -> Self {
        match value {
            LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault) -> Self {
        match value {
            LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault) -> Self {
        match value {
            LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LineAdjustmentStageJfTestDashInkCenteringWithShapedBoundsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[test]
fn dash_ink_centering_with_shaped_bounds() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageJfTest.dashInkCenteringWithShapedBounds", "org.tiqian.layout.LineAdjustmentStageJfTest.dashInkCenteringWithShapedBounds", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,65,100,106,117,115,116,109,101,110,116,83,116,97,103,101,74,102,84,101,115,116])));
        t.section(UStr::new(&[100,97,115,104,73,110,107,67,101,110,116,101,114,105,110,103,87,105,116,104,83,104,97,112,101,100,66,111,117,110,100,115]));
        let r = LineAdjustmentStageJfTestSupport::line_adjustment_stage_jf_test_support_layout(UStr::new(&[20013,8212,8212,20013]), 200 as f64, None, Some(false), Some(Arc::new(Mutex::new(DashBoundsShaper::new(false)))), None).unwrap();
        let x = if i32::from_ne_bytes(((u32::try_from(((r.glyph_runs[0usize]).clone().glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (1) { (r.glyph_runs[0usize]).clone().glyphs[1usize].x } else { (r.glyph_runs[0usize]).clone().glyphs[0usize].x };
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1 as f64, x, None).unwrap();
    });
}

#[test]
fn dash_ink_centering_with_wide_bounds_returns_same_glyph() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageJfTest.dashInkCenteringWithWideBoundsReturnsSameGlyph", "org.tiqian.layout.LineAdjustmentStageJfTest.dashInkCenteringWithWideBoundsReturnsSameGlyph", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,65,100,106,117,115,116,109,101,110,116,83,116,97,103,101,74,102,84,101,115,116])));
        t.section(UStr::new(&[100,97,115,104,73,110,107,67,101,110,116,101,114,105,110,103,87,105,116,104,87,105,100,101,66,111,117,110,100,115,82,101,116,117,114,110,115,83,97,109,101,71,108,121,112,104]));
        let r = LineAdjustmentStageJfTestSupport::line_adjustment_stage_jf_test_support_layout(UStr::new(&[20013,8212,8212,20013]), 200 as f64, None, Some(false), Some(Arc::new(Mutex::new(DashBoundsShaper::new(true)))), None).unwrap();
        let x = if i32::from_ne_bytes(((u32::try_from(((r.glyph_runs[0usize]).clone().glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (1) { (r.glyph_runs[0usize]).clone().glyphs[1usize].x } else { (r.glyph_runs[0usize]).clone().glyphs[0usize].x };
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, x, None).unwrap();
    });
}

#[test]
fn hyphen_squeeze_consumes_paired_leading_and_trailing_glue_under_taiwan_profile() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageJfTest.hyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfile", "org.tiqian.layout.LineAdjustmentStageJfTest.hyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfile", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,65,100,106,117,115,116,109,101,110,116,83,116,97,103,101,74,102,84,101,115,116])));
        t.section(UStr::new(&[104,121,112,104,101,110,83,113,117,101,101,122,101,67,111,110,115,117,109,101,115,80,97,105,114,101,100,76,101,97,100,105,110,103,65,110,100,84,114,97,105,108,105,110,103,71,108,117,101,85,110,100,101,114,84,97,105,119,97,110,80,114,111,102,105,108,101]));
        let r = LineAdjustmentStageJfTestSupport::line_adjustment_stage_jf_test_support_layout(UStr::new(&[20013,25991,65292,25991,105,110,116,101,114,110,97,116,105,111,110,97,108,105,122,97,116,105,111,110]), 112 as f64, None, Some(true), None, Some((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_TAIWAN_HORIZONTAL).clone())).unwrap();
        let mut a = 0.0f64;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_ustring() == UString::from("，") {
                a = r.clusters[usize::try_from(i).unwrap_or(0)].advance;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((a) < (16 as f64), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Comma advance should have shrunk: ")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(a)); __s }).as_str()))).unwrap();
    });
}

#[test]
fn inline_object_separator_space_trim_edge() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageJfTest.inlineObjectSeparatorSpaceTrimEdge", "org.tiqian.layout.LineAdjustmentStageJfTest.inlineObjectSeparatorSpaceTrimEdge", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,65,100,106,117,115,116,109,101,110,116,83,116,97,103,101,74,102,84,101,115,116])));
        t.section(UStr::new(&[105,110,108,105,110,101,79,98,106,101,99,116,83,101,112,97,114,97,116,111,114,83,112,97,99,101,84,114,105,109,69,100,103,101]));
        let o = vec![
    (InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 16 as f64 as f64, 12 as f64 as f64, 12 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
];
        let r = LineAdjustmentStageJfTestSupport::line_adjustment_stage_jf_test_support_layout(UStr::new(&[20013,65532,32,65292,25991,25991]), 34 as f64, Some((o).clone()), Some(false), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (1), None).unwrap();
    });
}

#[test]
fn inline_object_with_zero_discardable_advance() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageJfTest.inlineObjectWithZeroDiscardableAdvance", "org.tiqian.layout.LineAdjustmentStageJfTest.inlineObjectWithZeroDiscardableAdvance", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[76,105,110,101,65,100,106,117,115,116,109,101,110,116,83,116,97,103,101,74,102,84,101,115,116])));
        t.section(UStr::new(&[105,110,108,105,110,101,79,98,106,101,99,116,87,105,116,104,90,101,114,111,68,105,115,99,97,114,100,97,98,108,101,65,100,118,97,110,99,101]));
        let o = vec![
    (InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 24 as f64 as f64, 12 as f64 as f64, 12 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0), Some(0 as f64), Some(false)).unwrap())).unwrap()).clone(),
];
        let r = LineAdjustmentStageJfTestSupport::line_adjustment_stage_jf_test_support_layout(UStr::new(&[30002,65532,20057,19993,19969,25098]), 48 as f64, Some((o).clone()), Some(false), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((r.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
    });
}
