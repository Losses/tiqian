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
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;


#[derive(Debug, Clone, PartialEq)]
pub enum CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestRichTextBackgroundUsesHorizontalPaddingFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestRichTextBackgroundUniformParagraphStyleUsesParagraphStyleFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestRichTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEndFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestPositionedClustersProducesSourceStopsForLatinRunFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestPositionedClusterHeightReturnsDifferenceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestOffsetForXUsesSourceStopsWhenAvailableFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestMarkedFaceVerticalBoundsUsesFallbackWhenNoMetricMatchesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetSelectionWordBoundaryReturnsEmptyForEmptyTextFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForPunctuationReturnsSingleFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetSelectionWordBoundaryForEmojiZwjSequenceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault::NoSuchElementErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmptyFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault::NoSuchElementErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenBeforeFirstClusterFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault::NoSuchElementErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetSelectionOffsetForPositionReturnsNearestWhenAfterLastClusterFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetOffsetForPositionUsesMinByWhenOutsideClustersFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetLineForOffsetUsesNearestLineWhenGapBetweenLinesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetLineForOffsetReturnsNearestLineFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault::NoSuchElementErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetCursorRectReturnsCaretInClusterFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetBoundingBoxesIntDelegatesToTextRangeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
impl std::fmt::Display for CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            CoreLayoutQueriesGapsTestGetBoundingBoxesEmptyRangeReturnsEmptyListFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,76,97,121,111,117,116,81,117,101,114,105,101,115,71,97,112,115,84,101,115,116]))).section(UStr::new(&[112,111,115,105,116,105,111,110,101,100,67,108,117,115,116,101,114,72,101,105,103,104,116,82,101,116,117,114,110,115,68,105,102,102,101,114,101,110,99,101]));
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_sample_result().unwrap();
        let positions = LayoutQueries::layout_queries_positioned_clusters((result).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20.0f64, (positions[0usize]).clone().get_height(), None).unwrap();
    });
}

#[test]
fn get_line_for_offset_uses_nearest_line_when_gap_between_lines() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getLineForOffsetUsesNearestLineWhenGapBetweenLines", "org.tiqian.core.CoreLayoutQueriesGapsTest.getLineForOffsetUsesNearestLineWhenGapBetweenLines", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,76,97,121,111,117,116,81,117,101,114,105,101,115,71,97,112,115,84,101,115,116]))).section(UStr::new(&[103,101,116,76,105,110,101,70,111,114,79,102,102,115,101,116,85,115,101,115,78,101,97,114,101,115,116,76,105,110,101,87,104,101,110,71,97,112,66,101,116,119,101,101,110,76,105,110,101,115]));
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(UStr::new(&[97,98,99,100,101]), 100.0f64, None, Size::new(10.0f64, 40.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), UStr::new(&[97]), UStr::new(&[99,106,107]), 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), UStr::new(&[98]), UStr::new(&[99,106,107]), 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), UStr::new(&[99]), UStr::new(&[99,106,107]), 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(4u32, 5u32).unwrap(), UStr::new(&[101]), UStr::new(&[99,106,107]), 10.0f64)).clone(),
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
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,76,97,121,111,117,116,81,117,101,114,105,101,115,71,97,112,115,84,101,115,116]))).section(UStr::new(&[103,101,116,66,111,117,110,100,105,110,103,66,111,120,101,115,73,110,116,68,101,108,101,103,97,116,101,115,84,111,84,101,120,116,82,97,110,103,101]));
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_sample_result().unwrap();
        let from_int = LayoutQueries::layout_queries_get_bounding_boxes_int((result).clone(), 2, 4).unwrap();
        let from_range = LayoutQueries::layout_queries_get_bounding_boxes((result).clone(), TextRange::new(2u32, 4u32).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_render_rects(&from_range).unwrap().as_ustr(), CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_render_rects(&from_int).unwrap().as_ustr(), None).unwrap();
    });
}

