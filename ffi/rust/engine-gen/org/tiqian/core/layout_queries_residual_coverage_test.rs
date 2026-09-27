#![cfg(test)]

use crate::org::tiqian::core::bopomofo_decision_info::BopomofoDecisionInfo;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::cluster_geometry_decision_info::ClusterGeometryDecisionInfo;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::glyph_run::GlyphRun;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_object_boundary_adjustment::InlineObjectBoundaryAdjustment;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
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
use crate::org::tiqian::core::rich_text_corner_radii::RichTextCornerRadii;
use crate::org::tiqian::core::rich_text_line_segment::RichTextLineSegment;
use crate::org::tiqian::core::rich_text_paint::RichTextPaint;
use crate::org::tiqian::core::rich_text_role::RichTextRole;
use crate::org::tiqian::core::rich_text_span::RichTextSpan;
use crate::org::tiqian::core::ruby_decision_info::RubyDecisionInfo;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::size::Size;
use crate::org::tiqian::core::source_boundary_bias::SourceBoundaryBias;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::test::test_helpers::TestHelpers;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::std::u_string_exception::UStringFault;
use std::fmt::Write;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestZeroWidthClustersReturnTheirStartInHitTestsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestZeroWidthClustersReturnTheirStartInHitTestsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestZeroWidthClustersReturnTheirStartInHitTestsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestZeroWidthClustersReturnTheirStartInHitTestsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestZeroWidthClustersReturnTheirStartInHitTestsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestZeroWidthClustersReturnTheirStartInHitTestsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestZeroWidthClustersReturnTheirStartInHitTestsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestZeroWidthClustersReturnTheirStartInHitTestsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestZeroWidthClustersReturnTheirStartInHitTestsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestZeroWidthClustersReturnTheirStartInHitTestsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestZeroWidthClustersReturnTheirStartInHitTestsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestZeroWidthClustersReturnTheirStartInHitTestsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestZeroWidthClustersReturnTheirStartInHitTestsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestZeroWidthClustersReturnTheirStartInHitTestsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestZeroWidthClustersReturnTheirStartInHitTestsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestZeroWidthClustersReturnTheirStartInHitTestsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestWordBoundaryForPositionHandlesANonFiniteYFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestWordBoundaryForPositionHandlesANonFiniteYFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestWordBoundaryForPositionHandlesANonFiniteYFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestWordBoundaryForPositionHandlesANonFiniteYFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestWordBoundaryForPositionHandlesANonFiniteYFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestWordBoundaryForPositionHandlesANonFiniteYFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestWordBoundaryForPositionHandlesANonFiniteYFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestWordBoundaryForPositionHandlesANonFiniteYFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestWordBoundaryForPositionHandlesANonFiniteYFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestWordBoundaryForPositionHandlesANonFiniteYFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestWordBoundaryForPositionHandlesANonFiniteYFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestWordBoundaryForPositionHandlesANonFiniteYFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestWordBoundaryForPositionHandlesANonFiniteYFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestWordBoundaryForPositionHandlesANonFiniteYFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestWordBoundaryForPositionHandlesANonFiniteYFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestWordBoundaryForPositionHandlesANonFiniteYFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestUniformTextStylePrefersIdeographicMetricsThenAnyMatchingFaceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestUniformTextStylePrefersIdeographicMetricsThenAnyMatchingFaceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestUniformTextStylePrefersIdeographicMetricsThenAnyMatchingFaceFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestUniformTextStylePrefersIdeographicMetricsThenAnyMatchingFaceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestUniformTextStylePrefersIdeographicMetricsThenAnyMatchingFaceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestUniformTextStylePrefersIdeographicMetricsThenAnyMatchingFaceFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestUniformTextStylePrefersIdeographicMetricsThenAnyMatchingFaceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestUniformTextStylePrefersIdeographicMetricsThenAnyMatchingFaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestUniformTextStylePrefersIdeographicMetricsThenAnyMatchingFaceFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestUniformTextStylePrefersIdeographicMetricsThenAnyMatchingFaceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestUniformTextStylePrefersIdeographicMetricsThenAnyMatchingFaceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestUniformTextStylePrefersIdeographicMetricsThenAnyMatchingFaceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestUniformTextStylePrefersIdeographicMetricsThenAnyMatchingFaceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestUniformTextStylePrefersIdeographicMetricsThenAnyMatchingFaceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestUniformTextStylePrefersIdeographicMetricsThenAnyMatchingFaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestUniformTextStylePrefersIdeographicMetricsThenAnyMatchingFaceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestUniformTextStylePolicyResolvesSpanStyleOrParagraphStyleFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestUniformTextStylePolicyResolvesSpanStyleOrParagraphStyleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestUniformTextStylePolicyResolvesSpanStyleOrParagraphStyleFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestUniformTextStylePolicyResolvesSpanStyleOrParagraphStyleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestUniformTextStylePolicyResolvesSpanStyleOrParagraphStyleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestUniformTextStylePolicyResolvesSpanStyleOrParagraphStyleFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestUniformTextStylePolicyResolvesSpanStyleOrParagraphStyleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestUniformTextStylePolicyResolvesSpanStyleOrParagraphStyleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestUniformTextStylePolicyResolvesSpanStyleOrParagraphStyleFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestUniformTextStylePolicyResolvesSpanStyleOrParagraphStyleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestUniformTextStylePolicyResolvesSpanStyleOrParagraphStyleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestUniformTextStylePolicyResolvesSpanStyleOrParagraphStyleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestUniformTextStylePolicyResolvesSpanStyleOrParagraphStyleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestUniformTextStylePolicyResolvesSpanStyleOrParagraphStyleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestUniformTextStylePolicyResolvesSpanStyleOrParagraphStyleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestUniformTextStylePolicyResolvesSpanStyleOrParagraphStyleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestUniformTextStylePolicyPicksTheLastMatchingSpanFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestUniformTextStylePolicyPicksTheLastMatchingSpanFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestUniformTextStylePolicyPicksTheLastMatchingSpanFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestUniformTextStylePolicyPicksTheLastMatchingSpanFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestUniformTextStylePolicyPicksTheLastMatchingSpanFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestUniformTextStylePolicyPicksTheLastMatchingSpanFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestUniformTextStylePolicyPicksTheLastMatchingSpanFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestUniformTextStylePolicyPicksTheLastMatchingSpanFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestUniformTextStylePolicyPicksTheLastMatchingSpanFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestUniformTextStylePolicyPicksTheLastMatchingSpanFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestUniformTextStylePolicyPicksTheLastMatchingSpanFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestUniformTextStylePolicyPicksTheLastMatchingSpanFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestUniformTextStylePolicyPicksTheLastMatchingSpanFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestUniformTextStylePolicyPicksTheLastMatchingSpanFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestUniformTextStylePolicyPicksTheLastMatchingSpanFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestUniformTextStylePolicyPicksTheLastMatchingSpanFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestUniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMissesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestUniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMissesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestUniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMissesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestUniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMissesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestUniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMissesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestUniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMissesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestUniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMissesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestUniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMissesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestUniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMissesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestUniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMissesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestUniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMissesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestUniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMissesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestUniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMissesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestUniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMissesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestUniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMissesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestUniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMissesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestUniformTextStyleFallsBackWhenEveryMetricFieldDiffersFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestUniformTextStyleFallsBackWhenEveryMetricFieldDiffersFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestUniformTextStyleFallsBackWhenEveryMetricFieldDiffersFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestUniformTextStyleFallsBackWhenEveryMetricFieldDiffersFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestUniformTextStyleFallsBackWhenEveryMetricFieldDiffersFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestUniformTextStyleFallsBackWhenEveryMetricFieldDiffersFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestUniformTextStyleFallsBackWhenEveryMetricFieldDiffersFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestUniformTextStyleFallsBackWhenEveryMetricFieldDiffersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestUniformTextStyleFallsBackWhenEveryMetricFieldDiffersFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestUniformTextStyleFallsBackWhenEveryMetricFieldDiffersFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestUniformTextStyleFallsBackWhenEveryMetricFieldDiffersFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestUniformTextStyleFallsBackWhenEveryMetricFieldDiffersFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestUniformTextStyleFallsBackWhenEveryMetricFieldDiffersFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestUniformTextStyleFallsBackWhenEveryMetricFieldDiffersFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestUniformTextStyleFallsBackWhenEveryMetricFieldDiffersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestUniformTextStyleFallsBackWhenEveryMetricFieldDiffersFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestTrimmedDecorationSegmentsKeepOnlyDecorationRolesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestTrimmedDecorationSegmentsKeepOnlyDecorationRolesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestTrimmedDecorationSegmentsKeepOnlyDecorationRolesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestTrimmedDecorationSegmentsKeepOnlyDecorationRolesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestTrimmedDecorationSegmentsKeepOnlyDecorationRolesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestTrimmedDecorationSegmentsKeepOnlyDecorationRolesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestTrimmedDecorationSegmentsKeepOnlyDecorationRolesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestTrimmedDecorationSegmentsKeepOnlyDecorationRolesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestTrimmedDecorationSegmentsKeepOnlyDecorationRolesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestTrimmedDecorationSegmentsKeepOnlyDecorationRolesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestTrimmedDecorationSegmentsKeepOnlyDecorationRolesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestTrimmedDecorationSegmentsKeepOnlyDecorationRolesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestTrimmedDecorationSegmentsKeepOnlyDecorationRolesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestTrimmedDecorationSegmentsKeepOnlyDecorationRolesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestTrimmedDecorationSegmentsKeepOnlyDecorationRolesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestTrimmedDecorationSegmentsKeepOnlyDecorationRolesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestTrailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEndFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestTrailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEndFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestTrailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEndFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestTrailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEndFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestTrailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEndFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestTrailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEndFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestTrailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEndFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestTrailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEndFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestTrailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEndFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestTrailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEndFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestTrailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEndFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestTrailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEndFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestTrailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEndFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestTrailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEndFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestTrailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEndFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestTrailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEndFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestSupplementaryIdeographBeyondTheHanRangesIsItsOwnUnitFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestSupplementaryIdeographBeyondTheHanRangesIsItsOwnUnitFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestSupplementaryIdeographBeyondTheHanRangesIsItsOwnUnitFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSupplementaryIdeographBeyondTheHanRangesIsItsOwnUnitFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSupplementaryIdeographBeyondTheHanRangesIsItsOwnUnitFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestSupplementaryIdeographBeyondTheHanRangesIsItsOwnUnitFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSupplementaryIdeographBeyondTheHanRangesIsItsOwnUnitFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSupplementaryIdeographBeyondTheHanRangesIsItsOwnUnitFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestSupplementaryIdeographBeyondTheHanRangesIsItsOwnUnitFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSupplementaryIdeographBeyondTheHanRangesIsItsOwnUnitFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestSupplementaryIdeographBeyondTheHanRangesIsItsOwnUnitFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestSupplementaryIdeographBeyondTheHanRangesIsItsOwnUnitFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestSupplementaryIdeographBeyondTheHanRangesIsItsOwnUnitFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestSupplementaryIdeographBeyondTheHanRangesIsItsOwnUnitFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestSupplementaryIdeographBeyondTheHanRangesIsItsOwnUnitFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestSupplementaryIdeographBeyondTheHanRangesIsItsOwnUnitFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestSelectionWordKindCoversEveryHanBlockFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestSelectionWordKindCoversEveryHanBlockFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionWordKindCoversEveryHanBlockFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionWordKindCoversEveryHanBlockFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSelectionWordKindCoversEveryHanBlockFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionWordKindCoversEveryHanBlockFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionWordKindCoversEveryHanBlockFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSelectionWordKindCoversEveryHanBlockFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionWordKindCoversEveryHanBlockFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionWordKindCoversEveryHanBlockFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestSelectionWordKindCoversEveryHanBlockFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestSelectionWordKindCoversEveryHanBlockFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestSelectionWordKindCoversEveryHanBlockFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestSelectionWordKindCoversEveryHanBlockFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestSelectionWordKindCoversEveryHanBlockFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestSelectionWordKindCoversEveryHanBlockFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestSelectionWordBoundarySkipsInlineObjectsItDoesNotContainFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestSelectionWordBoundarySkipsInlineObjectsItDoesNotContainFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionWordBoundarySkipsInlineObjectsItDoesNotContainFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionWordBoundarySkipsInlineObjectsItDoesNotContainFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSelectionWordBoundarySkipsInlineObjectsItDoesNotContainFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionWordBoundarySkipsInlineObjectsItDoesNotContainFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionWordBoundarySkipsInlineObjectsItDoesNotContainFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSelectionWordBoundarySkipsInlineObjectsItDoesNotContainFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionWordBoundarySkipsInlineObjectsItDoesNotContainFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionWordBoundarySkipsInlineObjectsItDoesNotContainFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestSelectionWordBoundarySkipsInlineObjectsItDoesNotContainFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestSelectionWordBoundarySkipsInlineObjectsItDoesNotContainFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestSelectionWordBoundarySkipsInlineObjectsItDoesNotContainFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestSelectionWordBoundarySkipsInlineObjectsItDoesNotContainFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestSelectionWordBoundarySkipsInlineObjectsItDoesNotContainFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestSelectionWordBoundarySkipsInlineObjectsItDoesNotContainFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionRejectsDegenerateContentFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionRejectsDegenerateContentFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionRejectsDegenerateContentFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionRejectsDegenerateContentFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionRejectsDegenerateContentFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionRejectsDegenerateContentFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionRejectsDegenerateContentFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionRejectsDegenerateContentFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionRejectsDegenerateContentFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionRejectsDegenerateContentFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionRejectsDegenerateContentFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionRejectsDegenerateContentFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionRejectsDegenerateContentFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionRejectsDegenerateContentFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionRejectsDegenerateContentFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionRejectsDegenerateContentFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionPrefersTheCloserLaterLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionPrefersTheCloserLaterLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionPrefersTheCloserLaterLineFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionPrefersTheCloserLaterLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionPrefersTheCloserLaterLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionPrefersTheCloserLaterLineFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionPrefersTheCloserLaterLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionPrefersTheCloserLaterLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionPrefersTheCloserLaterLineFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionPrefersTheCloserLaterLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionPrefersTheCloserLaterLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionPrefersTheCloserLaterLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionPrefersTheCloserLaterLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionPrefersTheCloserLaterLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionPrefersTheCloserLaterLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionPrefersTheCloserLaterLineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionCoversDistancesAndFallbacksFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionCoversDistancesAndFallbacksFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionCoversDistancesAndFallbacksFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionCoversDistancesAndFallbacksFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionCoversDistancesAndFallbacksFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionCoversDistancesAndFallbacksFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionCoversDistancesAndFallbacksFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionCoversDistancesAndFallbacksFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionCoversDistancesAndFallbacksFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionCoversDistancesAndFallbacksFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionCoversDistancesAndFallbacksFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionCoversDistancesAndFallbacksFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionCoversDistancesAndFallbacksFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionCoversDistancesAndFallbacksFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionCoversDistancesAndFallbacksFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestSelectionWordBoundaryForPositionCoversDistancesAndFallbacksFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestSelectionWordBoundaryExpandsWordsAndHonoursInlineObjectsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestSelectionWordBoundaryExpandsWordsAndHonoursInlineObjectsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionWordBoundaryExpandsWordsAndHonoursInlineObjectsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionWordBoundaryExpandsWordsAndHonoursInlineObjectsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSelectionWordBoundaryExpandsWordsAndHonoursInlineObjectsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionWordBoundaryExpandsWordsAndHonoursInlineObjectsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionWordBoundaryExpandsWordsAndHonoursInlineObjectsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSelectionWordBoundaryExpandsWordsAndHonoursInlineObjectsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionWordBoundaryExpandsWordsAndHonoursInlineObjectsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionWordBoundaryExpandsWordsAndHonoursInlineObjectsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestSelectionWordBoundaryExpandsWordsAndHonoursInlineObjectsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestSelectionWordBoundaryExpandsWordsAndHonoursInlineObjectsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestSelectionWordBoundaryExpandsWordsAndHonoursInlineObjectsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestSelectionWordBoundaryExpandsWordsAndHonoursInlineObjectsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestSelectionWordBoundaryExpandsWordsAndHonoursInlineObjectsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestSelectionWordBoundaryExpandsWordsAndHonoursInlineObjectsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    LayoutQueriesGetSelectionOffsetForPositionFaultFault(crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault),
}

