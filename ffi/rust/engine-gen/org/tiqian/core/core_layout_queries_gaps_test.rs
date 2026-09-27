#![cfg(test)]

use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::glyph_run::GlyphRun;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_debug_info::LayoutDebugInfo;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_queries::LayoutQueries;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_box::LineBox;
use crate::org::tiqian::core::line_debug_info::LineDebugInfo;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::metric_decision_info::MetricDecisionInfo;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::rich_text_background_metric_policy::RichTextBackgroundMetricPolicy;
use crate::org::tiqian::core::rich_text_background_paint::RichTextBackgroundPaint;
use crate::org::tiqian::core::rich_text_paint::RichTextPaint;
use crate::org::tiqian::core::rich_text_span::RichTextSpan;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::size::Size;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::test::test_helpers::TestHelpers;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::std::u_string_exception::UStringFault;


#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    LayoutQueriesGetSelectionOffsetForPositionFaultFault(crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault),
}

impl From<CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault> for crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault> for CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault {
    fn from(value: crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    LayoutQueriesGetSelectionOffsetForPositionFaultFault(crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault),
}

impl From<CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault> for crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault> for CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault {
    fn from(value: crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    LayoutQueriesGetSelectionOffsetForPositionFaultFault(crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault),
}

impl From<CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault> for crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault {
    fn from(value: CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault> for CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault {
    fn from(value: crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault) -> Self {
        CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault {
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault) -> Self {
        match value {
            CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn positioned_cluster_height_returns_difference() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.positionedClusterHeightReturnsDifference", "org.tiqian.core.CoreLayoutQueriesGapsTest.positionedClusterHeightReturnsDifference", || {
        TestTraceRecorder::new("CoreLayoutQueriesGapsTest").section(&"positionedClusterHeightReturnsDifference");
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_sample_result().unwrap();
        let positions = LayoutQueries::layout_queries_positioned_clusters((result).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20.0f64, (positions[0usize]).clone().get_height(), None).unwrap();
    });
}

#[test]
fn get_line_for_offset_uses_nearest_line_when_gap_between_lines() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getLineForOffsetUsesNearestLineWhenGapBetweenLines", "org.tiqian.core.CoreLayoutQueriesGapsTest.getLineForOffsetUsesNearestLineWhenGapBetweenLines", || {
        TestTraceRecorder::new("CoreLayoutQueriesGapsTest").section(&"getLineForOffsetUsesNearestLineWhenGapBetweenLines");
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(&"abcde", 100.0f64, None, Size::new(10.0f64, 40.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", &"cjk", 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", &"cjk", 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"c", &"cjk", 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(4u32, 5u32).unwrap(), &"e", &"cjk", 10.0f64)).clone(),
], &vec![], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 15.0f64, 0.0f64, 20.0f64, 20.0f64, None)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(4u32, 5u32).unwrap(), 2, 3, 35.0f64, 25.0f64, 45.0f64, 10.0f64, None)).clone(),
], None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_line_for_offset((result).clone(), 3), None).unwrap();
    });
}

#[test]
fn get_bounding_boxes_int_delegates_to_text_range() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getBoundingBoxesIntDelegatesToTextRange", "org.tiqian.core.CoreLayoutQueriesGapsTest.getBoundingBoxesIntDelegatesToTextRange", || {
        TestTraceRecorder::new("CoreLayoutQueriesGapsTest").section(&"getBoundingBoxesIntDelegatesToTextRange");
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_sample_result().unwrap();
        let from_int = LayoutQueries::layout_queries_get_bounding_boxes_int((result).clone(), 2, 4).unwrap();
        let from_range = LayoutQueries::layout_queries_get_bounding_boxes((result).clone(), TextRange::new(2u32, 4u32).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_render_rects(&from_range).unwrap().as_str(),
CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_render_rects(&from_int).unwrap().as_str(), None).unwrap();
    });
}