#[test]
fn rich_text_background_uses_horizontal_padding() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.richTextBackgroundUsesHorizontalPadding", "org.tiqian.core.CoreLayoutQueriesGapsTest.richTextBackgroundUsesHorizontalPadding", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,76,97,121,111,117,116,81,117,101,114,105,101,115,71,97,112,115,84,101,115,116]))).section(UStr::new(&[114,105,99,104,84,101,120,116,66,97,99,107,103,114,111,117,110,100,85,115,101,115,72,111,114,105,122,111,110,116,97,108,80,97,100,100,105,110,103]));
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(UStr::new(&[65,66]), 100.0f64, Some(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_style(10.0f64)), Size::new(20.0f64, 20.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), UStr::new(&[65]), UStr::new(&[108,97,116,105,110]), 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), UStr::new(&[66]), UStr::new(&[108,97,116,105,110]), 10.0f64)).clone(),
], &vec![], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 15.0f64, 0.0f64, 20.0f64, 20.0f64, None)).clone(),
], Some(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_empty_debug())).unwrap();
        let span = RichTextSpan::new(TextRange::new(0u32, 2u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), RichTextPaint::rich_text_paint_with_background(RichTextBackgroundPaint::rich_text_background_paint_with_horizontal_padding(5.0f64).unwrap()).unwrap());
        let occupied = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &vec![(span).clone()]).unwrap();
        let segments = LayoutQueries::layout_queries_rich_text_background_segments((result).clone(), &occupied);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((segments.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn rich_text_background_trailing_padding_when_span_ends_at_segment_end() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.richTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEnd", "org.tiqian.core.CoreLayoutQueriesGapsTest.richTextBackgroundTrailingPaddingWhenSpanEndsAtSegmentEnd", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,76,97,121,111,117,116,81,117,101,114,105,101,115,71,97,112,115,84,101,115,116]))).section(UStr::new(&[114,105,99,104,84,101,120,116,66,97,99,107,103,114,111,117,110,100,84,114,97,105,108,105,110,103,80,97,100,100,105,110,103,87,104,101,110,83,112,97,110,69,110,100,115,65,116,83,101,103,109,101,110,116,69,110,100]));
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(UStr::new(&[65,66]), 100.0f64, Some(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_style(10.0f64)), Size::new(20.0f64, 20.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), UStr::new(&[65]), UStr::new(&[108,97,116,105,110]), 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), UStr::new(&[66]), UStr::new(&[108,97,116,105,110]), 10.0f64)).clone(),
], &vec![], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 15.0f64, 0.0f64, 20.0f64, 20.0f64, None)).clone(),
], Some(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_empty_debug())).unwrap();
        let span = RichTextSpan::new(TextRange::new(0u32, 2u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), RichTextPaint::rich_text_paint_with_background(RichTextBackgroundPaint::rich_text_background_paint_with_horizontal_padding(5.0f64).unwrap()).unwrap());
        let occupied = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &vec![(span).clone()]).unwrap();
        let segments = LayoutQueries::layout_queries_rich_text_background_segments((result).clone(), &occupied);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((segments.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((segments[0usize].right) > (15.0f64), None).unwrap();
    });
}