impl From<LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault> for crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault {
    fn from(value: LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault> for LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault {
    fn from(value: crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault) -> Self {
        LayoutQueriesResidualCoverageTestSelectionSnapPrefersTheCloserInlineObjectBoundaryFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestSameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegmentFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestSameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegmentFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestSameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegmentFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegmentFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegmentFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestSameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegmentFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegmentFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestSameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegmentFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestSameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegmentFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestSameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegmentFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestSameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegmentFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestSameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegmentFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestSameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegmentFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestSameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegmentFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestSameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegmentFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestSameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegmentFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestRubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnoredFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestRubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnoredFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestRubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnoredFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestRubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnoredFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestRubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnoredFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestRubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnoredFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestRubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnoredFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestRubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnoredFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestRubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnoredFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestRubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnoredFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestRubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnoredFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestRubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnoredFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestRubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnoredFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestRubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnoredFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestRubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnoredFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestRubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnoredFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestRubyGeometryRedistributesSelectionBoxesAndDropsSourceStopsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestRubyGeometryRedistributesSelectionBoxesAndDropsSourceStopsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestRubyGeometryRedistributesSelectionBoxesAndDropsSourceStopsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestRubyGeometryRedistributesSelectionBoxesAndDropsSourceStopsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestRubyGeometryRedistributesSelectionBoxesAndDropsSourceStopsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestRubyGeometryRedistributesSelectionBoxesAndDropsSourceStopsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestRubyGeometryRedistributesSelectionBoxesAndDropsSourceStopsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestRubyGeometryRedistributesSelectionBoxesAndDropsSourceStopsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestRubyGeometryRedistributesSelectionBoxesAndDropsSourceStopsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestRubyGeometryRedistributesSelectionBoxesAndDropsSourceStopsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestRubyGeometryRedistributesSelectionBoxesAndDropsSourceStopsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestRubyGeometryRedistributesSelectionBoxesAndDropsSourceStopsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestRubyGeometryRedistributesSelectionBoxesAndDropsSourceStopsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestRubyGeometryRedistributesSelectionBoxesAndDropsSourceStopsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestRubyGeometryRedistributesSelectionBoxesAndDropsSourceStopsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestRubyGeometryRedistributesSelectionBoxesAndDropsSourceStopsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestRubiesOnOtherLinesDoNotAffectThisLineGeometryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestRubiesOnOtherLinesDoNotAffectThisLineGeometryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestRubiesOnOtherLinesDoNotAffectThisLineGeometryFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestRubiesOnOtherLinesDoNotAffectThisLineGeometryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestRubiesOnOtherLinesDoNotAffectThisLineGeometryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestRubiesOnOtherLinesDoNotAffectThisLineGeometryFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestRubiesOnOtherLinesDoNotAffectThisLineGeometryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestRubiesOnOtherLinesDoNotAffectThisLineGeometryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestRubiesOnOtherLinesDoNotAffectThisLineGeometryFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestRubiesOnOtherLinesDoNotAffectThisLineGeometryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestRubiesOnOtherLinesDoNotAffectThisLineGeometryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestRubiesOnOtherLinesDoNotAffectThisLineGeometryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestRubiesOnOtherLinesDoNotAffectThisLineGeometryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestRubiesOnOtherLinesDoNotAffectThisLineGeometryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestRubiesOnOtherLinesDoNotAffectThisLineGeometryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestRubiesOnOtherLinesDoNotAffectThisLineGeometryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestRichTextSegmentsSplitOnLineBreaksAndClusterGapsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestRichTextSegmentsSplitOnLineBreaksAndClusterGapsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestRichTextSegmentsSplitOnLineBreaksAndClusterGapsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestRichTextSegmentsSplitOnLineBreaksAndClusterGapsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestRichTextSegmentsSplitOnLineBreaksAndClusterGapsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestRichTextSegmentsSplitOnLineBreaksAndClusterGapsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestRichTextSegmentsSplitOnLineBreaksAndClusterGapsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestRichTextSegmentsSplitOnLineBreaksAndClusterGapsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestRichTextSegmentsSplitOnLineBreaksAndClusterGapsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestRichTextSegmentsSplitOnLineBreaksAndClusterGapsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestRichTextSegmentsSplitOnLineBreaksAndClusterGapsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestRichTextSegmentsSplitOnLineBreaksAndClusterGapsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestRichTextSegmentsSplitOnLineBreaksAndClusterGapsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestRichTextSegmentsSplitOnLineBreaksAndClusterGapsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestRichTextSegmentsSplitOnLineBreaksAndClusterGapsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestRichTextSegmentsSplitOnLineBreaksAndClusterGapsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestRichTextSegmentsSkipZeroLengthClustersBetweenSlicesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestRichTextSegmentsSkipZeroLengthClustersBetweenSlicesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestRichTextSegmentsSkipZeroLengthClustersBetweenSlicesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestRichTextSegmentsSkipZeroLengthClustersBetweenSlicesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestRichTextSegmentsSkipZeroLengthClustersBetweenSlicesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestRichTextSegmentsSkipZeroLengthClustersBetweenSlicesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestRichTextSegmentsSkipZeroLengthClustersBetweenSlicesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestRichTextSegmentsSkipZeroLengthClustersBetweenSlicesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestRichTextSegmentsSkipZeroLengthClustersBetweenSlicesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestRichTextSegmentsSkipZeroLengthClustersBetweenSlicesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestRichTextSegmentsSkipZeroLengthClustersBetweenSlicesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestRichTextSegmentsSkipZeroLengthClustersBetweenSlicesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestRichTextSegmentsSkipZeroLengthClustersBetweenSlicesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestRichTextSegmentsSkipZeroLengthClustersBetweenSlicesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestRichTextSegmentsSkipZeroLengthClustersBetweenSlicesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestRichTextSegmentsSkipZeroLengthClustersBetweenSlicesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestResolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuationsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
}

impl From<LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        LayoutQueriesResidualCoverageTestPositionedClustersByLineRejectsForeignLinesFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestPositionedClustersAndSegmentsReturnEmptyWithoutLinesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestPositionedClustersAndSegmentsReturnEmptyWithoutLinesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestPositionedClustersAndSegmentsReturnEmptyWithoutLinesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestPositionedClustersAndSegmentsReturnEmptyWithoutLinesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestPositionedClustersAndSegmentsReturnEmptyWithoutLinesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestPositionedClustersAndSegmentsReturnEmptyWithoutLinesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestPositionedClustersAndSegmentsReturnEmptyWithoutLinesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestPositionedClustersAndSegmentsReturnEmptyWithoutLinesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestPositionedClustersAndSegmentsReturnEmptyWithoutLinesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestPositionedClustersAndSegmentsReturnEmptyWithoutLinesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestPositionedClustersAndSegmentsReturnEmptyWithoutLinesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestPositionedClustersAndSegmentsReturnEmptyWithoutLinesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestPositionedClustersAndSegmentsReturnEmptyWithoutLinesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestPositionedClustersAndSegmentsReturnEmptyWithoutLinesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestPositionedClustersAndSegmentsReturnEmptyWithoutLinesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestPositionedClustersAndSegmentsReturnEmptyWithoutLinesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestPlaneFourCodepointAboveTheHanBandsIsItsOwnUnitFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestPlaneFourCodepointAboveTheHanBandsIsItsOwnUnitFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestPlaneFourCodepointAboveTheHanBandsIsItsOwnUnitFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestPlaneFourCodepointAboveTheHanBandsIsItsOwnUnitFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestPlaneFourCodepointAboveTheHanBandsIsItsOwnUnitFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestPlaneFourCodepointAboveTheHanBandsIsItsOwnUnitFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestPlaneFourCodepointAboveTheHanBandsIsItsOwnUnitFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestPlaneFourCodepointAboveTheHanBandsIsItsOwnUnitFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestPlaneFourCodepointAboveTheHanBandsIsItsOwnUnitFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestPlaneFourCodepointAboveTheHanBandsIsItsOwnUnitFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestPlaneFourCodepointAboveTheHanBandsIsItsOwnUnitFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestPlaneFourCodepointAboveTheHanBandsIsItsOwnUnitFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestPlaneFourCodepointAboveTheHanBandsIsItsOwnUnitFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestPlaneFourCodepointAboveTheHanBandsIsItsOwnUnitFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestPlaneFourCodepointAboveTheHanBandsIsItsOwnUnitFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestPlaneFourCodepointAboveTheHanBandsIsItsOwnUnitFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    LayoutQueriesGetSelectionOffsetForPositionFaultFault(crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault),
}

impl From<LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault> for crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault {
    fn from(value: LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault> for LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault {
    fn from(value: crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault) -> Self {
        LayoutQueriesResidualCoverageTestOffsetForPositionCoversVerticalDistancesAndNaNPointsFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestNoArgPositionedClustersWalksEveryLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestNoArgPositionedClustersWalksEveryLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestNoArgPositionedClustersWalksEveryLineFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestNoArgPositionedClustersWalksEveryLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestNoArgPositionedClustersWalksEveryLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestNoArgPositionedClustersWalksEveryLineFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestNoArgPositionedClustersWalksEveryLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestNoArgPositionedClustersWalksEveryLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestNoArgPositionedClustersWalksEveryLineFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestNoArgPositionedClustersWalksEveryLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestNoArgPositionedClustersWalksEveryLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestNoArgPositionedClustersWalksEveryLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestNoArgPositionedClustersWalksEveryLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestNoArgPositionedClustersWalksEveryLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestNoArgPositionedClustersWalksEveryLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestNoArgPositionedClustersWalksEveryLineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestNearestLineSearchUpdatesToAStrictlyCloserLaterLineFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestNearestLineSearchCoversBothLambdaCopiesOfEachArmFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestNearestLineSearchCoversAllThreeDistanceArmsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestNearestLineFallsBackToTheOnlyLineAtItsEndOffsetFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestNearestLineFallsBackToTheOnlyLineAtItsEndOffsetFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestNearestLineFallsBackToTheOnlyLineAtItsEndOffsetFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestNearestLineFallsBackToTheOnlyLineAtItsEndOffsetFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestNearestLineFallsBackToTheOnlyLineAtItsEndOffsetFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestNearestLineFallsBackToTheOnlyLineAtItsEndOffsetFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestNearestLineFallsBackToTheOnlyLineAtItsEndOffsetFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestNearestLineFallsBackToTheOnlyLineAtItsEndOffsetFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestNearestLineFallsBackToTheOnlyLineAtItsEndOffsetFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestNearestLineFallsBackToTheOnlyLineAtItsEndOffsetFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestNearestLineFallsBackToTheOnlyLineAtItsEndOffsetFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestNearestLineFallsBackToTheOnlyLineAtItsEndOffsetFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestNearestLineFallsBackToTheOnlyLineAtItsEndOffsetFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestNearestLineFallsBackToTheOnlyLineAtItsEndOffsetFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestNearestLineFallsBackToTheOnlyLineAtItsEndOffsetFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestNearestLineFallsBackToTheOnlyLineAtItsEndOffsetFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestMetricDecisionsMustFullyContainTheClusterFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestMetricDecisionsMustFullyContainTheClusterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestMetricDecisionsMustFullyContainTheClusterFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestMetricDecisionsMustFullyContainTheClusterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestMetricDecisionsMustFullyContainTheClusterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestMetricDecisionsMustFullyContainTheClusterFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestMetricDecisionsMustFullyContainTheClusterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestMetricDecisionsMustFullyContainTheClusterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestMetricDecisionsMustFullyContainTheClusterFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestMetricDecisionsMustFullyContainTheClusterFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestMetricDecisionsMustFullyContainTheClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestMetricDecisionsMustFullyContainTheClusterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestMetricDecisionsMustFullyContainTheClusterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestMetricDecisionsMustFullyContainTheClusterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestMetricDecisionsMustFullyContainTheClusterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestMetricDecisionsMustFullyContainTheClusterFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestMarkedFacesUseMetricDecisionsWhenTheyCoverTheClusterFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestMarkedFacesUseMetricDecisionsWhenTheyCoverTheClusterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestMarkedFacesUseMetricDecisionsWhenTheyCoverTheClusterFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestMarkedFacesUseMetricDecisionsWhenTheyCoverTheClusterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestMarkedFacesUseMetricDecisionsWhenTheyCoverTheClusterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestMarkedFacesUseMetricDecisionsWhenTheyCoverTheClusterFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestMarkedFacesUseMetricDecisionsWhenTheyCoverTheClusterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestMarkedFacesUseMetricDecisionsWhenTheyCoverTheClusterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestMarkedFacesUseMetricDecisionsWhenTheyCoverTheClusterFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestMarkedFacesUseMetricDecisionsWhenTheyCoverTheClusterFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestMarkedFacesUseMetricDecisionsWhenTheyCoverTheClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestMarkedFacesUseMetricDecisionsWhenTheyCoverTheClusterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestMarkedFacesUseMetricDecisionsWhenTheyCoverTheClusterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestMarkedFacesUseMetricDecisionsWhenTheyCoverTheClusterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestMarkedFacesUseMetricDecisionsWhenTheyCoverTheClusterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestMarkedFacesUseMetricDecisionsWhenTheyCoverTheClusterFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestLineForOffsetInsideARangeTakesTheZeroDistanceArmFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestLineForOffsetInsideARangeTakesTheZeroDistanceArmFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestLineForOffsetInsideARangeTakesTheZeroDistanceArmFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestLineForOffsetInsideARangeTakesTheZeroDistanceArmFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestLineForOffsetInsideARangeTakesTheZeroDistanceArmFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestLineForOffsetInsideARangeTakesTheZeroDistanceArmFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestLineForOffsetInsideARangeTakesTheZeroDistanceArmFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestLineForOffsetInsideARangeTakesTheZeroDistanceArmFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestLineForOffsetInsideARangeTakesTheZeroDistanceArmFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestLineForOffsetInsideARangeTakesTheZeroDistanceArmFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestLineForOffsetInsideARangeTakesTheZeroDistanceArmFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestLineForOffsetInsideARangeTakesTheZeroDistanceArmFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestLineForOffsetInsideARangeTakesTheZeroDistanceArmFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestLineForOffsetInsideARangeTakesTheZeroDistanceArmFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestLineForOffsetInsideARangeTakesTheZeroDistanceArmFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestLineForOffsetInsideARangeTakesTheZeroDistanceArmFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnusableGlyphsAndReportsNullFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnusableGlyphsAndReportsNullFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnusableGlyphsAndReportsNullFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnusableGlyphsAndReportsNullFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnusableGlyphsAndReportsNullFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnusableGlyphsAndReportsNullFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnusableGlyphsAndReportsNullFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnusableGlyphsAndReportsNullFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnusableGlyphsAndReportsNullFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnusableGlyphsAndReportsNullFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnusableGlyphsAndReportsNullFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnusableGlyphsAndReportsNullFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnusableGlyphsAndReportsNullFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnusableGlyphsAndReportsNullFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnusableGlyphsAndReportsNullFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnusableGlyphsAndReportsNullFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInkFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInkFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInkFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInkFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInkFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInkFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInkFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInkFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInkFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInkFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInkFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInkFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInkFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInkFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInkFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestGlyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInkFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestGlyphInkBoundsRejectsEachNonFiniteEdgeIndependentlyFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestGlyphInkBoundsRejectsEachNonFiniteEdgeIndependentlyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestGlyphInkBoundsRejectsEachNonFiniteEdgeIndependentlyFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestGlyphInkBoundsRejectsEachNonFiniteEdgeIndependentlyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestGlyphInkBoundsRejectsEachNonFiniteEdgeIndependentlyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestGlyphInkBoundsRejectsEachNonFiniteEdgeIndependentlyFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestGlyphInkBoundsRejectsEachNonFiniteEdgeIndependentlyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestGlyphInkBoundsRejectsEachNonFiniteEdgeIndependentlyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestGlyphInkBoundsRejectsEachNonFiniteEdgeIndependentlyFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestGlyphInkBoundsRejectsEachNonFiniteEdgeIndependentlyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestGlyphInkBoundsRejectsEachNonFiniteEdgeIndependentlyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestGlyphInkBoundsRejectsEachNonFiniteEdgeIndependentlyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestGlyphInkBoundsRejectsEachNonFiniteEdgeIndependentlyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestGlyphInkBoundsRejectsEachNonFiniteEdgeIndependentlyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestGlyphInkBoundsRejectsEachNonFiniteEdgeIndependentlyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestGlyphInkBoundsRejectsEachNonFiniteEdgeIndependentlyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestGlueTrimSkipsInteriorSegmentEdgesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestGlueTrimSkipsInteriorSegmentEdgesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestGlueTrimSkipsInteriorSegmentEdgesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestGlueTrimSkipsInteriorSegmentEdgesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestGlueTrimSkipsInteriorSegmentEdgesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestGlueTrimSkipsInteriorSegmentEdgesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestGlueTrimSkipsInteriorSegmentEdgesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestGlueTrimSkipsInteriorSegmentEdgesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestGlueTrimSkipsInteriorSegmentEdgesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestGlueTrimSkipsInteriorSegmentEdgesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestGlueTrimSkipsInteriorSegmentEdgesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestGlueTrimSkipsInteriorSegmentEdgesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestGlueTrimSkipsInteriorSegmentEdgesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestGlueTrimSkipsInteriorSegmentEdgesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestGlueTrimSkipsInteriorSegmentEdgesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestGlueTrimSkipsInteriorSegmentEdgesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestEmptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRectsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    LayoutQueriesGetSelectionOffsetForPositionFaultFault(crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault),
}

impl From<LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault> for crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault {
    fn from(value: LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault> for LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault {
    fn from(value: crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault) -> Self {
        LayoutQueriesResidualCoverageTestEmptyLineResultsShortCircuitEveryQueryFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestDecorationStyleResolvesInsideSpansAndAtTheirEdgesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestDecorationStyleResolvesInsideSpansAndAtTheirEdgesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestDecorationStyleResolvesInsideSpansAndAtTheirEdgesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestDecorationStyleResolvesInsideSpansAndAtTheirEdgesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestDecorationStyleResolvesInsideSpansAndAtTheirEdgesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestDecorationStyleResolvesInsideSpansAndAtTheirEdgesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestDecorationStyleResolvesInsideSpansAndAtTheirEdgesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestDecorationStyleResolvesInsideSpansAndAtTheirEdgesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestDecorationStyleResolvesInsideSpansAndAtTheirEdgesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestDecorationStyleResolvesInsideSpansAndAtTheirEdgesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestDecorationStyleResolvesInsideSpansAndAtTheirEdgesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestDecorationStyleResolvesInsideSpansAndAtTheirEdgesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestDecorationStyleResolvesInsideSpansAndAtTheirEdgesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestDecorationStyleResolvesInsideSpansAndAtTheirEdgesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestDecorationStyleResolvesInsideSpansAndAtTheirEdgesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestDecorationStyleResolvesInsideSpansAndAtTheirEdgesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestDecorationLineYWithoutSpansUsesTheParagraphStyleFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestDecorationLineYWithoutSpansUsesTheParagraphStyleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestDecorationLineYWithoutSpansUsesTheParagraphStyleFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestDecorationLineYWithoutSpansUsesTheParagraphStyleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestDecorationLineYWithoutSpansUsesTheParagraphStyleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestDecorationLineYWithoutSpansUsesTheParagraphStyleFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestDecorationLineYWithoutSpansUsesTheParagraphStyleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestDecorationLineYWithoutSpansUsesTheParagraphStyleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestDecorationLineYWithoutSpansUsesTheParagraphStyleFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestDecorationLineYWithoutSpansUsesTheParagraphStyleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestDecorationLineYWithoutSpansUsesTheParagraphStyleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestDecorationLineYWithoutSpansUsesTheParagraphStyleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestDecorationLineYWithoutSpansUsesTheParagraphStyleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestDecorationLineYWithoutSpansUsesTheParagraphStyleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestDecorationLineYWithoutSpansUsesTheParagraphStyleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestDecorationLineYWithoutSpansUsesTheParagraphStyleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestDecorationLineYRequiresValidStrokeAndDecorationRolesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestDecorationLineYPicksTheLastMatchingSpanFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestDecorationLineYPicksTheLastMatchingSpanFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestDecorationLineYPicksTheLastMatchingSpanFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestDecorationLineYPicksTheLastMatchingSpanFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestDecorationLineYPicksTheLastMatchingSpanFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestDecorationLineYPicksTheLastMatchingSpanFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestDecorationLineYPicksTheLastMatchingSpanFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestDecorationLineYPicksTheLastMatchingSpanFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestDecorationLineYPicksTheLastMatchingSpanFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestDecorationLineYPicksTheLastMatchingSpanFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestDecorationLineYPicksTheLastMatchingSpanFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestDecorationLineYPicksTheLastMatchingSpanFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestDecorationLineYPicksTheLastMatchingSpanFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestDecorationLineYPicksTheLastMatchingSpanFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestDecorationLineYPicksTheLastMatchingSpanFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestDecorationLineYPicksTheLastMatchingSpanFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestDecorationLineYKeepsTheEarlierSpanWhenALaterOneMissesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestDecorationLineYKeepsTheEarlierSpanWhenALaterOneMissesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestDecorationLineYKeepsTheEarlierSpanWhenALaterOneMissesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestDecorationLineYKeepsTheEarlierSpanWhenALaterOneMissesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestDecorationLineYKeepsTheEarlierSpanWhenALaterOneMissesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestDecorationLineYKeepsTheEarlierSpanWhenALaterOneMissesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestDecorationLineYKeepsTheEarlierSpanWhenALaterOneMissesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestDecorationLineYKeepsTheEarlierSpanWhenALaterOneMissesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestDecorationLineYKeepsTheEarlierSpanWhenALaterOneMissesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestDecorationLineYKeepsTheEarlierSpanWhenALaterOneMissesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestDecorationLineYKeepsTheEarlierSpanWhenALaterOneMissesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestDecorationLineYKeepsTheEarlierSpanWhenALaterOneMissesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestDecorationLineYKeepsTheEarlierSpanWhenALaterOneMissesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestDecorationLineYKeepsTheEarlierSpanWhenALaterOneMissesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestDecorationLineYKeepsTheEarlierSpanWhenALaterOneMissesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestDecorationLineYKeepsTheEarlierSpanWhenALaterOneMissesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    TracedAssertionsAssertFailsWithNoSuchElementFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithNoSuchElementFault),
}

impl From<LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithNoSuchElementFault {
    fn from(value: LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault::TracedAssertionsAssertFailsWithNoSuchElementFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithNoSuchElementFault> for LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithNoSuchElementFault) -> Self {
        LayoutQueriesResidualCoverageTestCursorRectFindsLaterClustersAndRejectsGappedRangesFault::TracedAssertionsAssertFailsWithNoSuchElementFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestCursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClustersFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestCopyProjectionAppendsFullySelectedAnnotationsOnlyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestCopyProjectionAppendsFullySelectedAnnotationsOnlyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestCopyProjectionAppendsFullySelectedAnnotationsOnlyFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestCopyProjectionAppendsFullySelectedAnnotationsOnlyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestCopyProjectionAppendsFullySelectedAnnotationsOnlyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestCopyProjectionAppendsFullySelectedAnnotationsOnlyFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestCopyProjectionAppendsFullySelectedAnnotationsOnlyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestCopyProjectionAppendsFullySelectedAnnotationsOnlyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestCopyProjectionAppendsFullySelectedAnnotationsOnlyFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestCopyProjectionAppendsFullySelectedAnnotationsOnlyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestCopyProjectionAppendsFullySelectedAnnotationsOnlyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestCopyProjectionAppendsFullySelectedAnnotationsOnlyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestCopyProjectionAppendsFullySelectedAnnotationsOnlyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestCopyProjectionAppendsFullySelectedAnnotationsOnlyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestCopyProjectionAppendsFullySelectedAnnotationsOnlyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestCopyProjectionAppendsFullySelectedAnnotationsOnlyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestCompatibilityIdeographsFormIndividualWordUnitsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestCompatibilityIdeographsFormIndividualWordUnitsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestCompatibilityIdeographsFormIndividualWordUnitsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestCompatibilityIdeographsFormIndividualWordUnitsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestCompatibilityIdeographsFormIndividualWordUnitsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestCompatibilityIdeographsFormIndividualWordUnitsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestCompatibilityIdeographsFormIndividualWordUnitsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestCompatibilityIdeographsFormIndividualWordUnitsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestCompatibilityIdeographsFormIndividualWordUnitsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestCompatibilityIdeographsFormIndividualWordUnitsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestCompatibilityIdeographsFormIndividualWordUnitsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestCompatibilityIdeographsFormIndividualWordUnitsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestCompatibilityIdeographsFormIndividualWordUnitsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestCompatibilityIdeographsFormIndividualWordUnitsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestCompatibilityIdeographsFormIndividualWordUnitsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestCompatibilityIdeographsFormIndividualWordUnitsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestCoerceSelectionOffsetHonoursInlineObjectBoundariesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestCoerceSelectionOffsetHonoursInlineObjectBoundariesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestCoerceSelectionOffsetHonoursInlineObjectBoundariesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestCoerceSelectionOffsetHonoursInlineObjectBoundariesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestCoerceSelectionOffsetHonoursInlineObjectBoundariesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestCoerceSelectionOffsetHonoursInlineObjectBoundariesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestCoerceSelectionOffsetHonoursInlineObjectBoundariesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestCoerceSelectionOffsetHonoursInlineObjectBoundariesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestCoerceSelectionOffsetHonoursInlineObjectBoundariesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestCoerceSelectionOffsetHonoursInlineObjectBoundariesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestCoerceSelectionOffsetHonoursInlineObjectBoundariesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestCoerceSelectionOffsetHonoursInlineObjectBoundariesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestCoerceSelectionOffsetHonoursInlineObjectBoundariesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestCoerceSelectionOffsetHonoursInlineObjectBoundariesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestCoerceSelectionOffsetHonoursInlineObjectBoundariesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestCoerceSelectionOffsetHonoursInlineObjectBoundariesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestClearanceTakesTheSmallerSideWhicheverSegmentOwnsItFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestClearanceTakesTheSmallerSideWhicheverSegmentOwnsItFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestClearanceTakesTheSmallerSideWhicheverSegmentOwnsItFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestClearanceTakesTheSmallerSideWhicheverSegmentOwnsItFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestClearanceTakesTheSmallerSideWhicheverSegmentOwnsItFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestClearanceTakesTheSmallerSideWhicheverSegmentOwnsItFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestClearanceTakesTheSmallerSideWhicheverSegmentOwnsItFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestClearanceTakesTheSmallerSideWhicheverSegmentOwnsItFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestClearanceTakesTheSmallerSideWhicheverSegmentOwnsItFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestClearanceTakesTheSmallerSideWhicheverSegmentOwnsItFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestClearanceTakesTheSmallerSideWhicheverSegmentOwnsItFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestClearanceTakesTheSmallerSideWhicheverSegmentOwnsItFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestClearanceTakesTheSmallerSideWhicheverSegmentOwnsItFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestClearanceTakesTheSmallerSideWhicheverSegmentOwnsItFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestClearanceTakesTheSmallerSideWhicheverSegmentOwnsItFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestClearanceTakesTheSmallerSideWhicheverSegmentOwnsItFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestClearanceNeedsSameRoleAndUsesTheSmallerSideFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestClearanceNeedsSameRoleAndUsesTheSmallerSideFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestClearanceNeedsSameRoleAndUsesTheSmallerSideFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestClearanceNeedsSameRoleAndUsesTheSmallerSideFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestClearanceNeedsSameRoleAndUsesTheSmallerSideFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestClearanceNeedsSameRoleAndUsesTheSmallerSideFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestClearanceNeedsSameRoleAndUsesTheSmallerSideFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestClearanceNeedsSameRoleAndUsesTheSmallerSideFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestClearanceNeedsSameRoleAndUsesTheSmallerSideFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestClearanceNeedsSameRoleAndUsesTheSmallerSideFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestClearanceNeedsSameRoleAndUsesTheSmallerSideFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestClearanceNeedsSameRoleAndUsesTheSmallerSideFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestClearanceNeedsSameRoleAndUsesTheSmallerSideFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestClearanceNeedsSameRoleAndUsesTheSmallerSideFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestClearanceNeedsSameRoleAndUsesTheSmallerSideFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestClearanceNeedsSameRoleAndUsesTheSmallerSideFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestBoundingBoxesSliceZeroWidthAndEmptyClustersFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestBoundingBoxesSliceZeroWidthAndEmptyClustersFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestBoundingBoxesSliceZeroWidthAndEmptyClustersFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBoundingBoxesSliceZeroWidthAndEmptyClustersFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestBoundingBoxesSliceZeroWidthAndEmptyClustersFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestBoundingBoxesSliceZeroWidthAndEmptyClustersFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBoundingBoxesSliceZeroWidthAndEmptyClustersFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestBoundingBoxesSliceZeroWidthAndEmptyClustersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestBoundingBoxesSliceZeroWidthAndEmptyClustersFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBoundingBoxesSliceZeroWidthAndEmptyClustersFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestBoundingBoxesSliceZeroWidthAndEmptyClustersFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestBoundingBoxesSliceZeroWidthAndEmptyClustersFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestBoundingBoxesSliceZeroWidthAndEmptyClustersFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestBoundingBoxesSliceZeroWidthAndEmptyClustersFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestBoundingBoxesSliceZeroWidthAndEmptyClustersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestBoundingBoxesSliceZeroWidthAndEmptyClustersFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestBoundingBoxFallsBackToTheCursorRectAtClusterGapsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeUsesGlyphAdvancesWhenAvailableFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeUsesGlyphAdvancesWhenAvailableFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeUsesGlyphAdvancesWhenAvailableFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeUsesGlyphAdvancesWhenAvailableFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeUsesGlyphAdvancesWhenAvailableFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeUsesGlyphAdvancesWhenAvailableFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeUsesGlyphAdvancesWhenAvailableFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeUsesGlyphAdvancesWhenAvailableFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeUsesGlyphAdvancesWhenAvailableFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeUsesGlyphAdvancesWhenAvailableFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeUsesGlyphAdvancesWhenAvailableFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeUsesGlyphAdvancesWhenAvailableFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeUsesGlyphAdvancesWhenAvailableFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeUsesGlyphAdvancesWhenAvailableFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeUsesGlyphAdvancesWhenAvailableFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeUsesGlyphAdvancesWhenAvailableFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestBackgroundTrailingEdgePicksTheLargestGlyphAdvanceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestBackgroundTrailingEdgePicksTheLargestGlyphAdvanceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestBackgroundTrailingEdgePicksTheLargestGlyphAdvanceFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBackgroundTrailingEdgePicksTheLargestGlyphAdvanceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestBackgroundTrailingEdgePicksTheLargestGlyphAdvanceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestBackgroundTrailingEdgePicksTheLargestGlyphAdvanceFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBackgroundTrailingEdgePicksTheLargestGlyphAdvanceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestBackgroundTrailingEdgePicksTheLargestGlyphAdvanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestBackgroundTrailingEdgePicksTheLargestGlyphAdvanceFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBackgroundTrailingEdgePicksTheLargestGlyphAdvanceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestBackgroundTrailingEdgePicksTheLargestGlyphAdvanceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestBackgroundTrailingEdgePicksTheLargestGlyphAdvanceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestBackgroundTrailingEdgePicksTheLargestGlyphAdvanceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestBackgroundTrailingEdgePicksTheLargestGlyphAdvanceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestBackgroundTrailingEdgePicksTheLargestGlyphAdvanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestBackgroundTrailingEdgePicksTheLargestGlyphAdvanceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargestFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargestFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargestFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargestFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargestFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargestFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargestFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargestFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargestFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargestFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargestFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargestFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargestFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargestFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargestFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestBackgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargestFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestBackgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvancesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestBackgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvancesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestBackgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvancesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBackgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvancesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestBackgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvancesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestBackgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvancesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBackgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvancesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestBackgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvancesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestBackgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvancesFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBackgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvancesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestBackgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvancesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestBackgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvancesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestBackgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvancesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestBackgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvancesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestBackgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvancesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestBackgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvancesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestBackgroundSegmentsPassThroughUnmatchableSegmentsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestBackgroundSegmentsPassThroughUnmatchableSegmentsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestBackgroundSegmentsPassThroughUnmatchableSegmentsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBackgroundSegmentsPassThroughUnmatchableSegmentsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestBackgroundSegmentsPassThroughUnmatchableSegmentsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestBackgroundSegmentsPassThroughUnmatchableSegmentsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBackgroundSegmentsPassThroughUnmatchableSegmentsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestBackgroundSegmentsPassThroughUnmatchableSegmentsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestBackgroundSegmentsPassThroughUnmatchableSegmentsFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBackgroundSegmentsPassThroughUnmatchableSegmentsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestBackgroundSegmentsPassThroughUnmatchableSegmentsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestBackgroundSegmentsPassThroughUnmatchableSegmentsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestBackgroundSegmentsPassThroughUnmatchableSegmentsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestBackgroundSegmentsPassThroughUnmatchableSegmentsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestBackgroundSegmentsPassThroughUnmatchableSegmentsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestBackgroundSegmentsPassThroughUnmatchableSegmentsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestBackgroundSegmentOutsideEverySpanUsesTheParagraphStyleFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestBackgroundSegmentOutsideEverySpanUsesTheParagraphStyleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestBackgroundSegmentOutsideEverySpanUsesTheParagraphStyleFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBackgroundSegmentOutsideEverySpanUsesTheParagraphStyleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestBackgroundSegmentOutsideEverySpanUsesTheParagraphStyleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestBackgroundSegmentOutsideEverySpanUsesTheParagraphStyleFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBackgroundSegmentOutsideEverySpanUsesTheParagraphStyleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestBackgroundSegmentOutsideEverySpanUsesTheParagraphStyleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestBackgroundSegmentOutsideEverySpanUsesTheParagraphStyleFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestBackgroundSegmentOutsideEverySpanUsesTheParagraphStyleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestBackgroundSegmentOutsideEverySpanUsesTheParagraphStyleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestBackgroundSegmentOutsideEverySpanUsesTheParagraphStyleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestBackgroundSegmentOutsideEverySpanUsesTheParagraphStyleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestBackgroundSegmentOutsideEverySpanUsesTheParagraphStyleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestBackgroundSegmentOutsideEverySpanUsesTheParagraphStyleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestBackgroundSegmentOutsideEverySpanUsesTheParagraphStyleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesResidualCoverageTestAdjacentSameStyleSegmentsShareClearanceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LayoutQueriesResidualCoverageTestAdjacentSameStyleSegmentsShareClearanceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesResidualCoverageTestAdjacentSameStyleSegmentsShareClearanceFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestAdjacentSameStyleSegmentsShareClearanceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestAdjacentSameStyleSegmentsShareClearanceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesResidualCoverageTestAdjacentSameStyleSegmentsShareClearanceFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestAdjacentSameStyleSegmentsShareClearanceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesResidualCoverageTestAdjacentSameStyleSegmentsShareClearanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesResidualCoverageTestAdjacentSameStyleSegmentsShareClearanceFault) -> Self {
        match value {
            LayoutQueriesResidualCoverageTestAdjacentSameStyleSegmentsShareClearanceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesResidualCoverageTestAdjacentSameStyleSegmentsShareClearanceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesResidualCoverageTestAdjacentSameStyleSegmentsShareClearanceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesResidualCoverageTestAdjacentSameStyleSegmentsShareClearanceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesResidualCoverageTestAdjacentSameStyleSegmentsShareClearanceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesResidualCoverageTestAdjacentSameStyleSegmentsShareClearanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesResidualCoverageTestAdjacentSameStyleSegmentsShareClearanceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn corner_radii_predicates_cover_every_comparison() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.cornerRadiiPredicatesCoverEveryComparison", "org.tiqian.core.LayoutQueriesResidualCoverageTest.cornerRadiiPredicatesCoverEveryComparison", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"cornerRadiiPredicatesCoverEveryComparison");
        let _ = TracedAssertions::traced_assertions_assert_true(RichTextCornerRadii::new(0.0f64, 0.0f64, 0.0f64, 0.0f64).get_is_square(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!RichTextCornerRadii::new(1.0f64, 0.0f64, 0.0f64, 0.0f64).get_is_square(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!RichTextCornerRadii::new(0.0f64, 1.0f64, 0.0f64, 0.0f64).get_is_square(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!RichTextCornerRadii::new(0.0f64, 0.0f64, 1.0f64, 0.0f64).get_is_square(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!RichTextCornerRadii::new(0.0f64, 0.0f64, 0.0f64, 1.0f64).get_is_square(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(RichTextCornerRadii::new(2.0f64, 2.0f64, 2.0f64, 2.0f64).get_is_uniform(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!RichTextCornerRadii::new(1.0f64, 2.0f64, 2.0f64, 2.0f64).get_is_uniform(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!RichTextCornerRadii::new(2.0f64, 1.0f64, 2.0f64, 2.0f64).get_is_uniform(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!RichTextCornerRadii::new(2.0f64, 2.0f64, 1.0f64, 2.0f64).get_is_uniform(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(!RichTextCornerRadii::new(2.0f64, 2.0f64, 2.0f64, 1.0f64).get_is_uniform(), None).unwrap();
    });
}

#[test]
fn resolved_corner_radii_rejects_invalid_insets_and_resolves_continuations() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.resolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuations", "org.tiqian.core.LayoutQueriesResidualCoverageTest.resolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuations", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"resolvedCornerRadiiRejectsInvalidInsetsAndResolvesContinuations");
        let continuing = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(1u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e|
e.into_inner()); __guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(6.0f64), Some(2.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0, TextRange::new(0u32, 3u32).unwrap(), 0.0f64, 0.0f64, 30.0f64, 10.0f64, 15.0f64);
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let continuing = (continuing).clone(); Arc::new(move || {
        LayoutQueries::layout_queries_resolved_background_corner_radii((continuing).clone(), -1.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let continuing = (continuing).clone(); Arc::new(move || {
        LayoutQueries::layout_queries_resolved_background_corner_radii((continuing).clone(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_nan()).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let resolved = LayoutQueries::layout_queries_resolved_background_corner_radii((continuing).clone(), 0.0f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, resolved.top_left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, resolved.top_right, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, resolved.bottom_right, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, resolved.bottom_left, None).unwrap();
    });
}

#[test]
fn copy_projection_appends_fully_selected_annotations_only() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.copyProjectionAppendsFullySelectedAnnotationsOnly", "org.tiqian.core.LayoutQueriesResidualCoverageTest.copyProjectionAppendsFullySelectedAnnotationsOnly", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"copyProjectionAppendsFullySelectedAnnotationsOnly");
        let debug = LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![
    (RubyDecisionInfo::new(TextRange::new(0u32, 2u32).unwrap(), "zhù", 0u32, 10.0f64, 12.0f64, 6.0f64, 0.0f64, Some(0.0f64), Some(0.0f64), Some(12.0f64), Some(vec![]), Some(400), Some("zh-Hans".to_string()), Some(vec![]))).clone(),
]), Some(vec![
    (BopomofoDecisionInfo::new(TextRange::new(2u32, 4u32).unwrap(), "ㄋㄧˇ", 0u32, vec![].to_vec(), Some(vec![]), Some(400), Some("zh-Hans".to_string()))).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]));
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abcd", &vec![], &vec![], &vec![], &vec![], &vec![], (debug).clone(),
LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"", LayoutQueries::layout_queries_get_text_for_copy((content).clone(), TextRange::new(1u32, 1u32).unwrap()).unwrap().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"ab（zhù）c", LayoutQueries::layout_queries_get_text_for_copy((content).clone(), TextRange::new(0u32, 3u32).unwrap()).unwrap().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"ab（zhù）cd（ㄋㄧˇ）", LayoutQueries::layout_queries_get_text_for_copy((content).clone(), TextRange::new(0u32, 4u32).unwrap()).unwrap().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"d", LayoutQueries::layout_queries_get_text_for_copy((content).clone(), TextRange::new(3u32, 4u32).unwrap()).unwrap().as_str(), None).unwrap();
    });
}

#[test]
fn positioned_clusters_by_line_rejects_foreign_lines() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.positionedClustersByLineRejectsForeignLines", "org.tiqian.core.LayoutQueriesResidualCoverageTest.positionedClustersByLineRejectsForeignLines", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"positionedClustersByLineRejectsForeignLines");
        let owned = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 10.0f64);
        let foreign = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 99.0f64, 119.0f64, 114.0f64, 0.0f64, 10.0f64);
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 2u32).unwrap(), &"ab", 20.0f64)).clone(),
], &vec![(owned).clone()], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((LayoutQueries::layout_queries_positioned_clusters_for_line((content).clone(), (owned).clone()).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let error = TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let content = (content).clone(); let foreign = (foreign).clone(); Arc::new(move || {
        LayoutQueries::layout_queries_positioned_clusters_for_line((content).clone(), (foreign).clone()).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&format!("{}", error), "must belong", 0)).to_ne_bytes())) <= 2147483647, Some((format!("{}", error)).to_string())).unwrap();
    });
}

#[test]
fn glyph_ink_bounds_skips_unmatched_glyphs_and_returns_null_without_ink() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.glyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInk", "org.tiqian.core.LayoutQueriesResidualCoverageTest.glyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInk", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"glyphInkBoundsSkipsUnmatchedGlyphsAndReturnsNullWithoutInk");
        let clusters = vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
];
        let runs = vec![
    (GlyphRun::new(TextRange::new(0u32, 2u32).unwrap(), "test", vec![
    (Glyph::new(1u32, TextRange::new(0u32, 1u32).unwrap(), 10.0f64, Some(0.0f64), Some(0.0f64), None, Some(Rect::new(0.0f64, 2.0f64, 8.0f64, 12.0f64)), None, None)).clone(),
    (Glyph::new(2u32, TextRange::new(0u32, 1u32).unwrap(), 10.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None)).clone(),
    (Glyph::new(3u32, TextRange::new(5u32, 6u32).unwrap(), 10.0f64, Some(0.0f64), Some(0.0f64), None, Some(Rect::new(0.0f64, 0.0f64, 1.0f64, 1.0f64)), None, None)).clone(),
].to_vec(), 20.0f64, Some(vec![]))).clone(),
];
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &clusters, &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &runs, &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(Rect::new(0.0f64, 17.0f64, 8.0f64, 27.0f64).to_string().as_str(), LayoutQueries::layout_queries_glyph_ink_bounds((content).clone()).as_ref().unwrap().to_string().as_str(), None).unwrap();
        let no_ink = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &clusters, &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let no_ink_bounds = LayoutQueries::layout_queries_glyph_ink_bounds((no_ink).clone());
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(no_ink_bounds.is_none(), match &(no_ink_bounds) { None => "-".to_string(), Some(__option2) => __option2.to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn empty_line_results_short_circuit_every_query() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.emptyLineResultsShortCircuitEveryQuery", "org.tiqian.core.LayoutQueriesResidualCoverageTest.emptyLineResultsShortCircuitEveryQuery", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"emptyLineResultsShortCircuitEveryQuery");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![], &vec![], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(),
LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4294967295u32, LayoutQueries::layout_queries_get_line_for_offset((content).clone(), 0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(Rect::new(0.0f64, 0.0f64, 0.0f64, 0.0f64).to_string().as_str(), LayoutQueries::layout_queries_get_bounding_box((content).clone(), 0).unwrap().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(Rect::new(0.0f64, 0.0f64, 0.0f64, 0.0f64).to_string().as_str(), LayoutQueries::layout_queries_get_cursor_rect((content).clone(), 0).unwrap().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_offset_for_position((content).clone(), 5.0f64, 5.0f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_selection_offset_for_position((content).clone(), 5.0f64, 5.0f64).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[]", LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_render_rects(&LayoutQueries::layout_queries_get_bounding_boxes((content).clone(), TextRange::new(0u32,
2u32).unwrap())).unwrap().as_str(), None).unwrap();
        let no_word_boundary = LayoutQueries::layout_queries_get_selection_word_boundary_for_position((content).clone(), 5.0f64, 5.0f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(no_word_boundary.is_none(), match &(no_word_boundary) { None => "-".to_string(), Some(__option5) => __option5.to_string() }.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[]", LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_render_segments(&LayoutQueries::layout_queries_positioned_rich_text_segments((content).clone(), &vec![
    (RichTextSpan::new(TextRange::new(0u32, 1u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), RichTextPaint::new(None,
Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap())).clone(),
]).unwrap()).unwrap().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[]", LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_render_segments(&LayoutQueries::layout_queries_trimmed_rich_text_decoration_segments((content).clone(),
&vec![])).unwrap().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[]", LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_render_segments(&LayoutQueries::layout_queries_rich_text_background_segments((content).clone(),
&vec![])).unwrap().as_str(), None).unwrap();
    });
}