#[test]
fn rich_text_background_uses_horizontal_padding() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.richTextBackgroundUsesHorizontalPadding", "org.tiqian.core.CoreLayoutQueriesGapsTest.richTextBackgroundUsesHorizontalPadding", || {
        TestTraceRecorder::new("CoreLayoutQueriesGapsTest").section(&"richTextBackgroundUsesHorizontalPadding");
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(&"AB", 100.0f64, Some(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_style(10.0f64)), Size::new(20.0f64, 20.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"A", &"latin", 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"B", &"latin", 10.0f64)).clone(),
], &vec![], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 15.0f64, 0.0f64, 20.0f64, 20.0f64, None)).clone(),
], Some(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_empty_debug())).unwrap();
        let span = RichTextSpan::new(TextRange::new(0u32, 2u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }),
RichTextPaint::rich_text_paint_with_background(RichTextBackgroundPaint::rich_text_background_paint_with_horizontal_padding(5.0f64).unwrap()).unwrap());
        let occupied = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &vec![(span).clone()]).unwrap();
        let segments = LayoutQueries::layout_queries_rich_text_background_segments((result).clone(), &occupied);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((segments.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn rich_text_background_trailing_padding_when_span_ends_at_segment_end() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.richTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEnd", "org.tiqian.core.CoreLayoutQueriesGapsTest.richTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEnd", || {
        TestTraceRecorder::new("CoreLayoutQueriesGapsTest").section(&"richTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEnd");
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(&"AB", 100.0f64, Some(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_style(10.0f64)), Size::new(20.0f64, 20.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"A", &"latin", 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"B", &"latin", 10.0f64)).clone(),
], &vec![], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 15.0f64, 0.0f64, 20.0f64, 20.0f64, None)).clone(),
], Some(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_empty_debug())).unwrap();
        let span = RichTextSpan::new(TextRange::new(0u32, 2u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }),
RichTextPaint::rich_text_paint_with_background(RichTextBackgroundPaint::rich_text_background_paint_with_horizontal_padding(5.0f64).unwrap()).unwrap());
        let occupied = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &vec![(span).clone()]).unwrap();
        let segments = LayoutQueries::layout_queries_rich_text_background_segments((result).clone(), &occupied);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((segments.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((segments[0usize].right) > (15.0f64), None).unwrap();
    });
}

#[test]
fn rich_text_background_uniform_paragraph_style_uses_paragraph_style() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.richTextBackgroundUniformParagraphStyleUsesParagraphStyle", "org.tiqian.core.CoreLayoutQueriesGapsTest.richTextBackgroundUniformParagraphStyleUsesParagraphStyle", || {
        TestTraceRecorder::new("CoreLayoutQueriesGapsTest").section(&"richTextBackgroundUniformParagraphStyleUsesParagraphStyle");
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(&"AB", 100.0f64, Some(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_style(12.0f64)), Size::new(20.0f64, 20.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"A", &"latin", 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"B", &"latin", 10.0f64)).clone(),
], &vec![], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 15.0f64, 0.0f64, 20.0f64, 20.0f64, None)).clone(),
], Some(LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None,
None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])))).unwrap();
        let paint = RichTextPaint::rich_text_paint_with_background(RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::UniformParagraphStyle),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap()).unwrap();
        let span = RichTextSpan::new(TextRange::new(0u32, 2u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), (paint).clone());
        let occupied = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &vec![(span).clone()]).unwrap();
        let segments = LayoutQueries::layout_queries_rich_text_background_segments((result).clone(), &occupied);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((segments.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn marked_face_vertical_bounds_uses_fallback_when_no_metric_matches() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.markedFaceVerticalBoundsUsesFallbackWhenNoMetricMatches", "org.tiqian.core.CoreLayoutQueriesGapsTest.markedFaceVerticalBoundsUsesFallbackWhenNoMetricMatches", || {
        TestTraceRecorder::new("CoreLayoutQueriesGapsTest").section(&"markedFaceVerticalBoundsUsesFallbackWhenNoMetricMatches");
        let metric = MetricDecisionInfo::new(TextRange::new(0u32, 1u32).unwrap(), "test", "test", "test", 8.0f64, 2.0f64, 0.0f64, "test", 8.0f64, 2.0f64, "test", "test", "test", "test");
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(&"AB", 100.0f64, Some(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_style(10.0f64)), Size::new(20.0f64, 20.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(0u32, 2u32).unwrap(), &"AB", &"latin", 20.0f64)).clone(),
], &vec![], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 15.0f64, 0.0f64, 20.0f64, 20.0f64, None)).clone(),
], Some(LayoutDebugInfo::new(None, Some(vec![(metric).clone()]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None,
None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])))).unwrap();
        let span = RichTextSpan::new(TextRange::new(0u32, 2u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), RichTextPaint::new(None,
Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap());
        let occupied = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &vec![(span).clone()]).unwrap();
        let segments = LayoutQueries::layout_queries_rich_text_background_segments((result).clone(), &occupied);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((segments.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn get_selection_offset_for_position_returns_nearest_when_before_first_cluster() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionOffsetForPositionReturnsNearestWhenBeforeFirstCluster", "org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionOffsetForPositionReturnsNearestWhenBeforeFirstCluster", || {
        TestTraceRecorder::new("CoreLayoutQueriesGapsTest").section(&"getSelectionOffsetForPositionReturnsNearestWhenBeforeFirstCluster");
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_sample_result().unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_selection_offset_for_position((result).clone(), 3.0f64, 5.0f64).unwrap(), None).unwrap();
    });
}