#[test]
fn rich_text_background_uniform_paragraph_style_uses_paragraph_style() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.richTextBackgroundUniformParagraphStyleUsesParagraphStyle", "org.tiqian.core.CoreLayoutQueriesGapsTest.richTextBackgroundUniformParagraphStyleUsesParagraphStyle", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,76,97,121,111,117,116,81,117,101,114,105,101,115,71,97,112,115,84,101,115,116]))).section(UStr::new(&[114,105,99,104,84,101,120,116,66,97,99,107,103,114,111,117,110,100,85,110,105,102,111,114,109,80,97,114,97,103,114,97,112,104,83,116,121,108,101,85,115,101,115,80,97,114,97,103,114,97,112,104,83,116,121,108,101]));
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(UStr::new(&[65,66]), 100.0f64, Some(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_style(12.0f64)), Size::new(20.0f64, 20.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), UStr::new(&[65]), UStr::new(&[108,97,116,105,110]), 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), UStr::new(&[66]), UStr::new(&[108,97,116,105,110]), 10.0f64)).clone(),
], &vec![], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 15.0f64, 0.0f64, 20.0f64, 20.0f64, None)).clone(),
],
Some(LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])))).unwrap();
        let paint = RichTextPaint::rich_text_paint_with_background(RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::UniformParagraphStyle), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap()).unwrap();
        let span = RichTextSpan::new(TextRange::new(0u32, 2u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), (paint).clone());
        let occupied = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &vec![(span).clone()]).unwrap();
        let segments = LayoutQueries::layout_queries_rich_text_background_segments((result).clone(), &occupied);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((segments.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn marked_face_vertical_bounds_uses_fallback_when_no_metric_matches() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.markedFaceVerticalBoundsUsesFallbackWhenNoMetricMatches", "org.tiqian.core.CoreLayoutQueriesGapsTest.markedFaceVerticalBoundsUsesFallbackWhenNoMetricMatches", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,76,97,121,111,117,116,81,117,101,114,105,101,115,71,97,112,115,84,101,115,116]))).section(UStr::new(&[109,97,114,107,101,100,70,97,99,101,86,101,114,116,105,99,97,108,66,111,117,110,100,115,85,115,101,115,70,97,108,108,98,97,99,107,87,104,101,110,78,111,77,101,116,114,105,99,77,97,116,99,104,101,115]));
        let metric = MetricDecisionInfo::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[116,101,115,116])), &(UStr::new(&[116,101,115,116])), &(UStr::new(&[116,101,115,116])), 8.0f64, 2.0f64, 0.0f64, &(UStr::new(&[116,101,115,116])), 8.0f64, 2.0f64, &(UStr::new(&[116,101,115,116])), &(UStr::new(&[116,101,115,116])), &(UStr::new(&[116,101,115,116])), &(UStr::new(&[116,101,115,116])));
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(UStr::new(&[65,66]), 100.0f64, Some(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_style(10.0f64)), Size::new(20.0f64, 20.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(0u32, 2u32).unwrap(), UStr::new(&[65,66]), UStr::new(&[108,97,116,105,110]), 20.0f64)).clone(),
], &vec![], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 15.0f64, 0.0f64, 20.0f64, 20.0f64, None)).clone(),
],
Some(LayoutDebugInfo::new(None, Some(vec![(metric).clone()]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])))).unwrap();
        let span = RichTextSpan::new(TextRange::new(0u32, 2u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap());
        let occupied = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &vec![(span).clone()]).unwrap();
        let segments = LayoutQueries::layout_queries_rich_text_background_segments((result).clone(), &occupied);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((segments.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn get_selection_offset_for_position_returns_nearest_when_before_first_cluster() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionOffsetForPositionReturnsNearestWhenBeforeFirstCluster", "org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionOffsetForPositionReturnsNearestWhenBeforeFirstCluster", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,76,97,121,111,117,116,81,117,101,114,105,101,115,71,97,112,115,84,101,115,116]))).section(UStr::new(&[103,101,116,83,101,108,101,99,116,105,111,110,79,102,102,115,101,116,70,111,114,80,111,115,105,116,105,111,110,82,101,116,117,114,110,115,78,101,97,114,101,115,116,87,104,101,110,66,101,102,111,114,101,70,105,114,115,116,67,108,117,115,116,101,114]));
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_sample_result().unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_selection_offset_for_position((result).clone(), 3.0f64, 5.0f64).unwrap(), None).unwrap();
    });
}

#[test]
fn get_selection_offset_for_position_returns_nearest_when_after_last_cluster() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionOffsetForPositionReturnsNearestWhenAfterLastCluster", "org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionOffsetForPositionReturnsNearestWhenAfterLastCluster", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,76,97,121,111,117,116,81,117,101,114,105,101,115,71,97,112,115,84,101,115,116]))).section(UStr::new(&[103,101,116,83,101,108,101,99,116,105,111,110,79,102,102,115,101,116,70,111,114,80,111,115,105,116,105,111,110,82,101,116,117,114,110,115,78,101,97,114,101,115,116,87,104,101,110,65,102,116,101,114,76,97,115,116,67,108,117,115,116,101,114]));
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_sample_result().unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, LayoutQueries::layout_queries_get_selection_offset_for_position((result).clone(), 35.0f64, 25.0f64).unwrap(), None).unwrap();
    });
}

#[test]
fn get_selection_offset_for_position_returns_start_of_line_when_clusters_empty() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmpty", "org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionOffsetForPositionReturnsStartOfLineWhenClustersEmpty", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,76,97,121,111,117,116,81,117,101,114,105,101,115,71,97,112,115,84,101,115,116]))).section(UStr::new(&[103,101,116,83,101,108,101,99,116,105,111,110,79,102,102,115,101,116,70,111,114,80,111,115,105,116,105,111,110,82,101,116,117,114,110,115,83,116,97,114,116,79,102,76,105,110,101,87,104,101,110,67,108,117,115,116,101,114,115,69,109,112,116,121]));
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(UStr::new(&[]), 100.0f64, None, Size::new(0.0f64, 20.0f64), &vec![], &vec![], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(0u32, 0u32).unwrap(), 0, 4294967295u32, 15.0f64, 0.0f64, 20.0f64, 0.0f64, None)).clone(),
], None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_selection_offset_for_position((result).clone(), 5.0f64, 10.0f64).unwrap(), None).unwrap();
    });
}