#[test]
fn bounding_box_falls_back_to_the_cursor_rect_at_cluster_gaps() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.boundingBoxFallsBackToTheCursorRectAtClusterGaps", "org.tiqian.core.LayoutQueriesResidualCoverageTest.boundingBoxFallsBackToTheCursorRectAtClusterGaps", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"boundingBoxFallsBackToTheCursorRectAtClusterGaps");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abc", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"c", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 10.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(Rect::new(10.0f64, 0.0f64, 11.0f64, 20.0f64).to_string().as_str(), LayoutQueries::layout_queries_get_bounding_box((content).clone(), 1).unwrap().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(Rect::new(20.0f64, 0.0f64, 21.0f64, 20.0f64).to_string().as_str(), LayoutQueries::layout_queries_get_bounding_box((content).clone(), 3).unwrap().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[]", LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_render_rects(&LayoutQueries::layout_queries_get_bounding_boxes((content).clone(), TextRange::new(3u32,
5u32).unwrap())).unwrap().as_str(), None).unwrap();
    });
}

#[test]
fn rich_text_segments_split_on_line_breaks_and_cluster_gaps() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.richTextSegmentsSplitOnLineBreaksAndClusterGaps", "org.tiqian.core.LayoutQueriesResidualCoverageTest.richTextSegmentsSplitOnLineBreaksAndClusterGaps", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"richTextSegmentsSplitOnLineBreaksAndClusterGaps");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abcd", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"c", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(3u32, 4u32).unwrap(), &"d", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(3u32, 4u32).unwrap(), 2, 2, 20.0f64, 40.0f64, 35.0f64, 0.0f64, 10.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let span = RichTextSpan::new(TextRange::new(0u32, 4u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), RichTextPaint::new(None,
Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap());
        let split = LayoutQueries::layout_queries_positioned_rich_text_segments((content).clone(), &vec![(span).clone()]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((split.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 1u32).unwrap().to_string().as_str(), ((split[0usize]).clone().range).clone().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(2u32, 3u32).unwrap().to_string().as_str(), ((split[1usize]).clone().range).clone().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(3u32, 4u32).unwrap().to_string().as_str(), ((split[2usize]).clone().range).clone().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, split[0usize].line_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, split[1usize].line_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, split[2usize].line_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((LayoutQueries::layout_queries_positioned_rich_text_segments((content).clone(), &vec![
    (RichTextSpan::new(TextRange::new(5u32, 8u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), RichTextPaint::new(None,
Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap())).clone(),
]).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn rich_text_segments_skip_zero_length_clusters_between_slices() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.richTextSegmentsSkipZeroLengthClustersBetweenSlices", "org.tiqian.core.LayoutQueriesResidualCoverageTest.richTextSegmentsSkipZeroLengthClustersBetweenSlices", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"richTextSegmentsSkipZeroLengthClustersBetweenSlices");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 1u32).unwrap(), &"", 0.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 2, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let segments = LayoutQueries::layout_queries_positioned_rich_text_segments((content).clone(), &vec![
    (RichTextSpan::new(TextRange::new(0u32, 2u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), RichTextPaint::new(None,
Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap())).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((segments.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 2u32).unwrap().to_string().as_str(), ((segments[0usize]).clone().range).clone().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, segments[0usize].left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20.0f64, segments[0usize].right, None).unwrap();
    });
}

#[test]
fn trimmed_decoration_segments_keep_only_decoration_roles() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.trimmedDecorationSegmentsKeepOnlyDecorationRoles", "org.tiqian.core.LayoutQueriesResidualCoverageTest.trimmedDecorationSegmentsKeepOnlyDecorationRoles", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"trimmedDecorationSegmentsKeepOnlyDecorationRoles");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![], &vec![], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(),
LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let decoration = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e|
e.into_inner()); __guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0, TextRange::new(0u32, 2u32).unwrap(), 0.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64);
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_render_segments(&vec![(decoration).clone()]).unwrap().as_str(),
LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_render_segments(&LayoutQueries::layout_queries_trimmed_rich_text_decoration_segments((content).clone(), &vec![(decoration).clone()])).unwrap().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((LayoutQueries::layout_queries_trimmed_rich_text_decoration_segments((content).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_plain_segment(TextRange::new(0u32, 2u32).unwrap()).unwrap()).clone(),
]).len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn background_segments_pass_through_unmatchable_segments() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.backgroundSegmentsPassThroughUnmatchableSegments", "org.tiqian.core.LayoutQueriesResidualCoverageTest.backgroundSegmentsPassThroughUnmatchableSegments", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"backgroundSegmentsPassThroughUnmatchableSegments");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 10.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let far = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_plain_segment(TextRange::new(10u32, 12u32).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_render_segments(&vec![(far).clone()]).unwrap().as_str(),
LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_render_segments(&LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![(far).clone()])).unwrap().as_str(), None).unwrap();
        let orphan = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 1u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e|