#[test]
fn get_selection_offset_for_position_returns_nearest_when_after_last_cluster() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionOffsetForPositionReturnsNearestWhenAfterLastCluster", "org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionOffsetForPositionReturnsNearestWhenAfterLastCluster", || {
        TestTraceRecorder::new("CoreLayoutQueriesGapsTest").section(&"getSelectionOffsetForPositionReturnsNearestWhenAfterLastCluster");
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_sample_result().unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, LayoutQueries::layout_queries_get_selection_offset_for_position((result).clone(), 35.0f64, 25.0f64).unwrap(), None).unwrap();
    });
}

#[test]
fn get_selection_offset_for_position_returns_start_of_line_when_clusters_empty() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmpty", "org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmpty", || {
        TestTraceRecorder::new("CoreLayoutQueriesGapsTest").section(&"getSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmpty");
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(&"", 100.0f64, None, Size::new(0.0f64, 20.0f64), &vec![], &vec![], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(0u32, 0u32).unwrap(), 0, 4294967295u32, 15.0f64, 0.0f64, 20.0f64, 0.0f64, None)).clone(),
], None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_selection_offset_for_position((result).clone(), 5.0f64, 10.0f64).unwrap(), None).unwrap();
    });
}

#[test]
fn get_selection_word_boundary_for_emoji_zwj_sequence() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionWordBoundaryForEmojiZwjSequence", "org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionWordBoundaryForEmojiZwjSequence", || {
        TestTraceRecorder::new("CoreLayoutQueriesGapsTest").section(&"getSelectionWordBoundaryForEmojiZwjSequence");
        let text = TestHelpers::test_helpers_surrogate_text(&vec![55357, 56425, 8205, 55357, 56425]);
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(text.as_str(), 100.0f64, None, Size::new(50.0f64, 20.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(0u32, 5u32).unwrap(), text.as_str(), &"emoji", 50.0f64)).clone(),
], &vec![], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(0u32, 5u32).unwrap(), 0, 0, 15.0f64, 0.0f64, 20.0f64, 50.0f64, None)).clone(),
], None).unwrap();
        let boundary = LayoutQueries::layout_queries_get_selection_word_boundary((result).clone(), 5).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 5u32).unwrap().to_string().as_str(), boundary.to_string().as_str(), None).unwrap();
    });
}

#[test]
fn get_selection_word_boundary_for_punctuation_returns_single() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionWordBoundaryForPunctuationReturnsSingle", "org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionWordBoundaryForPunctuationReturnsSingle", || {
        TestTraceRecorder::new("CoreLayoutQueriesGapsTest").section(&"getSelectionWordBoundaryForPunctuationReturnsSingle");
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(&"A,B", 100.0f64, None, Size::new(30.0f64, 20.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"A", &"latin", 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &",", &"latin", 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"B", &"latin", 10.0f64)).clone(),
], &vec![], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 2, 15.0f64, 0.0f64, 20.0f64, 30.0f64, None)).clone(),
], None).unwrap();
        let boundary = LayoutQueries::layout_queries_get_selection_word_boundary((result).clone(), 1).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(1u32, 2u32).unwrap().to_string().as_str(), boundary.to_string().as_str(), None).unwrap();
    });
}