#[test]
fn get_selection_word_boundary_for_emoji_zwj_sequence() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionWordBoundaryForEmojiZwjSequence", "org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionWordBoundaryForEmojiZwjSequence", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,76,97,121,111,117,116,81,117,101,114,105,101,115,71,97,112,115,84,101,115,116]))).section(UStr::new(&[103,101,116,83,101,108,101,99,116,105,111,110,87,111,114,100,66,111,117,110,100,97,114,121,70,111,114,69,109,111,106,105,90,119,106,83,101,113,117,101,110,99,101]));
        let text = TestHelpers::test_helpers_surrogate_text(&vec![55357, 56425, 8205, 55357, 56425]);
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(text.as_ustr(), 100.0f64, None, Size::new(50.0f64, 20.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(0u32, 5u32).unwrap(), text.as_ustr(), UStr::new(&[101,109,111,106,105]), 50.0f64)).clone(),
], &vec![], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(0u32, 5u32).unwrap(), 0, 0, 15.0f64, 0.0f64, 20.0f64, 50.0f64, None)).clone(),
], None).unwrap();
        let boundary = LayoutQueries::layout_queries_get_selection_word_boundary((result).clone(), 5).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", TextRange::new(0u32, 5u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", boundary.to_string()).as_str()).as_ustr(), None).unwrap();
    });
}

#[test]
fn get_selection_word_boundary_for_punctuation_returns_single() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionWordBoundaryForPunctuationReturnsSingle", "org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionWordBoundaryForPunctuationReturnsSingle", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,76,97,121,111,117,116,81,117,101,114,105,101,115,71,97,112,115,84,101,115,116]))).section(UStr::new(&[103,101,116,83,101,108,101,99,116,105,111,110,87,111,114,100,66,111,117,110,100,97,114,121,70,111,114,80,117,110,99,116,117,97,116,105,111,110,82,101,116,117,114,110,115,83,105,110,103,108,101]));
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(UStr::new(&[65,44,66]), 100.0f64, None, Size::new(30.0f64, 20.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), UStr::new(&[65]), UStr::new(&[108,97,116,105,110]), 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), UStr::new(&[44]), UStr::new(&[108,97,116,105,110]), 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), UStr::new(&[66]), UStr::new(&[108,97,116,105,110]), 10.0f64)).clone(),
], &vec![], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 2, 15.0f64, 0.0f64, 20.0f64, 30.0f64, None)).clone(),
], None).unwrap();
        let boundary = LayoutQueries::layout_queries_get_selection_word_boundary((result).clone(), 1).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", TextRange::new(1u32, 2u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", boundary.to_string()).as_str()).as_ustr(), None).unwrap();
    });
}

#[test]
fn positioned_clusters_produces_source_stops_for_latin_run() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.positionedClustersProducesSourceStopsForLatinRun", "org.tiqian.core.CoreLayoutQueriesGapsTest.positionedClustersProducesSourceStopsForLatinRun", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,76,97,121,111,117,116,81,117,101,114,105,101,115,71,97,112,115,84,101,115,116]))).section(UStr::new(&[112,111,115,105,116,105,111,110,101,100,67,108,117,115,116,101,114,115,80,114,111,100,117,99,101,115,83,111,117,114,99,101,83,116,111,112,115,70,111,114,76,97,116,105,110,82,117,110]));
        let text = UString::from("Hi").to_ustring();
        let glyph_range = TextRange::new(0u32, 2u32).unwrap();
        let glyphs = vec![
    (Glyph::new(1u32, (glyph_range).clone(), 10.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None)).clone(),
    (Glyph::new(2u32, (glyph_range).clone(), 10.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None)).clone(),
];
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(text.as_ustr(), 100.0f64, Some(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_style(10.0f64)), Size::new(20.0f64, 20.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster((glyph_range).clone(), text.as_ustr(), UStr::new(&[108,97,116,105,110]), 20.0f64)).clone(),
], &vec![
    (GlyphRun::new((glyph_range).clone(), &(UStr::new(&[108,97,116,105,110])), glyphs.to_vec(), 20.0f64, Some(vec![]))).clone(),
], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line((glyph_range).clone(), 0, 0, 15.0f64, 0.0f64, 20.0f64, 20.0f64, None)).clone(),
], None).unwrap();
        let positioned = LayoutQueries::layout_queries_positioned_clusters((result).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::from_ne_bytes(((match &((positioned[0usize]).clone().source_stops) { None => 0, Some(__option1) => u32::try_from(((*__option1).clone().len()) & 0xFFFF_FFFF).unwrap_or(0) }) as u32).to_ne_bytes()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((positioned[0usize]).clone().source_stops.is_some(), None).unwrap();
    });
}

