#![cfg(test)]

use crate::org::tiqian::core::auto_space_decision_info::AutoSpaceDecisionInfo;
use crate::org::tiqian::core::bopomofo_decision_info::BopomofoDecisionInfo;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::cluster_geometry_decision_info::ClusterGeometryDecisionInfo;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::glyph_run::GlyphRun;
use crate::org::tiqian::core::ic::Ic;
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
use crate::org::tiqian::core::positioned_cluster::PositionedCluster;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::rich_text_background_metric_policy::RichTextBackgroundMetricPolicy;
use crate::org::tiqian::core::rich_text_background_paint::RichTextBackgroundPaint;
use crate::org::tiqian::core::rich_text_corner_radii::RichTextCornerRadii;
use crate::org::tiqian::core::rich_text_line_segment::RichTextLineSegment;
use crate::org::tiqian::core::rich_text_paint::RichTextPaint;
use crate::org::tiqian::core::rich_text_span::RichTextSpan;
use crate::org::tiqian::core::ruby_decision_info::RubyDecisionInfo;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::size::Size;
use crate::org::tiqian::core::source_boundary_bias::SourceBoundaryBias;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::test::test_helpers::TestHelpers;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;


#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault) -> Self {
        match value {
            LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault) -> Self {
        match value {
            LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault) -> Self {
        match value {
            LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestUniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPaddingFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    LayoutQueriesGetSelectionOffsetForPositionFaultFault(crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault),
}
impl std::fmt::Display for LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault::NoSuchElementErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault) -> Self {
        match value {
            LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault) -> Self {
        match value {
            LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault) -> Self {
        match value {
            LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault) -> Self {
        match value {
            LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault> for crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault {
    fn from(value: LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault) -> Self {
        match value {
            LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault> for LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault {
    fn from(value: crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault) -> Self {
        LayoutQueriesTestSupportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundariesFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault) -> Self {
        match value {
            LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault) -> Self {
        match value {
            LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault) -> Self {
        match value {
            LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestSelectionWordBoundaryExpandsLatinButKeepsHanAtomicFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    LayoutQueriesGetSelectionOffsetForPositionFaultFault(crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault),
}
impl std::fmt::Display for LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault::NoSuchElementErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault) -> Self {
        match value {
            LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault) -> Self {
        match value {
            LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault) -> Self {
        match value {
            LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault) -> Self {
        match value {
            LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault> for crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault {
    fn from(value: LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault) -> Self {
        match value {
            LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault> for LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault {
    fn from(value: crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault) -> Self {
        LayoutQueriesTestSelectionHitTestingKeepsSupportedSourceSequencesAtomicFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault) -> Self {
        match value {
            LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault) -> Self {
        match value {
            LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault) -> Self {
        match value {
            LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestRubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlapFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault) -> Self {
        match value {
            LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault) -> Self {
        match value {
            LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault) -> Self {
        match value {
            LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestRichTextSegmentsReusePositionedClusterGeometryAndSplitLinesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault) -> Self {
        match value {
            LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault) -> Self {
        match value {
            LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault) -> Self {
        match value {
            LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestRichTextDecorationTrimsOnlyOuterPunctuationGlueFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault) -> Self {
        match value {
            LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault) -> Self {
        match value {
            LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault) -> Self {
        match value {
            LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestRichTextDecorationKeepsPunctuationGlueInsideItsRangeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault) -> Self {
        match value {
            LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault) -> Self {
        match value {
            LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault) -> Self {
        match value {
            LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestRichTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwiceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault) -> Self {
        match value {
            LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault) -> Self {
        match value {
            LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault) -> Self {
        match value {
            LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestRichTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpaceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault) -> Self {
        match value {
            LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault) -> Self {
        match value {
            LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault) -> Self {
        match value {
            LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestRangeBoxesSplitMultiUnitClustersBySourceRangeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault::NoSuchElementErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault) -> Self {
        match value {
            LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault) -> Self {
        match value {
            LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault) -> Self {
        match value {
            LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault) -> Self {
        match value {
            LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOriginFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault::NoSuchElementErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault) -> Self {
        match value {
            LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault) -> Self {
        match value {
            LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault) -> Self {
        match value {
            LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault) -> Self {
        match value {
            LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestPositionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOriginFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault) -> Self {
        match value {
            LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault) -> Self {
        match value {
            LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault) -> Self {
        match value {
            LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestPositionedClustersFollowLineIndentAndAdvanceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault) -> Self {
        match value {
            LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault) -> Self {
        match value {
            LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault) -> Self {
        match value {
            LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestLineThroughBisectsTheIdeographicMetricBoxFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault::NoSuchElementErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault) -> Self {
        match value {
            LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault) -> Self {
        match value {
            LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault) -> Self {
        match value {
            LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault) -> Self {
        match value {
            LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestLineAndBoxQueriesUseTiqianLineGeometryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault) -> Self {
        match value {
            LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault) -> Self {
        match value {
            LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault) -> Self {
        match value {
            LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestInlineObjectSourceRangeIsOneSelectionUnitFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault) -> Self {
        match value {
            LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault) -> Self {
        match value {
            LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault) -> Self {
        match value {
            LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestHitTestingChoosesOffsetFromTiqianClusterAdvancesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault) -> Self {
        match value {
            LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault) -> Self {
        match value {
            LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault) -> Self {
        match value {
            LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestGlyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault) -> Self {
        match value {
            LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault) -> Self {
        match value {
            LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault) -> Self {
        match value {
            LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestExternalSelectionOffsetsRespectDirectionalBoundaryBiasFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault) -> Self {
        match value {
            LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault) -> Self {
        match value {
            LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault) -> Self {
        match value {
            LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestCustomLineStylesReuseTheRendererUnderlineHeightFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault) -> Self {
        match value {
            LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault) -> Self {
        match value {
            LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault) -> Self {
        match value {
            LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestClipboardProjectionRestoresSourceAndAddsFullySelectedAnnotationsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault) -> Self {
        match value {
            LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault) -> Self {
        match value {
            LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault) -> Self {
        match value {
            LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestBackgroundContinuationRadiusDefaultsToTheAuthoredCornerRadiusFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault) -> Self {
        match value {
            LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault) -> Self {
        match value {
            LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault) -> Self {
        match value {
            LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestBackgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRoundedFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault) -> Self {
        match value {
            LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault) -> Self {
        match value {
            LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault) -> Self {
        match value {
            LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestAdjacentLineDecorationsWithTheSameStyleShareOneClearanceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault) -> Self {
        match value {
            LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault) -> Self {
        match value {
            LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault) -> Self {
        match value {
            LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestAdjacentBackgroundsWithTheSameStyleShareOneClearanceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault) -> Self {
        match value {
            LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault) -> Self {
        match value {
            LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault) -> Self {
        match value {
            LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        LayoutQueriesTestAdjacentBackgroundAndUnderlineDoNotAvoidAcrossStylesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn clipboard_projection_restores_source_and_adds_fully_selected_annotations() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.clipboardProjectionRestoresSourceAndAddsFullySelectedAnnotations", "org.tiqian.core.LayoutQueriesTest.clipboardProjectionRestoresSourceAndAddsFullySelectedAnnotations", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[99,108,105,112,98,111,97,114,100,80,114,111,106,101,99,116,105,111,110,82,101,115,116,111,114,101,115,83,111,117,114,99,101,65,110,100,65,100,100,115,70,117,108,108,121,83,101,108,101,99,116,101,100,65,110,110,111,116,97,116,105,111,110,115]));
        let text = UString::from("提椠与您").to_ustring();
        let ruby = RubyDecisionInfo::new(TextRange::new(0u32, 2u32).unwrap(), &(UStr::new(&[116,237,113,105,224,110])), 0u32, 0.0f64, 0.0f64, 8.0f64, 0.0f64, Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(vec![]), Some(400), Some(UString::from("zh-Hans")), Some(vec![]));
        let bopomofo = BopomofoDecisionInfo::new(TextRange::new(3u32, 4u32).unwrap(), &(UStr::new(&[12555,12583,12579,714])), 0u32, vec![].to_vec(), Some(vec![]), Some(400), Some(UString::from("zh-Hans")));
        let debug = LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(ruby).clone()]), Some(vec![(bopomofo).clone()]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_result(text.as_ustr(), 200.0f64, None, Size::new(0.0f64, 0.0f64), &vec![], &vec![], &vec![], Some((debug).clone()), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[25552,26912,65288,116,237,113,105,224,110,65289,19982,24744,65288,12555,12583,12579,714,65289]), LayoutQueries::layout_queries_get_text_for_copy((result).clone(), TextRange::new(0u32, 4u32).unwrap()).unwrap().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[25552]), LayoutQueries::layout_queries_get_text_for_copy((result).clone(), TextRange::new(0u32, 1u32).unwrap()).unwrap().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[25552,26912,65288,116,237,113,105,224,110,65289]), LayoutQueries::layout_queries_get_text_for_copy((result).clone(), TextRange::new(0u32, 2u32).unwrap()).unwrap().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[24744,65288,12555,12583,12579,714,65289]), LayoutQueries::layout_queries_get_text_for_copy((result).clone(), TextRange::new(3u32, 4u32).unwrap()).unwrap().as_ustr(), None).unwrap();
    });
}

#[test]
fn positioned_clusters_follow_line_indent_and_advance() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.positionedClustersFollowLineIndentAndAdvance", "org.tiqian.core.LayoutQueriesTest.positionedClustersFollowLineIndentAndAdvance", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[112,111,115,105,116,105,111,110,101,100,67,108,117,115,116,101,114,115,70,111,108,108,111,119,76,105,110,101,73,110,100,101,110,116,65,110,100,65,100,118,97,110,99,101]));
        let positions = LayoutQueries::layout_queries_positioned_clusters(LayoutQueriesTestHelpers::layout_queries_test_helpers_sample_result().unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(4.0f64, 0.0f64, 14.0f64, 20.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", (positions[0usize]).clone().get_rect().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(14.0f64, 0.0f64, 34.0f64, 20.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", (positions[1usize]).clone().get_rect().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(0.0f64, 20.0f64, 10.0f64, 40.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", (positions[2usize]).clone().get_rect().to_string()).as_str()).as_ustr(), None).unwrap();
    });
}

#[test]
fn positioned_clusters_separate_occupied_box_from_auto_space_draw_origin() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.positionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOrigin", "org.tiqian.core.LayoutQueriesTest.positionedClustersSeparateOccupiedBoxFromAutoSpaceDrawOrigin", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[112,111,115,105,116,105,111,110,101,100,67,108,117,115,116,101,114,115,83,101,112,97,114,97,116,101,79,99,99,117,112,105,101,100,66,111,120,70,114,111,109,65,117,116,111,83,112,97,99,101,68,114,97,119,79,114,105,103,105,110]));
        let decision = AutoSpaceDecisionInfo::new(TextRange::new(1u32, 3u32).unwrap(), &(UStr::new(&[108,101,97,100,105,110,103])), &(UStr::new(&[67,106,107,76,97,116,105,110])), &(UStr::new(&[73,110,115,101,114,116])), 1u32, -2.5f64, -2.5f64, &(UStr::new(&[84,101,120,116,65,117,116,111,83,112,97,99,101,73,110,115,101,114,116,58,105,100,101,111,103,114,97,112,104,45,97,108,112,104,97,58,113,117,97,114,116,101,114,45,101,109])));
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_result(UStr::new(&[20013,72,105]), 40.0f64, Some(LayoutQueriesTestHelpers::layout_queries_test_helpers_style(10.0f64)), Size::new(32.5f64, 20.0f64), &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), UStr::new(&[20013]), UStr::new(&[99,106,107]), 10.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(1u32, 3u32).unwrap(), UStr::new(&[72,105]), UStr::new(&[108,97,116,105,110]), 22.5f64)).clone(),
], &vec![], &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_line(TextRange::new(0u32, 3u32).unwrap(), 0, 1, 15.0f64, 0.0f64, 20.0f64, 32.5f64, 32.5f64, 32.5f64, None)).clone(),
],
Some(LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![(decision).clone()]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))), None).unwrap();
        let positions = LayoutQueries::layout_queries_positioned_clusters((result).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(10.0f64, 0.0f64, 32.5f64, 20.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", (positions[1usize]).clone().get_rect().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(12.5f64, positions[1usize].draw_x, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(10.0f64, 0.0f64, 32.5f64, 20.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", LayoutQueries::layout_queries_get_bounding_box((result).clone(), 1).unwrap().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, LayoutQueries::layout_queries_get_offset_for_position((result).clone(), 11.0f64, 5.0f64), None).unwrap();
    });
}

#[test]
fn positioned_clusters_separate_occupied_box_from_consumed_leading_glue_draw_origin() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.positionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOrigin", "org.tiqian.core.LayoutQueriesTest.positionedClustersSeparateOccupiedBoxFromConsumedLeadingGlueDrawOrigin", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[112,111,115,105,116,105,111,110,101,100,67,108,117,115,116,101,114,115,83,101,112,97,114,97,116,101,79,99,99,117,112,105,101,100,66,111,120,70,114,111,109,67,111,110,115,117,109,101,100,76,101,97,100,105,110,103,71,108,117,101,68,114,97,119,79,114,105,103,105,110]));
        let geometry = LayoutQueriesTestHelpers::layout_queries_test_helpers_punctuation_geometry(TextRange::new(0u32, 1u32).unwrap(), UStr::new(&[65288]), 4.0f64, 0.0f64, 4.0f64, 4.0f64);
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_result(UStr::new(&[65288]), 10.0f64, Some(LayoutQueriesTestHelpers::layout_queries_test_helpers_style(10.0f64)), Size::new(6.0f64, 20.0f64), &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(0u32, 1u32).unwrap(), UStr::new(&[65288]), UStr::new(&[99,106,107]), 6.0f64)).clone(),
], &vec![], &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_line(TextRange::new(0u32, 1u32).unwrap(), 0, 0, 15.0f64, 0.0f64, 20.0f64, 6.0f64, 6.0f64, 6.0f64, None)).clone(),
],
Some(LayoutDebugInfo::new(None, Some(vec![]), Some(vec![(geometry).clone()]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))), None).unwrap();
        let positioned = LayoutQueries::layout_queries_positioned_clusters((result).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(0.0f64, 0.0f64, 6.0f64, 20.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", (positioned[0usize]).clone().get_rect().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(-4.0f64, positioned[0usize].draw_x, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(0.0f64, 0.0f64, 1.0f64, 20.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", LayoutQueries::layout_queries_get_cursor_rect((result).clone(), 0).unwrap().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_offset_for_position((result).clone(), -3.0f64, 5.0f64), None).unwrap();
    });
}

#[test]
fn glyph_ink_bounds_keep_italic_overhang_separate_from_occupied_geometry() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.glyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometry", "org.tiqian.core.LayoutQueriesTest.glyphInkBoundsKeepItalicOverhangSeparateFromOccupiedGeometry", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[103,108,121,112,104,73,110,107,66,111,117,110,100,115,75,101,101,112,73,116,97,108,105,99,79,118,101,114,104,97,110,103,83,101,112,97,114,97,116,101,70,114,111,109,79,99,99,117,112,105,101,100,71,101,111,109,101,116,114,121]));
        let glyph_range = TextRange::new(0u32, 1u32).unwrap();
        let glyph = Glyph::new(1u32, (glyph_range).clone(), 10.0f64, Some(0.0f64), Some(0.0f64), None, Some(Rect::new(-3.0f64, -9.0f64, 12.0f64, 2.0f64)), None, None);
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_result(UStr::new(&[102]), 10.0f64, Some(LayoutQueriesTestHelpers::layout_queries_test_helpers_style(10.0f64)), Size::new(10.0f64, 20.0f64), &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster((glyph_range).clone(), UStr::new(&[102]), UStr::new(&[108,97,116,105,110]), 10.0f64)).clone(),
], &vec![
    (GlyphRun::new((glyph_range).clone(), &(UStr::new(&[108,97,116,105,110])), vec![(glyph).clone()].to_vec(), 10.0f64, Some(vec![]))).clone(),
], &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_line((glyph_range).clone(), 0, 0, 14.0f64, 0.0f64, 20.0f64, 10.0f64, 10.0f64, 10.0f64, None)).clone(),
], None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(0.0f64, 0.0f64, 10.0f64, 20.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", (LayoutQueries::layout_queries_positioned_clusters((result).clone())[0usize]).clone().get_rect().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(-3.0f64, 5.0f64, 12.0f64, 16.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", LayoutQueries::layout_queries_glyph_ink_bounds((result).clone()).as_ref().unwrap().to_string()).as_str()).as_ustr(), None).unwrap();
    });
}

#[test]
fn line_and_box_queries_use_tiqian_line_geometry() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.lineAndBoxQueriesUseTiqianLineGeometry", "org.tiqian.core.LayoutQueriesTest.lineAndBoxQueriesUseTiqianLineGeometry", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[108,105,110,101,65,110,100,66,111,120,81,117,101,114,105,101,115,85,115,101,84,105,113,105,97,110,76,105,110,101,71,101,111,109,101,116,114,121]));
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_sample_result().unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_line_for_offset((result).clone(), 1), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, LayoutQueries::layout_queries_get_line_for_offset((result).clone(), 3), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(14.0f64, 0.0f64, 34.0f64, 20.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", LayoutQueries::layout_queries_get_bounding_box((result).clone(), 1).unwrap().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(10.0f64, 20.0f64, 11.0f64, 40.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", LayoutQueries::layout_queries_get_cursor_rect((result).clone(), 4).unwrap().to_string()).as_str()).as_ustr(), None).unwrap();
    });
}

#[test]
fn range_boxes_split_multi_unit_clusters_by_source_range() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.rangeBoxesSplitMultiUnitClustersBySourceRange", "org.tiqian.core.LayoutQueriesTest.rangeBoxesSplitMultiUnitClustersBySourceRange", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[114,97,110,103,101,66,111,120,101,115,83,112,108,105,116,77,117,108,116,105,85,110,105,116,67,108,117,115,116,101,114,115,66,121,83,111,117,114,99,101,82,97,110,103,101]));
        let boxes = LayoutQueries::layout_queries_get_bounding_boxes(LayoutQueriesTestHelpers::layout_queries_test_helpers_sample_result().unwrap(), TextRange::new(2u32, 4u32).unwrap());
        let expected = vec![
    (Rect::new(24.0f64, 0.0f64, 34.0f64, 20.0f64)).clone(),
    (Rect::new(0.0f64, 20.0f64, 10.0f64, 40.0f64)).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(LayoutQueriesTestHelpers::layout_queries_test_helpers_render_rects(&expected).unwrap().as_ustr(), LayoutQueriesTestHelpers::layout_queries_test_helpers_render_rects(&boxes).unwrap().as_ustr(), None).unwrap();
    });
}

#[test]
fn rich_text_segments_reuse_positioned_cluster_geometry_and_split_lines() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.richTextSegmentsReusePositionedClusterGeometryAndSplitLines", "org.tiqian.core.LayoutQueriesTest.richTextSegmentsReusePositionedClusterGeometryAndSplitLines", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[114,105,99,104,84,101,120,116,83,101,103,109,101,110,116,115,82,101,117,115,101,80,111,115,105,116,105,111,110,101,100,67,108,117,115,116,101,114,71,101,111,109,101,116,114,121,65,110,100,83,112,108,105,116,76,105,110,101,115]));
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_sample_result().unwrap();
        let paint = RichTextPaint::rich_text_paint_with_argb(872349696).unwrap();
        let span = RichTextSpan::new(TextRange::new(1u32, 4u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), (paint).clone());
        let segments = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &vec![(span).clone()]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((segments.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", TextRange::new(1u32, 3u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", ((segments[0usize]).clone().range).clone().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(14.0f64, 0.0f64, 34.0f64, 20.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", (segments[0usize]).clone().get_rect().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", TextRange::new(3u32, 4u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", ((segments[1usize]).clone().range).clone().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(0.0f64, 20.0f64, 10.0f64, 40.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", (segments[1usize]).clone().get_rect().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", span.to_string()).as_str()).as_ustr(), UString::from(format!("{}", ((segments[0usize]).clone().span).clone().to_string()).as_str()).as_ustr(), None).unwrap();
    });
}

#[test]
fn rich_text_decoration_trims_only_outer_punctuation_glue() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.richTextDecorationTrimsOnlyOuterPunctuationGlue", "org.tiqian.core.LayoutQueriesTest.richTextDecorationTrimsOnlyOuterPunctuationGlue", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[114,105,99,104,84,101,120,116,68,101,99,111,114,97,116,105,111,110,84,114,105,109,115,79,110,108,121,79,117,116,101,114,80,117,110,99,116,117,97,116,105,111,110,71,108,117,101]));
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_punctuation_glue_result().unwrap();
        let underline = RichTextSpan::new(TextRange::new(0u32, 4u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap());
        let occupied = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &vec![(underline).clone()]).unwrap();
        let decorations = LayoutQueries::layout_queries_trimmed_rich_text_decoration_segments((result).clone(), &occupied);
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(0.0f64, 0.0f64, 40.0f64, 20.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", (occupied[0usize]).clone().get_rect().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(5.0f64, 0.0f64, 35.0f64, 20.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", (decorations[0usize]).clone().get_rect().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", TextRange::new(0u32, 4u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", ((decorations[0usize]).clone().range).clone().to_string()).as_str()).as_ustr(), None).unwrap();
    });
}

#[test]
fn rich_text_decoration_keeps_punctuation_glue_inside_its_range() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.richTextDecorationKeepsPunctuationGlueInsideItsRange", "org.tiqian.core.LayoutQueriesTest.richTextDecorationKeepsPunctuationGlueInsideItsRange", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[114,105,99,104,84,101,120,116,68,101,99,111,114,97,116,105,111,110,75,101,101,112,115,80,117,110,99,116,117,97,116,105,111,110,71,108,117,101,73,110,115,105,100,101,73,116,115,82,97,110,103,101]));
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_punctuation_glue_result().unwrap();
        let underline = RichTextSpan::new(TextRange::new(1u32, 4u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap());
        let occupied = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &vec![(underline).clone()]).unwrap();
        let decorations = LayoutQueries::layout_queries_trimmed_rich_text_decoration_segments((result).clone(), &occupied);
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(10.0f64, 0.0f64, 35.0f64, 20.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", (decorations[0usize]).clone().get_rect().to_string()).as_str()).as_ustr(), None).unwrap();
    });
}

#[test]
fn rich_text_decoration_does_not_trim_already_consumed_opening_glue_twice() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.richTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwice", "org.tiqian.core.LayoutQueriesTest.richTextDecorationDoesNotTrimAlreadyConsumedOpeningGlueTwice", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[114,105,99,104,84,101,120,116,68,101,99,111,114,97,116,105,111,110,68,111,101,115,78,111,116,84,114,105,109,65,108,114,101,97,100,121,67,111,110,115,117,109,101,100,79,112,101,110,105,110,103,71,108,117,101,84,119,105,99,101]));
        let original = LayoutQueriesTestHelpers::layout_queries_test_helpers_punctuation_glue_result().unwrap();
        let mut geometry: Vec<ClusterGeometryDecisionInfo> = vec![];
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from(((original.debug).clone().geometry_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let decision = ((original.debug).clone().geometry_decisions[usize::try_from(index).unwrap_or(0)]).clone();
            if LayoutQueriesTestHelpers::layout_queries_test_helpers_same_range((decision.range).clone(), TextRange::new(0u32, 1u32).unwrap()) {
                geometry.push(ClusterGeometryDecisionInfo::new((decision.range).clone(), (decision.source_text).to_ustring().as_ustr(), (decision.display_text).to_ustring().as_ustr(), decision.base_advance, decision.body_width, decision.leading_glue_natural, decision.leading_glue_natural, decision.trailing_glue_natural, decision.trailing_glue_consumed, decision.justification_delta, decision.resolved_advance, (decision.source).to_ustring().as_ustr(), (decision.reason).to_ustring().as_ustr(), Some(decision.ruby_spread), Some(decision.glyph_inline_shift), decision.glyph_placement_reason.clone()));
            } else {
                geometry.push(decision.clone());
            }
            index = u32::wrapping_add(index, 1);
        }
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_copy_result_with_debug((original).clone(), LayoutDebugInfo::new(None, Some(vec![]), Some((geometry).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])));
        let underline = RichTextSpan::new(TextRange::new(0u32, 1u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap());
        let occupied = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &vec![(underline).clone()]).unwrap();
        let decorations = LayoutQueries::layout_queries_trimmed_rich_text_decoration_segments((result).clone(), &occupied);
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, decorations[0usize].left, None).unwrap();
    });
}

#[test]
fn custom_line_styles_reuse_the_renderer_underline_height() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.customLineStylesReuseTheRendererUnderlineHeight", "org.tiqian.core.LayoutQueriesTest.customLineStylesReuseTheRendererUnderlineHeight", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[99,117,115,116,111,109,76,105,110,101,83,116,121,108,101,115,82,101,117,115,101,84,104,101,82,101,110,100,101,114,101,114,85,110,100,101,114,108,105,110,101,72,101,105,103,104,116]));
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_punctuation_glue_result().unwrap();
        let underline = RichTextSpan::new(TextRange::new(0u32, 4u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap());
        let occupied = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &vec![(underline).clone()]).unwrap();
        let segment = LayoutQueries::layout_queries_trimmed_rich_text_decoration_segments((result).clone(), &occupied);
        let expected = segment[0usize].baseline + ((result.input).clone().text_style).clone().font_size * 0.18f64;
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(expected, LayoutQueries::layout_queries_rich_text_decoration_line_y((result).clone(), (segment[0usize]).clone(), 1.0f64).unwrap(), 0.001f64, None).unwrap();
    });
}

#[test]
fn line_through_bisects_the_ideographic_metric_box() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.lineThroughBisectsTheIdeographicMetricBox", "org.tiqian.core.LayoutQueriesTest.lineThroughBisectsTheIdeographicMetricBox", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[108,105,110,101,84,104,114,111,117,103,104,66,105,115,101,99,116,115,84,104,101,73,100,101,111,103,114,97,112,104,105,99,77,101,116,114,105,99,66,111,120]));
        let original = LayoutQueriesTestHelpers::layout_queries_test_helpers_background_geometry_result().unwrap();
        let metric = LayoutQueriesTestHelpers::layout_queries_test_helpers_background_metric(TextRange::new(0u32, 3u32).unwrap(), UStr::new(&[73,100,101,111,103,114,97,112,104,105,99,69,109,66,111,120]), 8.0f64, 2.0f64);
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_copy_result_with_debug((original).clone(), LayoutDebugInfo::new(None, Some(vec![(metric).clone()]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])));
        let line_through = RichTextSpan::new(TextRange::new(0u32, 3u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::LINE_THROUGH_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap());
        let occupied = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &vec![(line_through).clone()]).unwrap();
        let segment = LayoutQueries::layout_queries_trimmed_rich_text_decoration_segments((result).clone(), &occupied);
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(17.0f64, LayoutQueries::layout_queries_rich_text_decoration_line_y((result).clone(), (segment[0usize]).clone(), 1.0f64).unwrap(), 0.001f64, None).unwrap();
    });
}

#[test]
fn rich_text_background_keeps_internal_gaps_but_trims_its_outer_layout_space() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.richTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpace", "org.tiqian.core.LayoutQueriesTest.richTextBackgroundKeepsInternalGapsButTrimsItsOuterLayoutSpace", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[114,105,99,104,84,101,120,116,66,97,99,107,103,114,111,117,110,100,75,101,101,112,115,73,110,116,101,114,110,97,108,71,97,112,115,66,117,116,84,114,105,109,115,73,116,115,79,117,116,101,114,76,97,121,111,117,116,83,112,97,99,101]));
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_background_geometry_result().unwrap();
        let full = RichTextSpan::new(TextRange::new(0u32, 3u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap());
        let final_character = RichTextSpan::new(TextRange::new(2u32, 3u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(0.0f64)).unwrap());
        let full_occupied = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &vec![(full).clone()]).unwrap();
        let final_occupied = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &vec![(final_character).clone()]).unwrap();
        let full_segment = LayoutQueries::layout_queries_rich_text_background_segments((result).clone(), &full_occupied);
        let final_segment = LayoutQueries::layout_queries_rich_text_background_segments((result).clone(), &final_occupied);
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(0.0f64, 11.2f64, 29.0f64, 21.2f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", (full_segment[0usize]).clone().get_rect().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(19.0f64, 11.2f64, 29.0f64, 21.2f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", (final_segment[0usize]).clone().get_rect().to_string()).as_str()).as_ustr(), None).unwrap();
    });
}

#[test]
fn uniform_text_style_background_ignores_fallback_face_height_and_adds_padding() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.uniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPadding", "org.tiqian.core.LayoutQueriesTest.uniformTextStyleBackgroundIgnoresFallbackFaceHeightAndAddsPadding", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[117,110,105,102,111,114,109,84,101,120,116,83,116,121,108,101,66,97,99,107,103,114,111,117,110,100,73,103,110,111,114,101,115,70,97,108,108,98,97,99,107,70,97,99,101,72,101,105,103,104,116,65,110,100,65,100,100,115,80,97,100,100,105,110,103]));
        let original = LayoutQueriesTestHelpers::layout_queries_test_helpers_background_geometry_result().unwrap();
        let metrics = vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_background_metric(TextRange::new(0u32, 1u32).unwrap(), UStr::new(&[73,100,101,111,103,114,97,112,104,105,99,69,109,66,111,120]), 8.0f64, 2.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_background_metric(TextRange::new(1u32, 3u32).unwrap(), UStr::new(&[82,97,119,70,111,110,116,66,111,120]), 12.0f64, 4.0f64)).clone(),
];
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_copy_result_with_debug((original).clone(), LayoutDebugInfo::new(None, Some((metrics).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])));
        let background = RichTextBackgroundPaint::new(Some(0.0f64), Some(1.0f64), Some(2.0f64), Some(2.0f64), Some(RichTextBackgroundMetricPolicy::UniformTextStyle), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap();
        let paint = RichTextPaint::rich_text_paint_with_background((background).clone()).unwrap();
        let first = RichTextSpan::new(TextRange::new(0u32, 1u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), (paint).clone());
        let mixed = RichTextSpan::new(TextRange::new(0u32, 3u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), (paint).clone());
        let occupied = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &vec![(first).clone(), (mixed).clone()]).unwrap();
        let segments = LayoutQueries::layout_queries_rich_text_background_segments((result).clone(), &occupied);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((segments.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(11.0f64, segments[0usize].top, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(23.0f64, segments[0usize].bottom, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(segments[0usize].top, segments[1usize].top, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(segments[0usize].bottom, segments[1usize].bottom, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(2.0f64, ((((segments[0usize]).clone().span).clone().paint).clone().background).clone().corner_radius, None).unwrap();
    });
}

#[test]
fn background_continuation_corners_keep_only_true_source_ends_fully_rounded() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.backgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRounded", "org.tiqian.core.LayoutQueriesTest.backgroundContinuationCornersKeepOnlyTrueSourceEndsFullyRounded", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[98,97,99,107,103,114,111,117,110,100,67,111,110,116,105,110,117,97,116,105,111,110,67,111,114,110,101,114,115,75,101,101,112,79,110,108,121,84,114,117,101,83,111,117,114,99,101,69,110,100,115,70,117,108,108,121,82,111,117,110,100,101,100]));
        let background = RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(3.0f64), Some(1.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap();
        let span = RichTextSpan::new(TextRange::new(0u32, 12u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::INLINE_CODE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), RichTextPaint::rich_text_paint_with_background((background).clone()).unwrap());
        let first = LayoutQueriesTestHelpers::layout_queries_test_helpers_segment_for((span).clone(), 0, 4).unwrap();
        let middle = LayoutQueriesTestHelpers::layout_queries_test_helpers_segment_for((span).clone(), 4, 8).unwrap();
        let last = LayoutQueriesTestHelpers::layout_queries_test_helpers_segment_for((span).clone(), 8, 12).unwrap();
        let whole = LayoutQueriesTestHelpers::layout_queries_test_helpers_segment_for((span).clone(), 0, 12).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", RichTextCornerRadii::new(3.0f64, 1.0f64, 1.0f64, 3.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", LayoutQueries::layout_queries_resolved_background_corner_radii((first).clone(), 0.0f64).unwrap().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", RichTextCornerRadii::new(1.0f64, 1.0f64, 1.0f64, 1.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", LayoutQueries::layout_queries_resolved_background_corner_radii((middle).clone(), 0.0f64).unwrap().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", RichTextCornerRadii::new(1.0f64, 3.0f64, 3.0f64, 1.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", LayoutQueries::layout_queries_resolved_background_corner_radii((last).clone(), 0.0f64).unwrap().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", RichTextCornerRadii::new(3.0f64, 3.0f64, 3.0f64, 3.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", LayoutQueries::layout_queries_resolved_background_corner_radii((whole).clone(), 0.0f64).unwrap().to_string()).as_str()).as_ustr(), None).unwrap();
    });
}

#[test]
fn background_continuation_radius_defaults_to_the_authored_corner_radius() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.backgroundContinuationRadiusDefaultsToTheAuthoredCornerRadius", "org.tiqian.core.LayoutQueriesTest.backgroundContinuationRadiusDefaultsToTheAuthoredCornerRadius", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[98,97,99,107,103,114,111,117,110,100,67,111,110,116,105,110,117,97,116,105,111,110,82,97,100,105,117,115,68,101,102,97,117,108,116,115,84,111,84,104,101,65,117,116,104,111,114,101,100,67,111,114,110,101,114,82,97,100,105,117,115]));
        let background = RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(5.0f64), Some(5.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(5.0f64, background.continuation_corner_radius, None).unwrap();
    });
}

#[test]
fn adjacent_backgrounds_with_the_same_style_share_one_clearance() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.adjacentBackgroundsWithTheSameStyleShareOneClearance", "org.tiqian.core.LayoutQueriesTest.adjacentBackgroundsWithTheSameStyleShareOneClearance", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[97,100,106,97,99,101,110,116,66,97,99,107,103,114,111,117,110,100,115,87,105,116,104,84,104,101,83,97,109,101,83,116,121,108,101,83,104,97,114,101,79,110,101,67,108,101,97,114,97,110,99,101]));
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_sample_result().unwrap();
        let paint = RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(2.0f64)).unwrap();
        let spans = vec![
    (RichTextSpan::new(TextRange::new(0u32, 1u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), (paint).clone())).clone(),
    (RichTextSpan::new(TextRange::new(1u32, 3u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), (paint).clone())).clone(),
];
        let occupied = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &spans).unwrap();
        let segments = LayoutQueries::layout_queries_rich_text_background_segments((result).clone(), &occupied);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((segments.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(2.0f64, segments[1usize].left - segments[0usize].right, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(13.0f64, segments[0usize].right, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(15.0f64, segments[1usize].left, 0.001f64, None).unwrap();
    });
}

#[test]
fn adjacent_line_decorations_with_the_same_style_share_one_clearance() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.adjacentLineDecorationsWithTheSameStyleShareOneClearance", "org.tiqian.core.LayoutQueriesTest.adjacentLineDecorationsWithTheSameStyleShareOneClearance", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[97,100,106,97,99,101,110,116,76,105,110,101,68,101,99,111,114,97,116,105,111,110,115,87,105,116,104,84,104,101,83,97,109,101,83,116,121,108,101,83,104,97,114,101,79,110,101,67,108,101,97,114,97,110,99,101]));
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_sample_result().unwrap();
        let paint = RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(2.0f64)).unwrap();
        let spans = vec![
    (RichTextSpan::new(TextRange::new(0u32, 1u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), (paint).clone())).clone(),
    (RichTextSpan::new(TextRange::new(1u32, 3u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), (paint).clone())).clone(),
];
        let occupied = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &spans).unwrap();
        let segments = LayoutQueries::layout_queries_trimmed_rich_text_decoration_segments((result).clone(), &occupied);
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((segments.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(2.0f64, segments[1usize].left - segments[0usize].right, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(13.0f64, segments[0usize].right, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(15.0f64, segments[1usize].left, 0.001f64, None).unwrap();
    });
}

#[test]
fn adjacent_background_and_underline_do_not_avoid_across_styles() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.adjacentBackgroundAndUnderlineDoNotAvoidAcrossStyles", "org.tiqian.core.LayoutQueriesTest.adjacentBackgroundAndUnderlineDoNotAvoidAcrossStyles", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[97,100,106,97,99,101,110,116,66,97,99,107,103,114,111,117,110,100,65,110,100,85,110,100,101,114,108,105,110,101,68,111,78,111,116,65,118,111,105,100,65,99,114,111,115,115,83,116,121,108,101,115]));
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_sample_result().unwrap();
        let paint = RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()))).unwrap(), Some(2.0f64)).unwrap();
        let background = RichTextSpan::new(TextRange::new(0u32, 1u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::BACKGROUND_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), (paint).clone());
        let underline = RichTextSpan::new(TextRange::new(1u32, 3u32).unwrap(), Box::new({ let __guard = crate::org::tiqian::core::rich_text_role::UNDERLINE_INSTANCE.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), (paint).clone());
        let occupied = LayoutQueries::layout_queries_positioned_rich_text_segments((result).clone(), &vec![(background).clone(), (underline).clone()]).unwrap();
        let fill = LayoutQueries::layout_queries_rich_text_background_segments((result).clone(), &occupied);
        let line_segments = LayoutQueries::layout_queries_trimmed_rich_text_decoration_segments((result).clone(), &occupied);
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(14.0f64, fill[0usize].right, 0.001f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(14.0f64, line_segments[0usize].left, 0.001f64, None).unwrap();
    });
}

#[test]
fn hit_testing_chooses_offset_from_tiqian_cluster_advances() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.hitTestingChoosesOffsetFromTiqianClusterAdvances", "org.tiqian.core.LayoutQueriesTest.hitTestingChoosesOffsetFromTiqianClusterAdvances", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[104,105,116,84,101,115,116,105,110,103,67,104,111,111,115,101,115,79,102,102,115,101,116,70,114,111,109,84,105,113,105,97,110,67,108,117,115,116,101,114,65,100,118,97,110,99,101,115]));
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_sample_result().unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_offset_for_position((result).clone(), 3.0f64, 5.0f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, LayoutQueries::layout_queries_get_offset_for_position((result).clone(), 18.0f64, 5.0f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, LayoutQueries::layout_queries_get_offset_for_position((result).clone(), 24.0f64, 5.0f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, LayoutQueries::layout_queries_get_offset_for_position((result).clone(), 4.0f64, 25.0f64), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, LayoutQueries::layout_queries_get_offset_for_position((result).clone(), 30.0f64, 25.0f64), None).unwrap();
    });
}

#[test]
fn selection_hit_testing_keeps_supported_source_sequences_atomic() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.selectionHitTestingKeepsSupportedSourceSequencesAtomic", "org.tiqian.core.LayoutQueriesTest.selectionHitTestingKeepsSupportedSourceSequencesAtomic", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[115,101,108,101,99,116,105,111,110,72,105,116,84,101,115,116,105,110,103,75,101,101,112,115,83,117,112,112,111,114,116,101,100,83,111,117,114,99,101,83,101,113,117,101,110,99,101,115,65,116,111,109,105,99]));
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_interaction_boundary_result().unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_selection_offset_for_position((result).clone(), 5.0f64, 10.0f64).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, LayoutQueries::layout_queries_get_selection_offset_for_position((result).clone(), 15.0f64, 10.0f64).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, LayoutQueries::layout_queries_get_selection_offset_for_position((result).clone(), 25.0f64, 10.0f64).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, LayoutQueries::layout_queries_get_selection_offset_for_position((result).clone(), 35.0f64, 10.0f64).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, LayoutQueries::layout_queries_get_selection_offset_for_position((result).clone(), 45.0f64, 10.0f64).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(9, LayoutQueries::layout_queries_get_selection_offset_for_position((result).clone(), 75.0f64, 10.0f64).unwrap(), None).unwrap();
    });
}

#[test]
fn external_selection_offsets_respect_directional_boundary_bias() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.externalSelectionOffsetsRespectDirectionalBoundaryBias", "org.tiqian.core.LayoutQueriesTest.externalSelectionOffsetsRespectDirectionalBoundaryBias", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[101,120,116,101,114,110,97,108,83,101,108,101,99,116,105,111,110,79,102,102,115,101,116,115,82,101,115,112,101,99,116,68,105,114,101,99,116,105,111,110,97,108,66,111,117,110,100,97,114,121,66,105,97,115]));
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_interaction_boundary_result().unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, LayoutQueries::layout_queries_coerce_selection_offset((result).clone(), 3, SourceBoundaryBias::Backward).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, LayoutQueries::layout_queries_coerce_selection_offset((result).clone(), 3, SourceBoundaryBias::Forward).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, LayoutQueries::layout_queries_coerce_selection_offset((result).clone(), 3, SourceBoundaryBias::Nearest).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, LayoutQueries::layout_queries_coerce_selection_offset((result).clone(), 6, SourceBoundaryBias::Backward).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(9, LayoutQueries::layout_queries_coerce_selection_offset((result).clone(), 6, SourceBoundaryBias::Forward).unwrap(), None).unwrap();
    });
}

#[test]
fn supported_source_sequence_remains_atomic_across_engine_cluster_boundaries() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.supportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundaries", "org.tiqian.core.LayoutQueriesTest.supportedSourceSequenceRemainsAtomicAcrossEngineClusterBoundaries", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[115,117,112,112,111,114,116,101,100,83,111,117,114,99,101,83,101,113,117,101,110,99,101,82,101,109,97,105,110,115,65,116,111,109,105,99,65,99,114,111,115,115,69,110,103,105,110,101,67,108,117,115,116,101,114,66,111,117,110,100,97,114,105,101,115]));
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_cross_cluster_interaction_boundary_result().unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_coerce_selection_offset((result).clone(), 1, SourceBoundaryBias::Backward).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, LayoutQueries::layout_queries_coerce_selection_offset((result).clone(), 1, SourceBoundaryBias::Forward).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_selection_offset_for_position((result).clone(), 8.0f64, 10.0f64).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, LayoutQueries::layout_queries_get_selection_offset_for_position((result).clone(), 12.0f64, 10.0f64).unwrap(), None).unwrap();
    });
}

#[test]
fn inline_object_source_range_is_one_selection_unit() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.inlineObjectSourceRangeIsOneSelectionUnit", "org.tiqian.core.LayoutQueriesTest.inlineObjectSourceRangeIsOneSelectionUnit", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[105,110,108,105,110,101,79,98,106,101,99,116,83,111,117,114,99,101,82,97,110,103,101,73,115,79,110,101,83,101,108,101,99,116,105,111,110,85,110,105,116]));
        let source = UString::from("a\\operatorname{lim}b").to_ustring();
        let object_range = TextRange::new(1u32, u32::wrapping_sub(u_string::unit_count(&(source)), 1)).unwrap();
        let object = InlineObjectSpan::new((object_range).clone(), 40.0f64, 12.0f64, 4.0f64, Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap()), Some(InlineObjectBoundaryAdjustment::inline_object_boundary_adjustment_fixed().unwrap())).unwrap();
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_result(source.as_ustr(), 200.0f64, None, Size::new(60.0f64, 20.0f64), &vec![], &vec![], &vec![], Some(LayoutQueriesTestHelpers::layout_queries_test_helpers_empty_debug()), Some(vec![(object).clone()])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, LayoutQueries::layout_queries_coerce_selection_offset((result).clone(), 5, SourceBoundaryBias::Backward).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(object_range.end, LayoutQueries::layout_queries_coerce_selection_offset((result).clone(), 5, SourceBoundaryBias::Forward).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, LayoutQueries::layout_queries_coerce_selection_offset((result).clone(), 5, SourceBoundaryBias::Nearest).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(object_range.end, LayoutQueries::layout_queries_coerce_selection_offset((result).clone(), u32::wrapping_sub(object_range.end, 1), SourceBoundaryBias::Nearest).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", object_range.to_string()).as_str()).as_ustr(), UString::from(format!("{}", LayoutQueries::layout_queries_get_selection_word_boundary((result).clone(), 5).unwrap().to_string()).as_str()).as_ustr(), None).unwrap();
    });
}

#[test]
fn selection_word_boundary_expands_latin_but_keeps_han_atomic() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.selectionWordBoundaryExpandsLatinButKeepsHanAtomic", "org.tiqian.core.LayoutQueriesTest.selectionWordBoundaryExpandsLatinButKeepsHanAtomic", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[115,101,108,101,99,116,105,111,110,87,111,114,100,66,111,117,110,100,97,114,121,69,120,112,97,110,100,115,76,97,116,105,110,66,117,116,75,101,101,112,115,72,97,110,65,116,111,109,105,99]));
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_word_boundary_result().unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", TextRange::new(2u32, 10u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", LayoutQueries::layout_queries_get_selection_word_boundary((result).clone(), 6).unwrap().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", TextRange::new(0u32, 1u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", LayoutQueries::layout_queries_get_selection_word_boundary((result).clone(), 0).unwrap().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", TextRange::new(1u32, 2u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", LayoutQueries::layout_queries_get_selection_word_boundary((result).clone(), 1).unwrap().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", TextRange::new(11u32, 12u32).unwrap().to_string()).as_str()).as_ustr(), UString::from(format!("{}", LayoutQueries::layout_queries_get_selection_word_boundary((result).clone(), 12).unwrap().to_string()).as_str()).as_ustr(), None).unwrap();
        let first = LayoutQueries::layout_queries_get_selection_word_boundary_for_position((result).clone(), 5.0f64, 10.0f64).unwrap();
        let second = LayoutQueries::layout_queries_get_selection_word_boundary_for_position((result).clone(), 60.0f64, 10.0f64).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", TextRange::new(0u32, 1u32).unwrap().to_string()).as_str()).as_ustr(), match &(first) { None => UString::from("null"), Some(__option2) => UString::from(format!("{}", __option2.to_string()).as_str()) }.as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", TextRange::new(2u32, 10u32).unwrap().to_string()).as_str()).as_ustr(), match &(second) { None => UString::from("null"), Some(__option5) => UString::from(format!("{}", __option5.to_string()).as_str()) }.as_ustr(), None).unwrap();
    });
}

