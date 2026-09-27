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


#[derive(Debug, Clone, PartialEq)]
pub enum LineAdjustmentStageJfTestInlineObjectWithZeroDiscardableAdvanceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
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
        let mut t = TestTraceRecorder::new("LineAdjustmentStageJfTest");
        t.section(&"dashInkCenteringWithShapedBounds");
        let r = LineAdjustmentStageJfTestSupport::line_adjustment_stage_jf_test_support_layout(&"中——中", 200 as f64, None, Some(false), Some(Box::new(DashBoundsShaper::new(false))), None).unwrap();
        let x = if i32::from_ne_bytes((u32::try_from(((r.glyph_runs[0usize]).clone().glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (1) { (r.glyph_runs[0usize]).clone().glyphs[1usize].x } else { (r.glyph_runs[0usize]).clone().glyphs[0usize].x };
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1 as f64, x, None).unwrap();
    });
}

#[test]
fn dash_ink_centering_with_wide_bounds_returns_same_glyph() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageJfTest.dashInkCenteringWithWideBoundsReturnsSameGlyph", "org.tiqian.layout.LineAdjustmentStageJfTest.dashInkCenteringWithWideBoundsReturnsSameGlyph", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageJfTest");
        t.section(&"dashInkCenteringWithWideBoundsReturnsSameGlyph");
        let r = LineAdjustmentStageJfTestSupport::line_adjustment_stage_jf_test_support_layout(&"中——中", 200 as f64, None, Some(false), Some(Box::new(DashBoundsShaper::new(true))), None).unwrap();
        let x = if i32::from_ne_bytes((u32::try_from(((r.glyph_runs[0usize]).clone().glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (1) { (r.glyph_runs[0usize]).clone().glyphs[1usize].x } else { (r.glyph_runs[0usize]).clone().glyphs[0usize].x };
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, x, None).unwrap();
    });
}

#[test]
fn hyphen_squeeze_consumes_paired_leading_and_trailing_glue_under_taiwan_profile() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageJfTest.hyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfile", "org.tiqian.layout.LineAdjustmentStageJfTest.hyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfile", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageJfTest");
        t.section(&"hyphenSqueezeConsumesPairedLeadingAndTrailingGlueUnderTaiwanProfile");
        let r = LineAdjustmentStageJfTestSupport::line_adjustment_stage_jf_test_support_layout(&"中文，文internationalization", 112 as f64, None, Some(true), None, Some((*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_TAIWAN_HORIZONTAL).clone())).unwrap();
        let mut a = 0.0f64;
        for i in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_string() == "，" {
                a = r.clusters[usize::try_from(i).unwrap_or(0)].advance;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true((a) < (16 as f64), Some((format!("{}{}",
            "Comma advance should have shrunk: ",
            a
        )).to_string())).unwrap();
    });
}

#[test]
fn inline_object_separator_space_trim_edge() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageJfTest.inlineObjectSeparatorSpaceTrimEdge", "org.tiqian.layout.LineAdjustmentStageJfTest.inlineObjectSeparatorSpaceTrimEdge", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageJfTest");
        t.section(&"inlineObjectSeparatorSpaceTrimEdge");
        let o = vec![
    (InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 16 as f64 as f64, 12 as f64 as f64, 12 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
];
        let r = LineAdjustmentStageJfTestSupport::line_adjustment_stage_jf_test_support_layout(&"中￼ ，文文", 34 as f64, Some((o).clone()), Some(false), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((r.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (1), None).unwrap();
    });
}

#[test]
fn inline_object_with_zero_discardable_advance() {
    testlib::run("org.tiqian.layout.LineAdjustmentStageJfTest.inlineObjectWithZeroDiscardableAdvance", "org.tiqian.layout.LineAdjustmentStageJfTest.inlineObjectWithZeroDiscardableAdvance", || {
        let mut t = TestTraceRecorder::new("LineAdjustmentStageJfTest");
        t.section(&"inlineObjectWithZeroDiscardableAdvance");
        let o = vec![
    (InlineObjectSpan::new(TextRange::new(1u32, 2u32).unwrap(), 24 as f64 as f64, 12 as f64 as f64, 12 as f64 as f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0),
Some(0 as f64), Some(false)).unwrap())).unwrap()).clone(),
];
        let r = LineAdjustmentStageJfTestSupport::line_adjustment_stage_jf_test_support_layout(&"甲￼乙丙丁戊", 48 as f64, Some((o).clone()), Some(false), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_range(IntRange::new(0u32, 1u32), ((r.lines[0usize]).clone().cluster_range).clone(), None).unwrap();
    });
}