#[test]
fn offset_for_x_uses_source_stops_when_available() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.offsetForXUsesSourceStopsWhenAvailable", "org.tiqian.core.CoreLayoutQueriesGapsTest.offsetForXUsesSourceStopsWhenAvailable", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,76,97,121,111,117,116,81,117,101,114,105,101,115,71,97,112,115,84,101,115,116]))).section(UStr::new(&[111,102,102,115,101,116,70,111,114,88,85,115,101,115,83,111,117,114,99,101,83,116,111,112,115,87,104,101,110,65,118,97,105,108,97,98,108,101]));
        let text = UString::from("Hi").to_ustring();
        let glyph_range = TextRange::new(0u32, 2u32).unwrap();
        let glyphs = vec![
    (Glyph::new(1u32, (glyph_range).clone(), 10.0f64, Some(5.0f64), Some(0.0f64), None, None, None, None)).clone(),
    (Glyph::new(2u32, (glyph_range).clone(), 10.0f64, Some(15.0f64), Some(0.0f64), None, None, None, None)).clone(),
];
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(text.as_ustr(), 100.0f64, Some(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_style(10.0f64)), Size::new(20.0f64, 20.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster((glyph_range).clone(), text.as_ustr(), UStr::new(&[108,97,116,105,110]), 20.0f64)).clone(),
], &vec![
    (GlyphRun::new((glyph_range).clone(), &(UStr::new(&[108,97,116,105,110])), glyphs.to_vec(), 20.0f64, Some(vec![]))).clone(),
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
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,76,97,121,111,117,116,81,117,101,114,105,101,115,71,97,112,115,84,101,115,116]))).section(UStr::new(&[103,101,116,66,111,117,110,100,105,110,103,66,111,120,101,115,69,109,112,116,121,82,97,110,103,101,82,101,116,117,114,110,115,69,109,112,116,121,76,105,115,116]));
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_sample_result().unwrap();
        let boxes = LayoutQueries::layout_queries_get_bounding_boxes((result).clone(), TextRange::new(2u32, 2u32).unwrap());
        let mut rendered_boxes = UString::from("[").to_ustring();
        let mut box_index = 0u32;
        while (i32::from_ne_bytes(((box_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((boxes.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if i32::from_ne_bytes(((box_index) as i32).to_ne_bytes()) > (0) {
                rendered_boxes += &(UString::from(", "));
            }
            rendered_boxes += &((boxes[usize::try_from(box_index).unwrap_or(0)]).clone().to_string());
            box_index = u32::wrapping_add(box_index, 1);
        }
        rendered_boxes += &(UString::from("]"));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UStr::new(&[91,93]), rendered_boxes.as_ustr(), None).unwrap();
    });
}

#[test]
fn get_line_for_offset_returns_nearest_line() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getLineForOffsetReturnsNearestLine", "org.tiqian.core.CoreLayoutQueriesGapsTest.getLineForOffsetReturnsNearestLine", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,76,97,121,111,117,116,81,117,101,114,105,101,115,71,97,112,115,84,101,115,116]))).section(UStr::new(&[103,101,116,76,105,110,101,70,111,114,79,102,102,115,101,116,82,101,116,117,114,110,115,78,101,97,114,101,115,116,76,105,110,101]));
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(UStr::new(&[97,98,99]), 100.0f64, None, Size::new(30.0f64, 40.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), UStr::new(&[97]), UStr::new(&[99,106,107]), 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), UStr::new(&[98]), UStr::new(&[99,106,107]), 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), UStr::new(&[99]), UStr::new(&[99,106,107]), 10.0f64)).clone(),
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
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,76,97,121,111,117,116,81,117,101,114,105,101,115,71,97,112,115,84,101,115,116]))).section(UStr::new(&[103,101,116,67,117,114,115,111,114,82,101,99,116,82,101,116,117,114,110,115,67,97,114,101,116,73,110,67,108,117,115,116,101,114]));
        let rect = LayoutQueries::layout_queries_get_cursor_rect(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_sample_result().unwrap(), 2).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(24.0f64, 0.0f64, 25.0f64, 20.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", rect.to_string()).as_str()).as_ustr(), None).unwrap();
    });
}