e.into_inner()); __guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 5, TextRange::new(0u32, 1u32).unwrap(), 0.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64);
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_render_segments(&vec![(orphan).clone()]).unwrap().as_str(),
LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_render_segments(&LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![(orphan).clone()])).unwrap().as_str(), None).unwrap();
        let underline = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 1u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e|
e.into_inner()); __guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0, TextRange::new(0u32, 1u32).unwrap(), 0.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64);
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![(underline).clone()]).len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn background_segments_trim_glue_apply_padding_and_use_glyph_advances() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.backgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvances", "org.tiqian.core.LayoutQueriesResidualCoverageTest.backgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvances", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"backgroundSegmentsTrimGlueApplyPaddingAndUseGlyphAdvances");
        let clusters = vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"，", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"字", 10.0f64)).clone(),
];
        let glue = ClusterGeometryDecisionInfo::new(TextRange::new(0u32, 1u32).unwrap(), "，", "，", 10.0f64, 5.0f64, 4.0f64, 1.0f64, 4.0f64, 1.0f64, 0.0f64, 10.0f64, "test", "test", Some(0.0f64), Some(0.0f64), None);
        let runs = vec![
    (GlyphRun::new(TextRange::new(1u32, 2u32).unwrap(), "test", vec![
    (Glyph::new(9u32, TextRange::new(1u32, 2u32).unwrap(), 9.0f64, Some(1.0f64), Some(0.0f64), None, None, None, None)).clone(),
].to_vec(), 10.0f64, Some(vec![]))).clone(),
];
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"，字", &clusters, &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &runs, &vec![], &vec![], LayoutDebugInfo::new(None, Some(vec![]), Some(vec![(glue).clone()]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let full = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0, TextRange::new(0u32, 2u32).unwrap(), 0.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64)).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(3.0f64, full[0usize].left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20.0f64, full[0usize].right, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(6.19999999999999929f64, full[0usize].top, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.2f64, full[0usize].bottom, None).unwrap();
        let head_paint = RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(5.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap();
        let head = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), (head_paint).clone(), 0, TextRange::new(0u32, 3u32).unwrap(), 0.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64)).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(3.0f64, head[0usize].left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20.0f64, head[0usize].right, None).unwrap();
        let continuation = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(1u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), (head_paint).clone(), 0, TextRange::new(0u32, 3u32).unwrap(), 10.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64)).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, continuation[0usize].left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20.0f64, continuation[0usize].right, None).unwrap();
    });
}

#[test]
fn marked_faces_use_metric_decisions_when_they_cover_the_cluster() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.markedFacesUseMetricDecisionsWhenTheyCoverTheCluster", "org.tiqian.core.LayoutQueriesResidualCoverageTest.markedFacesUseMetricDecisionsWhenTheyCoverTheCluster", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"markedFacesUseMetricDecisionsWhenTheyCoverTheCluster");
        let decision = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_metric(TextRange::new(0u32, 2u32).unwrap(), &"IdeographicEmBox", 7.0f64, 3.0f64, &"ideographic");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutDebugInfo::new(None, Some(vec![(decision).clone()]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let r#box = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0, TextRange::new(0u32, 2u32).unwrap(), 0.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64)).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8.0f64, r#box[0usize].top, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18.0f64, r#box[0usize].bottom, None).unwrap();
    });
}