#[test]
fn positioned_clusters_produces_source_stops_for_latin_run() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.positionedClustersProducesSourceStopsForLatinRun", "org.tiqian.core.CoreLayoutQueriesGapsTest.positionedClustersProducesSourceStopsForLatinRun", || {
        TestTraceRecorder::new("CoreLayoutQueriesGapsTest").section(&"positionedClustersProducesSourceStopsForLatinRun");
        let text = "Hi".to_string();
        let glyph_range = TextRange::new(0u32, 2u32).unwrap();
        let glyphs = vec![
    (Glyph::new(1u32, (glyph_range).clone(), 10.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None)).clone(),
    (Glyph::new(2u32, (glyph_range).clone(), 10.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None)).clone(),
];
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(text.as_str(), 100.0f64, Some(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_style(10.0f64)), Size::new(20.0f64, 20.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster((glyph_range).clone(), text.as_str(), &"latin", 20.0f64)).clone(),
], &vec![
    (GlyphRun::new((glyph_range).clone(), "latin", glyphs.to_vec(), 20.0f64, Some(vec![]))).clone(),
], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line((glyph_range).clone(), 0, 0, 15.0f64, 0.0f64, 20.0f64, 20.0f64, None)).clone(),
], None).unwrap();
        let positioned = LayoutQueries::layout_queries_positioned_clusters((result).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::from_ne_bytes((match &((positioned[0usize]).clone().source_stops) { None => 0, Some(__option1) => u32::try_from(((*__option1).clone().len()) & 0xFFFF_FFFF).unwrap_or(0) }).to_ne_bytes()),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((positioned[0usize]).clone().source_stops.is_some(), None).unwrap();
    });
}

#[test]
fn offset_for_x_uses_source_stops_when_available() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.offsetForXUsesSourceStopsWhenAvailable", "org.tiqian.core.CoreLayoutQueriesGapsTest.offsetForXUsesSourceStopsWhenAvailable", || {
        TestTraceRecorder::new("CoreLayoutQueriesGapsTest").section(&"offsetForXUsesSourceStopsWhenAvailable");
        let text = "Hi".to_string();
        let glyph_range = TextRange::new(0u32, 2u32).unwrap();
        let glyphs = vec![
    (Glyph::new(1u32, (glyph_range).clone(), 10.0f64, Some(5.0f64), Some(0.0f64), None, None, None, None)).clone(),
    (Glyph::new(2u32, (glyph_range).clone(), 10.0f64, Some(15.0f64), Some(0.0f64), None, None, None, None)).clone(),
];
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(text.as_str(), 100.0f64, Some(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_style(10.0f64)), Size::new(20.0f64, 20.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster((glyph_range).clone(), text.as_str(), &"latin", 20.0f64)).clone(),
], &vec![
    (GlyphRun::new((glyph_range).clone(), "latin", glyphs.to_vec(), 20.0f64, Some(vec![]))).clone(),
], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line((glyph_range).clone(), 0, 0, 15.0f64, 0.0f64, 20.0f64, 20.0f64, None)).clone(),
], Some(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_empty_debug())).unwrap();
        let positioned = LayoutQueries::layout_queries_positioned_clusters((result).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_offset_for_position((result).clone(), positioned[0usize].left, 10.0f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, LayoutQueries::layout_queries_get_offset_for_position((result).clone(), 15.0f64, 10.0f64), None).unwrap();
    });
}

