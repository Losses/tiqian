#![cfg(test)]

use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_box_outer_spacing::InlineBoxOuterSpacing;
use crate::org::tiqian::core::inline_box_span::InlineBoxSpan;
use crate::org::tiqian::core::inline_object_boundary_adjustment::InlineObjectBoundaryAdjustment;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::line_break_policy::LineBreakPolicy;
use crate::org::tiqian::core::line_break_span::LineBreakSpan;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphLayoutEngineValidationCoverageTestSourceTextMustNotContainUnpairedSurrogatesFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault),
}

impl From<ParagraphLayoutEngineValidationCoverageTestSourceTextMustNotContainUnpairedSurrogatesFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestSourceTextMustNotContainUnpairedSurrogatesFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestSourceTextMustNotContainUnpairedSurrogatesFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestSourceTextMustNotContainUnpairedSurrogatesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestSourceTextMustNotContainUnpairedSurrogatesFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestSourceTextMustNotContainUnpairedSurrogatesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestSourceTextMustNotContainUnpairedSurrogatesFault> for crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestSourceTextMustNotContainUnpairedSurrogatesFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestSourceTextMustNotContainUnpairedSurrogatesFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ParagraphLayoutEngineValidationCoverageTestSourceTextMustNotContainUnpairedSurrogatesFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestSourceTextMustNotContainUnpairedSurrogatesFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphLayoutEngineValidationCoverageTestSourceTextMustNotContainUnpairedSurrogatesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestSourceTextMustNotContainUnpairedSurrogatesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault> for ParagraphLayoutEngineValidationCoverageTestSourceTextMustNotContainUnpairedSurrogatesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        ParagraphLayoutEngineValidationCoverageTestSourceTextMustNotContainUnpairedSurrogatesFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphLayoutEngineValidationCoverageTestLineBreakSpansMustBeNonEmptyInBoundsRangesFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault),
}