#[test]
fn uniform_text_style_falls_back_when_every_metric_field_differs() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.uniformTextStyleFallsBackWhenEveryMetricFieldDiffers", "org.tiqian.core.LayoutQueriesResidualCoverageTest.uniformTextStyleFallsBackWhenEveryMetricFieldDiffers", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"uniformTextStyleFallsBackWhenEveryMetricFieldDiffers");
        let base = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64);
        let variants = vec![
    (TextStyle::new(Some(vec!["other".to_string()]), Some(10.0f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0f64), Some(InlineAttachment::None))).clone(),
    (TextStyle::new(Some(vec![]), Some(11.0f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0f64), Some(InlineAttachment::None))).clone(),
    (TextStyle::new(Some(vec![]), Some(10.0f64), Some("ja-JP".to_string()), Some(400), Some(false), Some(0.0f64), Some(InlineAttachment::None))).clone(),
    (TextStyle::new(Some(vec![]), Some(10.0f64), Some("zh-Hans".to_string()), Some(700), Some(false), Some(0.0f64), Some(InlineAttachment::None))).clone(),
    (TextStyle::new(Some(vec![]), Some(10.0f64), Some("zh-Hans".to_string()), Some(400), Some(true), Some(0.0f64), Some(InlineAttachment::None))).clone(),
    (TextStyle::new(Some(vec![]), Some(10.0f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(2.0f64), Some(InlineAttachment::None))).clone(),
];
        let clusters = vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
];
        let decision = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_metric(TextRange::new(1u32, 2u32).unwrap(), &"LatinBox", 9.0f64, 1.0f64, &"latin");
        let paint = RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::UniformTextStyle),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap();
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((variants.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let variant = (variants[usize::try_from(index).unwrap_or(0)]).clone();
            let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &clusters, &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![(TextSpan::new(TextRange::new(0u32, 1u32).unwrap(), (variant).clone())).clone()], &vec![], LayoutDebugInfo::new(None, Some(vec![(decision).clone()]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), (base).clone()).unwrap();
            let r#box = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), (paint).clone(), 0, TextRange::new(0u32, 2u32).unwrap(), 0.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64)).clone(),
]);
            let message = format!("{}{}",
            "variant=",
            variant.to_string()
        );
            let _ = TracedAssertions::traced_assertions_assert_equals_float(15.0f64 - variant.font_size * 0.88f64, r#box[0usize].top, Some((message).to_string())).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_float(15.0f64 + variant.font_size * 0.12f64, r#box[0usize].bottom, Some((message).to_string())).unwrap();
            index = u32::wrapping_add(index, 1);
        }
    });
}

#[test]
fn uniform_text_style_prefers_ideographic_metrics_then_any_matching_face() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.uniformTextStylePrefersIdeographicMetricsThenAnyMatchingFace", "org.tiqian.core.LayoutQueriesResidualCoverageTest.uniformTextStylePrefersIdeographicMetricsThenAnyMatchingFace", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"uniformTextStylePrefersIdeographicMetricsThenAnyMatchingFace");
        let paint = RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::UniformTextStyle),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap();
        let clusters = vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
];
        let line_value = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64);
        let latin = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &clusters, &vec![(line_value).clone()], &vec![], &vec![], &vec![], LayoutDebugInfo::new(None, Some(vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_metric(TextRange::new(0u32, 2u32).unwrap(), &"LatinBox", 9.0f64, 1.0f64, &"latin")).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let latin_box = LayoutQueries::layout_queries_rich_text_background_segments((latin).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), (paint).clone(), 0, TextRange::new(0u32, 2u32).unwrap(), 0.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64)).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(6.0f64, latin_box[0usize].top, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.0f64, latin_box[0usize].bottom, None).unwrap();
        let both_metrics = vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_metric(TextRange::new(0u32, 1u32).unwrap(), &"LatinBox", 9.0f64, 1.0f64, &"latin")).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_metric(TextRange::new(0u32, 2u32).unwrap(), &"IdeographicEmBox", 8.0f64, 2.0f64, &"ideographic")).clone(),
];
        let both = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &clusters, &vec![(line_value).clone()], &vec![], &vec![], &vec![], LayoutDebugInfo::new(None, Some((both_metrics).clone()), Some(vec![]), Some(vec![]),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]), Some(vec![]), Some(vec![])), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let ideographic_box = LayoutQueries::layout_queries_rich_text_background_segments((both).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), (paint).clone(), 0, TextRange::new(0u32, 2u32).unwrap(), 0.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64)).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(7.0f64, ideographic_box[0usize].top, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(17.0f64, ideographic_box[0usize].bottom, None).unwrap();
    });
}

#[test]
fn adjacent_same_style_segments_share_clearance() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.adjacentSameStyleSegmentsShareClearance", "org.tiqian.core.LayoutQueriesResidualCoverageTest.adjacentSameStyleSegmentsShareClearance", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"adjacentSameStyleSegmentsShareClearance");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let paint = RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(4.0f64)).unwrap();
        let first = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 1u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e|
e.into_inner()); __guard.clone() })).clone(), (paint).clone(), 0, TextRange::new(0u32, 1u32).unwrap(), 0.0f64, 0.0f64, 10.0f64, 20.0f64, 15.0f64);
        let second = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(1u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e|
e.into_inner()); __guard.clone() })).clone(), (paint).clone(), 0, TextRange::new(1u32, 2u32).unwrap(), 10.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64);
        let cleared = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![(first).clone(), (second).clone()]);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((cleared.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8.0f64, cleared[0usize].right, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(12.0f64, cleared[1usize].left, None).unwrap();
        let other = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(1u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e|
e.into_inner()); __guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(3.0f64), Some(3.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0, TextRange::new(1u32, 2u32).unwrap(), 10.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64);
        let untouched = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![(first).clone(), (other).clone()]);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((untouched.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, untouched[0usize].right, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, untouched[1usize].left, None).unwrap();
    });
}

#[test]
fn decoration_line_y_requires_valid_stroke_and_decoration_roles() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.decorationLineYRequiresValidStrokeAndDecorationRoles", "org.tiqian.core.LayoutQueriesResidualCoverageTest.decorationLineYRequiresValidStrokeAndDecorationRoles", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"decorationLineYRequiresValidStrokeAndDecorationRoles");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 10.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let underline = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 1u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e|
e.into_inner()); __guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0, TextRange::new(0u32, 1u32).unwrap(), 0.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64);
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let content = (content).clone(); let underline = (underline).clone(); Arc::new(move || {
        LayoutQueries::layout_queries_rich_text_decoration_line_y((content).clone(), (underline).clone(), -1.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let content = (content).clone(); let underline = (underline).clone(); Arc::new(move || {
        LayoutQueries::layout_queries_rich_text_decoration_line_y((content).clone(), (underline).clone(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_nan()).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let error = TracedAssertions::traced_assertions_assert_fails_with(None.clone(), { let content = (content).clone(); Arc::new(move || {
        LayoutQueries::layout_queries_rich_text_decoration_line_y((content).clone(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_plain_segment(TextRange::new(0u32, 1u32).map_err(|e| IllegalStateException::new(&format!("{}",
e)))?).map_err(|e| IllegalStateException::new(&format!("{}", e)))?, 1.0f64).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&format!("{}", error), "underline and line-through", 0)).to_ne_bytes())) <= 2147483647, Some((format!("{}", error)).to_string())).unwrap();
        let with_span_style = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 10.0f64)).clone(),
], &vec![], &vec![
    (TextSpan::new(TextRange::new(0u32, 1u32).unwrap(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64))).clone(),
], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let y = LayoutQueries::layout_queries_rich_text_decoration_line_y((with_span_style).clone(), (underline).clone(), 1.0f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((y) >= underline.top && (y) <= underline.bottom, Some((crate::runtime::fp_helper::FPHelper::format_float(y)).to_string())).unwrap();
        let line_through = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 1u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::LINE_THROUGH_INSTANCE.lock().unwrap_or_else(|e|
e.into_inner()); __guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0, TextRange::new(0u32, 1u32).unwrap(), 0.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64);
        let strike = LayoutQueries::layout_queries_rich_text_decoration_line_y((with_span_style).clone(), (line_through).clone(), 1.0f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(11.2f64, strike, 0.001f64, None).unwrap();
    });
}

#[test]
fn cursor_rect_covers_empty_lines_empty_clusters_and_multi_unit_clusters() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.cursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClusters", "org.tiqian.core.LayoutQueriesResidualCoverageTest.cursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClusters", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"cursorRectCoversEmptyLinesEmptyClustersAndMultiUnitClusters");
        let empty_cluster_line = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 0u32).unwrap(), 0, 4294967295u32, 0.0f64, 20.0f64, 15.0f64, 6.0f64, 10.0f64);
        let with_empty_line = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"a", &vec![], &vec![(empty_cluster_line).clone()], &vec![], &vec![], &vec![],
LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(Rect::new(6.0f64, 0.0f64, 7.0f64, 20.0f64).to_string().as_str(), LayoutQueries::layout_queries_get_cursor_rect((with_empty_line).clone(), 0).unwrap().to_string().as_str(), None).unwrap();
        let linear = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 2u32).unwrap(), &"ab", 20.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, LayoutQueries::layout_queries_get_cursor_rect((linear).clone(), 1).unwrap().left, None).unwrap();
        let stops = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 2u32).unwrap(), &"ab", 20.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![
    (GlyphRun::new(TextRange::new(0u32, 2u32).unwrap(), "test", vec![
    (Glyph::new(1u32, TextRange::new(0u32, 2u32).unwrap(), 10.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None)).clone(),
    (Glyph::new(2u32, TextRange::new(0u32, 2u32).unwrap(), 10.0f64, Some(12.0f64), Some(0.0f64), None, None, None, None)).clone(),
].to_vec(), 20.0f64, Some(vec![]))).clone(),
], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(12.0f64, LayoutQueries::layout_queries_get_cursor_rect((stops).clone(), 1).unwrap().left, None).unwrap();
    });
}

#[test]
fn offset_for_position_covers_vertical_distances_and_na_n_points() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.offsetForPositionCoversVerticalDistancesAndNaNPoints", "org.tiqian.core.LayoutQueriesResidualCoverageTest.offsetForPositionCoversVerticalDistancesAndNaNPoints", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"offsetForPositionCoversVerticalDistancesAndNaNPoints");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(2u32, 2u32).unwrap(), 2, 4294967295u32, 20.0f64, 40.0f64, 35.0f64, 0.0f64, 0.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_offset_for_position((content).clone(), 2.0f64, -50.0f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, LayoutQueries::layout_queries_get_offset_for_position((content).clone(), 5.0f64, 90.0f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, LayoutQueries::layout_queries_get_offset_for_position((content).clone(), 5.0f64, 30.0f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_selection_offset_for_position((content).clone(), 2.0f64, -50.0f64).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, LayoutQueries::layout_queries_get_selection_offset_for_position((content).clone(), 5.0f64, 90.0f64).unwrap(), None).unwrap();
        let with_stops = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 2u32).unwrap(), &"ab", 20.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![
    (GlyphRun::new(TextRange::new(0u32, 2u32).unwrap(), "test", vec![
    (Glyph::new(1u32, TextRange::new(0u32, 2u32).unwrap(), 10.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None)).clone(),
    (Glyph::new(2u32, TextRange::new(0u32, 2u32).unwrap(), 10.0f64, Some(10.0f64), Some(0.0f64), None, None, None, None)).clone(),
].to_vec(), 20.0f64, Some(vec![]))).clone(),
], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_offset_for_position((with_stops).clone(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_nan(), 5.0f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_selection_offset_for_position((with_stops).clone(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_nan(), 5.0f64).unwrap(),
None).unwrap();
    });
}

#[test]
fn selection_snap_prefers_the_closer_inline_object_boundary() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.selectionSnapPrefersTheCloserInlineObjectBoundary", "org.tiqian.core.LayoutQueriesResidualCoverageTest.selectionSnapPrefersTheCloserInlineObjectBoundary", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"selectionSnapPrefersTheCloserInlineObjectBoundary");
        let object = InlineObjectSpan::new(TextRange::new(1u32, 3u32).unwrap(), 8.0f64, 4.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abb", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 3u32).unwrap(), &"abb", 30.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 30.0f64)).clone(),
], &vec![], &vec![], &vec![(object).clone()], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, LayoutQueries::layout_queries_get_selection_offset_for_position((content).clone(), 15.0f64, 5.0f64).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, LayoutQueries::layout_queries_get_selection_offset_for_position((content).clone(), 21.0f64, 5.0f64).unwrap(), None).unwrap();
    });
}

#[test]
fn selection_word_boundary_for_position_rejects_degenerate_content() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.selectionWordBoundaryForPositionRejectsDegenerateContent", "org.tiqian.core.LayoutQueriesResidualCoverageTest.selectionWordBoundaryForPositionRejectsDegenerateContent", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"selectionWordBoundaryForPositionRejectsDegenerateContent");
        let empty_text = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"", &vec![], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 0u32).unwrap(), 0, 4294967295u32, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 0.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let empty_text_boundary = LayoutQueries::layout_queries_get_selection_word_boundary_for_position((empty_text).clone(), 0.0f64, 0.0f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(empty_text_boundary.is_none(), match &(empty_text_boundary) { None => "-".to_string(), Some(__option8) => __option8.to_string() }.as_str(), None).unwrap();
        let empty_line = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"a", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 1u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(1u32, 1u32).unwrap(), 1, 4294967295u32, 20.0f64, 40.0f64, 35.0f64, 0.0f64, 0.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let empty_line_boundary = LayoutQueries::layout_queries_get_selection_word_boundary_for_position((empty_line).clone(), 5.0f64, 30.0f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(empty_line_boundary.is_none(), match &(empty_line_boundary) { None => "-".to_string(), Some(__option11) => __option11.to_string() }.as_str(), None).unwrap();
        let leading_empty = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"a", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 0u32).unwrap(), &"", 0.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 1u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 10.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let leading_empty_boundary = LayoutQueries::layout_queries_get_selection_word_boundary_for_position((leading_empty).clone(), 0.0f64, 5.0f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(leading_empty_boundary.is_none(), match &(leading_empty_boundary) { None => "-".to_string(), Some(__option14) => __option14.to_string() }.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 1u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary_for_position((leading_empty).clone(), 5.0f64,
5.0f64).unwrap().as_ref().unwrap().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn zero_width_clusters_return_their_start_in_hit_tests() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.zeroWidthClustersReturnTheirStartInHitTests", "org.tiqian.core.LayoutQueriesResidualCoverageTest.zeroWidthClustersReturnTheirStartInHitTests", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"zeroWidthClustersReturnTheirStartInHitTests");
        let empty_range = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 0u32).unwrap(), &"", 5.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 0u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 5.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_offset_for_position((empty_range).clone(), 2.0f64, 5.0f64), None).unwrap();
        let zero_advance = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 0.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 1u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(1u32, 2u32).unwrap(), 1, 1, 20.0f64, 40.0f64, 35.0f64, 0.0f64, 0.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 2u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary_for_position((zero_advance).clone(), 0.0f64,
30.0f64).unwrap().as_ref().unwrap().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn coerce_selection_offset_honours_inline_object_boundaries() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.coerceSelectionOffsetHonoursInlineObjectBoundaries", "org.tiqian.core.LayoutQueriesResidualCoverageTest.coerceSelectionOffsetHonoursInlineObjectBoundaries", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"coerceSelectionOffsetHonoursInlineObjectBoundaries");
        let object = InlineObjectSpan::new(TextRange::new(1u32, 3u32).unwrap(), 8.0f64, 4.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abb", &vec![], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 0.0f64)).clone(),
], &vec![], &vec![], &vec![(object).clone()], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, LayoutQueries::layout_queries_coerce_selection_offset((content).clone(), 2, SourceBoundaryBias::Backward).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, LayoutQueries::layout_queries_coerce_selection_offset((content).clone(), 2, SourceBoundaryBias::Forward).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, LayoutQueries::layout_queries_coerce_selection_offset((content).clone(), 2, SourceBoundaryBias::Nearest).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, LayoutQueries::layout_queries_coerce_selection_offset((content).clone(), 1, SourceBoundaryBias::Nearest).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, LayoutQueries::layout_queries_coerce_selection_offset((content).clone(), 3, SourceBoundaryBias::Nearest).unwrap(), None).unwrap();
    });
}

#[test]
fn selection_word_boundary_expands_words_and_honours_inline_objects() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.selectionWordBoundaryExpandsWordsAndHonoursInlineObjects", "org.tiqian.core.LayoutQueriesResidualCoverageTest.selectionWordBoundaryExpandsWordsAndHonoursInlineObjects", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"selectionWordBoundaryExpandsWordsAndHonoursInlineObjects");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"hello", &vec![], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 5u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 0.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 5u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary((content).clone(), 2).unwrap().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 5u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary((content).clone(), 5).unwrap().to_string().as_str(), None).unwrap();
        let emoji_text = TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832]);
        let emoji = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(emoji_text.as_str(), &vec![], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 0.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 2u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary((emoji).clone(), 1).unwrap().to_string().as_str(), None).unwrap();
        let object = InlineObjectSpan::new(TextRange::new(1u32, 3u32).unwrap(), 8.0f64, 4.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
        let with_object = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abb", &vec![], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 0.0f64)).clone(),
], &vec![], &vec![], &vec![(object).clone()], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(1u32, 3u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary((with_object).clone(), 2).unwrap().to_string().as_str(), None).unwrap();
        let mandatory = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&concat!("a\n",
"b"), &vec![], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 0.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(1u32, 2u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary((mandatory).clone(), 1).unwrap().to_string().as_str(), None).unwrap();
        let connectors = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"a_b", &vec![], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 0.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 3u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary((connectors).clone(), 1).unwrap().to_string().as_str(), None).unwrap();
        let empty = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"", &vec![], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 0u32).unwrap(), 0, 4294967295u32, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 0.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 0u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary((empty).clone(), 0).unwrap().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn selection_word_kind_covers_every_han_block() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.selectionWordKindCoversEveryHanBlock", "org.tiqian.core.LayoutQueriesResidualCoverageTest.selectionWordKindCoversEveryHanBlock", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"selectionWordKindCoversEveryHanBlock");
        let supplementary = TestHelpers::test_helpers_surrogate_text(&vec![55360, 56320]);
        let values = vec!["㐀".to_string(), "一".to_string(), "豈".to_string(), supplementary.clone()];
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let text = (values[usize::try_from(index).unwrap_or(0)]).clone();
            let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(text.as_str(), &vec![], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, u_string::unit_count(&(text))).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 0.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, u_string::unit_count(&(text))).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary((content).clone(), 0).unwrap().to_string().as_str(),