#[test]
fn get_bounding_boxes_empty_range_returns_empty_list() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getBoundingBoxesEmptyRangeReturnsEmptyList", "org.tiqian.core.CoreLayoutQueriesGapsTest.getBoundingBoxesEmptyRangeReturnsEmptyList", || {
        TestTraceRecorder::new("CoreLayoutQueriesGapsTest").section(&"getBoundingBoxesEmptyRangeReturnsEmptyList");
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_sample_result().unwrap();
        let boxes = LayoutQueries::layout_queries_get_bounding_boxes((result).clone(), TextRange::new(2u32, 2u32).unwrap());
        let mut rendered_boxes = "[".to_string();
        let mut box_index = 0u32;
        while (i32::from_ne_bytes((box_index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((boxes.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if i32::from_ne_bytes((box_index).to_ne_bytes()) > (0) {
                rendered_boxes += &(", ");
            }
            rendered_boxes += &((boxes[usize::try_from(box_index).unwrap_or(0)]).clone().to_string());
            box_index = u32::wrapping_add(box_index, 1);
        }
        rendered_boxes += &("]");
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[]", rendered_boxes.as_str(), None).unwrap();
    });
}

#[test]
fn get_line_for_offset_returns_nearest_line() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getLineForOffsetReturnsNearestLine", "org.tiqian.core.CoreLayoutQueriesGapsTest.getLineForOffsetReturnsNearestLine", || {
        TestTraceRecorder::new("CoreLayoutQueriesGapsTest").section(&"getLineForOffsetReturnsNearestLine");
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(&"abc", 100.0f64, None, Size::new(30.0f64, 40.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", &"cjk", 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", &"cjk", 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"c", &"cjk", 10.0f64)).clone(),
], &vec![], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(0u32, 1u32).unwrap(), 0, 0, 15.0f64, 0.0f64, 20.0f64, 10.0f64, None)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(1u32, 2u32).unwrap(), 1, 1, 35.0f64, 25.0f64, 45.0f64, 10.0f64, None)).clone(),
], None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, LayoutQueries::layout_queries_get_line_for_offset((result).clone(), 10), None).unwrap();
    });
}

#[test]
fn get_cursor_rect_returns_caret_in_cluster() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getCursorRectReturnsCaretInCluster", "org.tiqian.core.CoreLayoutQueriesGapsTest.getCursorRectReturnsCaretInCluster", || {
        TestTraceRecorder::new("CoreLayoutQueriesGapsTest").section(&"getCursorRectReturnsCaretInCluster");
        let rect = LayoutQueries::layout_queries_get_cursor_rect(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_sample_result().unwrap(), 2).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(Rect::new(24.0f64, 0.0f64, 25.0f64, 20.0f64).to_string().as_str(), rect.to_string().as_str(), None).unwrap();
    });
}

#[test]
fn get_offset_for_position_uses_min_by_when_outside_clusters() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getOffsetForPositionUsesMinByWhenOutsideClusters", "org.tiqian.core.CoreLayoutQueriesGapsTest.getOffsetForPositionUsesMinByWhenOutsideClusters", || {
        TestTraceRecorder::new("CoreLayoutQueriesGapsTest").section(&"getOffsetForPositionUsesMinByWhenOutsideClusters");
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, LayoutQueries::layout_queries_get_offset_for_position(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_sample_result().unwrap(), 12.0f64, 5.0f64), None).unwrap();
    });
}

#[test]
fn get_selection_word_boundary_returns_empty_for_empty_text() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionWordBoundaryReturnsEmptyForEmptyText", "org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionWordBoundaryReturnsEmptyForEmptyText", || {
        TestTraceRecorder::new("CoreLayoutQueriesGapsTest").section(&"getSelectionWordBoundaryReturnsEmptyForEmptyText");
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(&"", 100.0f64, None, Size::new(0.0f64, 20.0f64), &vec![], &vec![], &vec![], None).unwrap();
        let boundary = LayoutQueries::layout_queries_get_selection_word_boundary((result).clone(), 0).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 0u32).unwrap().to_string().as_str(), boundary.to_string().as_str(), None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct CoreLayoutQueriesGapsTestHelpers;

impl CoreLayoutQueriesGapsTestHelpers {
    pub fn core_layout_queries_gaps_test_helpers_content(text: &str) -> TiqianTextContent {
        return TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
    }

    pub fn core_layout_queries_gaps_test_helpers_constraints(max_width: f64) -> Result<LayoutConstraints, TextRangeError> {
        return Ok(LayoutConstraints::new(max_width, Some(f64::INFINITY), Some(2147483647))?);
    }