impl From<ParagraphLayoutEngineValidationCoverageTestLineBreakSpansMustBeNonEmptyInBoundsRangesFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestLineBreakSpansMustBeNonEmptyInBoundsRangesFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestLineBreakSpansMustBeNonEmptyInBoundsRangesFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestLineBreakSpansMustBeNonEmptyInBoundsRangesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestLineBreakSpansMustBeNonEmptyInBoundsRangesFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestLineBreakSpansMustBeNonEmptyInBoundsRangesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestLineBreakSpansMustBeNonEmptyInBoundsRangesFault> for crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestLineBreakSpansMustBeNonEmptyInBoundsRangesFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestLineBreakSpansMustBeNonEmptyInBoundsRangesFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ParagraphLayoutEngineValidationCoverageTestLineBreakSpansMustBeNonEmptyInBoundsRangesFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestLineBreakSpansMustBeNonEmptyInBoundsRangesFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphLayoutEngineValidationCoverageTestLineBreakSpansMustBeNonEmptyInBoundsRangesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestLineBreakSpansMustBeNonEmptyInBoundsRangesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault> for ParagraphLayoutEngineValidationCoverageTestLineBreakSpansMustBeNonEmptyInBoundsRangesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        ParagraphLayoutEngineValidationCoverageTestLineBreakSpansMustBeNonEmptyInBoundsRangesFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphLayoutEngineValidationCoverageTestInlineObjectTrailingBoundaryMustNotExceedAdvanceFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault),
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectTrailingBoundaryMustNotExceedAdvanceFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectTrailingBoundaryMustNotExceedAdvanceFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectTrailingBoundaryMustNotExceedAdvanceFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectTrailingBoundaryMustNotExceedAdvanceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectTrailingBoundaryMustNotExceedAdvanceFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectTrailingBoundaryMustNotExceedAdvanceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectTrailingBoundaryMustNotExceedAdvanceFault> for crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectTrailingBoundaryMustNotExceedAdvanceFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectTrailingBoundaryMustNotExceedAdvanceFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ParagraphLayoutEngineValidationCoverageTestInlineObjectTrailingBoundaryMustNotExceedAdvanceFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectTrailingBoundaryMustNotExceedAdvanceFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphLayoutEngineValidationCoverageTestInlineObjectTrailingBoundaryMustNotExceedAdvanceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectTrailingBoundaryMustNotExceedAdvanceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault> for ParagraphLayoutEngineValidationCoverageTestInlineObjectTrailingBoundaryMustNotExceedAdvanceFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectTrailingBoundaryMustNotExceedAdvanceFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustNotOverlapFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault),
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustNotOverlapFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustNotOverlapFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustNotOverlapFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustNotOverlapFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustNotOverlapFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustNotOverlapFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustNotOverlapFault> for crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustNotOverlapFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustNotOverlapFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustNotOverlapFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustNotOverlapFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustNotOverlapFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustNotOverlapFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault> for ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustNotOverlapFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustNotOverlapFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustBeUniqueFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault),
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustBeUniqueFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustBeUniqueFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustBeUniqueFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustBeUniqueFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustBeUniqueFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustBeUniqueFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustBeUniqueFault> for crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustBeUniqueFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustBeUniqueFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustBeUniqueFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustBeUniqueFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustBeUniqueFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustBeUniqueFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault> for ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustBeUniqueFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectRangesMustBeUniqueFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphLayoutEngineValidationCoverageTestInlineObjectMustHaveFinitePositiveGeometryFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault),
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectMustHaveFinitePositiveGeometryFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectMustHaveFinitePositiveGeometryFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectMustHaveFinitePositiveGeometryFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectMustHaveFinitePositiveGeometryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectMustHaveFinitePositiveGeometryFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectMustHaveFinitePositiveGeometryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectMustHaveFinitePositiveGeometryFault> for crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectMustHaveFinitePositiveGeometryFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectMustHaveFinitePositiveGeometryFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ParagraphLayoutEngineValidationCoverageTestInlineObjectMustHaveFinitePositiveGeometryFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectMustHaveFinitePositiveGeometryFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphLayoutEngineValidationCoverageTestInlineObjectMustHaveFinitePositiveGeometryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectMustHaveFinitePositiveGeometryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault> for ParagraphLayoutEngineValidationCoverageTestInlineObjectMustHaveFinitePositiveGeometryFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectMustHaveFinitePositiveGeometryFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphLayoutEngineValidationCoverageTestInlineObjectMustCoverANonEmptyInBoundsRangeFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault),
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectMustCoverANonEmptyInBoundsRangeFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectMustCoverANonEmptyInBoundsRangeFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectMustCoverANonEmptyInBoundsRangeFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectMustCoverANonEmptyInBoundsRangeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectMustCoverANonEmptyInBoundsRangeFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectMustCoverANonEmptyInBoundsRangeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectMustCoverANonEmptyInBoundsRangeFault> for crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectMustCoverANonEmptyInBoundsRangeFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectMustCoverANonEmptyInBoundsRangeFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ParagraphLayoutEngineValidationCoverageTestInlineObjectMustCoverANonEmptyInBoundsRangeFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectMustCoverANonEmptyInBoundsRangeFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphLayoutEngineValidationCoverageTestInlineObjectMustCoverANonEmptyInBoundsRangeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectMustCoverANonEmptyInBoundsRangeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault> for ParagraphLayoutEngineValidationCoverageTestInlineObjectMustCoverANonEmptyInBoundsRangeFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectMustCoverANonEmptyInBoundsRangeFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphLayoutEngineValidationCoverageTestInlineObjectMinimumClearanceEmMustBeFiniteAndNonNegativeFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault),
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectMinimumClearanceEmMustBeFiniteAndNonNegativeFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectMinimumClearanceEmMustBeFiniteAndNonNegativeFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectMinimumClearanceEmMustBeFiniteAndNonNegativeFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectMinimumClearanceEmMustBeFiniteAndNonNegativeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectMinimumClearanceEmMustBeFiniteAndNonNegativeFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectMinimumClearanceEmMustBeFiniteAndNonNegativeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectMinimumClearanceEmMustBeFiniteAndNonNegativeFault> for crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectMinimumClearanceEmMustBeFiniteAndNonNegativeFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectMinimumClearanceEmMustBeFiniteAndNonNegativeFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ParagraphLayoutEngineValidationCoverageTestInlineObjectMinimumClearanceEmMustBeFiniteAndNonNegativeFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectMinimumClearanceEmMustBeFiniteAndNonNegativeFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphLayoutEngineValidationCoverageTestInlineObjectMinimumClearanceEmMustBeFiniteAndNonNegativeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectMinimumClearanceEmMustBeFiniteAndNonNegativeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault> for ParagraphLayoutEngineValidationCoverageTestInlineObjectMinimumClearanceEmMustBeFiniteAndNonNegativeFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectMinimumClearanceEmMustBeFiniteAndNonNegativeFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphLayoutEngineValidationCoverageTestInlineObjectLeadingBoundaryMustBeFixedFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault),
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectLeadingBoundaryMustBeFixedFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectLeadingBoundaryMustBeFixedFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectLeadingBoundaryMustBeFixedFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectLeadingBoundaryMustBeFixedFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectLeadingBoundaryMustBeFixedFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectLeadingBoundaryMustBeFixedFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineObjectLeadingBoundaryMustBeFixedFault> for crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineObjectLeadingBoundaryMustBeFixedFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineObjectLeadingBoundaryMustBeFixedFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ParagraphLayoutEngineValidationCoverageTestInlineObjectLeadingBoundaryMustBeFixedFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectLeadingBoundaryMustBeFixedFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphLayoutEngineValidationCoverageTestInlineObjectLeadingBoundaryMustBeFixedFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectLeadingBoundaryMustBeFixedFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault> for ParagraphLayoutEngineValidationCoverageTestInlineObjectLeadingBoundaryMustBeFixedFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineObjectLeadingBoundaryMustBeFixedFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustHaveFiniteInlineEdgesFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault),
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustHaveFiniteInlineEdgesFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustHaveFiniteInlineEdgesFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustHaveFiniteInlineEdgesFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustHaveFiniteInlineEdgesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustHaveFiniteInlineEdgesFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustHaveFiniteInlineEdgesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustHaveFiniteInlineEdgesFault> for crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustHaveFiniteInlineEdgesFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustHaveFiniteInlineEdgesFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustHaveFiniteInlineEdgesFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustHaveFiniteInlineEdgesFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustHaveFiniteInlineEdgesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustHaveFiniteInlineEdgesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault> for ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustHaveFiniteInlineEdgesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustHaveFiniteInlineEdgesFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustBeANonEmptyInBoundsRangeFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault),
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustBeANonEmptyInBoundsRangeFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustBeANonEmptyInBoundsRangeFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustBeANonEmptyInBoundsRangeFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustBeANonEmptyInBoundsRangeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustBeANonEmptyInBoundsRangeFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustBeANonEmptyInBoundsRangeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustBeANonEmptyInBoundsRangeFault> for crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustBeANonEmptyInBoundsRangeFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustBeANonEmptyInBoundsRangeFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustBeANonEmptyInBoundsRangeFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustBeANonEmptyInBoundsRangeFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustBeANonEmptyInBoundsRangeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustBeANonEmptyInBoundsRangeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault> for ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustBeANonEmptyInBoundsRangeFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        ParagraphLayoutEngineValidationCoverageTestInlineBoxSpanMustBeANonEmptyInBoundsRangeFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphLayoutEngineValidationCoverageTestEmphasisDotGapEmMustBeFiniteAndNonNegativeFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault),
}