#[test]
fn get_offset_for_position_uses_min_by_when_outside_clusters() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getOffsetForPositionUsesMinByWhenOutsideClusters", "org.tiqian.core.CoreLayoutQueriesGapsTest.getOffsetForPositionUsesMinByWhenOutsideClusters", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,76,97,121,111,117,116,81,117,101,114,105,101,115,71,97,112,115,84,101,115,116]))).section(UStr::new(&[103,101,116,79,102,102,115,101,116,70,111,114,80,111,115,105,116,105,111,110,85,115,101,115,77,105,110,66,121,87,104,101,110,79,117,116,115,105,100,101,67,108,117,115,116,101,114,115]));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, LayoutQueries::layout_queries_get_offset_for_position(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_sample_result().unwrap(), 12.0f64, 5.0f64), None).unwrap();
    });
}

#[test]
fn get_selection_word_boundary_returns_empty_for_empty_text() {
    testlib::run("org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionWordBoundaryReturnsEmptyForEmptyText", "org.tiqian.core.CoreLayoutQueriesGapsTest.getSelectionWordBoundaryReturnsEmptyForEmptyText", || {
        TestTraceRecorder::new(&(UStr::new(&[67,111,114,101,76,97,121,111,117,116,81,117,101,114,105,101,115,71,97,112,115,84,101,115,116]))).section(UStr::new(&[103,101,116,83,101,108,101,99,116,105,111,110,87,111,114,100,66,111,117,110,100,97,114,121,82,101,116,117,114,110,115,69,109,112,116,121,70,111,114,69,109,112,116,121,84,101,120,116]));
        let result = CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(UStr::new(&[]), 100.0f64, None, Size::new(0.0f64, 20.0f64), &vec![], &vec![], &vec![], None).unwrap();
        let boundary = LayoutQueries::layout_queries_get_selection_word_boundary((result).clone(), 0).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", TextRange::new(0u32, 0u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", boundary.to_string()).as_str()).as_ustr(), None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct CoreLayoutQueriesGapsTestHelpers;

impl CoreLayoutQueriesGapsTestHelpers {
    pub fn core_layout_queries_gaps_test_helpers_content(text: &UStr) -> TiqianTextContent {
        return TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
    }

    pub fn core_layout_queries_gaps_test_helpers_constraints(max_width: f64) -> Result<LayoutConstraints, TextRangeError> {
        return Ok(LayoutConstraints::new(max_width, Some(f64::INFINITY), Some(2147483647))?);
    }

    pub fn core_layout_queries_gaps_test_helpers_style(font_size: f64) -> TextStyle {
        return TextStyle::new(Some(vec![]), Some(font_size), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0f64), Some(InlineAttachment::None));
    }

    pub fn core_layout_queries_gaps_test_helpers_input(text: &UStr, max_width: f64, text_style: Option<TextStyle>) -> Result<LayoutInput, TextRangeError> {
        let style = match &(text_style) { None => TextStyle::new(Some(vec![]), Some(16.0f64), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0f64), Some(InlineAttachment::None)), Some(__option2) => (*__option2).clone() };
        return Ok(LayoutInput::new(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_content(text), Some((style).clone()), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some((Ic::zero()).clone()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0f64), Some(1.0f64), Some(2.0f64))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_constraints(max_width)?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])));
    }

    pub fn core_layout_queries_gaps_test_helpers_cluster(range: TextRange, text: &UStr, font_key: &UStr, advance: f64) -> Cluster {
        return Cluster::new((range).clone(), text, font_key, advance, Some((text).to_ustring()), Some(0.0f64), Some(0.0f64), Some(0.0f64));
    }

    pub fn core_layout_queries_gaps_test_helpers_substituted_cluster(range: TextRange, text: &UStr, display_text: &UStr, font_key: &UStr, advance: f64) -> Cluster {
        return Cluster::new((range).clone(), text, font_key, advance, Some((display_text).to_ustring()), Some(0.0f64), Some(0.0f64), Some(0.0f64));
    }

    pub fn core_layout_queries_gaps_test_helpers_line(range: TextRange, cluster_start: u32, cluster_end: u32, baseline: f64, top: f64, bottom: f64, width: f64, indent: Option<f64>) -> LineBox {
        return LineBox::new((range).clone(), IntRange::new(cluster_start, cluster_end), baseline, top, bottom, width, width, width, Some(0.0f64), indent, Some(LineEndReason::ParagraphEnd), Some(0.0f64), Some(vec![]), LineDebugInfo::new(None.clone(), Some(vec![])));
    }

    pub fn core_layout_queries_gaps_test_helpers_empty_debug() -> LayoutDebugInfo {
        return LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
    }

    pub fn core_layout_queries_gaps_test_helpers_render_rects(values: &Vec<Rect>) -> Result<UString, UStringFault> {
        let mut output = Vec::<u16>::new();
        if let Some(&unit) = output.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("[").is_empty() {
                if !UString::from("[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        output.extend(UString::from("[").encode_utf16());
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if i32::from_ne_bytes(((index) as i32).to_ne_bytes()) > (0) {
                if let Some(&unit) = output.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(", ").is_empty() {
                        if !UString::from(", ").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                output.extend(UString::from(", ").encode_utf16());
            }
            if let Some(&unit) = output.last() {
                if unit >= 55296 && unit <= 56319 && !(values[usize::try_from(index).unwrap_or(0)]).clone().to_string().is_empty() {
                    if !(values[usize::try_from(index).unwrap_or(0)]).clone().to_string().encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            output.extend((values[usize::try_from(index).unwrap_or(0)]).clone().to_string().encode_utf16());
            index = u32::wrapping_add(index, 1);
        }
        if let Some(&unit) = output.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                if !UString::from("]").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        output.extend(UString::from("]").encode_utf16());
        return Ok(UString::from_utf16(output.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(output[output.len() - 1]) })?);
    }

    pub fn core_layout_queries_gaps_test_helpers_result_with(text: &UStr, max_width: f64, text_style: Option<TextStyle>, size: Size, clusters: &Vec<Cluster>, glyph_runs: &Vec<GlyphRun>, lines: &Vec<LineBox>, debug: Option<LayoutDebugInfo>) -> Result<LayoutResult,
TextRangeError> {
        return Ok(LayoutResult::new(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_input(text, max_width, (text_style).clone())?, (size).clone(), (clusters).clone(), (glyph_runs).clone(), (lines).clone(), match &(debug) { None => CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_empty_debug(), Some(__option3) => (*__option3).clone() }));
    }

    pub fn core_layout_queries_gaps_test_helpers_sample_result() -> Result<LayoutResult, TextRangeError> {
        let text = UString::from("甲——乙").to_ustring();
        let dash_range = TextRange::new(1u32, 3u32)?;
        return Ok(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_result_with(text.as_ustr(), 40.0f64, Some(CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_style(10.0f64)), Size::new(34.0f64, 40.0f64), &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(0u32, 1u32)?, UStr::new(&[30002]), UStr::new(&[99,106,107]), 10.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_substituted_cluster((dash_range).clone(), UStr::new(&[8212,8212]), UStr::new(&[11834]), UStr::new(&[99,106,107]), 20.0f64)).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_cluster(TextRange::new(3u32, 4u32)?, UStr::new(&[20057]), UStr::new(&[99,106,107]), 10.0f64)).clone(),
], &vec![], &vec![
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(0u32, 3u32)?, 0, 1, 15.0f64, 0.0f64, 20.0f64, 30.0f64, Some(4.0f64))).clone(),
    (CoreLayoutQueriesGapsTestHelpers::core_layout_queries_gaps_test_helpers_line(TextRange::new(3u32, 4u32)?, 2, 2, 35.0f64, 20.0f64, 40.0f64, 10.0f64, None)).clone(),
], None)?);
    }
}