    pub fn core_layout_queries_gaps_test_helpers_style(font_size: f64) -> TextStyle {
        return TextStyle::new(Some(vec![]), Some(font_size), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0f64), Some(InlineAttachment::None));
    }

    pub fn core_layout_queries_gaps_test_helpers_input(text: &str, max_width: f64, text_style: Option<TextStyle>) -> Result<LayoutInput, TextRangeError> {
        let style = match &(text_style) { None => TextStyle::new(Some(vec![]), Some(16.0f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0f64), Some(InlineAttachment::None)), Some(__option2) => (*__option2).clone() };
        return Ok(LayoutInput::new(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_content(text), Some((style).clone()), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some((Ic::zero()).clone()),
Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0f64), Some(1.0f64), Some(2.0f64))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_constraints(max_width)?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])));
    }

    pub fn core_layout_queries_gaps_test_helpers_cluster(range: TextRange, text: &str, font_key: &str, advance: f64) -> Cluster {
        return Cluster::new((range).clone(), text, font_key, advance, Some((text).to_string()), Some(0.0f64), Some(0.0f64), Some(0.0f64));
    }

    pub fn core_layout_queries_gaps_test_helpers_substituted_cluster(range: TextRange, text: &str, display_text: &str, font_key: &str, advance: f64) -> Cluster {
        return Cluster::new((range).clone(), text, font_key, advance, Some((display_text).to_string()), Some(0.0f64), Some(0.0f64), Some(0.0f64));
    }

    pub fn core_layout_queries_gaps_test_helpers_line(range: TextRange, cluster_start: u32, cluster_end: u32, baseline: f64, top: f64, bottom: f64, width: f64, indent: Option<f64>) -> LineBox {
        return LineBox::new((range).clone(), IntRange::new(cluster_start, cluster_end), baseline, top, bottom, width, width, width, Some(0.0f64), indent, Some(LineEndReason::ParagraphEnd), Some(0.0f64), Some(vec![]), LineDebugInfo::new(None.clone(), Some(vec![])));
    }

    pub fn core_layout_queries_gaps_test_helpers_empty_debug() -> LayoutDebugInfo {
        return LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None,
None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
    }

    pub fn core_layout_queries_gaps_test_helpers_render_rects(values: &Vec<Rect>) -> Result<String, UStringFault> {
        let mut output = Vec::<u16>::new();
        if let Some(&unit) = output.last() {
            if unit >= 55296 && unit <= 56319 && !"[".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        output.extend("[".encode_utf16());
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if i32::from_ne_bytes((index).to_ne_bytes()) > (0) {
                if let Some(&unit) = output.last() {
                    if unit >= 55296 && unit <= 56319 && !", ".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                output.extend(", ".encode_utf16());
            }
            if let Some(&unit) = output.last() {
                if unit >= 55296 && unit <= 56319 && !(values[usize::try_from(index).unwrap_or(0)]).clone().to_string().is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            output.extend((values[usize::try_from(index).unwrap_or(0)]).clone().to_string().encode_utf16());
            index = u32::wrapping_add(index, 1);
        }
        if let Some(&unit) = output.last() {
            if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        output.extend("]".encode_utf16());
        return Ok(String::from_utf16(output.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(output[output.len() - 1]) })?);
    }

    pub fn core_layout_queries_gaps_test_helpers_result_with(text: &str, max_width: f64, text_style: Option<TextStyle>, size: Size, clusters: &Vec<Cluster>, glyph_runs: &Vec<GlyphRun>, lines: &Vec<LineBox>, debug: Option<LayoutDebugInfo>) -> Result<LayoutResult, TextRangeError> {
        return Ok(LayoutResult::new(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_input(text, max_width, (text_style).clone())?, (size).clone(), (clusters).clone(), (glyph_runs).clone(), (lines).clone(), match &(debug) { None =>
CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_empty_debug(), Some(__option3) => (*__option3).clone() }));
    }

    pub fn core_layout_queries_gaps_test_helpers_sample_result() -> Result<LayoutResult, TextRangeError> {
        let text = "甲——乙".to_string();
        let dash_range = TextRange::new(1u32, 3u32)?;
        return Ok(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(text.as_str(), 40.0f64, Some(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_style(10.0f64)), Size::new(34.0f64, 40.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(0u32, 1u32)?, &"甲", &"cjk", 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_substituted_cluster((dash_range).clone(), &"——", &"⸺", &"cjk", 20.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(3u32, 4u32)?, &"乙", &"cjk", 10.0f64)).clone(),
], &vec![], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(0u32, 3u32)?, 0, 1, 15.0f64, 0.0f64, 20.0f64, 30.0f64, Some(4.0f64))).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(3u32, 4u32)?, 2, 2, 35.0f64, 20.0f64, 40.0f64, 10.0f64, None)).clone(),
], None)?);
    }
}