impl From<ParagraphLayoutEngineValidationCoverageTestEmphasisDotGapEmMustBeFiniteAndNonNegativeFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestEmphasisDotGapEmMustBeFiniteAndNonNegativeFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestEmphasisDotGapEmMustBeFiniteAndNonNegativeFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestEmphasisDotGapEmMustBeFiniteAndNonNegativeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestEmphasisDotGapEmMustBeFiniteAndNonNegativeFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestEmphasisDotGapEmMustBeFiniteAndNonNegativeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestEmphasisDotGapEmMustBeFiniteAndNonNegativeFault> for crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestEmphasisDotGapEmMustBeFiniteAndNonNegativeFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestEmphasisDotGapEmMustBeFiniteAndNonNegativeFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ParagraphLayoutEngineValidationCoverageTestEmphasisDotGapEmMustBeFiniteAndNonNegativeFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestEmphasisDotGapEmMustBeFiniteAndNonNegativeFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphLayoutEngineValidationCoverageTestEmphasisDotGapEmMustBeFiniteAndNonNegativeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestEmphasisDotGapEmMustBeFiniteAndNonNegativeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault> for ParagraphLayoutEngineValidationCoverageTestEmphasisDotGapEmMustBeFiniteAndNonNegativeFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        ParagraphLayoutEngineValidationCoverageTestEmphasisDotGapEmMustBeFiniteAndNonNegativeFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphLayoutEngineValidationCoverageTestAutoSpaceSuppressedRangesMustBeNonEmptyInBoundsFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault),
}