Some((format!("{}{}",
            "text=",
            text
        )).to_string())).unwrap();
            index = u32::wrapping_add(index, 1);
        }
    });
}

#[test]
fn nearest_line_falls_back_to_the_only_line_at_its_end_offset() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.nearestLineFallsBackToTheOnlyLineAtItsEndOffset", "org.tiqian.core.LayoutQueriesResidualCoverageTest.nearestLineFallsBackToTheOnlyLineAtItsEndOffset", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"nearestLineFallsBackToTheOnlyLineAtItsEndOffset");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abc", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 2u32).unwrap(), &"ab", 20.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_line_for_offset((content).clone(), 2), None).unwrap();
    });
}

#[test]
fn ruby_geometry_redistributes_selection_boxes_and_drops_source_stops() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.rubyGeometryRedistributesSelectionBoxesAndDropsSourceStops", "org.tiqian.core.LayoutQueriesResidualCoverageTest.rubyGeometryRedistributesSelectionBoxesAndDropsSourceStops", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"rubyGeometryRedistributesSelectionBoxesAndDropsSourceStops");
        let clusters = vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 2u32).unwrap(), &"ab", 20.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"c", 10.0f64)).clone(),
];
        let runs = vec![
    (GlyphRun::new(TextRange::new(0u32, 2u32).unwrap(), "test", vec![
    (Glyph::new(1u32, TextRange::new(0u32, 2u32).unwrap(), 10.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None)).clone(),
    (Glyph::new(2u32, TextRange::new(0u32, 2u32).unwrap(), 10.0f64, Some(10.0f64), Some(0.0f64), None, None, None, None)).clone(),
].to_vec(), 20.0f64, Some(vec![]))).clone(),
];
        let matching = RubyDecisionInfo::new(TextRange::new(0u32, 3u32).unwrap(), "zhù", 0u32, 15.0f64, 4.0f64, 6.0f64, 0.0f64, Some(0.0f64), Some(0.0f64), Some(30.0f64), Some(vec![]), Some(400), Some("zh-Hans".to_string()), Some(vec![]));
        let stray = RubyDecisionInfo::new(TextRange::new(5u32, 6u32).unwrap(), "x", 0u32, 0.0f64, 4.0f64, 6.0f64, 0.0f64, Some(0.0f64), Some(0.0f64), Some(6.0f64), Some(vec![]), Some(400), Some("zh-Hans".to_string()), Some(vec![]));
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abc", &clusters, &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 30.0f64)).clone(),
], &runs, &vec![], &vec![], LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(matching).clone(), (stray).clone()]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])),
LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let positioned = LayoutQueries::layout_queries_positioned_clusters((content).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((positioned.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let first_stops = (positioned[0usize]).clone().source_stops;
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(first_stops.is_none(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_render_nullable_source_stops((first_stops).clone()).as_str(), None).unwrap();
        let second_stops = (positioned[1usize]).clone().source_stops;
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(second_stops.is_none(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_render_nullable_source_stops((second_stops).clone()).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, positioned[0usize].left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(17.5f64, positioned[0usize].right, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(17.5f64, positioned[1usize].left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(30.0f64, positioned[1usize].right, None).unwrap();
    });
}

#[test]
fn bounding_boxes_slice_zero_width_and_empty_clusters() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.boundingBoxesSliceZeroWidthAndEmptyClusters", "org.tiqian.core.LayoutQueriesResidualCoverageTest.boundingBoxesSliceZeroWidthAndEmptyClusters", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"boundingBoxesSliceZeroWidthAndEmptyClusters");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 0.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 10.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let boxes = LayoutQueries::layout_queries_get_bounding_boxes((content).clone(), TextRange::new(0u32, 2u32).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((boxes.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, boxes[1usize].left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, boxes[1usize].right, None).unwrap();
        let tail = LayoutQueries::layout_queries_get_bounding_boxes((content).clone(), TextRange::new(1u32, 2u32).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((tail.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, tail[0usize].left, None).unwrap();
    });
}

#[test]
fn positioned_clusters_and_segments_return_empty_without_lines() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.positionedClustersAndSegmentsReturnEmptyWithoutLines", "org.tiqian.core.LayoutQueriesResidualCoverageTest.positionedClustersAndSegmentsReturnEmptyWithoutLines", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"positionedClustersAndSegmentsReturnEmptyWithoutLines");
        let no_lines = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
], &vec![], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((LayoutQueries::layout_queries_positioned_clusters((no_lines).clone()).len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((LayoutQueries::layout_queries_positioned_rich_text_segments((no_lines).clone(), &vec![
    (RichTextSpan::new(TextRange::new(0u32, 2u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), RichTextPaint::new(None,
Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap())).clone(),
]).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let no_spans = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 1u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 10.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((LayoutQueries::layout_queries_positioned_rich_text_segments((no_spans).clone(), &vec![]).unwrap().len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn same_span_slices_across_a_source_boundary_merge_into_one_segment() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.sameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegment", "org.tiqian.core.LayoutQueriesResidualCoverageTest.sameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegment", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"sameSpanSlicesAcrossASourceBoundaryMergeIntoOneSegment");
        let input_content = TiqianTextContent::new("ab", Some(vec![]), Some(vec![1]), Some(vec![]), Some(vec![]));
        let input = LayoutInput::new((input_content).clone(), Some(LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None,
Some((Ic::zero()).clone()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0f64), Some(1.0f64), Some(2.0f64))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(100.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]),
Some(vec![]), Some(vec![]), Some(vec![]));
        let content = LayoutResult::new((input).clone(), Size::new(20.0f64, 20.0f64), vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
], vec![], vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug());
        let segments = LayoutQueries::layout_queries_positioned_rich_text_segments((content).clone(), &vec![
    (RichTextSpan::new(TextRange::new(0u32, 2u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), RichTextPaint::new(None,
Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap())).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((segments.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 2u32).unwrap().to_string().as_str(), ((segments[0usize]).clone().range).clone().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, segments[0usize].left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20.0f64, segments[0usize].right, None).unwrap();
    });
}

#[test]
fn glyph_ink_bounds_skips_unusable_glyphs_and_reports_null() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.glyphInkBoundsSkipsUnusableGlyphsAndReportsNull", "org.tiqian.core.LayoutQueriesResidualCoverageTest.glyphInkBoundsSkipsUnusableGlyphsAndReportsNull", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"glyphInkBoundsSkipsUnusableGlyphsAndReportsNull");
        let clusters = vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
];
        let lines = vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
];
        let no_bounds = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &clusters, &lines, &vec![
    (GlyphRun::new(TextRange::new(0u32, 2u32).unwrap(), "test", vec![
    (Glyph::new(1u32, TextRange::new(0u32, 1u32).unwrap(), 10.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None)).clone(),
].to_vec(), 20.0f64, Some(vec![]))).clone(),
], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let absent_bounds = LayoutQueries::layout_queries_glyph_ink_bounds((no_bounds).clone());
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(absent_bounds.is_none(), match &(absent_bounds) { None => "-".to_string(), Some(__option17) => __option17.to_string() }.as_str(), None).unwrap();
        let nan_placed = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &clusters, &lines, &vec![
    (GlyphRun::new(TextRange::new(1u32, 2u32).unwrap(), "test", vec![
    (Glyph::new(9u32, TextRange::new(1u32, 2u32).unwrap(), 9.0f64, Some(LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_nan()), Some(0.0f64), None, Some(Rect::new(1.0f64, 2.0f64, 8.0f64, 4.0f64)), None, None)).clone(),
].to_vec(), 10.0f64, Some(vec![]))).clone(),
], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let nan_placed_bounds = LayoutQueries::layout_queries_glyph_ink_bounds((nan_placed).clone());
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(nan_placed_bounds.is_none(), match &(nan_placed_bounds) { None => "-".to_string(), Some(__option20) => __option20.to_string() }.as_str(), None).unwrap();
        let usable = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &clusters, &lines, &vec![
    (GlyphRun::new(TextRange::new(0u32, 2u32).unwrap(), "test", vec![
    (Glyph::new(1u32, TextRange::new(0u32, 1u32).unwrap(), 10.0f64, Some(2.0f64), Some(1.0f64), None, Some(Rect::new(1.0f64, 2.0f64, 8.0f64, 4.0f64)), None, None)).clone(),
    (Glyph::new(2u32, TextRange::new(1u32, 2u32).unwrap(), 10.0f64, Some(1.0f64), Some(0.0f64), None, Some(Rect::new(0.0f64, 1.0f64, 9.0f64, 3.0f64)), None, None)).clone(),
].to_vec(), 20.0f64, Some(vec![]))).clone(),
], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let ink = LayoutQueries::layout_queries_glyph_ink_bounds((usable).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(3.0f64, ink.as_ref().unwrap().left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20.0f64, ink.as_ref().unwrap().right, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.0f64, ink.as_ref().unwrap().top, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20.0f64, ink.as_ref().unwrap().bottom, None).unwrap();
    });
}

#[test]
fn background_trailing_edge_uses_glyph_advances_when_available() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.backgroundTrailingEdgeUsesGlyphAdvancesWhenAvailable", "org.tiqian.core.LayoutQueriesResidualCoverageTest.backgroundTrailingEdgeUsesGlyphAdvancesWhenAvailable", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"backgroundTrailingEdgeUsesGlyphAdvancesWhenAvailable");
        let clusters = vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
];
        let lines = vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
];
        let short_glyph = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &clusters, &lines, &vec![
    (GlyphRun::new(TextRange::new(1u32, 2u32).unwrap(), "test", vec![
    (Glyph::new(2u32, TextRange::new(1u32, 2u32).unwrap(), 5.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None)).clone(),
].to_vec(), 10.0f64, Some(vec![]))).clone(),
], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let short_segments = LayoutQueries::layout_queries_rich_text_background_segments((short_glyph).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0, TextRange::new(0u32, 2u32).unwrap(), 0.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64)).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(15.0f64, short_segments[0usize].right, None).unwrap();
        let empty_glyph_run = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &clusters, &lines, &vec![
    (GlyphRun::new(TextRange::new(1u32, 2u32).unwrap(), "test", vec![].to_vec(), 10.0f64, Some(vec![]))).clone(),
], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let empty_segments = LayoutQueries::layout_queries_rich_text_background_segments((empty_glyph_run).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0, TextRange::new(0u32, 2u32).unwrap(), 0.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64)).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20.0f64, empty_segments[0usize].right, None).unwrap();
    });
}

#[test]
fn clearance_needs_same_role_and_uses_the_smaller_side() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.clearanceNeedsSameRoleAndUsesTheSmallerSide", "org.tiqian.core.LayoutQueriesResidualCoverageTest.clearanceNeedsSameRoleAndUsesTheSmallerSide", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"clearanceNeedsSameRoleAndUsesTheSmallerSide");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let background = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 1u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e|
e.into_inner()); __guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(4.0f64)).unwrap(), 0, TextRange::new(0u32, 1u32).unwrap(), 0.0f64, 0.0f64, 10.0f64, 20.0f64, 15.0f64);
        let inline_code = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(1u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::INLINE_CODE_INSTANCE.lock().unwrap_or_else(|e|
e.into_inner()); __guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(4.0f64)).unwrap(), 0, TextRange::new(1u32, 2u32).unwrap(), 10.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64);
        let by_role = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![(background).clone(), (inline_code).clone()]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, by_role[0usize].right, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, by_role[1usize].left, None).unwrap();
        let weak = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 1u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e|
e.into_inner()); __guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(2.0f64)).unwrap(), 0, TextRange::new(0u32, 1u32).unwrap(), 0.0f64, 0.0f64, 10.0f64, 20.0f64, 15.0f64);
        let strong = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(1u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e|
e.into_inner()); __guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(6.0f64)).unwrap(), 0, TextRange::new(1u32, 2u32).unwrap(), 10.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64);
        let cleared = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![(weak).clone(), (strong).clone()]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(9.0f64, cleared[0usize].right, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(11.0f64, cleared[1usize].left, None).unwrap();
    });
}

#[test]
fn metric_decisions_must_fully_contain_the_cluster() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.metricDecisionsMustFullyContainTheCluster", "org.tiqian.core.LayoutQueriesResidualCoverageTest.metricDecisionsMustFullyContainTheCluster", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"metricDecisionsMustFullyContainTheCluster");
        let first = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_metric_bounds(TextRange::new(1u32, 2u32).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(6.19999999999999929f64, first[0usize], None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.2f64, first[1usize], None).unwrap();
        let second = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_metric_bounds(TextRange::new(0u32, 1u32).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(6.19999999999999929f64, second[0usize], None).unwrap();
    });
}

#[test]
fn decoration_style_resolves_inside_spans_and_at_their_edges() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.decorationStyleResolvesInsideSpansAndAtTheirEdges", "org.tiqian.core.LayoutQueriesResidualCoverageTest.decorationStyleResolvesInsideSpansAndAtTheirEdges", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"decorationStyleResolvesInsideSpansAndAtTheirEdges");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abc", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"c", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 2, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 30.0f64)).clone(),
], &vec![], &vec![
    (TextSpan::new(TextRange::new(0u32, 1u32).unwrap(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64))).clone(),
    (TextSpan::new(TextRange::new(2u32, 3u32).unwrap(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(20.0f64))).clone(),
], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let between = LayoutQueries::layout_queries_rich_text_decoration_line_y((content).clone(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(1u32, 2u32).unwrap(), (Box::new({ let __guard =
crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())),
RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0,
TextRange::new(1u32, 2u32).unwrap(), 10.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64), 1.0f64).unwrap();
        let inside = LayoutQueries::layout_queries_rich_text_decoration_line_y((content).clone(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(2u32, 3u32).unwrap(), (Box::new({ let __guard =
crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())),
RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0,
TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 0.0f64, 30.0f64, 20.0f64, 15.0f64), 1.0f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.8f64, between, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18.6f64, inside, None).unwrap();
    });
}

#[test]
fn glue_trim_skips_interior_segment_edges() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.glueTrimSkipsInteriorSegmentEdges", "org.tiqian.core.LayoutQueriesResidualCoverageTest.glueTrimSkipsInteriorSegmentEdges", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"glueTrimSkipsInteriorSegmentEdges");
        let glue = ClusterGeometryDecisionInfo::new(TextRange::new(0u32, 2u32).unwrap(), "ab", "ab", 20.0f64, 10.0f64, 4.0f64, 1.0f64, 4.0f64, 1.0f64, 0.0f64, 20.0f64, "test", "test", Some(0.0f64), Some(0.0f64), None);
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 2u32).unwrap(), &"ab", 20.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutDebugInfo::new(None, Some(vec![]), Some(vec![(glue).clone()]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let interior_start = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(1u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0, TextRange::new(1u32, 2u32).unwrap(), 10.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64)).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, interior_start[0usize].left, None).unwrap();
        let interior_end = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 1u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0, TextRange::new(0u32, 1u32).unwrap(), 0.0f64, 0.0f64, 10.0f64, 20.0f64, 15.0f64)).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, interior_end[0usize].right, None).unwrap();
    });
}

#[test]
fn background_segment_outside_every_span_uses_the_paragraph_style() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.backgroundSegmentOutsideEverySpanUsesTheParagraphStyle", "org.tiqian.core.LayoutQueriesResidualCoverageTest.backgroundSegmentOutsideEverySpanUsesTheParagraphStyle", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"backgroundSegmentOutsideEverySpanUsesTheParagraphStyle");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abc", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"c", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 2, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 30.0f64)).clone(),
], &vec![], &vec![
    (TextSpan::new(TextRange::new(1u32, 2u32).unwrap(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(40.0f64))).clone(),
], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let before = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 1u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0, TextRange::new(0u32, 1u32).unwrap(), 0.0f64, 0.0f64, 10.0f64, 20.0f64, 15.0f64)).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(6.19999999999999929f64, before[0usize].top, None).unwrap();
        let at_end = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(2u32, 3u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0, TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 0.0f64, 30.0f64, 20.0f64, 15.0f64)).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(6.19999999999999929f64, at_end[0usize].top, None).unwrap();
    });
}

#[test]
fn cursor_rect_finds_later_clusters_and_rejects_gapped_ranges() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.cursorRectFindsLaterClustersAndRejectsGappedRanges", "org.tiqian.core.LayoutQueriesResidualCoverageTest.cursorRectFindsLaterClustersAndRejectsGappedRanges", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"cursorRectFindsLaterClustersAndRejectsGappedRanges");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abc", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"c", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 2, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 30.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20.0f64, LayoutQueries::layout_queries_get_cursor_rect((content).clone(), 2).unwrap().left, None).unwrap();
        let gapped = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abcde", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(4u32, 5u32).unwrap(), &"e", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 5u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        TracedAssertions::traced_assertions_assert_fails_with_no_such_element(None.clone(), { let gapped = (gapped).clone(); Arc::new(move || {
        LayoutQueries::layout_queries_get_cursor_rect((gapped).clone(), 2)?;
        Ok(())
}) }).unwrap();
    });
}