#[test]
fn ruby_selection_geometry_redistributes_avoidance_spread_without_overlap() {
    testlib::run("org.tiqian.core.LayoutQueriesTest.rubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlap", "org.tiqian.core.LayoutQueriesTest.rubySelectionGeometryRedistributesAvoidanceSpreadWithoutOverlap", || {
        TestTraceRecorder::new(&(UStr::new(&[76,97,121,111,117,116,81,117,101,114,105,101,115,84,101,115,116]))).section(UStr::new(&[114,117,98,121,83,101,108,101,99,116,105,111,110,71,101,111,109,101,116,114,121,82,101,100,105,115,116,114,105,98,117,116,101,115,65,118,111,105,100,97,110,99,101,83,112,114,101,97,100,87,105,116,104,111,117,116,79,118,101,114,108,97,112]));
        let result = LayoutQueriesTestHelpers::layout_queries_test_helpers_ruby_selection_result().unwrap();
        let positioned = LayoutQueries::layout_queries_positioned_clusters((result).clone());
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(-6.0f64, 0.0f64, 26.0f64, 20.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", (positioned[0usize]).clone().get_rect().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(29.0f64, 0.0f64, 61.0f64, 20.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", (positioned[1usize]).clone().get_rect().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(64.0f64, 0.0f64, 96.0f64, 20.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", (positioned[2usize]).clone().get_rect().to_string()).as_str()).as_ustr(), None).unwrap();
        let mut no_overlap = true;
        let mut index = 0u32;
        while (i32::from_ne_bytes(((u32::wrapping_add(index, 1)) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((positioned.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if !((positioned[usize::try_from(index).unwrap_or(0)].right) <= positioned[usize::try_from(u32::wrapping_add(index, 1)).unwrap_or(0)].left) {
                no_overlap = false;
            }
            index = u32::wrapping_add(index, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(no_overlap, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ruby selection rects must not overlap: ")); __s += LayoutQueriesTestHelpers::layout_queries_test_helpers_render_positioned(&positioned).unwrap().as_ustr(); __s }).as_str()))).unwrap();
        let first = LayoutQueries::layout_queries_get_bounding_boxes((result).clone(), TextRange::new(0u32, 1u32).unwrap());
        let second = LayoutQueries::layout_queries_get_bounding_boxes((result).clone(), TextRange::new(1u32, 2u32).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(-6.0f64, 0.0f64, 26.0f64, 20.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", (first[0usize]).clone().to_string()).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(format!("{}", Rect::new(29.0f64, 0.0f64, 61.0f64, 20.0f64).to_string()).as_str()).as_ustr(), UString::from(format!("{}", (second[0usize]).clone().to_string()).as_str()).as_ustr(), None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct LayoutQueriesTestHelpers;

impl LayoutQueriesTestHelpers {
    pub fn layout_queries_test_helpers_content(text: &UStr) -> TiqianTextContent {
        return TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
    }

    pub fn layout_queries_test_helpers_style(font_size: f64) -> TextStyle {
        return TextStyle::new(Some(vec![]), Some(font_size), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0f64), Some(InlineAttachment::None));
    }

    pub fn layout_queries_test_helpers_constraints(max_width: f64) -> Result<LayoutConstraints, TextRangeError> {
        return Ok(LayoutConstraints::new(max_width, Some(f64::INFINITY), Some(2147483647))?);
    }

    pub fn layout_queries_test_helpers_input(text: &UStr, max_width: f64, text_style: Option<TextStyle>, inline_objects: Option<Vec<InlineObjectSpan>>) -> Result<LayoutInput, TextRangeError> {
        return Ok(LayoutInput::new(LayoutQueriesTestHelpers::layout_queries_test_helpers_content(text), (text_style).clone(), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some((Ic::zero()).clone()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0f64), Some(1.0f64), Some(2.0f64))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutQueriesTestHelpers::layout_queries_test_helpers_constraints(max_width)?, Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), (inline_objects).clone()));
    }

    pub fn layout_queries_test_helpers_cluster(range: TextRange, text: &UStr, font_key: &UStr, advance: f64) -> Cluster {
        return Cluster::new((range).clone(), text, font_key, advance, Some((text).to_ustring()), Some(0.0f64), Some(0.0f64), Some(0.0f64));
    }

    pub fn layout_queries_test_helpers_line(range: TextRange, cluster_start: u32, cluster_end: u32, baseline: f64, top: f64, bottom: f64, natural_width: f64, adjusted_width: f64, visual_width: f64, indent: Option<f64>) -> LineBox {
        return LineBox::new((range).clone(), IntRange::new(cluster_start, cluster_end), baseline, top, bottom, natural_width, adjusted_width, visual_width, Some(0.0f64), indent, Some(LineEndReason::ParagraphEnd), Some(0.0f64), Some(vec![]), LineDebugInfo::new(None.clone(), Some(vec![])));
    }

    pub fn layout_queries_test_helpers_result(text: &UStr, max_width: f64, text_style: Option<TextStyle>, size: Size, clusters: &Vec<Cluster>, glyph_runs: &Vec<GlyphRun>, lines: &Vec<LineBox>, debug: Option<LayoutDebugInfo>, inline_objects: Option<Vec<InlineObjectSpan>>) -> Result<LayoutResult, TextRangeError> {
        return Ok(LayoutResult::new(LayoutQueriesTestHelpers::layout_queries_test_helpers_input(text, max_width, (text_style).clone(), (inline_objects).clone())?, (size).clone(), (clusters).clone(), (glyph_runs).clone(), (lines).clone(), match &(debug) { None => LayoutQueriesTestHelpers::layout_queries_test_helpers_empty_debug(), Some(__option6) => (*__option6).clone() }));
    }

    pub fn layout_queries_test_helpers_empty_debug() -> LayoutDebugInfo {
        return LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
    }

    pub fn layout_queries_test_helpers_render_rects(values: &Vec<Rect>) -> Result<UString, UStringFault> {
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

    pub fn layout_queries_test_helpers_render_positioned(values: &Vec<PositionedCluster>) -> Result<UString, UStringFault> {
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

    pub fn layout_queries_test_helpers_sample_result() -> Result<LayoutResult, TextRangeError> {
        let text = UString::from("甲——乙").to_ustring();
        let dash_range = TextRange::new(1u32, 3u32)?;
        return Ok(LayoutQueriesTestHelpers::layout_queries_test_helpers_result(text.as_ustr(), 40.0f64, Some(LayoutQueriesTestHelpers::layout_queries_test_helpers_style(10.0f64)), Size::new(34.0f64, 40.0f64), &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(0u32, 1u32)?, UStr::new(&[30002]), UStr::new(&[99,106,107]), 10.0f64)).clone(),
    (Cluster::new((dash_range).clone(), &(UStr::new(&[8212,8212])), &(UStr::new(&[99,106,107])), 20.0f64, Some(UString::from("⸺")), Some(0.0f64), Some(0.0f64), Some(0.0f64))).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(3u32, 4u32)?, UStr::new(&[20057]), UStr::new(&[99,106,107]), 10.0f64)).clone(),
], &vec![], &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_line(TextRange::new(0u32, 3u32)?, 0, 1, 15.0f64, 0.0f64, 20.0f64, 30.0f64, 30.0f64, 30.0f64, Some(4.0f64))).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_line(TextRange::new(3u32, 4u32)?, 2, 2, 35.0f64, 20.0f64, 40.0f64, 10.0f64, 10.0f64, 10.0f64, None)).clone(),
], None, None)?);
    }

    pub fn layout_queries_test_helpers_background_geometry_result() -> Result<LayoutResult, TextRangeError> {
        let text = UString::from("A B").to_ustring();
        let glyph_a = Glyph::new(1u32, TextRange::new(0u32, 1u32)?, 10.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None);
        let glyph_b = Glyph::new(2u32, TextRange::new(2u32, 3u32)?, 10.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None);
        let glyph_runs = vec![
    (GlyphRun::new(TextRange::new(0u32, 1u32)?, &(UStr::new(&[108,97,116,105,110])), vec![(glyph_a).clone()].to_vec(), 10.0f64, Some(vec![]))).clone(),
    (GlyphRun::new(TextRange::new(2u32, 3u32)?, &(UStr::new(&[108,97,116,105,110])), vec![(glyph_b).clone()].to_vec(), 10.0f64, Some(vec![]))).clone(),
];
        let auto_space = AutoSpaceDecisionInfo::new(TextRange::new(2u32, 3u32)?, &(UStr::new(&[108,101,97,100,105,110,103])), &(UStr::new(&[67,106,107,76,97,116,105,110])), &(UStr::new(&[73,110,115,101,114,116])), 1u32, -2.0f64, -2.0f64, &(UStr::new(&[116,101,115,116,45,108,101,97,100,105,110,103,45,103,97,112])));
        return Ok(LayoutQueriesTestHelpers::layout_queries_test_helpers_result(text.as_ustr(), 31.0f64, Some(LayoutQueriesTestHelpers::layout_queries_test_helpers_style(10.0f64)), Size::new(31.0f64, 30.0f64), &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(0u32, 1u32)?, UStr::new(&[65]), UStr::new(&[108,97,116,105,110]), 12.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(1u32, 2u32)?, UStr::new(&[32]), UStr::new(&[108,97,116,105,110]), 5.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(2u32, 3u32)?, UStr::new(&[66]), UStr::new(&[108,97,116,105,110]), 14.0f64)).clone(),
], &glyph_runs, &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_line(TextRange::new(0u32, 3u32)?, 0, 2, 20.0f64, 0.0f64, 30.0f64, 31.0f64, 31.0f64, 31.0f64, None)).clone(),
],
Some(LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![(auto_space).clone()]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))), None)?);
    }

    pub fn layout_queries_test_helpers_background_metric(range: TextRange, metric_box: &UStr, ascent: f64, descent: f64) -> MetricDecisionInfo {
        return MetricDecisionInfo::new((range).clone(), &(UStr::new(&[116,101,115,116])), &(UStr::new(&[116,101,115,116])), &(UStr::new(&[116,101,115,116])), ascent, descent, 0.0f64, &(UStr::new(&[116,101,115,116])), ascent, descent, &(UStr::new(&[116,101,115,116])), metric_box, &(UStr::new(&[116,101,115,116])), &(UStr::new(&[116,101,115,116])));
    }

    pub fn layout_queries_test_helpers_punctuation_glue_result() -> Result<LayoutResult, TextRangeError> {
        let text = UString::from("（，中）").to_ustring();
        let geometries = vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_punctuation_geometry(TextRange::new(0u32, 1u32)?, UStr::new(&[65288]), 5.0f64, 0.0f64, 0.0f64, 0.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_punctuation_geometry(TextRange::new(1u32, 2u32)?, UStr::new(&[65292]), 0.0f64, 5.0f64, 0.0f64, 0.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_punctuation_geometry(TextRange::new(2u32, 3u32)?, UStr::new(&[20013]), 0.0f64, 0.0f64, 0.0f64, 0.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_punctuation_geometry(TextRange::new(3u32, 4u32)?, UStr::new(&[65289]), 0.0f64, 5.0f64, 0.0f64, 0.0f64)).clone(),
];
        return Ok(LayoutQueriesTestHelpers::layout_queries_test_helpers_result(text.as_ustr(), 40.0f64, Some(LayoutQueriesTestHelpers::layout_queries_test_helpers_style(10.0f64)), Size::new(40.0f64, 20.0f64), &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(0u32, 1u32)?, UStr::new(&[65288]), UStr::new(&[99,106,107]), 10.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(1u32, 2u32)?, UStr::new(&[65292]), UStr::new(&[99,106,107]), 10.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(2u32, 3u32)?, UStr::new(&[20013]), UStr::new(&[99,106,107]), 10.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(3u32, 4u32)?, UStr::new(&[65289]), UStr::new(&[99,106,107]), 10.0f64)).clone(),
], &vec![], &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_line(TextRange::new(0u32, 4u32)?, 0, 3, 15.0f64, 0.0f64, 20.0f64, 40.0f64, 40.0f64, 40.0f64, None)).clone(),
],
Some(LayoutDebugInfo::new(None, Some(vec![]), Some((geometries).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))), None)?);
    }

    pub fn layout_queries_test_helpers_interaction_boundary_result() -> Result<LayoutResult, TextRangeError> {
        let text = { let mut __s = UString::new(); __s += TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832]).as_ustr(); __s += &(UString::from("é")); __s += TestHelpers::test_helpers_surrogate_text(&vec![55357, 56425, 8205, 55357, 56425]).as_ustr(); __s };
        return Ok(LayoutQueriesTestHelpers::layout_queries_test_helpers_result(text.as_ustr(), 90.0f64, Some(LayoutQueriesTestHelpers::layout_queries_test_helpers_style(10.0f64)), Size::new(90.0f64, 20.0f64), &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(0u32, 2u32)?, TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832]).as_ustr(), UStr::new(&[101,109,111,106,105]), 20.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(2u32, 4u32)?, UStr::new(&[101,769]), UStr::new(&[108,97,116,105,110]), 20.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(4u32, 9u32)?, TestHelpers::test_helpers_surrogate_text(&vec![55357, 56425, 8205, 55357, 56425]).as_ustr(), UStr::new(&[101,109,111,106,105]), 50.0f64)).clone(),
], &vec![], &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_line(TextRange::new(0u32, 9u32)?, 0, 2, 15.0f64, 0.0f64, 20.0f64, 90.0f64, 90.0f64, 90.0f64, None)).clone(),
], None, None)?);
    }

    pub fn layout_queries_test_helpers_word_boundary_result() -> Result<LayoutResult, TextRangeError> {
        let text = UString::from("前 template 后").to_ustring();
        return Ok(LayoutQueriesTestHelpers::layout_queries_test_helpers_result(text.as_ustr(), 120.0f64, Some(LayoutQueriesTestHelpers::layout_queries_test_helpers_style(10.0f64)), Size::new(120.0f64, 20.0f64), &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(0u32, 1u32)?, UStr::new(&[21069]), UStr::new(&[99,106,107]), 10.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(1u32, 2u32)?, UStr::new(&[32]), UStr::new(&[108,97,116,105,110]), 10.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(2u32, 10u32)?, UStr::new(&[116,101,109,112,108,97,116,101]), UStr::new(&[108,97,116,105,110]), 80.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(10u32, 11u32)?, UStr::new(&[32]), UStr::new(&[108,97,116,105,110]), 10.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(11u32, 12u32)?, UStr::new(&[21518]), UStr::new(&[99,106,107]), 10.0f64)).clone(),
], &vec![], &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_line(TextRange::new(0u32, 12u32)?, 0, 4, 15.0f64, 0.0f64, 20.0f64, 120.0f64, 120.0f64, 120.0f64, None)).clone(),
], None, None)?);
    }

    pub fn layout_queries_test_helpers_cross_cluster_interaction_boundary_result() -> Result<LayoutResult, TextRangeError> {
        let text = UString::from("é").to_ustring();
        return Ok(LayoutQueriesTestHelpers::layout_queries_test_helpers_result(text.as_ustr(), 20.0f64, Some(LayoutQueriesTestHelpers::layout_queries_test_helpers_style(10.0f64)), Size::new(20.0f64, 20.0f64), &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(0u32, 1u32)?, UStr::new(&[101]), UStr::new(&[108,97,116,105,110]), 10.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(1u32, 2u32)?, UStr::new(&[769]), UStr::new(&[108,97,116,105,110]), 10.0f64)).clone(),
], &vec![], &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_line(TextRange::new(0u32, 2u32)?, 0, 1, 15.0f64, 0.0f64, 20.0f64, 20.0f64, 20.0f64, 20.0f64, None)).clone(),
], None, None)?);
    }

    pub fn layout_queries_test_helpers_punctuation_geometry(range: TextRange, text: &UStr, leading_glue: f64, trailing_glue: f64, leading_consumed: f64, trailing_consumed: f64) -> ClusterGeometryDecisionInfo {
        return ClusterGeometryDecisionInfo::new((range).clone(), text, text, 10.0f64, (10.0f64 - leading_glue) - trailing_glue, leading_glue, leading_consumed, trailing_glue, trailing_consumed, 0.0f64, 10.0f64, &(UStr::new(&[116,101,115,116])), &(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,71,108,117,101,84,101,115,116])), Some(0.0f64), Some(0.0f64), None);
    }

    pub fn layout_queries_test_helpers_ruby_selection_result() -> Result<LayoutResult, TextRangeError> {
        let text = UString::from("张王李").to_ustring();
        let glyphs = vec![
    (Glyph::new(1u32, TextRange::new(0u32, 1u32)?, 20.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None)).clone(),
    (Glyph::new(2u32, TextRange::new(1u32, 2u32)?, 20.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None)).clone(),
    (Glyph::new(3u32, TextRange::new(2u32, 3u32)?, 20.0f64, Some(0.0f64), Some(0.0f64), None, None, None, None)).clone(),
];
        let rubies = vec![
    (RubyDecisionInfo::new(TextRange::new(0u32, 1u32)?, &(UStr::new(&[122,104,117,257,110,103])), 0u32, 10.0f64, 0.0f64, 10.0f64, 6.0f64, Some(0.0f64), Some(0.0f64), Some(32.0f64), Some(vec![]), Some(400), Some(UString::from("zh-Hans")), Some(vec![]))).clone(),
    (RubyDecisionInfo::new(TextRange::new(1u32, 2u32)?, &(UStr::new(&[99,104,117,225,110,103])), 0u32, 45.0f64, 0.0f64, 10.0f64, 6.0f64, Some(0.0f64), Some(0.0f64), Some(32.0f64), Some(vec![]), Some(400), Some(UString::from("zh-Hans")), Some(vec![]))).clone(),
    (RubyDecisionInfo::new(TextRange::new(2u32, 3u32)?, &(UStr::new(&[115,104,117,257,110,103])), 0u32, 80.0f64, 0.0f64, 10.0f64, 6.0f64, Some(0.0f64), Some(0.0f64), Some(32.0f64), Some(vec![]), Some(400), Some(UString::from("zh-Hans")), Some(vec![]))).clone(),
];
        let geometries = vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_ruby_geometry(TextRange::new(0u32, 1u32)?, UStr::new(&[24352]), 15.0f64, 35.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_ruby_geometry(TextRange::new(1u32, 2u32)?, UStr::new(&[29579]), 15.0f64, 35.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_ruby_geometry(TextRange::new(2u32, 3u32)?, UStr::new(&[26446]), 0.0f64, 20.0f64)).clone(),
];
        return Ok(LayoutQueriesTestHelpers::layout_queries_test_helpers_result(text.as_ustr(), 200.0f64, Some(LayoutQueriesTestHelpers::layout_queries_test_helpers_style(20.0f64)), Size::new(90.0f64, 20.0f64), &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(0u32, 1u32)?, UStr::new(&[24352]), UStr::new(&[99,106,107]), 35.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(1u32, 2u32)?, UStr::new(&[29579]), UStr::new(&[99,106,107]), 35.0f64)).clone(),
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_cluster(TextRange::new(2u32, 3u32)?, UStr::new(&[26446]), UStr::new(&[99,106,107]), 20.0f64)).clone(),
], &vec![
    (GlyphRun::new(TextRange::new(0u32, 3u32)?, &(UStr::new(&[99,106,107])), glyphs.to_vec(), 60.0f64, Some(vec![]))).clone(),
], &vec![
    (LayoutQueriesTestHelpers::layout_queries_test_helpers_line(TextRange::new(0u32, 3u32)?, 0, 2, 15.0f64, 0.0f64, 20.0f64, 60.0f64, 90.0f64, 90.0f64, None)).clone(),
],
Some(LayoutDebugInfo::new(None, Some(vec![]), Some((geometries).clone()), Some(vec![]), Some((rubies).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))), None)?);
    }

    pub fn layout_queries_test_helpers_ruby_geometry(range: TextRange, text: &UStr, ruby_spread: f64, resolved_advance: f64) -> ClusterGeometryDecisionInfo {
        return ClusterGeometryDecisionInfo::new((range).clone(), text, text, 20.0f64, 20.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, resolved_advance, &(UStr::new(&[116,101,115,116])), &(UStr::new(&[82,117,98,121,65,118,111,105,100,97,110,99,101,83,112,114,101,97,100])), Some(ruby_spread), Some(0.0f64), None);
    }

    pub fn layout_queries_test_helpers_segment_for(span: RichTextSpan, start: u32, end: u32) -> Result<RichTextLineSegment, TextRangeError> {
        return Ok(RichTextLineSegment::new((span).clone(), 0u32, TextRange::new(start, end)?, 0.0f64, 0.0f64, 40.0f64, 20.0f64, 16.0f64));
    }

    pub fn layout_queries_test_helpers_same_range(first: TextRange, second: TextRange) -> bool {
        return first.start == second.start && first.end == second.end;
    }

    pub fn layout_queries_test_helpers_copy_result_with_debug(original: LayoutResult, debug: LayoutDebugInfo) -> LayoutResult {
        return LayoutResult::new((original.input).clone(), (original.size).clone(), (original.clusters).clone(), (original.glyph_runs).clone(), (original.lines).clone(), (debug).clone());
    }
}