impl From<ParagraphLayoutEngineValidationCoverageTestAutoSpaceSuppressedRangesMustBeNonEmptyInBoundsFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestAutoSpaceSuppressedRangesMustBeNonEmptyInBoundsFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestAutoSpaceSuppressedRangesMustBeNonEmptyInBoundsFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestAutoSpaceSuppressedRangesMustBeNonEmptyInBoundsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestAutoSpaceSuppressedRangesMustBeNonEmptyInBoundsFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestAutoSpaceSuppressedRangesMustBeNonEmptyInBoundsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphLayoutEngineValidationCoverageTestAutoSpaceSuppressedRangesMustBeNonEmptyInBoundsFault> for crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault {
    fn from(value: ParagraphLayoutEngineValidationCoverageTestAutoSpaceSuppressedRangesMustBeNonEmptyInBoundsFault) -> Self {
        match value {
            ParagraphLayoutEngineValidationCoverageTestAutoSpaceSuppressedRangesMustBeNonEmptyInBoundsFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for ParagraphLayoutEngineValidationCoverageTestAutoSpaceSuppressedRangesMustBeNonEmptyInBoundsFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestAutoSpaceSuppressedRangesMustBeNonEmptyInBoundsFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphLayoutEngineValidationCoverageTestAutoSpaceSuppressedRangesMustBeNonEmptyInBoundsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphLayoutEngineValidationCoverageTestAutoSpaceSuppressedRangesMustBeNonEmptyInBoundsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault> for ParagraphLayoutEngineValidationCoverageTestAutoSpaceSuppressedRangesMustBeNonEmptyInBoundsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine_validation_coverage_support::ParagraphLayoutEngineValidationCoverageSupportRejectFault) -> Self {
        ParagraphLayoutEngineValidationCoverageTestAutoSpaceSuppressedRangesMustBeNonEmptyInBoundsFault::ParagraphLayoutEngineValidationCoverageSupportRejectFaultFault(value)
    }
}

#[test]
fn emphasis_dot_gap_em_must_be_finite_and_non_negative() {
    testlib::run("org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.emphasisDotGapEmMustBeFiniteAndNonNegative", "org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.emphasisDotGapEmMustBeFiniteAndNonNegative", || {
        let mut t = TestTraceRecorder::new("ParagraphLayoutEngineValidationCoverageTest");
        t.section(&"emphasisDotGapEmMustBeFiniteAndNonNegative");
        let _ =
ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(Some(ParagraphStyle::new(Some(LastLineAlignment::Start),
Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(f64::NAN))), None, None, None).unwrap(), &"emphasisDotGapEm").unwrap();
        let _ =
ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(Some(ParagraphStyle::new(Some(LastLineAlignment::Start),
Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(-0.1f64))), None, None, None).unwrap(), &"emphasisDotGapEm").unwrap();
    });
}