#[test]
fn empty_mid_cluster_holds_the_caret_and_slices_keep_degenerate_rects() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.emptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRects", "org.tiqian.core.LayoutQueriesResidualCoverageTest.emptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRects", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"emptyMidClusterHoldsTheCaretAndSlicesKeepDegenerateRects");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abc", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(2u32, 2u32).unwrap(), &"", 0.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"c", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 2, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, LayoutQueries::layout_queries_get_cursor_rect((content).clone(), 2).unwrap().left, None).unwrap();
        let with_empty = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 1u32).unwrap(), &"", 0.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 2, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let boxes = LayoutQueries::layout_queries_get_bounding_boxes((with_empty).clone(), TextRange::new(0u32, 2u32).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((boxes.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, boxes[0usize].left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, boxes[0usize].right, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, boxes[1usize].left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20.0f64, boxes[1usize].right, None).unwrap();
        let zero_advance = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abc", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 0.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"c", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 2, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let degenerate = LayoutQueries::layout_queries_get_bounding_boxes((zero_advance).clone(), TextRange::new(0u32, 3u32).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((degenerate.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, degenerate[1usize].left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, degenerate[1usize].right, None).unwrap();
    });
}

#[test]
fn selection_word_boundary_skips_inline_objects_it_does_not_contain() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.selectionWordBoundarySkipsInlineObjectsItDoesNotContain", "org.tiqian.core.LayoutQueriesResidualCoverageTest.selectionWordBoundarySkipsInlineObjectsItDoesNotContain", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"selectionWordBoundarySkipsInlineObjectsItDoesNotContain");
        let objects = vec![
    (InlineObjectSpan::new(TextRange::new(1u32, 3u32).unwrap(), 8.0f64, 4.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
    (InlineObjectSpan::new(TextRange::new(5u32, 7u32).unwrap(), 8.0f64, 4.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()),
Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap()).clone(),
];
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abcdefg", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 7u32).unwrap(), &"abcdefg", 70.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 7u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 70.0f64)).clone(),
], &vec![], &vec![], &objects, LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 7u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary((content).clone(), 4).unwrap().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(1u32, 3u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary((content).clone(), 2).unwrap().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn selection_word_boundary_for_position_covers_distances_and_fallbacks() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.selectionWordBoundaryForPositionCoversDistancesAndFallbacks", "org.tiqian.core.LayoutQueriesResidualCoverageTest.selectionWordBoundaryForPositionCoversDistancesAndFallbacks", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"selectionWordBoundaryForPositionCoversDistancesAndFallbacks");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"甲乙", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"甲", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"乙", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 1u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary_for_position((content).clone(), 5.0f64,
10.0f64).unwrap().as_ref().unwrap().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 1u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary_for_position((content).clone(), 5.0f64,
-10.0f64).unwrap().as_ref().unwrap().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 1u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary_for_position((content).clone(), 5.0f64,
60.0f64).unwrap().as_ref().unwrap().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 1u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary_for_position((content).clone(), -50.0f64,
10.0f64).unwrap().as_ref().unwrap().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(1u32, 2u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary_for_position((content).clone(), 500.0f64,
10.0f64).unwrap().as_ref().unwrap().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn line_for_offset_inside_a_range_takes_the_zero_distance_arm() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.lineForOffsetInsideARangeTakesTheZeroDistanceArm", "org.tiqian.core.LayoutQueriesResidualCoverageTest.lineForOffsetInsideARangeTakesTheZeroDistanceArm", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"lineForOffsetInsideARangeTakesTheZeroDistanceArm");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abcde", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(4u32, 5u32).unwrap(), &"e", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(4u32, 5u32).unwrap(), 1, 1, 20.0f64, 40.0f64, 35.0f64, 0.0f64, 10.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_line_for_offset((content).clone(), 1), None).unwrap();
    });
}

#[test]
fn compatibility_ideographs_form_individual_word_units() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.compatibilityIdeographsFormIndividualWordUnits", "org.tiqian.core.LayoutQueriesResidualCoverageTest.compatibilityIdeographsFormIndividualWordUnits", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"compatibilityIdeographsFormIndividualWordUnits");
        let text = format!("{}{}",
            TestHelpers::test_helpers_surrogate_text(&vec![55360, 56320]),
            "豈"
        );
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(text.as_str(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 2u32).unwrap(), TestHelpers::test_helpers_surrogate_text(&vec![55360, 56320]).as_str(), 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"豈", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 2u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary((content).clone(), 0).unwrap().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(2u32, 3u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary((content).clone(), 2).unwrap().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn ruby_spread_shifts_selection_boxes_and_zero_width_rubies_are_ignored() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.rubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnored", "org.tiqian.core.LayoutQueriesResidualCoverageTest.rubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnored", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"rubySpreadShiftsSelectionBoxesAndZeroWidthRubiesAreIgnored");
        let clusters = vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 2u32).unwrap(), &"ab", 20.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"c", 10.0f64)).clone(),
];
        let mut geometries = vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_geometry(TextRange::new(0u32, 2u32).unwrap(), &"ab", 0.0f64, 0.0f64, 0.0f64, 0.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_geometry(TextRange::new(2u32, 3u32).unwrap(), &"c", 0.0f64, 0.0f64, 0.0f64, 0.0f64)).clone(),
];
        let first_geometry = ClusterGeometryDecisionInfo::new(TextRange::new(0u32, 2u32).unwrap(), "ab", "ab", 20.0f64, 10.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 20.0f64, "test", "test", Some(5.0f64), Some(0.0f64), None);
        let second_geometry = ClusterGeometryDecisionInfo::new(TextRange::new(2u32, 3u32).unwrap(), "c", "c", 10.0f64, 10.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 10.0f64, "test", "test", Some(2.0f64), Some(0.0f64), None);
        geometries[0usize] = first_geometry;
        geometries[1usize] = second_geometry;
        let rubies = vec![
    (RubyDecisionInfo::new(TextRange::new(0u32, 3u32).unwrap(), "zhù", 0u32, 15.0f64, 4.0f64, 6.0f64, 0.0f64, Some(0.0f64), Some(0.0f64), Some(30.0f64), Some(vec![]), Some(400), Some("zh-Hans".to_string()), Some(vec![]))).clone(),
    (RubyDecisionInfo::new(TextRange::new(2u32, 3u32).unwrap(), "x", 0u32, 25.0f64, 4.0f64, 6.0f64, 0.0f64, Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(vec![]), Some(400), Some("zh-Hans".to_string()), Some(vec![]))).clone(),
    (RubyDecisionInfo::new(TextRange::new(5u32, 6u32).unwrap(), "y", 0u32, 25.0f64, 4.0f64, 6.0f64, 0.0f64, Some(0.0f64), Some(0.0f64), Some(6.0f64), Some(vec![]), Some(400), Some("zh-Hans".to_string()), Some(vec![]))).clone(),
];
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abc", &clusters, &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 30.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutDebugInfo::new(None, Some(vec![]), Some((geometries).clone()), Some(vec![]), Some((rubies).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let positioned = LayoutQueries::layout_queries_positioned_clusters((content).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, positioned[0usize].left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(15.75f64, positioned[0usize].right, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(15.75f64, positioned[1usize].left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(30.0f64, positioned[1usize].right, None).unwrap();
        let glyph_result = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abc", &clusters, &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 30.0f64)).clone(),
], &vec![
    (GlyphRun::new(TextRange::new(0u32, 3u32).unwrap(), "test", vec![
    (Glyph::new(1u32, TextRange::new(0u32, 2u32).unwrap(), 16.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None)).clone(),
    (Glyph::new(2u32, TextRange::new(2u32, 3u32).unwrap(), 8.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None)).clone(),
].to_vec(), 30.0f64, Some(vec![]))).clone(),
], &vec![], &vec![], LayoutDebugInfo::new(None, Some(vec![]), Some((geometries).clone()), Some(vec![]), Some((rubies).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let glyph_positioned = LayoutQueries::layout_queries_positioned_clusters((glyph_result).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, glyph_positioned[0usize].left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.0f64, glyph_positioned[0usize].right, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.0f64, glyph_positioned[1usize].left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(30.0f64, glyph_positioned[1usize].right, None).unwrap();
    });
}

#[test]
fn no_arg_positioned_clusters_walks_every_line() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.noArgPositionedClustersWalksEveryLine", "org.tiqian.core.LayoutQueriesResidualCoverageTest.noArgPositionedClustersWalksEveryLine", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"noArgPositionedClustersWalksEveryLine");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abcd", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"c", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(3u32, 4u32).unwrap(), &"d", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(2u32, 4u32).unwrap(), 2, 3, 20.0f64, 40.0f64, 35.0f64, 0.0f64, 20.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(4u32, 4u32).unwrap(), 2, 1, 40.0f64, 60.0f64, 55.0f64, 0.0f64, 0.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let positioned = LayoutQueries::layout_queries_positioned_clusters((content).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, u32::try_from((positioned.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, positioned[0usize].line_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, positioned[2usize].line_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20.0f64, positioned[3usize].right, None).unwrap();
    });
}

#[test]
fn glyph_ink_bounds_rejects_each_non_finite_edge_independently() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.glyphInkBoundsRejectsEachNonFiniteEdgeIndependently", "org.tiqian.core.LayoutQueriesResidualCoverageTest.glyphInkBoundsRejectsEachNonFiniteEdgeIndependently", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"glyphInkBoundsRejectsEachNonFiniteEdgeIndependently");
        let non_finite_left = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_ink_with_bounds(Rect::new(LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_nan(), 2.0f64, 8.0f64, 4.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(non_finite_left.is_none(), match &(non_finite_left) { None => "-".to_string(), Some(__option23) => __option23.to_string() }.as_str(), None).unwrap();
        let non_finite_top = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_ink_with_bounds(Rect::new(1.0f64, LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_nan(), 8.0f64, 4.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(non_finite_top.is_none(), match &(non_finite_top) { None => "-".to_string(), Some(__option26) => __option26.to_string() }.as_str(), None).unwrap();
        let non_finite_right = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_ink_with_bounds(Rect::new(1.0f64, 2.0f64, LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_nan(), 4.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(non_finite_right.is_none(), match &(non_finite_right) { None => "-".to_string(), Some(__option29) => __option29.to_string() }.as_str(), None).unwrap();
        let non_finite_bottom = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_ink_with_bounds(Rect::new(1.0f64, 2.0f64, 8.0f64, LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_nan())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_null_rendered(non_finite_bottom.is_none(), match &(non_finite_bottom) { None => "-".to_string(), Some(__option32) => __option32.to_string() }.as_str(), None).unwrap();
    });
}

#[test]
fn clearance_takes_the_smaller_side_whichever_segment_owns_it() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.clearanceTakesTheSmallerSideWhicheverSegmentOwnsIt", "org.tiqian.core.LayoutQueriesResidualCoverageTest.clearanceTakesTheSmallerSideWhicheverSegmentOwnsIt", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"clearanceTakesTheSmallerSideWhicheverSegmentOwnsIt");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let weak_first = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 1u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e|
e.into_inner()); __guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(6.0f64)).unwrap(), 0, TextRange::new(0u32, 1u32).unwrap(), 0.0f64, 0.0f64, 10.0f64, 20.0f64, 15.0f64);
        let strong_second = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(1u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e|
e.into_inner()); __guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(2.0f64)).unwrap(), 0, TextRange::new(1u32, 2u32).unwrap(), 10.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64);
        let cleared = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![(weak_first).clone(), (strong_second).clone()]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(9.0f64, cleared[0usize].right, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(11.0f64, cleared[1usize].left, None).unwrap();
        let styled_a = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(1u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e|
e.into_inner()); __guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(4.0f64)).unwrap(), 0, TextRange::new(1u32, 2u32).unwrap(), 10.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64);
        let scan_past = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(1u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::INLINE_CODE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(4.0f64)).unwrap(), 0, TextRange::new(1u32, 2u32).unwrap(), 10.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 1u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(4.0f64)).unwrap(), 0, TextRange::new(0u32, 1u32).unwrap(), 0.0f64, 0.0f64, 10.0f64, 20.0f64, 15.0f64)).clone(),
    (styled_a).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((scan_past.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8.0f64, scan_past[1usize].right, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(12.0f64, scan_past[2usize].left, None).unwrap();
    });
}

#[test]
fn uniform_text_style_policy_resolves_span_style_or_paragraph_style() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.uniformTextStylePolicyResolvesSpanStyleOrParagraphStyle", "org.tiqian.core.LayoutQueriesResidualCoverageTest.uniformTextStylePolicyResolvesSpanStyleOrParagraphStyle", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"uniformTextStylePolicyResolvesSpanStyleOrParagraphStyle");
        let uniform = RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::UniformTextStyle),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap();
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abc", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"c", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 2, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 30.0f64)).clone(),
], &vec![], &vec![
    (TextSpan::new(TextRange::new(1u32, 2u32).unwrap(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(40.0f64))).clone(),
], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let outside = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 1u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), (uniform).clone(), 0, TextRange::new(0u32, 1u32).unwrap(), 0.0f64, 0.0f64, 10.0f64, 20.0f64, 15.0f64)).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(6.19999999999999929f64, outside[0usize].top, None).unwrap();
        let inside = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(1u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), (uniform).clone(), 0, TextRange::new(1u32, 2u32).unwrap(), 10.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64)).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, inside[0usize].top, None).unwrap();
    });
}

#[test]
fn trailing_glue_is_skipped_when_no_cluster_ends_before_the_segment_end() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.trailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEnd", "org.tiqian.core.LayoutQueriesResidualCoverageTest.trailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEnd", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"trailingGlueIsSkippedWhenNoClusterEndsBeforeTheSegmentEnd");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 10.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let out = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 1u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0, TextRange::new(0u32, 1u32).unwrap(), 0.0f64, 0.0f64, 10.0f64, 20.0f64, 15.0f64)).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, out[0usize].right, None).unwrap();
    });
}

#[test]
fn decoration_line_y_without_spans_uses_the_paragraph_style() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.decorationLineYWithoutSpansUsesTheParagraphStyle", "org.tiqian.core.LayoutQueriesResidualCoverageTest.decorationLineYWithoutSpansUsesTheParagraphStyle", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"decorationLineYWithoutSpansUsesTheParagraphStyle");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 2u32).unwrap(), &"ab", 20.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let value = LayoutQueries::layout_queries_rich_text_decoration_line_y((content).clone(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 2u32).unwrap(), (Box::new({ let __guard =
crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())),
RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0,
TextRange::new(0u32, 2u32).unwrap(), 0.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64), 1.0f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.8f64, value, None).unwrap();
    });
}

#[test]
fn word_boundary_for_position_handles_a_non_finite_y() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.wordBoundaryForPositionHandlesANonFiniteY", "org.tiqian.core.LayoutQueriesResidualCoverageTest.wordBoundaryForPositionHandlesANonFiniteY", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"wordBoundaryForPositionHandlesANonFiniteY");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"甲乙", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"甲", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"乙", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 1u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary_for_position((content).clone(), 5.0f64,
LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_nan()).unwrap().as_ref().unwrap().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn supplementary_ideograph_beyond_the_han_ranges_is_its_own_unit() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.supplementaryIdeographBeyondTheHanRangesIsItsOwnUnit", "org.tiqian.core.LayoutQueriesResidualCoverageTest.supplementaryIdeographBeyondTheHanRangesIsItsOwnUnit", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"supplementaryIdeographBeyondTheHanRangesIsItsOwnUnit");
        let text = TestHelpers::test_helpers_surrogate_text(&vec![55424, 56320]);
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(text.as_str(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 2u32).unwrap(), text.as_str(), 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 10.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 2u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary((content).clone(), 0).unwrap().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn plane_four_codepoint_above_the_han_bands_is_its_own_unit() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.planeFourCodepointAboveTheHanBandsIsItsOwnUnit", "org.tiqian.core.LayoutQueriesResidualCoverageTest.planeFourCodepointAboveTheHanBandsIsItsOwnUnit", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"planeFourCodepointAboveTheHanBandsIsItsOwnUnit");
        let text = TestHelpers::test_helpers_surrogate_text(&vec![55552, 56320]);
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(text.as_str(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 2u32).unwrap(), text.as_str(), 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 10.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 2u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary((content).clone(), 0).unwrap().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn nearest_line_search_covers_all_three_distance_arms() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.nearestLineSearchCoversAllThreeDistanceArms", "org.tiqian.core.LayoutQueriesResidualCoverageTest.nearestLineSearchCoversAllThreeDistanceArms", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"nearestLineSearchCoversAllThreeDistanceArms");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abcde", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(4u32, 5u32).unwrap(), &"e", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(4u32, 5u32).unwrap(), 1, 1, 20.0f64, 40.0f64, 35.0f64, 0.0f64, 10.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, LayoutQueries::layout_queries_get_cursor_rect((content).clone(), 2).unwrap().left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, LayoutQueries::layout_queries_get_cursor_rect((content).clone(), 3).unwrap().left, None).unwrap();
    });
}

