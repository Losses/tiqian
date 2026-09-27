#![cfg(test)]

use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::inline_object_boundary_adjustment::InlineObjectBoundaryAdjustment;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_kind::RubyKind;
use crate::org::tiqian::core::ruby_line_height_decision_info::RubyLineHeightDecisionInfo;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::ruby_span::compare_ruby_span;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::baseline_class::BaselineClass;
use crate::org::tiqian::font::baseline_policy::BaselinePolicy;
use crate::org::tiqian::font::font_metric_source::FontMetricSource;
use crate::org::tiqian::font::font_metrics::FontMetricsRequest;
use crate::org::tiqian::font::font_metrics_policy::FontMetricsPolicy;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::layout_font_metrics::LayoutFontMetrics;
use crate::org::tiqian::font::metric_box::MetricBox;
use crate::org::tiqian::font::raw_font_metrics::RawFontMetrics;
use crate::org::tiqian::layout::annotation_geometry_stage::RubyFontGeometry;
use crate::org::tiqian::layout::line_geometry_stage::ClusterMetricDecision;
use crate::org::tiqian::layout::line_geometry_stage::LineGeometryStageFns;
use crate::org::tiqian::layout::line_geometry_stage::LineVerticalGeometryStageResult;
use crate::org::tiqian::layout::line_geometry_stage::ResolvedLineMetrics;
use crate::org::tiqian::layout::line_optimization::LineCandidate;
use crate::org::tiqian::layout::line_optimization::LineSolution;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use std::collections::HashMap;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub enum LineGeometryDirectTailTestRubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtentsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineGeometryDirectTailTestRubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtentsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineGeometryDirectTailTestRubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtentsFault) -> Self {
        match value {
            LineGeometryDirectTailTestRubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtentsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineGeometryDirectTailTestRubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtentsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineGeometryDirectTailTestRubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtentsFault) -> Self {
        match value {
            LineGeometryDirectTailTestRubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtentsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineGeometryDirectTailTestRubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtentsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineGeometryDirectTailTestRubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtentsFault) -> Self {
        match value {
            LineGeometryDirectTailTestRubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtentsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineGeometryDirectTailTestRubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtentsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineGeometryDirectTailTestRubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtentsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineGeometryDirectTailTestRubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtentsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineGeometryDirectTailTestRubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtentsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineGeometryDirectTailTestRubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtentsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineGeometryDirectTailTestRubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtentsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineGeometryDirectTailTestRubiesOnBothLinesExerciseBothSidesOfTheOverlapTestFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineGeometryDirectTailTestRubiesOnBothLinesExerciseBothSidesOfTheOverlapTestFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineGeometryDirectTailTestRubiesOnBothLinesExerciseBothSidesOfTheOverlapTestFault) -> Self {
        match value {
            LineGeometryDirectTailTestRubiesOnBothLinesExerciseBothSidesOfTheOverlapTestFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineGeometryDirectTailTestRubiesOnBothLinesExerciseBothSidesOfTheOverlapTestFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineGeometryDirectTailTestRubiesOnBothLinesExerciseBothSidesOfTheOverlapTestFault) -> Self {
        match value {
            LineGeometryDirectTailTestRubiesOnBothLinesExerciseBothSidesOfTheOverlapTestFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineGeometryDirectTailTestRubiesOnBothLinesExerciseBothSidesOfTheOverlapTestFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineGeometryDirectTailTestRubiesOnBothLinesExerciseBothSidesOfTheOverlapTestFault) -> Self {
        match value {
            LineGeometryDirectTailTestRubiesOnBothLinesExerciseBothSidesOfTheOverlapTestFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineGeometryDirectTailTestRubiesOnBothLinesExerciseBothSidesOfTheOverlapTestFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineGeometryDirectTailTestRubiesOnBothLinesExerciseBothSidesOfTheOverlapTestFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineGeometryDirectTailTestRubiesOnBothLinesExerciseBothSidesOfTheOverlapTestFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineGeometryDirectTailTestRubiesOnBothLinesExerciseBothSidesOfTheOverlapTestFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineGeometryDirectTailTestRubiesOnBothLinesExerciseBothSidesOfTheOverlapTestFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineGeometryDirectTailTestRubiesOnBothLinesExerciseBothSidesOfTheOverlapTestFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineGeometryDirectTailTestObjectTopIntrusionDominatingRubyDemandAddsBoundaryClearanceFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineGeometryDirectTailTestObjectTopIntrusionDominatingRubyDemandAddsBoundaryClearanceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineGeometryDirectTailTestObjectTopIntrusionDominatingRubyDemandAddsBoundaryClearanceFault) -> Self {
        match value {
            LineGeometryDirectTailTestObjectTopIntrusionDominatingRubyDemandAddsBoundaryClearanceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineGeometryDirectTailTestObjectTopIntrusionDominatingRubyDemandAddsBoundaryClearanceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineGeometryDirectTailTestObjectTopIntrusionDominatingRubyDemandAddsBoundaryClearanceFault) -> Self {
        match value {
            LineGeometryDirectTailTestObjectTopIntrusionDominatingRubyDemandAddsBoundaryClearanceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineGeometryDirectTailTestObjectTopIntrusionDominatingRubyDemandAddsBoundaryClearanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineGeometryDirectTailTestObjectTopIntrusionDominatingRubyDemandAddsBoundaryClearanceFault) -> Self {
        match value {
            LineGeometryDirectTailTestObjectTopIntrusionDominatingRubyDemandAddsBoundaryClearanceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineGeometryDirectTailTestObjectTopIntrusionDominatingRubyDemandAddsBoundaryClearanceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineGeometryDirectTailTestObjectTopIntrusionDominatingRubyDemandAddsBoundaryClearanceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineGeometryDirectTailTestObjectTopIntrusionDominatingRubyDemandAddsBoundaryClearanceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineGeometryDirectTailTestObjectTopIntrusionDominatingRubyDemandAddsBoundaryClearanceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineGeometryDirectTailTestObjectTopIntrusionDominatingRubyDemandAddsBoundaryClearanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineGeometryDirectTailTestObjectTopIntrusionDominatingRubyDemandAddsBoundaryClearanceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineGeometryDirectTailTestObjectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZeroFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineGeometryDirectTailTestObjectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZeroFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineGeometryDirectTailTestObjectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZeroFault) -> Self {
        match value {
            LineGeometryDirectTailTestObjectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZeroFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineGeometryDirectTailTestObjectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZeroFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineGeometryDirectTailTestObjectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZeroFault) -> Self {
        match value {
            LineGeometryDirectTailTestObjectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZeroFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineGeometryDirectTailTestObjectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZeroFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineGeometryDirectTailTestObjectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZeroFault) -> Self {
        match value {
            LineGeometryDirectTailTestObjectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZeroFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineGeometryDirectTailTestObjectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZeroFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineGeometryDirectTailTestObjectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZeroFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineGeometryDirectTailTestObjectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZeroFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineGeometryDirectTailTestObjectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZeroFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineGeometryDirectTailTestObjectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZeroFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineGeometryDirectTailTestObjectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZeroFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineGeometryDirectTailTestObjectFlushWithBaseTopSkipsIntrusionConjunctionEarlyFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineGeometryDirectTailTestObjectFlushWithBaseTopSkipsIntrusionConjunctionEarlyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineGeometryDirectTailTestObjectFlushWithBaseTopSkipsIntrusionConjunctionEarlyFault) -> Self {
        match value {
            LineGeometryDirectTailTestObjectFlushWithBaseTopSkipsIntrusionConjunctionEarlyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineGeometryDirectTailTestObjectFlushWithBaseTopSkipsIntrusionConjunctionEarlyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineGeometryDirectTailTestObjectFlushWithBaseTopSkipsIntrusionConjunctionEarlyFault) -> Self {
        match value {
            LineGeometryDirectTailTestObjectFlushWithBaseTopSkipsIntrusionConjunctionEarlyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineGeometryDirectTailTestObjectFlushWithBaseTopSkipsIntrusionConjunctionEarlyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineGeometryDirectTailTestObjectFlushWithBaseTopSkipsIntrusionConjunctionEarlyFault) -> Self {
        match value {
            LineGeometryDirectTailTestObjectFlushWithBaseTopSkipsIntrusionConjunctionEarlyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineGeometryDirectTailTestObjectFlushWithBaseTopSkipsIntrusionConjunctionEarlyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineGeometryDirectTailTestObjectFlushWithBaseTopSkipsIntrusionConjunctionEarlyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineGeometryDirectTailTestObjectFlushWithBaseTopSkipsIntrusionConjunctionEarlyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineGeometryDirectTailTestObjectFlushWithBaseTopSkipsIntrusionConjunctionEarlyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineGeometryDirectTailTestObjectFlushWithBaseTopSkipsIntrusionConjunctionEarlyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineGeometryDirectTailTestObjectFlushWithBaseTopSkipsIntrusionConjunctionEarlyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineGeometryDirectTailTestMetricListWithoutIdeographicEmBoxFallsBackToAllClustersFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineGeometryDirectTailTestMetricListWithoutIdeographicEmBoxFallsBackToAllClustersFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineGeometryDirectTailTestMetricListWithoutIdeographicEmBoxFallsBackToAllClustersFault) -> Self {
        match value {
            LineGeometryDirectTailTestMetricListWithoutIdeographicEmBoxFallsBackToAllClustersFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineGeometryDirectTailTestMetricListWithoutIdeographicEmBoxFallsBackToAllClustersFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineGeometryDirectTailTestMetricListWithoutIdeographicEmBoxFallsBackToAllClustersFault) -> Self {
        match value {
            LineGeometryDirectTailTestMetricListWithoutIdeographicEmBoxFallsBackToAllClustersFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineGeometryDirectTailTestMetricListWithoutIdeographicEmBoxFallsBackToAllClustersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineGeometryDirectTailTestMetricListWithoutIdeographicEmBoxFallsBackToAllClustersFault) -> Self {
        match value {
            LineGeometryDirectTailTestMetricListWithoutIdeographicEmBoxFallsBackToAllClustersFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineGeometryDirectTailTestMetricListWithoutIdeographicEmBoxFallsBackToAllClustersFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineGeometryDirectTailTestMetricListWithoutIdeographicEmBoxFallsBackToAllClustersFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineGeometryDirectTailTestMetricListWithoutIdeographicEmBoxFallsBackToAllClustersFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineGeometryDirectTailTestMetricListWithoutIdeographicEmBoxFallsBackToAllClustersFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineGeometryDirectTailTestMetricListWithoutIdeographicEmBoxFallsBackToAllClustersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineGeometryDirectTailTestMetricListWithoutIdeographicEmBoxFallsBackToAllClustersFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineGeometryDirectTailTestEmptyLineSolutionYieldsZeroArraysAndZeroMaxExtraFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<LineGeometryDirectTailTestEmptyLineSolutionYieldsZeroArraysAndZeroMaxExtraFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LineGeometryDirectTailTestEmptyLineSolutionYieldsZeroArraysAndZeroMaxExtraFault) -> Self {
        match value {
            LineGeometryDirectTailTestEmptyLineSolutionYieldsZeroArraysAndZeroMaxExtraFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineGeometryDirectTailTestEmptyLineSolutionYieldsZeroArraysAndZeroMaxExtraFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LineGeometryDirectTailTestEmptyLineSolutionYieldsZeroArraysAndZeroMaxExtraFault) -> Self {
        match value {
            LineGeometryDirectTailTestEmptyLineSolutionYieldsZeroArraysAndZeroMaxExtraFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LineGeometryDirectTailTestEmptyLineSolutionYieldsZeroArraysAndZeroMaxExtraFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LineGeometryDirectTailTestEmptyLineSolutionYieldsZeroArraysAndZeroMaxExtraFault) -> Self {
        match value {
            LineGeometryDirectTailTestEmptyLineSolutionYieldsZeroArraysAndZeroMaxExtraFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LineGeometryDirectTailTestEmptyLineSolutionYieldsZeroArraysAndZeroMaxExtraFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LineGeometryDirectTailTestEmptyLineSolutionYieldsZeroArraysAndZeroMaxExtraFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LineGeometryDirectTailTestEmptyLineSolutionYieldsZeroArraysAndZeroMaxExtraFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LineGeometryDirectTailTestEmptyLineSolutionYieldsZeroArraysAndZeroMaxExtraFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LineGeometryDirectTailTestEmptyLineSolutionYieldsZeroArraysAndZeroMaxExtraFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LineGeometryDirectTailTestEmptyLineSolutionYieldsZeroArraysAndZeroMaxExtraFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn ruby_base_range_crossing_cluster_boundaries_drops_out_of_per_line_extents() {
    testlib::run("org.tiqian.layout.LineGeometryDirectTailTest.rubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtents", "org.tiqian.layout.LineGeometryDirectTailTest.rubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtents", || {
        LineGeometryDirectTailSupport::line_geometry_direct_tail_support_start(&"rubyBaseRangeCrossingClusterBoundariesDropsOutOfPerLineExtents");
        let a = LineGeometryDirectTailSupport::line_geometry_direct_tail_support_ruby(TextRange::new(4u32, 8u32).unwrap(), &"y");
        let m = LineGeometryDirectTailSupport::line_geometry_direct_tail_support_ruby(TextRange::new(1u32, 3u32).unwrap(), &"w");
        let z = LineGeometryDirectTailSupport::line_geometry_direct_tail_support_ruby(TextRange::new(0u32, 4u32).unwrap(), &"z");
        let r = LineGeometryDirectTailSupport::line_geometry_direct_tail_support_geometry(Some(vec![(a).clone(), (m).clone(), (z).clone()]), &vec![
    (LineGeometryDirectTailSupport::line_geometry_direct_tail_support_line(IntRange::new(0u32, 3u32), &LineGeometryDirectTailSupport::line_geometry_direct_tail_support_clusters().unwrap()).unwrap()).clone(),
], Some(LineGeometryDirectTailSupport::line_geometry_direct_tail_support_map_ruby(&vec![
    (RubyEntry { s: a, g: LineGeometryDirectTailSupport::line_geometry_direct_tail_support_rg(12 as f64, 8 as f64) }).clone(),
])), None, Some(0.0f64), Some(8.0f64), Some(4.0f64), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((r.line_baseline.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((r.line_baseline[0usize]) > (0 as f64), None).unwrap();
    });
}

#[test]
fn rubies_on_both_lines_exercise_both_sides_of_the_overlap_test() {
    testlib::run("org.tiqian.layout.LineGeometryDirectTailTest.rubiesOnBothLinesExerciseBothSidesOfTheOverlapTest", "org.tiqian.layout.LineGeometryDirectTailTest.rubiesOnBothLinesExerciseBothSidesOfTheOverlapTest", || {
        LineGeometryDirectTailSupport::line_geometry_direct_tail_support_start(&"rubiesOnBothLinesExerciseBothSidesOfTheOverlapTest");
        let a = LineGeometryDirectTailSupport::line_geometry_direct_tail_support_ruby(TextRange::new(0u32, 4u32).unwrap(), &"a");
        let b = LineGeometryDirectTailSupport::line_geometry_direct_tail_support_ruby(TextRange::new(4u32, 8u32).unwrap(), &"b");
        let r = LineGeometryDirectTailSupport::line_geometry_direct_tail_support_geometry(Some(vec![(a).clone(), (b).clone()]), &vec![
    (LineGeometryDirectTailSupport::line_geometry_direct_tail_support_line(IntRange::new(0u32, 1u32), &LineGeometryDirectTailSupport::line_geometry_direct_tail_support_clusters().unwrap()).unwrap()).clone(),
    (LineGeometryDirectTailSupport::line_geometry_direct_tail_support_line(IntRange::new(2u32, 3u32), &LineGeometryDirectTailSupport::line_geometry_direct_tail_support_clusters().unwrap()).unwrap()).clone(),
], Some(LineGeometryDirectTailSupport::line_geometry_direct_tail_support_map_ruby(&vec![
    (RubyEntry { s: a, g: LineGeometryDirectTailSupport::line_geometry_direct_tail_support_rg(12 as f64, 8 as f64) }).clone(),
    (RubyEntry { s: b, g: LineGeometryDirectTailSupport::line_geometry_direct_tail_support_rg(12 as f64, 6 as f64) }).clone(),
])), None, Some(0.0f64), Some(8.0f64), Some(4.0f64), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.line_baseline.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((r.line_baseline[1usize]) > (r.line_baseline[0usize]), None).unwrap();
    });
}

#[test]
fn empty_line_solution_yields_zero_arrays_and_zero_max_extra() {
    testlib::run("org.tiqian.layout.LineGeometryDirectTailTest.emptyLineSolutionYieldsZeroArraysAndZeroMaxExtra", "org.tiqian.layout.LineGeometryDirectTailTest.emptyLineSolutionYieldsZeroArraysAndZeroMaxExtra", || {
        LineGeometryDirectTailSupport::line_geometry_direct_tail_support_start(&"emptyLineSolutionYieldsZeroArraysAndZeroMaxExtra");
        let r = LineGeometryDirectTailSupport::line_geometry_direct_tail_support_geometry(Some(vec![
    (LineGeometryDirectTailSupport::line_geometry_direct_tail_support_ruby(TextRange::new(0u32, 2u32).unwrap(), &"y")).clone(),
]), &vec![], None, None, Some(0.0f64), Some(8.0f64), Some(4.0f64), None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::try_from((r.line_baseline.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::try_from((r.line_top.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, u32::try_from((r.line_bottom.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_not_null_rendered(r.ruby_line_height_decision.is_some(), LineGeometryDirectTailSupport::line_geometry_direct_tail_support_render_nullable_decision((r.ruby_line_height_decision).clone()).as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, r.ruby_line_height_decision.as_ref().unwrap().max_extra, None).unwrap();
    });
}

#[test]
fn object_top_intrusion_below_ruby_demand_keeps_boundary_clearance_zero() {
    testlib::run("org.tiqian.layout.LineGeometryDirectTailTest.objectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZero", "org.tiqian.layout.LineGeometryDirectTailTest.objectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZero", || {
        LineGeometryDirectTailSupport::line_geometry_direct_tail_support_start(&"objectTopIntrusionBelowRubyDemandKeepsBoundaryClearanceZero");
        let r = LineGeometryDirectTailSupport::line_geometry_direct_tail_support_object_case(10 as f64, 8 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.line_baseline.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((r.line_baseline[1usize]) > (r.line_baseline[0usize]), None).unwrap();
    });
}

#[test]
fn object_top_intrusion_dominating_ruby_demand_adds_boundary_clearance() {
    testlib::run("org.tiqian.layout.LineGeometryDirectTailTest.objectTopIntrusionDominatingRubyDemandAddsBoundaryClearance", "org.tiqian.layout.LineGeometryDirectTailTest.objectTopIntrusionDominatingRubyDemandAddsBoundaryClearance", || {
        LineGeometryDirectTailSupport::line_geometry_direct_tail_support_start(&"objectTopIntrusionDominatingRubyDemandAddsBoundaryClearance");
        let r = LineGeometryDirectTailSupport::line_geometry_direct_tail_support_object_case(20 as f64, 8 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.line_baseline.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((r.line_baseline[1usize]) > (r.line_baseline[0usize]), None).unwrap();
    });
}

#[test]
fn object_flush_with_base_top_skips_intrusion_conjunction_early() {
    testlib::run("org.tiqian.layout.LineGeometryDirectTailTest.objectFlushWithBaseTopSkipsIntrusionConjunctionEarly", "org.tiqian.layout.LineGeometryDirectTailTest.objectFlushWithBaseTopSkipsIntrusionConjunctionEarly", || {
        LineGeometryDirectTailSupport::line_geometry_direct_tail_support_start(&"objectFlushWithBaseTopSkipsIntrusionConjunctionEarly");
        let r = LineGeometryDirectTailSupport::line_geometry_direct_tail_support_object_case(8 as f64, 8 as f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((r.line_baseline.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((r.line_baseline[1usize]) > (r.line_baseline[0usize]), None).unwrap();
    });
}

#[test]
fn metric_list_without_ideographic_em_box_falls_back_to_all_clusters() {
    testlib::run("org.tiqian.layout.LineGeometryDirectTailTest.metricListWithoutIdeographicEmBoxFallsBackToAllClusters", "org.tiqian.layout.LineGeometryDirectTailTest.metricListWithoutIdeographicEmBoxFallsBackToAllClusters", || {
        LineGeometryDirectTailSupport::line_geometry_direct_tail_support_start(&"metricListWithoutIdeographicEmBoxFallsBackToAllClusters");
        let q = FontMetricsRequest::new("latin", 16 as f64 as f64, FontRole::LatinText, "zh-Hans", Some(vec![]), Some(400), Some(false), Some("".to_string()));
        let d = ClusterMetricDecision::new(TextRange::new(0u32, 1u32).unwrap(), "a", (q).clone(), RawFontMetrics::new(14 as f64 as f64, 4 as f64 as f64, Some(0 as f64), Some(FontMetricSource::RawTables), None, None), LayoutFontMetrics::new(14 as f64 as f64, 4 as f64 as f64, 0 as
f64 as f64, FontMetricsPolicy::Raw, BaselinePolicy::Alphabetic, Some(BaselineClass::Roman), Some(MetricBox::RawFontBox), Some(FontMetricSource::RawTables), Some("".to_string())));
        let r = LineGeometryStageFns::line_geometry_stage_fns_line_metrics(&vec![(d).clone()], None, 24 as f64, Some(0 as f64 as f64));
        let _ = TracedAssertions::traced_assertions_assert_true((r.baseline) >= 14 as f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((r.height) >= 18 as f64, None).unwrap();
    });
}

#[test]
fn empty_metric_list_takes_empty_paragraph_baseline_fallback() {
    testlib::run("org.tiqian.layout.LineGeometryDirectTailTest.emptyMetricListTakesEmptyParagraphBaselineFallback", "org.tiqian.layout.LineGeometryDirectTailTest.emptyMetricListTakesEmptyParagraphBaselineFallback", || {
        LineGeometryDirectTailSupport::line_geometry_direct_tail_support_start(&"emptyMetricListTakesEmptyParagraphBaselineFallback");
        let a = LineGeometryStageFns::line_geometry_stage_fns_line_metrics(&vec![], None, 24 as f64, Some(0 as f64 as f64));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(24 as f64, a.height, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(18 as f64, a.baseline, None).unwrap();
        let b = LineGeometryStageFns::line_geometry_stage_fns_line_metrics(&vec![], Some(30 as f64 as f64), 24 as f64, Some(0 as f64 as f64));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(30 as f64, b.height, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(22.5f64, b.baseline, None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct LineGeometryDirectTailSupport;

impl LineGeometryDirectTailSupport {
    pub fn line_geometry_direct_tail_support_render_nullable_decision(v: Option<RubyLineHeightDecisionInfo>) -> String {
        return match &(v) { None => "null".to_string(), Some(__option) => LineGeometryDirectTailSupport::line_geometry_direct_tail_support_render_decision(((*__option).clone()).clone()).to_string() };
    }

    pub fn line_geometry_direct_tail_support_render_decision(v: RubyLineHeightDecisionInfo) -> String {
        return v.to_string();
    }

    pub fn line_geometry_direct_tail_support_start(n: &str) {
        TestTraceRecorder::new("LineGeometryDirectTailTest").section(n);
    }

    pub fn line_geometry_direct_tail_support_c(index: u32) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(u32::wrapping_mul(index, 2), u32::wrapping_add(u32::wrapping_mul(index, 2), 2))?, "𠀀", "k", 16.0f64, Some("𠀀".to_string()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn line_geometry_direct_tail_support_clusters() -> Result<Vec<Cluster>, TextRangeError> {
        let mut r: Vec<Cluster> = vec![];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (4) {
            r.push(LineGeometryDirectTailSupport::line_geometry_direct_tail_support_c(i)?);
            i = u32::wrapping_add(i, 1);
        }
        return Ok(r);
    }

    pub fn line_geometry_direct_tail_support_input() -> Result<LayoutInput, TextRangeError> {
        return Ok(LayoutInput::new(TiqianTextContent::new("中文测试", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some((Ic::zero()).clone()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0f64), Some(1.0f64), Some(2.0f64))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320.0f64, Some(f64::INFINITY), Some(2147483647))?,
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])));
    }

    pub fn line_geometry_direct_tail_support_line(range: IntRange, _cs: &Vec<Cluster>) -> Result<LineCandidate, TextRangeError> {
        return Ok(LineCandidate::new((range).clone(), TextRange::new(u32::wrapping_mul(range.start, 2), u32::wrapping_mul(u32::wrapping_add(range.end, 1), 2))?, 32.0f64, 32.0f64, Some(LineEndReason::AutoWrap), None, Some(vec![]),
Some(LineCandidate::line_candidate_empty_hanging()))?);
    }

    pub fn line_geometry_direct_tail_support_map_ruby(entries: &Vec<RubyEntry>) -> SortedMapTable<RubySpan, RubyFontGeometry> {
        let mut b: SortedMapTableBuilder<RubySpan, RubyFontGeometry> = SortedTable::sorted_table_map_builder::<RubySpan, RubyFontGeometry>(Arc::new(compare_ruby_span));
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((entries.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            b.put(&(((entries[usize::try_from(i).unwrap_or(0)]).clone().s).clone()), &(((entries[usize::try_from(i).unwrap_or(0)]).clone().g).clone()));
            i = u32::wrapping_add(i, 1);
        }
        return b.clone().build();
    }

    pub fn line_geometry_direct_tail_support_geometry(spans: Option<Vec<RubySpan>>, lines: &Vec<LineCandidate>, rmap: Option<SortedMapTable<RubySpan, RubyFontGeometry>>, objects: Option<HashMap<u32, InlineObjectSpan>>, existing: Option<f64>, ascent: Option<f64>, descent:
Option<f64>, object_index: Option<u32>, object_span: Option<InlineObjectSpan>) -> Result<LineVerticalGeometryStageResult, TextRangeError> {
        let objects = objects.unwrap_or_else(|| HashMap::new());
        let cs = LineGeometryDirectTailSupport::line_geometry_direct_tail_support_clusters()?;
        let mut inline_objects = objects;
        match &(object_index) {
            Some(__option1) => {
                if object_span.is_some() {
                inline_objects.insert(*__option1, (object_span).as_ref().unwrap().clone());
                }
            }
            None => {
            }
        }
        return Ok(LineGeometryStageFns::line_geometry_stage_fns_resolve_line_vertical_geometry(LineGeometryDirectTailSupport::line_geometry_direct_tail_support_input()?, 16.0f64, &match &(spans) { None => vec![], Some(__option4) => (*__option4).clone() }, &cs,
LineSolution::new(Some((lines).clone()), Some(0 as f64))?, match &(rmap) { None => SortedTable::sorted_table_map_builder::<RubySpan, RubyFontGeometry>(Arc::new(compare_ruby_span)).clone().build(), Some(__option5) => (*__option5).clone() }, (existing).unwrap(),
ResolvedLineMetrics::new(12.0f64, 16.0f64, Some(0 as f64)), 16.0f64, 0.0f64, (inline_objects).clone(), (ascent).unwrap(), (descent).unwrap()));
    }

    pub fn line_geometry_direct_tail_support_ruby(range: TextRange, text: &str) -> RubySpan {
        return RubySpan::new((range).clone(), text, Some(vec![]), RubyKind::Pinyin, None);
    }

    pub fn line_geometry_direct_tail_support_rg(width: f64, required: f64) -> RubyFontGeometry {
        return RubyFontGeometry::new(width, 6.0f64, 2.0f64, required, vec![].to_vec());
    }

    pub fn line_geometry_direct_tail_support_object_case(object_ascent: f64, extent: f64) -> Result<LineVerticalGeometryStageResult, TextRangeError> {
        let r = LineGeometryDirectTailSupport::line_geometry_direct_tail_support_ruby(TextRange::new(4u32, 8u32)?, &"y");
        return Ok(LineGeometryDirectTailSupport::line_geometry_direct_tail_support_geometry(Some(vec![(r).clone()]), &vec![
    (LineGeometryDirectTailSupport::line_geometry_direct_tail_support_line(IntRange::new(0u32, 1u32), &LineGeometryDirectTailSupport::line_geometry_direct_tail_support_clusters()?)?).clone(),
    (LineGeometryDirectTailSupport::line_geometry_direct_tail_support_line(IntRange::new(2u32, 3u32), &LineGeometryDirectTailSupport::line_geometry_direct_tail_support_clusters()?)?).clone(),
], Some(LineGeometryDirectTailSupport::line_geometry_direct_tail_support_map_ruby(&vec![
    (RubyEntry { s: r, g: LineGeometryDirectTailSupport::line_geometry_direct_tail_support_rg(12 as f64, extent) }).clone(),
])), None, Some(0 as f64 as f64), Some(8 as f64 as f64), Some(4 as f64 as f64), Some(2), Some(InlineObjectSpan::new(TextRange::new(4u32, 6u32)?, 16.0f64, object_ascent, 2.0f64, Some((InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed()?).clone()),
Some((InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed()?).clone()))?))?);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RubyEntry {
    pub s: RubySpan,
    pub g: RubyFontGeometry,
}