#[test]
fn inline_object_minimum_clearance_em_must_be_finite_and_non_negative() {
    testlib::run("org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.inlineObjectMinimumClearanceEmMustBeFiniteAndNonNegative", "org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.inlineObjectMinimumClearanceEmMustBeFiniteAndNonNegative", || {
        let mut t = TestTraceRecorder::new("ParagraphLayoutEngineValidationCoverageTest");
        t.section(&"inlineObjectMinimumClearanceEmMustBeFiniteAndNonNegative");
        let _ =
ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(Some(ParagraphStyle::new(Some(LastLineAlignment::Start),
Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(f64::NAN),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), None, None, None).unwrap(), &"inlineObjectMinimumClearanceEm").unwrap();
        let _ =
ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(Some(ParagraphStyle::new(Some(LastLineAlignment::Start),
Some(WritingMode::HorizontalTb), None, None, Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(i32::from_ne_bytes((4294967295u32).to_ne_bytes()) as
f64), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), None, None, None).unwrap(), &"inlineObjectMinimumClearanceEm").unwrap();
    });
}

#[test]
fn source_text_must_not_contain_unpaired_surrogates() {
    testlib::record_not_applicable("org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.sourceTextMustNotContainUnpairedSurrogates", "org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.sourceTextMustNotContainUnpairedSurrogates");
}

#[test]
fn inline_box_span_must_be_a_non_empty_in_bounds_range() {
    testlib::run("org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.inlineBoxSpanMustBeANonEmptyInBoundsRange", "org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.inlineBoxSpanMustBeANonEmptyInBoundsRange", || {
        let mut t = TestTraceRecorder::new("ParagraphLayoutEngineValidationCoverageTest");
        t.section(&"inlineBoxSpanMustBeANonEmptyInBoundsRange");
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, Some(vec![
    (InlineBoxSpan::new(TextRange::new(0u32, 0u32).unwrap(), Some(0.0), Some(0.0), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]), None, None).unwrap(), &"non-empty source range").unwrap();
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, Some(vec![
    (InlineBoxSpan::new(TextRange::new(1u32, 9u32).unwrap(), Some(0.0), Some(0.0), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]), None, None).unwrap(), &"non-empty source range").unwrap();
    });
}

#[test]
fn inline_box_span_must_have_finite_inline_edges() {
    testlib::run("org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.inlineBoxSpanMustHaveFiniteInlineEdges", "org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.inlineBoxSpanMustHaveFiniteInlineEdges", || {
        let mut t = TestTraceRecorder::new("ParagraphLayoutEngineValidationCoverageTest");
        t.section(&"inlineBoxSpanMustHaveFiniteInlineEdges");
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, Some(vec![
    (InlineBoxSpan::new(TextRange::new(0u32, 1u32).unwrap(), Some(f64::NAN), Some(0.0), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]), None, None).unwrap(), &"finite inline edges").unwrap();
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, Some(vec![
    (InlineBoxSpan::new(TextRange::new(0u32, 1u32).unwrap(), Some(f64::INFINITY), Some(0.0), Some(InlineBoxOuterSpacing::Narrow))).clone(),
]), None, None).unwrap(), &"finite inline edges").unwrap();
    });
}