#[test]
fn rubies_on_other_lines_do_not_affect_this_line_geometry() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.rubiesOnOtherLinesDoNotAffectThisLineGeometry", "org.tiqian.core.LayoutQueriesResidualCoverageTest.rubiesOnOtherLinesDoNotAffectThisLineGeometry", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"rubiesOnOtherLinesDoNotAffectThisLineGeometry");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![
    (RubyDecisionInfo::new(TextRange::new(0u32, 2u32).unwrap(), "zhù", 1u32, 10.0f64, 4.0f64, 6.0f64, 0.0f64, Some(0.0f64), Some(0.0f64), Some(30.0f64), Some(vec![]), Some(400), Some("zh-Hans".to_string()), Some(vec![]))).clone(),
]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]), Some(vec![])), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let positioned = LayoutQueries::layout_queries_positioned_clusters((content).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, positioned[0usize].left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, positioned[0usize].right, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, positioned[1usize].left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20.0f64, positioned[1usize].right, None).unwrap();
    });
}

#[test]
fn background_trailing_edge_picks_the_largest_glyph_advance() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.backgroundTrailingEdgePicksTheLargestGlyphAdvance", "org.tiqian.core.LayoutQueriesResidualCoverageTest.backgroundTrailingEdgePicksTheLargestGlyphAdvance", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"backgroundTrailingEdgePicksTheLargestGlyphAdvance");
        let clusters = vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
];
        let runs = vec![
    (GlyphRun::new(TextRange::new(1u32, 2u32).unwrap(), "test", vec![
    (Glyph::new(1u32, TextRange::new(1u32, 2u32).unwrap(), 5.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None)).clone(),
    (Glyph::new(2u32, TextRange::new(1u32, 2u32).unwrap(), 6.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None)).clone(),
].to_vec(), 10.0f64, Some(vec![]))).clone(),
];
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &clusters, &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &runs, &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let output = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0, TextRange::new(0u32, 2u32).unwrap(), 0.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64)).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.0f64, output[0usize].right, None).unwrap();
    });
}

#[test]
fn background_trailing_edge_keeps_the_first_glyph_when_it_is_largest() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.backgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargest", "org.tiqian.core.LayoutQueriesResidualCoverageTest.backgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargest", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"backgroundTrailingEdgeKeepsTheFirstGlyphWhenItIsLargest");
        let clusters = vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
];
        let runs = vec![
    (GlyphRun::new(TextRange::new(1u32, 2u32).unwrap(), "test", vec![
    (Glyph::new(1u32, TextRange::new(1u32, 2u32).unwrap(), 6.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None)).clone(),
    (Glyph::new(2u32, TextRange::new(1u32, 2u32).unwrap(), 5.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None)).clone(),
].to_vec(), 10.0f64, Some(vec![]))).clone(),
];
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &clusters, &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &runs, &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let output = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 2u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64),
Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0, TextRange::new(0u32, 2u32).unwrap(), 0.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64)).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.0f64, output[0usize].right, None).unwrap();
    });
}

#[test]
fn selection_word_boundary_for_position_prefers_the_closer_later_line() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.selectionWordBoundaryForPositionPrefersTheCloserLaterLine", "org.tiqian.core.LayoutQueriesResidualCoverageTest.selectionWordBoundaryForPositionPrefersTheCloserLaterLine", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"selectionWordBoundaryForPositionPrefersTheCloserLaterLine");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"甲乙丙丁", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"甲", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"乙", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"丙", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(3u32, 4u32).unwrap(), &"丁", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(2u32, 4u32).unwrap(), 2, 3, 40.0f64, 60.0f64, 55.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(2u32, 3u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary_for_position((content).clone(), 5.0f64,
50.0f64).unwrap().as_ref().unwrap().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 1u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary_for_position((content).clone(), 5.0f64,
30.0f64).unwrap().as_ref().unwrap().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 1u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary_for_position((content).clone(), 5.0f64,
-10.0f64).unwrap().as_ref().unwrap().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(0u32, 1u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary_for_position((content).clone(), 5.0f64,
10.0f64).unwrap().as_ref().unwrap().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(2u32, 3u32).unwrap().to_string().as_str(), LayoutQueries::layout_queries_get_selection_word_boundary_for_position((content).clone(), 5.0f64,
100.0f64).unwrap().as_ref().unwrap().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn nearest_line_search_updates_to_a_strictly_closer_later_line() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.nearestLineSearchUpdatesToAStrictlyCloserLaterLine", "org.tiqian.core.LayoutQueriesResidualCoverageTest.nearestLineSearchUpdatesToAStrictlyCloserLaterLine", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"nearestLineSearchUpdatesToAStrictlyCloserLaterLine");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abcde", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(5u32, 6u32).unwrap(), &"e", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32).unwrap(), 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(5u32, 7u32).unwrap(), 1, 1, 20.0f64, 40.0f64, 35.0f64, 10.0f64, 10.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10.0f64, LayoutQueries::layout_queries_get_cursor_rect((content).clone(), 4).unwrap().left, None).unwrap();
    });
}

#[test]
fn nearest_line_search_covers_both_lambda_copies_of_each_arm() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.nearestLineSearchCoversBothLambdaCopiesOfEachArm", "org.tiqian.core.LayoutQueriesResidualCoverageTest.nearestLineSearchCoversBothLambdaCopiesOfEachArm", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"nearestLineSearchCoversBothLambdaCopiesOfEachArm");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abcdefghij", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"c", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(3u32, 4u32).unwrap(), &"d", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(6u32, 7u32).unwrap(), &"g", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(7u32, 8u32).unwrap(), &"h", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(2u32, 4u32).unwrap(), 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(6u32, 8u32).unwrap(), 2, 3, 20.0f64, 40.0f64, 35.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, LayoutQueries::layout_queries_get_cursor_rect((content).clone(), 1).unwrap().left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20.0f64, LayoutQueries::layout_queries_get_cursor_rect((content).clone(), 8).unwrap().left, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(20.0f64, LayoutQueries::layout_queries_get_cursor_rect((content).clone(), 9).unwrap().left, None).unwrap();
    });
}

#[test]
fn uniform_text_style_policy_picks_the_last_matching_span() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.uniformTextStylePolicyPicksTheLastMatchingSpan", "org.tiqian.core.LayoutQueriesResidualCoverageTest.uniformTextStylePolicyPicksTheLastMatchingSpan", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"uniformTextStylePolicyPicksTheLastMatchingSpan");
        let uniform = RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::UniformTextStyle),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap();
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abc", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"c", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 2, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 30.0f64)).clone(),
], &vec![], &vec![
    (TextSpan::new(TextRange::new(0u32, 2u32).unwrap(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64))).clone(),
    (TextSpan::new(TextRange::new(1u32, 3u32).unwrap(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(40.0f64))).clone(),
], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let inside = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(2u32, 3u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), (uniform).clone(), 0, TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 0.0f64, 30.0f64, 20.0f64, 15.0f64)).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, inside[0usize].top, None).unwrap();
    });
}

#[test]
fn decoration_line_y_picks_the_last_matching_span() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.decorationLineYPicksTheLastMatchingSpan", "org.tiqian.core.LayoutQueriesResidualCoverageTest.decorationLineYPicksTheLastMatchingSpan", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"decorationLineYPicksTheLastMatchingSpan");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abc", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"c", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 2, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 30.0f64)).clone(),
], &vec![], &vec![
    (TextSpan::new(TextRange::new(0u32, 2u32).unwrap(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64))).clone(),
    (TextSpan::new(TextRange::new(1u32, 3u32).unwrap(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(20.0f64))).clone(),
], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let value = LayoutQueries::layout_queries_rich_text_decoration_line_y((content).clone(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(2u32, 3u32).unwrap(), (Box::new({ let __guard =
crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())),
RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0,
TextRange::new(2u32, 3u32).unwrap(), 20.0f64, 0.0f64, 30.0f64, 20.0f64, 15.0f64), 1.0f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18.6f64, value, None).unwrap();
    });
}

#[test]
fn uniform_text_style_policy_keeps_the_earlier_span_when_a_later_one_misses() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.uniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMisses", "org.tiqian.core.LayoutQueriesResidualCoverageTest.uniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMisses", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"uniformTextStylePolicyKeepsTheEarlierSpanWhenALaterOneMisses");
        let uniform = RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::UniformTextStyle),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap();
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abc", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"c", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 2, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 30.0f64)).clone(),
], &vec![], &vec![
    (TextSpan::new(TextRange::new(0u32, 3u32).unwrap(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(40.0f64))).clone(),
    (TextSpan::new(TextRange::new(1u32, 2u32).unwrap(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64))).clone(),
], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let output = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 1u32).unwrap(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
__guard.clone() })).clone(), (uniform).clone(), 0, TextRange::new(0u32, 1u32).unwrap(), 0.0f64, 0.0f64, 10.0f64, 20.0f64, 15.0f64)).clone(),
]);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, output[0usize].top, None).unwrap();
    });
}

#[test]
fn decoration_line_y_keeps_the_earlier_span_when_a_later_one_misses() {
    testlib::run("org.tiqian.core.LayoutQueriesResidualCoverageTest.decorationLineYKeepsTheEarlierSpanWhenALaterOneMisses", "org.tiqian.core.LayoutQueriesResidualCoverageTest.decorationLineYKeepsTheEarlierSpanWhenALaterOneMisses", || {
        TestTraceRecorder::new("LayoutQueriesResidualCoverageTest").section(&"decorationLineYKeepsTheEarlierSpanWhenALaterOneMisses");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"abc", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32).unwrap(), &"b", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(2u32, 3u32).unwrap(), &"c", 10.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 2, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 30.0f64)).clone(),
], &vec![], &vec![
    (TextSpan::new(TextRange::new(0u32, 3u32).unwrap(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(20.0f64))).clone(),
    (TextSpan::new(TextRange::new(1u32, 2u32).unwrap(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64))).clone(),
], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64)).unwrap();
        let value = LayoutQueries::layout_queries_rich_text_decoration_line_y((content).clone(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 1u32).unwrap(), (Box::new({ let __guard =
crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() })).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())),
RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap(), 0,
TextRange::new(0u32, 1u32).unwrap(), 0.0f64, 0.0f64, 10.0f64, 20.0f64, 15.0f64), 1.0f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18.6f64, value, None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct LayoutQueriesResidualCoverageTestHelpers;

impl LayoutQueriesResidualCoverageTestHelpers {
    pub fn layout_queries_residual_coverage_test_helpers_render_nullable_source_stops(v: Option<Vec<f64>>) -> String {
        return match &(v) { None => "-".to_string(), Some(__option33) => LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_render_source_stops(&(*__option33).clone()).to_string() };
    }

    pub fn layout_queries_residual_coverage_test_helpers_render_source_stops(v: &[f64]) -> String {
        return {
        let mut out = String::new();
        out.push('[');
        let arr = v;
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    };
    }

    pub fn layout_queries_residual_coverage_test_helpers_cluster(range: TextRange, text: &str, advance: f64) -> Cluster {
        return Cluster::new((range).clone(), text, "test", advance, Some((text).to_string()), Some(0.0f64), Some(0.0f64), Some(0.0f64));
    }

    pub fn layout_queries_residual_coverage_test_helpers_line(range: TextRange, cluster_start: u32, cluster_end: u32, top: f64, bottom: f64, baseline: f64, indent: f64, width: f64) -> LineBox {
        return LineBox::new((range).clone(), IntRange::new(cluster_start, cluster_end), baseline, top, bottom, width, width, width, Some(0.0f64), Some(indent), Some(LineEndReason::ParagraphEnd), Some(0.0f64), Some(vec![]), LineDebugInfo::new(None.clone(), Some(vec![])));
    }

    pub fn layout_queries_residual_coverage_test_helpers_style(font_size: f64) -> TextStyle {
        return TextStyle::new(Some(vec![]), Some(font_size), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0f64), Some(InlineAttachment::None));
    }

    pub fn layout_queries_residual_coverage_test_helpers_empty_debug() -> LayoutDebugInfo {
        return LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None,
None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
    }

    pub fn layout_queries_residual_coverage_test_helpers_result(text: &str, clusters: &Vec<Cluster>, lines: &Vec<LineBox>, glyph_runs: &Vec<GlyphRun>, spans: &Vec<TextSpan>, inline_objects: &Vec<InlineObjectSpan>, debug: LayoutDebugInfo, text_style: TextStyle) ->
Result<LayoutResult, TextRangeError> {
        let content = TiqianTextContent::new(text, Some((spans).clone()), Some(vec![]), Some(vec![]), Some(vec![]));
        let input = LayoutInput::new((content).clone(), Some((text_style).clone()), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some((Ic::zero()).clone()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0f64),
Some(1.0f64), Some(2.0f64))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))),
LayoutConstraints::new(100.0f64, Some(f64::INFINITY), Some(2147483647))?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some((inline_objects).clone()));
        return Ok(LayoutResult::new((input).clone(), Size::new(30.0f64, 40.0f64), (clusters).clone(), (glyph_runs).clone(), (lines).clone(), (debug).clone()));
    }

    pub fn layout_queries_residual_coverage_test_helpers_segment(range: TextRange, role: Box<dyn RichTextRole>, paint: RichTextPaint, line_index: u32, span_range: TextRange, left: f64, top: f64, right: f64, bottom: f64, baseline: f64) -> RichTextLineSegment {
        return RichTextLineSegment::new(RichTextSpan::new((span_range).clone(), role.clone(), (paint).clone()), line_index, (range).clone(), left, top, right, bottom, baseline);
    }

    pub fn layout_queries_residual_coverage_test_helpers_plain_segment(range: TextRange) -> Result<RichTextLineSegment, TextRangeError> {
        return Ok(LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment((range).clone(), (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone()
})).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone())))?, Some(0.0f64))?, 0, (range).clone(), 0.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64));
    }

    pub fn layout_queries_residual_coverage_test_helpers_render_segments(values: &Vec<RichTextLineSegment>) -> Result<String, UStringFault> {
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
                if unit >= 55296 && unit <= 56319 && !TestTraceRender::test_trace_render_cap((values[usize::try_from(index).unwrap_or(0)]).clone().to_string().as_str())?.is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            output.extend(TestTraceRender::test_trace_render_cap((values[usize::try_from(index).unwrap_or(0)]).clone().to_string().as_str())?.encode_utf16());
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

    pub fn layout_queries_residual_coverage_test_helpers_render_rects(values: &Vec<Rect>) -> Result<String, UStringFault> {
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

    pub fn layout_queries_residual_coverage_test_helpers_nan() -> f64 {
        return 0.0f64 / 0.0f64;
    }

    pub fn layout_queries_residual_coverage_test_helpers_metric(range: TextRange, metric_box: &str, ascent: f64, descent: f64, baseline_class: &str) -> MetricDecisionInfo {
        return MetricDecisionInfo::new((range).clone(), "ab", "body", "test", 8.0f64, 2.0f64, 0.0f64, "stub", ascent, descent, baseline_class, metric_box, "normalized", "test");
    }

    pub fn layout_queries_residual_coverage_test_helpers_geometry(range: TextRange, source_text: &str, leading: f64, leading_consumed: f64, trailing: f64, trailing_consumed: f64) -> ClusterGeometryDecisionInfo {
        return ClusterGeometryDecisionInfo::new((range).clone(), source_text, source_text, 10.0f64, (10.0f64 - leading) - trailing, leading, leading_consumed, trailing, trailing_consumed, 0.0f64, 10.0f64, "test", "test", Some(0.0f64), Some(0.0f64), None);
    }

    pub fn layout_queries_residual_coverage_test_helpers_metric_bounds(decision_range: TextRange) -> Result<Vec<f64>, TextRangeError> {
        let decision = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_metric((decision_range).clone(), &"IdeographicEmBox", 7.0f64, 3.0f64, &"ideographic");
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 2u32)?, &"ab", 20.0f64)).clone(),
], &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32)?, 0, 0, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![], &vec![], &vec![], LayoutDebugInfo::new(None, Some(vec![(decision).clone()]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64))?;
        let r#box = LayoutQueries::layout_queries_rich_text_background_segments((content).clone(), &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_segment(TextRange::new(0u32, 2u32)?, (Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone()
})).clone(), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces),
Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone())))?, Some(0.0f64))?, 0, TextRange::new(0u32, 2u32)?, 0.0f64, 0.0f64, 20.0f64, 20.0f64, 15.0f64)).clone(),
]);
        return Ok(vec![r#box[0usize].top, r#box[0usize].bottom]);
    }

    pub fn layout_queries_residual_coverage_test_helpers_ink_with_bounds(bounds: Rect) -> Result<Option<Rect>, TextRangeError> {
        let clusters = vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(0u32, 1u32)?, &"a", 10.0f64)).clone(),
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_cluster(TextRange::new(1u32, 2u32)?, &"b", 10.0f64)).clone(),
];
        let content = LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_result(&"ab", &clusters, &vec![
    (LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_line(TextRange::new(0u32, 2u32)?, 0, 1, 0.0f64, 20.0f64, 15.0f64, 0.0f64, 20.0f64)).clone(),
], &vec![
    (GlyphRun::new(TextRange::new(0u32, 2u32)?, "test", vec![
    (Glyph::new(1u32, TextRange::new(0u32, 1u32)?, 10.0f64, Some(0.0f64), Some(0.0f64), None, Some((bounds).clone()), None, None)).clone(),
].to_vec(), 20.0f64, Some(vec![]))).clone(),
], &vec![], &vec![], LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_empty_debug(), LayoutQueriesResidualCoverageTestHelpers::layout_queries_residual_coverage_test_helpers_style(10.0f64))?;
        return Ok(LayoutQueries::layout_queries_glyph_ink_bounds((content).clone()));
    }
}
