#![cfg(test)]

use crate::org::tiqian::core::cluster::Cluster;
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
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::size::Size;
use crate::org::tiqian::core::source_boundary_bias::SourceBoundaryBias;
use crate::org::tiqian::core::source_interaction_boundaries::SourceInteractionBoundaries;
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


#[derive(Debug, Clone, PartialEq)]
pub enum CoreBoundaryTestSourceGraphemeBoundariesWithRegionalIndicatorFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreBoundaryTestSourceGraphemeBoundariesWithRegionalIndicatorFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreBoundaryTestSourceGraphemeBoundariesWithRegionalIndicatorFault) -> Self {
        match value {
            CoreBoundaryTestSourceGraphemeBoundariesWithRegionalIndicatorFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestSourceGraphemeBoundariesWithRegionalIndicatorFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreBoundaryTestSourceGraphemeBoundariesWithRegionalIndicatorFault) -> Self {
        match value {
            CoreBoundaryTestSourceGraphemeBoundariesWithRegionalIndicatorFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestSourceGraphemeBoundariesWithRegionalIndicatorFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreBoundaryTestSourceGraphemeBoundariesWithRegionalIndicatorFault) -> Self {
        match value {
            CoreBoundaryTestSourceGraphemeBoundariesWithRegionalIndicatorFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreBoundaryTestSourceGraphemeBoundariesWithRegionalIndicatorFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreBoundaryTestSourceGraphemeBoundariesWithRegionalIndicatorFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreBoundaryTestSourceGraphemeBoundariesWithRegionalIndicatorFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreBoundaryTestSourceGraphemeBoundariesWithRegionalIndicatorFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreBoundaryTestSourceGraphemeBoundariesWithRegionalIndicatorFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreBoundaryTestSourceGraphemeBoundariesWithRegionalIndicatorFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreBoundaryTestSourceGraphemeBoundariesWithHangulSyllableFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreBoundaryTestSourceGraphemeBoundariesWithHangulSyllableFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreBoundaryTestSourceGraphemeBoundariesWithHangulSyllableFault) -> Self {
        match value {
            CoreBoundaryTestSourceGraphemeBoundariesWithHangulSyllableFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestSourceGraphemeBoundariesWithHangulSyllableFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreBoundaryTestSourceGraphemeBoundariesWithHangulSyllableFault) -> Self {
        match value {
            CoreBoundaryTestSourceGraphemeBoundariesWithHangulSyllableFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestSourceGraphemeBoundariesWithHangulSyllableFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreBoundaryTestSourceGraphemeBoundariesWithHangulSyllableFault) -> Self {
        match value {
            CoreBoundaryTestSourceGraphemeBoundariesWithHangulSyllableFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreBoundaryTestSourceGraphemeBoundariesWithHangulSyllableFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreBoundaryTestSourceGraphemeBoundariesWithHangulSyllableFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreBoundaryTestSourceGraphemeBoundariesWithHangulSyllableFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreBoundaryTestSourceGraphemeBoundariesWithHangulSyllableFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreBoundaryTestSourceGraphemeBoundariesWithHangulSyllableFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreBoundaryTestSourceGraphemeBoundariesWithHangulSyllableFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreBoundaryTestSourceGraphemeBoundariesWithHangulLeadingJamoFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreBoundaryTestSourceGraphemeBoundariesWithHangulLeadingJamoFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreBoundaryTestSourceGraphemeBoundariesWithHangulLeadingJamoFault) -> Self {
        match value {
            CoreBoundaryTestSourceGraphemeBoundariesWithHangulLeadingJamoFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestSourceGraphemeBoundariesWithHangulLeadingJamoFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreBoundaryTestSourceGraphemeBoundariesWithHangulLeadingJamoFault) -> Self {
        match value {
            CoreBoundaryTestSourceGraphemeBoundariesWithHangulLeadingJamoFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestSourceGraphemeBoundariesWithHangulLeadingJamoFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreBoundaryTestSourceGraphemeBoundariesWithHangulLeadingJamoFault) -> Self {
        match value {
            CoreBoundaryTestSourceGraphemeBoundariesWithHangulLeadingJamoFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreBoundaryTestSourceGraphemeBoundariesWithHangulLeadingJamoFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreBoundaryTestSourceGraphemeBoundariesWithHangulLeadingJamoFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreBoundaryTestSourceGraphemeBoundariesWithHangulLeadingJamoFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreBoundaryTestSourceGraphemeBoundariesWithHangulLeadingJamoFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreBoundaryTestSourceGraphemeBoundariesWithHangulLeadingJamoFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreBoundaryTestSourceGraphemeBoundariesWithHangulLeadingJamoFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreBoundaryTestSourceGraphemeBoundariesWithEmojiZwjSequenceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreBoundaryTestSourceGraphemeBoundariesWithEmojiZwjSequenceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreBoundaryTestSourceGraphemeBoundariesWithEmojiZwjSequenceFault) -> Self {
        match value {
            CoreBoundaryTestSourceGraphemeBoundariesWithEmojiZwjSequenceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestSourceGraphemeBoundariesWithEmojiZwjSequenceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreBoundaryTestSourceGraphemeBoundariesWithEmojiZwjSequenceFault) -> Self {
        match value {
            CoreBoundaryTestSourceGraphemeBoundariesWithEmojiZwjSequenceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestSourceGraphemeBoundariesWithEmojiZwjSequenceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreBoundaryTestSourceGraphemeBoundariesWithEmojiZwjSequenceFault) -> Self {
        match value {
            CoreBoundaryTestSourceGraphemeBoundariesWithEmojiZwjSequenceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreBoundaryTestSourceGraphemeBoundariesWithEmojiZwjSequenceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreBoundaryTestSourceGraphemeBoundariesWithEmojiZwjSequenceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreBoundaryTestSourceGraphemeBoundariesWithEmojiZwjSequenceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreBoundaryTestSourceGraphemeBoundariesWithEmojiZwjSequenceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreBoundaryTestSourceGraphemeBoundariesWithEmojiZwjSequenceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreBoundaryTestSourceGraphemeBoundariesWithEmojiZwjSequenceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreBoundaryTestSourceGraphemeBoundariesWithEmojiModifierFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreBoundaryTestSourceGraphemeBoundariesWithEmojiModifierFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreBoundaryTestSourceGraphemeBoundariesWithEmojiModifierFault) -> Self {
        match value {
            CoreBoundaryTestSourceGraphemeBoundariesWithEmojiModifierFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestSourceGraphemeBoundariesWithEmojiModifierFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreBoundaryTestSourceGraphemeBoundariesWithEmojiModifierFault) -> Self {
        match value {
            CoreBoundaryTestSourceGraphemeBoundariesWithEmojiModifierFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestSourceGraphemeBoundariesWithEmojiModifierFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreBoundaryTestSourceGraphemeBoundariesWithEmojiModifierFault) -> Self {
        match value {
            CoreBoundaryTestSourceGraphemeBoundariesWithEmojiModifierFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreBoundaryTestSourceGraphemeBoundariesWithEmojiModifierFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreBoundaryTestSourceGraphemeBoundariesWithEmojiModifierFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreBoundaryTestSourceGraphemeBoundariesWithEmojiModifierFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreBoundaryTestSourceGraphemeBoundariesWithEmojiModifierFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreBoundaryTestSourceGraphemeBoundariesWithEmojiModifierFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreBoundaryTestSourceGraphemeBoundariesWithEmojiModifierFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreBoundaryTestSourceGraphemeBoundariesReturnsSingleBoundaryForEmptyTextFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreBoundaryTestSourceGraphemeBoundariesReturnsSingleBoundaryForEmptyTextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreBoundaryTestSourceGraphemeBoundariesReturnsSingleBoundaryForEmptyTextFault) -> Self {
        match value {
            CoreBoundaryTestSourceGraphemeBoundariesReturnsSingleBoundaryForEmptyTextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestSourceGraphemeBoundariesReturnsSingleBoundaryForEmptyTextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreBoundaryTestSourceGraphemeBoundariesReturnsSingleBoundaryForEmptyTextFault) -> Self {
        match value {
            CoreBoundaryTestSourceGraphemeBoundariesReturnsSingleBoundaryForEmptyTextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestSourceGraphemeBoundariesReturnsSingleBoundaryForEmptyTextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreBoundaryTestSourceGraphemeBoundariesReturnsSingleBoundaryForEmptyTextFault) -> Self {
        match value {
            CoreBoundaryTestSourceGraphemeBoundariesReturnsSingleBoundaryForEmptyTextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreBoundaryTestSourceGraphemeBoundariesReturnsSingleBoundaryForEmptyTextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreBoundaryTestSourceGraphemeBoundariesReturnsSingleBoundaryForEmptyTextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreBoundaryTestSourceGraphemeBoundariesReturnsSingleBoundaryForEmptyTextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreBoundaryTestSourceGraphemeBoundariesReturnsSingleBoundaryForEmptyTextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreBoundaryTestSourceGraphemeBoundariesReturnsSingleBoundaryForEmptyTextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreBoundaryTestSourceGraphemeBoundariesReturnsSingleBoundaryForEmptyTextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreBoundaryTestInteractionBoundariesWithTextRangeFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreBoundaryTestInteractionBoundariesWithTextRangeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreBoundaryTestInteractionBoundariesWithTextRangeFault) -> Self {
        match value {
            CoreBoundaryTestInteractionBoundariesWithTextRangeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestInteractionBoundariesWithTextRangeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreBoundaryTestInteractionBoundariesWithTextRangeFault) -> Self {
        match value {
            CoreBoundaryTestInteractionBoundariesWithTextRangeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestInteractionBoundariesWithTextRangeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreBoundaryTestInteractionBoundariesWithTextRangeFault) -> Self {
        match value {
            CoreBoundaryTestInteractionBoundariesWithTextRangeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreBoundaryTestInteractionBoundariesWithTextRangeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreBoundaryTestInteractionBoundariesWithTextRangeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreBoundaryTestInteractionBoundariesWithTextRangeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreBoundaryTestInteractionBoundariesWithTextRangeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreBoundaryTestInteractionBoundariesWithTextRangeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreBoundaryTestInteractionBoundariesWithTextRangeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    LayoutQueriesGetSelectionOffsetForPositionFaultFault(crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault),
}

impl From<CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault) -> Self {
        match value {
            CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault) -> Self {
        match value {
            CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault) -> Self {
        match value {
            CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault) -> Self {
        match value {
            CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault> for crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault {
    fn from(value: CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault) -> Self {
        match value {
            CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault> for CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault {
    fn from(value: crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault) -> Self {
        CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClustersFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    LayoutQueriesGetSelectionOffsetForPositionFaultFault(crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault),
}

impl From<CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault) -> Self {
        match value {
            CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault) -> Self {
        match value {
            CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault) -> Self {
        match value {
            CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault) -> Self {
        match value {
            CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault> for crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault {
    fn from(value: CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault) -> Self {
        match value {
            CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault> for CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault {
    fn from(value: crate::org::tiqian::core::layout_queries::LayoutQueriesGetSelectionOffsetForPositionFault) -> Self {
        CoreBoundaryTestGetSelectionOffsetForPositionReturnsStartOfFirstClusterFault::LayoutQueriesGetSelectionOffsetForPositionFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreBoundaryTestCoerceToInteractionBoundaryWithSurrogatePairFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreBoundaryTestCoerceToInteractionBoundaryWithSurrogatePairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreBoundaryTestCoerceToInteractionBoundaryWithSurrogatePairFault) -> Self {
        match value {
            CoreBoundaryTestCoerceToInteractionBoundaryWithSurrogatePairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestCoerceToInteractionBoundaryWithSurrogatePairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreBoundaryTestCoerceToInteractionBoundaryWithSurrogatePairFault) -> Self {
        match value {
            CoreBoundaryTestCoerceToInteractionBoundaryWithSurrogatePairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestCoerceToInteractionBoundaryWithSurrogatePairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreBoundaryTestCoerceToInteractionBoundaryWithSurrogatePairFault) -> Self {
        match value {
            CoreBoundaryTestCoerceToInteractionBoundaryWithSurrogatePairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreBoundaryTestCoerceToInteractionBoundaryWithSurrogatePairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreBoundaryTestCoerceToInteractionBoundaryWithSurrogatePairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreBoundaryTestCoerceToInteractionBoundaryWithSurrogatePairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreBoundaryTestCoerceToInteractionBoundaryWithSurrogatePairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreBoundaryTestCoerceToInteractionBoundaryWithSurrogatePairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreBoundaryTestCoerceToInteractionBoundaryWithSurrogatePairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreBoundaryTestCoerceToInteractionBoundaryWithInvalidSurrogatePairFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreBoundaryTestCoerceToInteractionBoundaryWithInvalidSurrogatePairFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreBoundaryTestCoerceToInteractionBoundaryWithInvalidSurrogatePairFault) -> Self {
        match value {
            CoreBoundaryTestCoerceToInteractionBoundaryWithInvalidSurrogatePairFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestCoerceToInteractionBoundaryWithInvalidSurrogatePairFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreBoundaryTestCoerceToInteractionBoundaryWithInvalidSurrogatePairFault) -> Self {
        match value {
            CoreBoundaryTestCoerceToInteractionBoundaryWithInvalidSurrogatePairFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestCoerceToInteractionBoundaryWithInvalidSurrogatePairFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreBoundaryTestCoerceToInteractionBoundaryWithInvalidSurrogatePairFault) -> Self {
        match value {
            CoreBoundaryTestCoerceToInteractionBoundaryWithInvalidSurrogatePairFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreBoundaryTestCoerceToInteractionBoundaryWithInvalidSurrogatePairFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreBoundaryTestCoerceToInteractionBoundaryWithInvalidSurrogatePairFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreBoundaryTestCoerceToInteractionBoundaryWithInvalidSurrogatePairFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreBoundaryTestCoerceToInteractionBoundaryWithInvalidSurrogatePairFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreBoundaryTestCoerceToInteractionBoundaryWithInvalidSurrogatePairFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreBoundaryTestCoerceToInteractionBoundaryWithInvalidSurrogatePairFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreBoundaryTestCoerceToInteractionBoundaryNearestChoosesCloserFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreBoundaryTestCoerceToInteractionBoundaryNearestChoosesCloserFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreBoundaryTestCoerceToInteractionBoundaryNearestChoosesCloserFault) -> Self {
        match value {
            CoreBoundaryTestCoerceToInteractionBoundaryNearestChoosesCloserFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestCoerceToInteractionBoundaryNearestChoosesCloserFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreBoundaryTestCoerceToInteractionBoundaryNearestChoosesCloserFault) -> Self {
        match value {
            CoreBoundaryTestCoerceToInteractionBoundaryNearestChoosesCloserFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestCoerceToInteractionBoundaryNearestChoosesCloserFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreBoundaryTestCoerceToInteractionBoundaryNearestChoosesCloserFault) -> Self {
        match value {
            CoreBoundaryTestCoerceToInteractionBoundaryNearestChoosesCloserFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreBoundaryTestCoerceToInteractionBoundaryNearestChoosesCloserFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreBoundaryTestCoerceToInteractionBoundaryNearestChoosesCloserFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreBoundaryTestCoerceToInteractionBoundaryNearestChoosesCloserFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreBoundaryTestCoerceToInteractionBoundaryNearestChoosesCloserFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreBoundaryTestCoerceToInteractionBoundaryNearestChoosesCloserFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreBoundaryTestCoerceToInteractionBoundaryNearestChoosesCloserFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreBoundaryTestCoerceToInteractionBoundaryForwardReturnsNextBoundaryFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreBoundaryTestCoerceToInteractionBoundaryForwardReturnsNextBoundaryFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreBoundaryTestCoerceToInteractionBoundaryForwardReturnsNextBoundaryFault) -> Self {
        match value {
            CoreBoundaryTestCoerceToInteractionBoundaryForwardReturnsNextBoundaryFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestCoerceToInteractionBoundaryForwardReturnsNextBoundaryFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreBoundaryTestCoerceToInteractionBoundaryForwardReturnsNextBoundaryFault) -> Self {
        match value {
            CoreBoundaryTestCoerceToInteractionBoundaryForwardReturnsNextBoundaryFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestCoerceToInteractionBoundaryForwardReturnsNextBoundaryFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreBoundaryTestCoerceToInteractionBoundaryForwardReturnsNextBoundaryFault) -> Self {
        match value {
            CoreBoundaryTestCoerceToInteractionBoundaryForwardReturnsNextBoundaryFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreBoundaryTestCoerceToInteractionBoundaryForwardReturnsNextBoundaryFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreBoundaryTestCoerceToInteractionBoundaryForwardReturnsNextBoundaryFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreBoundaryTestCoerceToInteractionBoundaryForwardReturnsNextBoundaryFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreBoundaryTestCoerceToInteractionBoundaryForwardReturnsNextBoundaryFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreBoundaryTestCoerceToInteractionBoundaryForwardReturnsNextBoundaryFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreBoundaryTestCoerceToInteractionBoundaryForwardReturnsNextBoundaryFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreBoundaryTestCoerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEndFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<CoreBoundaryTestCoerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEndFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: CoreBoundaryTestCoerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEndFault) -> Self {
        match value {
            CoreBoundaryTestCoerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEndFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestCoerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEndFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: CoreBoundaryTestCoerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEndFault) -> Self {
        match value {
            CoreBoundaryTestCoerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEndFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<CoreBoundaryTestCoerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEndFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: CoreBoundaryTestCoerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEndFault) -> Self {
        match value {
            CoreBoundaryTestCoerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEndFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for CoreBoundaryTestCoerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEndFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        CoreBoundaryTestCoerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEndFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for CoreBoundaryTestCoerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEndFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        CoreBoundaryTestCoerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEndFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for CoreBoundaryTestCoerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEndFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        CoreBoundaryTestCoerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEndFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn coerce_to_interaction_boundary_backward_returns_boundary_when_at_end() {
    testlib::run("org.tiqian.core.CoreBoundaryTest.coerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEnd", "org.tiqian.core.CoreBoundaryTest.coerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEnd", || {
        TestTraceRecorder::new("CoreBoundaryTest").section(&"coerceToInteractionBoundaryBackwardReturnsBoundaryWhenAtEnd");
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, SourceInteractionBoundaries::source_interaction_boundaries_coerce_to_interaction_boundary(&"abc", 3, TextRange::new(0u32, 3u32).unwrap(), SourceBoundaryBias::Backward), None).unwrap();
    });
}

#[test]
fn coerce_to_interaction_boundary_forward_returns_next_boundary() {
    testlib::run("org.tiqian.core.CoreBoundaryTest.coerceToInteractionBoundaryForwardReturnsNextBoundary", "org.tiqian.core.CoreBoundaryTest.coerceToInteractionBoundaryForwardReturnsNextBoundary", || {
        TestTraceRecorder::new("CoreBoundaryTest").section(&"coerceToInteractionBoundaryForwardReturnsNextBoundary");
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, SourceInteractionBoundaries::source_interaction_boundaries_coerce_to_interaction_boundary(&"abc", 3, TextRange::new(0u32, 3u32).unwrap(), SourceBoundaryBias::Forward), None).unwrap();
    });
}

#[test]
fn coerce_to_interaction_boundary_nearest_chooses_closer() {
    testlib::run("org.tiqian.core.CoreBoundaryTest.coerceToInteractionBoundaryNearestChoosesCloser", "org.tiqian.core.CoreBoundaryTest.coerceToInteractionBoundaryNearestChoosesCloser", || {
        TestTraceRecorder::new("CoreBoundaryTest").section(&"coerceToInteractionBoundaryNearestChoosesCloser");
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, SourceInteractionBoundaries::source_interaction_boundaries_coerce_to_interaction_boundary(&"abcdef", 3, TextRange::new(0u32, 6u32).unwrap(), SourceBoundaryBias::Nearest), None).unwrap();
    });
}

#[test]
fn coerce_to_interaction_boundary_with_surrogate_pair() {
    testlib::run("org.tiqian.core.CoreBoundaryTest.coerceToInteractionBoundaryWithSurrogatePair", "org.tiqian.core.CoreBoundaryTest.coerceToInteractionBoundaryWithSurrogatePair", || {
        TestTraceRecorder::new("CoreBoundaryTest").section(&"coerceToInteractionBoundaryWithSurrogatePair");
        let emoji = TestHelpers::test_helpers_surrogate_text(&vec![55357, 56832]);
        let text = format!("{}{}{}",
            "a",
            emoji,
            "b"
        );
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, SourceInteractionBoundaries::source_interaction_boundaries_coerce_to_interaction_boundary(text.as_str(), 3, TextRange::new(0u32, u_string::unit_count(&(text))).unwrap(), SourceBoundaryBias::Nearest),
None).unwrap();
    });
}

#[test]
fn coerce_to_interaction_boundary_with_invalid_surrogate_pair() {
    testlib::run("org.tiqian.core.CoreBoundaryTest.coerceToInteractionBoundaryWithInvalidSurrogatePair", "org.tiqian.core.CoreBoundaryTest.coerceToInteractionBoundaryWithInvalidSurrogatePair", || {
        TestTraceRecorder::new("CoreBoundaryTest").section(&"coerceToInteractionBoundaryWithInvalidSurrogatePair");
        let text = format!("{}{}",
            TestHelpers::test_helpers_surrogate_text(&vec![55296]),
            "A"
        );
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, SourceInteractionBoundaries::source_interaction_boundaries_coerce_to_interaction_boundary(text.as_str(), 1, TextRange::new(0u32, u_string::unit_count(&(text))).unwrap(), SourceBoundaryBias::Nearest),
None).unwrap();
    });
}

#[test]
fn source_grapheme_boundaries_with_hangul_leading_jamo() {
    testlib::run("org.tiqian.core.CoreBoundaryTest.sourceGraphemeBoundariesWithHangulLeadingJamo", "org.tiqian.core.CoreBoundaryTest.sourceGraphemeBoundariesWithHangulLeadingJamo", || {
        TestTraceRecorder::new("CoreBoundaryTest").section(&"sourceGraphemeBoundariesWithHangulLeadingJamo");
        let text = "각".to_string();
        let boundaries = SourceInteractionBoundaries::source_interaction_boundaries_source_grapheme_boundaries(text.as_str(), TextRange::new(0u32, 3u32).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((match boundaries.iter().position(|e| e == &3) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[test]
fn source_grapheme_boundaries_with_hangul_syllable() {
    testlib::run("org.tiqian.core.CoreBoundaryTest.sourceGraphemeBoundariesWithHangulSyllable", "org.tiqian.core.CoreBoundaryTest.sourceGraphemeBoundariesWithHangulSyllable", || {
        TestTraceRecorder::new("CoreBoundaryTest").section(&"sourceGraphemeBoundariesWithHangulSyllable");
        let text = "가".to_string();
        let boundaries = SourceInteractionBoundaries::source_interaction_boundaries_source_grapheme_boundaries(text.as_str(), TextRange::new(0u32, 1u32).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, boundaries[0usize], None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, boundaries[usize::try_from(u32::wrapping_sub(u32::try_from((boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)], None).unwrap();
    });
}

#[test]
fn source_grapheme_boundaries_with_regional_indicator() {
    testlib::run("org.tiqian.core.CoreBoundaryTest.sourceGraphemeBoundariesWithRegionalIndicator", "org.tiqian.core.CoreBoundaryTest.sourceGraphemeBoundariesWithRegionalIndicator", || {
        TestTraceRecorder::new("CoreBoundaryTest").section(&"sourceGraphemeBoundariesWithRegionalIndicator");
        let text = TestHelpers::test_helpers_surrogate_text(&vec![55356, 56808, 55356, 56806]);
        let boundaries = SourceInteractionBoundaries::source_interaction_boundaries_source_grapheme_boundaries(text.as_str(), TextRange::new(0u32, u_string::unit_count(&(text))).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((match boundaries.iter().position(|e| e == &u_string::unit_count(&(text))) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }).to_ne_bytes())) <= 2147483647,
None).unwrap();
    });
}

#[test]
fn source_grapheme_boundaries_with_emoji_zwj_sequence() {
    testlib::run("org.tiqian.core.CoreBoundaryTest.sourceGraphemeBoundariesWithEmojiZwjSequence", "org.tiqian.core.CoreBoundaryTest.sourceGraphemeBoundariesWithEmojiZwjSequence", || {
        TestTraceRecorder::new("CoreBoundaryTest").section(&"sourceGraphemeBoundariesWithEmojiZwjSequence");
        let text = TestHelpers::test_helpers_surrogate_text(&vec![55357, 56425, 8205, 55357, 56425]);
        let boundaries = SourceInteractionBoundaries::source_interaction_boundaries_source_grapheme_boundaries(text.as_str(), TextRange::new(0u32, u_string::unit_count(&(text))).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, boundaries[0usize], None).unwrap();
    });
}

#[test]
fn source_grapheme_boundaries_with_emoji_modifier() {
    testlib::run("org.tiqian.core.CoreBoundaryTest.sourceGraphemeBoundariesWithEmojiModifier", "org.tiqian.core.CoreBoundaryTest.sourceGraphemeBoundariesWithEmojiModifier", || {
        TestTraceRecorder::new("CoreBoundaryTest").section(&"sourceGraphemeBoundariesWithEmojiModifier");
        let text = TestHelpers::test_helpers_surrogate_text(&vec![55357, 56425, 55356, 57339]);
        let boundaries = SourceInteractionBoundaries::source_interaction_boundaries_source_grapheme_boundaries(text.as_str(), TextRange::new(0u32, u_string::unit_count(&(text))).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((match boundaries.iter().position(|e| e == &u_string::unit_count(&(text))) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }).to_ne_bytes())) <= 2147483647,
None).unwrap();
    });
}

#[test]
fn source_grapheme_boundaries_returns_single_boundary_for_empty_text() {
    testlib::run("org.tiqian.core.CoreBoundaryTest.sourceGraphemeBoundariesReturnsSingleBoundaryForEmptyText", "org.tiqian.core.CoreBoundaryTest.sourceGraphemeBoundariesReturnsSingleBoundaryForEmptyText", || {
        TestTraceRecorder::new("CoreBoundaryTest").section(&"sourceGraphemeBoundariesReturnsSingleBoundaryForEmptyText");
        let boundaries = SourceInteractionBoundaries::source_interaction_boundaries_source_grapheme_boundaries(&"", TextRange::new(0u32, 0u32).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, boundaries[0usize], None).unwrap();
    });
}

#[test]
fn interaction_boundaries_with_text_range() {
    testlib::run("org.tiqian.core.CoreBoundaryTest.interactionBoundariesWithTextRange", "org.tiqian.core.CoreBoundaryTest.interactionBoundariesWithTextRange", || {
        TestTraceRecorder::new("CoreBoundaryTest").section(&"interactionBoundariesWithTextRange");
        let boundaries = SourceInteractionBoundaries::source_interaction_boundaries_interaction_boundaries(&"abc", TextRange::new(1u32, 2u32).unwrap());
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(&"[1, 2]", CoreBoundaryTestHelpers::core_boundary_test_helpers_render_ints(&boundaries).as_str(), None).unwrap();
    });
}

#[test]
fn get_selection_offset_for_position_returns_start_of_first_cluster() {
    testlib::run("org.tiqian.core.CoreBoundaryTest.getSelectionOffsetForPositionReturnsStartOfFirstCluster", "org.tiqian.core.CoreBoundaryTest.getSelectionOffsetForPositionReturnsStartOfFirstCluster", || {
        TestTraceRecorder::new("CoreBoundaryTest").section(&"getSelectionOffsetForPositionReturnsStartOfFirstCluster");
        let value = CoreBoundaryTestHelpers::core_boundary_test_helpers_interaction_result(&"abc").unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_selection_offset_for_position((value).clone(), 0.0f64, 10.0f64).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, LayoutQueries::layout_queries_get_selection_offset_for_position((value).clone(), 10.0f64, 10.0f64).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, LayoutQueries::layout_queries_get_selection_offset_for_position((value).clone(), 20.0f64, 10.0f64).unwrap(), None).unwrap();
    });
}

#[test]
fn get_selection_offset_for_position_returns_start_of_line_when_empty_clusters() {
    testlib::run("org.tiqian.core.CoreBoundaryTest.getSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClusters", "org.tiqian.core.CoreBoundaryTest.getSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClusters", || {
        TestTraceRecorder::new("CoreBoundaryTest").section(&"getSelectionOffsetForPositionReturnsStartOfLineWhenEmptyClusters");
        let input = LayoutInput::new(TiqianTextContent::new("", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0f64), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some((Ic::zero()).clone()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0f64), Some(1.0f64), Some(2.0f64))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(100.0f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        let line = LineBox::new(TextRange::new(0u32, 0u32).unwrap(), IntRange::new(0u32, 4294967295u32), 15.0f64, 0.0f64, 20.0f64, 0.0f64, 0.0f64, 0.0f64, Some(0.0f64), Some(0.0f64), Some(LineEndReason::ParagraphEnd), Some(0.0f64), Some(vec![]), LineDebugInfo::new(None.clone(),
Some(vec![])));
        let value = LayoutResult::new((input).clone(), Size::new(0.0f64, 20.0f64), vec![], vec![], vec![(line).clone()], LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])));
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, LayoutQueries::layout_queries_get_selection_offset_for_position((value).clone(), 5.0f64, 10.0f64).unwrap(), None).unwrap();
    });
}

#[derive(Clone, Copy)]
pub struct CoreBoundaryTestHelpers;

impl CoreBoundaryTestHelpers {
    pub fn core_boundary_test_helpers_interaction_result(text: &str) -> Result<LayoutResult, TextRangeError> {
        let clusters = vec![
    (Cluster::new(TextRange::new(0u32, 1u32)?, "a", "latin", 10.0f64, Some("a".to_string()), Some(0.0f64), Some(0.0f64), Some(0.0f64))).clone(),
    (Cluster::new(TextRange::new(1u32, 2u32)?, "b", "latin", 10.0f64, Some("b".to_string()), Some(0.0f64), Some(0.0f64), Some(0.0f64))).clone(),
    (Cluster::new(TextRange::new(2u32, 3u32)?, "c", "latin", 10.0f64, Some("c".to_string()), Some(0.0f64), Some(0.0f64), Some(0.0f64))).clone(),
];
        let line = LineBox::new(TextRange::new(0u32, 3u32)?, IntRange::new(0u32, 2u32), 15.0f64, 0.0f64, 20.0f64, 30.0f64, 30.0f64, 30.0f64, Some(0.0f64), Some(0.0f64), Some(LineEndReason::ParagraphEnd), Some(0.0f64), Some(vec![]), LineDebugInfo::new(None.clone(), Some(vec![])));
        let input = LayoutInput::new(TiqianTextContent::new(text, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0f64), Some(InlineAttachment::None))),
Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, None, Some((Ic::zero()).clone()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0f64), Some(1.0f64), Some(2.0f64))), Some(LineLengthGrid::new(Some(true), None)),
Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(100.0f64, Some(f64::INFINITY), Some(2147483647))?,
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]));
        return Ok(LayoutResult::new((input).clone(), Size::new(30.0f64, 20.0f64), (clusters).clone(), vec![], vec![(line).clone()], LayoutDebugInfo::new(None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]),
Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), None, None, None, None, Some(vec![]), None, None, Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))));
    }

    pub fn core_boundary_test_helpers_render_ints(values: &Vec<u32>) -> String {
        let mut output = "[".to_string();
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if i32::from_ne_bytes((index).to_ne_bytes()) > (0) {
                output += &(", ");
            }
            output += &(crate::runtime::int_text::IntText::int_text(values[usize::try_from(index).unwrap_or(0)]));
            index = u32::wrapping_add(index, 1);
        }
        return format!("{}{}",
            output,
            "]"
        );
    }
}