#[test]
fn line_break_spans_must_be_non_empty_in_bounds_ranges() {
    testlib::run("org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.lineBreakSpansMustBeNonEmptyInBoundsRanges", "org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.lineBreakSpansMustBeNonEmptyInBoundsRanges", || {
        let mut t = TestTraceRecorder::new("ParagraphLayoutEngineValidationCoverageTest");
        t.section(&"lineBreakSpansMustBeNonEmptyInBoundsRanges");
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, None, None, Some(TiqianTextContent::new("甲乙",
Some(vec![]), Some(vec![]), Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, 0u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
]), Some(vec![])))).unwrap(), &"LineBreakSpan").unwrap();
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, None, None, Some(TiqianTextContent::new("甲乙",
Some(vec![]), Some(vec![]), Some(vec![
    (LineBreakSpan::new(TextRange::new(2u32, 3u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
]), Some(vec![])))).unwrap(), &"LineBreakSpan").unwrap();
    });
}

#[test]
fn auto_space_suppressed_ranges_must_be_non_empty_in_bounds() {
    testlib::run("org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.autoSpaceSuppressedRangesMustBeNonEmptyInBounds", "org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.autoSpaceSuppressedRangesMustBeNonEmptyInBounds", || {
        let mut t = TestTraceRecorder::new("ParagraphLayoutEngineValidationCoverageTest");
        t.section(&"autoSpaceSuppressedRangesMustBeNonEmptyInBounds");
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, None, None, Some(TiqianTextContent::new("甲乙",
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(TextRange::new(1u32, 1u32).unwrap()).clone()])))).unwrap(), &"Auto-space suppressed range").unwrap();
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, None, None, Some(TiqianTextContent::new("甲乙",
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![(TextRange::new(0u32, 8u32).unwrap()).clone()])))).unwrap(), &"Auto-space suppressed range").unwrap();
    });
}

#[test]
fn inline_object_ranges_must_be_unique() {
    testlib::run("org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.inlineObjectRangesMustBeUnique", "org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.inlineObjectRangesMustBeUnique", || {
        let mut t = TestTraceRecorder::new("ParagraphLayoutEngineValidationCoverageTest");
        t.section(&"inlineObjectRangesMustBeUnique");
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, None, Some(vec![
    (ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_obj(Some(TextRange::new(0u32, 1u32).unwrap()), None, None, None, None, None).unwrap()).clone(),
    (ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_obj(Some(TextRange::new(0u32, 1u32).unwrap()), None, None, None, None, None).unwrap()).clone(),
]), None).unwrap(), &"unique").unwrap();
    });
}

#[test]
fn inline_object_ranges_must_not_overlap() {
    testlib::run("org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.inlineObjectRangesMustNotOverlap", "org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.inlineObjectRangesMustNotOverlap", || {
        let mut t = TestTraceRecorder::new("ParagraphLayoutEngineValidationCoverageTest");
        t.section(&"inlineObjectRangesMustNotOverlap");
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, None, Some(vec![
    (ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_obj(Some(TextRange::new(0u32, 2u32).unwrap()), None, None, None, None, None).unwrap()).clone(),
    (ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_obj(Some(TextRange::new(1u32, 2u32).unwrap()), None, None, None, None, None).unwrap()).clone(),
]), None).unwrap(), &"overlap").unwrap();
    });
}

#[test]
fn inline_object_must_cover_a_non_empty_in_bounds_range() {
    testlib::run("org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.inlineObjectMustCoverANonEmptyInBoundsRange", "org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.inlineObjectMustCoverANonEmptyInBoundsRange", || {
        let mut t = TestTraceRecorder::new("ParagraphLayoutEngineValidationCoverageTest");
        t.section(&"inlineObjectMustCoverANonEmptyInBoundsRange");
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, None, Some(vec![
    (ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_obj(Some(TextRange::new(1u32, 1u32).unwrap()), None, None, None, None, None).unwrap()).clone(),
]), None).unwrap(), &"non-empty source range").unwrap();
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, None, Some(vec![
    (ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_obj(Some(TextRange::new(0u32, 9u32).unwrap()), None, None, None, None, None).unwrap()).clone(),
]), None).unwrap(), &"non-empty source range").unwrap();
    });
}

#[test]
fn inline_object_must_have_finite_positive_geometry() {
    testlib::run("org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.inlineObjectMustHaveFinitePositiveGeometry", "org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.inlineObjectMustHaveFinitePositiveGeometry", || {
        let mut t = TestTraceRecorder::new("ParagraphLayoutEngineValidationCoverageTest");
        t.section(&"inlineObjectMustHaveFinitePositiveGeometry");
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, None, Some(vec![
    (ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_obj(None, Some(0 as f64 as f64), None, None, None, None).unwrap()).clone(),
]), None).unwrap(), &"finite positive geometry").unwrap();
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, None, Some(vec![
    (ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_obj(None, Some(f64::NAN), None, None, None, None).unwrap()).clone(),
]), None).unwrap(), &"finite positive geometry").unwrap();
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, None, Some(vec![
    (ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_obj(None, None, Some(i32::from_ne_bytes((4294967295u32).to_ne_bytes()) as f64 as f64), None, None, None).unwrap()).clone(),
]), None).unwrap(), &"finite positive geometry").unwrap();
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, None, Some(vec![
    (ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_obj(None, None, Some(f64::NAN), None, None, None).unwrap()).clone(),
]), None).unwrap(), &"finite positive geometry").unwrap();
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, None, Some(vec![
    (ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_obj(None, None, None, Some(f64::NAN), None, None).unwrap()).clone(),
]), None).unwrap(), &"finite positive geometry").unwrap();
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, None, Some(vec![
    (ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_obj(None, None, None, Some(i32::from_ne_bytes((4294967295u32).to_ne_bytes()) as f64 as f64), None, None).unwrap()).clone(),
]), None).unwrap(), &"finite positive geometry").unwrap();
    });
}

#[test]
fn inline_object_leading_boundary_must_be_fixed() {
    testlib::run("org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.inlineObjectLeadingBoundaryMustBeFixed", "org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.inlineObjectLeadingBoundaryMustBeFixed", || {
        let mut t = TestTraceRecorder::new("ParagraphLayoutEngineValidationCoverageTest");
        t.section(&"inlineObjectLeadingBoundaryMustBeFixed");
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, None, Some(vec![
    (ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_obj(None, None, None, None, Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.5f64), Some(0.0), Some(false)).unwrap()), None).unwrap()).clone(),
]), None).unwrap(), &"cannot shrink its leading boundary").unwrap();
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, None, Some(vec![
    (ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_obj(None, None, None, None, Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0), Some(0.5f64), Some(false)).unwrap()), None).unwrap()).clone(),
]), None).unwrap(), &"cannot discard advance at its leading boundary").unwrap();
    });
}

#[test]
fn inline_object_trailing_boundary_must_not_exceed_advance() {
    testlib::run("org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.inlineObjectTrailingBoundaryMustNotExceedAdvance", "org.tiqian.layout.ParagraphLayoutEngineValidationCoverageTest.inlineObjectTrailingBoundaryMustNotExceedAdvance", || {
        let mut t = TestTraceRecorder::new("ParagraphLayoutEngineValidationCoverageTest");
        t.section(&"inlineObjectTrailingBoundaryMustNotExceedAdvance");
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, None, Some(vec![
    (ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_obj(None, None, None, None, None, Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(10.5f64), Some(0.0), Some(false)).unwrap())).unwrap()).clone(),
]), None).unwrap(), &"trailing shrink capacity").unwrap();
        let _ = ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_reject(ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_input(None, None, Some(vec![
    (ParagraphLayoutEngineValidationCoverageSupport::paragraph_layout_engine_validation_coverage_support_obj(None, None, None, None, None, Some(InlineObjectBoundaryAdjustment::new(Some(false), None, Some(0.0), Some(10.5f64), Some(false)).unwrap())).unwrap()).clone(),
]), None).unwrap(), &"trailing line-end discard").unwrap();
    });
}
