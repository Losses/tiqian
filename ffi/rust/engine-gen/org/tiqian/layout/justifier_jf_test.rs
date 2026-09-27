#![cfg(test)]

use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::east_asian_spacing_edges::EastAsianSpacingEdges;
use crate::org::tiqian::core::east_asian_spacing_value::EastAsianSpacingValue;
use crate::org::tiqian::core::inline_object_preferred_stretch::InlineObjectPreferredStretch;
use crate::org::tiqian::core::inline_object_preferred_stretch_kind::InlineObjectPreferredStretchKind;
use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::justifier::JustificationAllocation;
use crate::org::tiqian::layout::justifier::JustificationPlan;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkChannel;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::layout::punctuation_model::GlueKind;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault) -> Self {
        match value {
            JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault) -> Self {
        match value {
            JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault) -> Self {
        match value {
            JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierJfTestZeroCjkLatinHeadroomProducesNoOpportunitiesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault) -> Self {
        match value {
            JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault) -> Self {
        match value {
            JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault) -> Self {
        match value {
            JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierJfTestVirtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalseFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault) -> Self {
        match value {
            JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault) -> Self {
        match value {
            JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault) -> Self {
        match value {
            JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierJfTestVirtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalseFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault) -> Self {
        match value {
            JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault) -> Self {
        match value {
            JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault) -> Self {
        match value {
            JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierJfTestTypedSpaceAndWordSpacePredicateEdgeConditionsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault) -> Self {
        match value {
            JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault) -> Self {
        match value {
            JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault) -> Self {
        match value {
            JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierJfTestSingleClusterRangeProducesNoOpportunitiesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault) -> Self {
        match value {
            JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault) -> Self {
        match value {
            JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault) -> Self {
        match value {
            JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierJfTestPreferredInlineObjectBoundaryOutOfBoundsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault) -> Self {
        match value {
            JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault) -> Self {
        match value {
            JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault) -> Self {
        match value {
            JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierJfTestEmptyLineClusterRangeSkipsUniformSpaceLoopFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault) -> Self {
        match value {
            JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault) -> Self {
        match value {
            JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault) -> Self {
        match value {
            JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierJfTestCompressionWithZeroSurplusAndZeroCapacityFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault) -> Self {
        match value {
            JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault) -> Self {
        match value {
            JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault) -> Self {
        match value {
            JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierJfTestCompressSubnormalUnderflowShrinkZeroFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault) -> Self {
        match value {
            JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault) -> Self {
        match value {
            JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault) -> Self {
        match value {
            JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierJfTestClosedSpaceGapInUniformSpaceWhenWordSpaceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault) -> Self {
        match value {
            JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault) -> Self {
        match value {
            JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault) -> Self {
        match value {
            JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierJfTestClosedSpaceGapInTypedSinoWesternAndUniformSpaceFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault) -> Self {
        match value {
            JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault) -> Self {
        match value {
            JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault) -> Self {
        match value {
            JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierJfTestCjkLatinMixedZeroAndPositiveCapacityAllocationFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault) -> Self {
        match value {
            JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault) -> Self {
        match value {
            JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault) -> Self {
        match value {
            JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierJfTestAttachedInlineVirtualSinoWesternZeroHeadroomInAllocateFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault) -> Self {
        match value {
            JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault) -> Self {
        match value {
            JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault) -> Self {
        match value {
            JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        JustifierJfTestAttachedInlineVirtualSinoWesternBoundaryOutOfBoundsFault::TracedAssertionsFailFaultFault(value)
    }
}

pub static JUSTIFIER_JF_TEST_SUPPORT_EM: Mutex<f64> = Mutex::new(16.0f64);

#[derive(Clone, Copy)]
pub struct JustifierJfTestSupport;

impl JustifierJfTestSupport {

    pub fn justifier_jf_test_support_c(text: &UStr, index: u32, advance: Option<f64>, font_key: Option<UString>) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(index, u32::wrapping_add(index, u_string::unit_count(&(text))))?, text, match &(font_key) { None => UString::from("k"), Some(__option) => __option.to_ustring() }.as_ustr(), match &(advance) { None => { let __guard = JUSTIFIER_JF_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, Some(__option1) => *__option1 }, Some(text.to_ustring()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn justifier_jf_test_support_e(leading: Option<EastAsianSpacingValue>, trailing: Option<EastAsianSpacingValue>, wide: Option<bool>) -> EastAsianSpacingEdges {
        return EastAsianSpacingEdges::new(match &(leading) { None => EastAsianSpacingValue::Other, Some(__option10) => *__option10 }, match &(trailing) { None => EastAsianSpacingValue::Other, Some(__option11) => *__option11 }, match &(wide) { None => false, Some(__option12) => *__option12 });
    }

    pub fn justifier_jf_test_support_justify(c: &Vec<Cluster>, roles: &Vec<FontRole>, edges: &Vec<EastAsianSpacingEdges>, r: IntRange, max_width: f64, font_size: Option<f64>, skip: Option<bool>, skip_reason: Option<UString>, allow: Option<bool>, base: Option<f64>, max: Option<f64>, ns: Option<SortedSetTable<u32>>, nsa: Option<SortedSetTable<u32>>, br: Option<SortedSetTable<u32>>, ph: Option<SortedSetTable<u32>>, v: Option<SortedMapTable<u32, u32>>, vs: Option<SortedSetTable<u32>>, uo: Option<SortedSetTable<u32>>, pref: Option<SortedMapTable<u32, InlineObjectPreferredStretch>>, te: Option<SortedMapTable<u32, ProgressiveBreakTier>>, emg: Option<SortedMapTable<u32, UString>>, pem: Option<SortedMapTable<u32, UString>>) -> Result<JustificationPlan, TextRangeError> {
        let x = Justifier::new(Some(0.5), Some(0.25));
        return Ok(x.justify(&c, &roles, &edges, (r).clone(), max_width, match &(font_size) { None => { let __guard = JUSTIFIER_JF_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }, Some(__option18) => *__option18 }, match &(skip) { None => false, Some(__option19) => *__option19 }, match &(skip_reason) { Some(v) => Some(v.to_ustring()), None => None }.clone(), Some(match &(allow) { None => true, Some(__option20) => *__option20 }), match &(base) { None => 0.25f64, Some(__option21) => *__option21 }, match &(max) { None => 0.5f64, Some(__option22) => *__option22 }, (ns).clone(), (nsa).clone(), (br).clone(), (ph).clone(), (v).clone(), (vs).clone(), (uo).clone(), (pref).clone(), (te).clone(), (emg).clone(), (pem).clone())?);
    }

    pub fn justifier_jf_test_support_set(xs: &Vec<u32>) -> SortedSetTable<u32> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        let mut i0 = 0u32;
        while (i32::from_ne_bytes(((i0) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            b.put(&(xs[usize::try_from(i0).unwrap_or(0)]));
            i0 = u32::wrapping_add(i0, 1);
        }
        return b.clone().build();
    }

    pub fn justifier_jf_test_support_int_map(xs: &Vec<u32>, ys: &Vec<u32>) -> SortedMapTable<u32, u32> {
        let mut b: SortedMapTableBuilder<u32, u32> = SortedTable::sorted_table_map_builder::<u32, u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((xs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            b.put(&(xs[usize::try_from(i).unwrap_or(0)]), &(ys[usize::try_from(i).unwrap_or(0)]));
            i = u32::wrapping_add(i, 1);
        }
        return b.clone().build();
    }

    pub fn justifier_jf_test_support_sec(s: &UStr) {
        TestTraceRecorder::new(&(UStr::new(&[74,117,115,116,105,102,105,101,114,74,102,84,101,115,116]))).section(s);
    }
}

#[test]
fn attached_inline_virtual_sino_western_boundary_out_of_bounds() {
    testlib::run("org.tiqian.layout.JustifierJfTest.attachedInlineVirtualSinoWesternBoundaryOutOfBounds", "org.tiqian.layout.JustifierJfTest.attachedInlineVirtualSinoWesternBoundaryOutOfBounds", || {
        JustifierJfTestSupport::justifier_jf_test_support_sec(UStr::new(&[97,116,116,97,99,104,101,100,73,110,108,105,110,101,86,105,114,116,117,97,108,83,105,110,111,87,101,115,116,101,114,110,66,111,117,110,100,97,114,121,79,117,116,79,102,66,111,117,110,100,115]));
        let c = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[20013]), 0, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[25991]), 1, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[97]), 2, None, None).unwrap()).clone(),
];
        let r = vec![FontRole::CjkText, FontRole::CjkText, FontRole::LatinText];
        let e = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
];
        let v: SortedMapTable<u32, u32> = JustifierJfTestSupport::justifier_jf_test_support_int_map(&vec![4294967295u32, 2, 5], &vec![4294967294u32, 1, 4]);
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((JustifierJfTestSupport::justifier_jf_test_support_justify(&c, &r, &e, IntRange::new(0u32, 2u32), 60 as f64, Some({ let __guard = JUSTIFIER_JF_TEST_SUPPORT_EM.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }), Some(false), None.clone(), Some(true), Some(0.25f64), Some(0.5f64), None, None, None, None, Some((v).clone()), Some(JustifierJfTestSupport::justifier_jf_test_support_set(&vec![4294967295u32, 2, 5])), None, None, None, None, None).unwrap().allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn attached_inline_virtual_sino_western_zero_headroom_in_allocate() {
    testlib::run("org.tiqian.layout.JustifierJfTest.attachedInlineVirtualSinoWesternZeroHeadroomInAllocate", "org.tiqian.layout.JustifierJfTest.attachedInlineVirtualSinoWesternZeroHeadroomInAllocate", || {
        JustifierJfTestSupport::justifier_jf_test_support_sec(UStr::new(&[97,116,116,97,99,104,101,100,73,110,108,105,110,101,86,105,114,116,117,97,108,83,105,110,111,87,101,115,116,101,114,110,90,101,114,111,72,101,97,100,114,111,111,109,73,110,65,108,108,111,99,97,116,101]));
        let c = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[20013]), 0, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[25991]), 1, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[97]), 2, None, None).unwrap()).clone(),
];
        let r = vec![FontRole::CjkText, FontRole::CjkText, FontRole::LatinText];
        let e = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
];
        let p = JustifierJfTestSupport::justifier_jf_test_support_justify(&c, &r, &e, IntRange::new(0u32, 2u32), 60 as f64, Some(16 as f64 as f64), Some(false), None.clone(), Some(true), Some(0.5f64), Some(0.5f64), None, None, None, None, Some(JustifierJfTestSupport::justifier_jf_test_support_int_map(&vec![1], &vec![0])), Some(JustifierJfTestSupport::justifier_jf_test_support_set(&vec![1])), None, None, None, None, None).unwrap();
        let mut has_cjk_latin = false;
        let mut has_inter = false;
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if p.allocations[usize::try_from(i).unwrap_or(0)].kind == GlueKind::CjkLatinSpace {
                has_cjk_latin = true;
            }
            if p.allocations[usize::try_from(i).unwrap_or(0)].kind == GlueKind::CjkInterChar {
                has_inter = true;
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(!has_cjk_latin, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(has_inter, None).unwrap();
    });
}

#[test]
fn cjk_latin_mixed_zero_and_positive_capacity_allocation() {
    testlib::run("org.tiqian.layout.JustifierJfTest.cjkLatinMixedZeroAndPositiveCapacityAllocation", "org.tiqian.layout.JustifierJfTest.cjkLatinMixedZeroAndPositiveCapacityAllocation", || {
        JustifierJfTestSupport::justifier_jf_test_support_sec(UStr::new(&[99,106,107,76,97,116,105,110,77,105,120,101,100,90,101,114,111,65,110,100,80,111,115,105,116,105,118,101,67,97,112,97,99,105,116,121,65,108,108,111,99,97,116,105,111,110]));
        let c = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[20013]), 0, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[32]), 1, Some(2 as f64 as f64), None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[97]), 2, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[98]), 3, None, None).unwrap()).clone(),
];
        let r = vec![FontRole::CjkText, FontRole::LatinText, FontRole::LatinText, FontRole::LatinText];
        let e = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(None, None, None)).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
];
        let p = JustifierJfTestSupport::justifier_jf_test_support_justify(&c, &r, &e, IntRange::new(0u32, 3u32), 54 as f64, Some(16 as f64 as f64), Some(false), None.clone(), Some(true), Some(0.5f64), Some(0.5f64), None, None, None, None, Some(JustifierJfTestSupport::justifier_jf_test_support_int_map(&vec![2], &vec![0])), Some(JustifierJfTestSupport::justifier_jf_test_support_set(&vec![2])), None, None, None, None, None).unwrap();
        let mut la: Vec<JustificationAllocation> = vec![];
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if p.allocations[usize::try_from(i).unwrap_or(0)].kind == GlueKind::CjkLatinSpace {
                la.push((p.allocations[usize::try_from(i).unwrap_or(0)]).clone());
            }
            i = u32::wrapping_add(i, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((la.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, la[0usize].target_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, la[0usize].delta, None).unwrap();
        let q = JustifierJfTestSupport::justifier_jf_test_support_justify(&c, &r, &e, IntRange::new(0u32, 3u32), 60 as f64, Some(16 as f64 as f64), Some(false), None.clone(), Some(true), Some(0.5f64), Some(0.5f64), None, None, None, None, Some(JustifierJfTestSupport::justifier_jf_test_support_int_map(&vec![2], &vec![0])), Some(JustifierJfTestSupport::justifier_jf_test_support_set(&vec![2])), None, None, None, None, None).unwrap();
        let mut lz: Vec<JustificationAllocation> = vec![];
        let mut j = 0u32;
        while (i32::from_ne_bytes(((j) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((q.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if q.allocations[usize::try_from(j).unwrap_or(0)].kind == GlueKind::CjkLatinSpace {
                lz.push((q.allocations[usize::try_from(j).unwrap_or(0)]).clone());
            }
            j = u32::wrapping_add(j, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_equals(1, u32::try_from((lz.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(1, lz[0usize].target_cluster_index, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(6 as f64, lz[0usize].delta, None).unwrap();
    });
}

#[test]
fn closed_space_gap_in_typed_sino_western_and_uniform_space() {
    testlib::run("org.tiqian.layout.JustifierJfTest.closedSpaceGapInTypedSinoWesternAndUniformSpace", "org.tiqian.layout.JustifierJfTest.closedSpaceGapInTypedSinoWesternAndUniformSpace", || {
        JustifierJfTestSupport::justifier_jf_test_support_sec(UStr::new(&[99,108,111,115,101,100,83,112,97,99,101,71,97,112,73,110,84,121,112,101,100,83,105,110,111,87,101,115,116,101,114,110,65,110,100,85,110,105,102,111,114,109,83,112,97,99,101]));
        let c = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[20013]), 0, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[32]), 1, Some(4 as f64 as f64), None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[97]), 2, None, None).unwrap()).clone(),
];
        let r = vec![FontRole::CjkText, FontRole::LatinText, FontRole::LatinText];
        let e = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(None, None, None)).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
];
        let p = JustifierJfTestSupport::justifier_jf_test_support_justify(&c, &r, &e, IntRange::new(0u32, 2u32), 60 as f64, Some(16 as f64 as f64), Some(false), None.clone(), Some(true), Some(0.25f64), Some(0.5f64), Some(JustifierJfTestSupport::justifier_jf_test_support_set(&vec![2])), None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 || p.allocations[0usize].target_cluster_index != 1, None).unwrap();
    });
}

#[test]
fn closed_space_gap_in_uniform_space_when_word_space() {
    testlib::run("org.tiqian.layout.JustifierJfTest.closedSpaceGapInUniformSpaceWhenWordSpace", "org.tiqian.layout.JustifierJfTest.closedSpaceGapInUniformSpaceWhenWordSpace", || {
        JustifierJfTestSupport::justifier_jf_test_support_sec(UStr::new(&[99,108,111,115,101,100,83,112,97,99,101,71,97,112,73,110,85,110,105,102,111,114,109,83,112,97,99,101,87,104,101,110,87,111,114,100,83,112,97,99,101]));
        let c = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[97]), 0, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[32]), 1, Some(4 as f64 as f64), None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[98]), 2, None, None).unwrap()).clone(),
];
        let r = vec![FontRole::LatinText, FontRole::LatinText, FontRole::LatinText];
        let e = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(None, None, None)).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
];
        let p = JustifierJfTestSupport::justifier_jf_test_support_justify(&c, &r, &e, IntRange::new(0u32, 2u32), 60 as f64, Some(16 as f64 as f64), Some(false), None.clone(), Some(true), Some(0.25f64), Some(0.5f64), Some(JustifierJfTestSupport::justifier_jf_test_support_set(&vec![0])), None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 || p.allocations[0usize].target_cluster_index != 1, None).unwrap();
    });
}

#[test]
fn compress_subnormal_underflow_shrink_zero() {
    testlib::run("org.tiqian.layout.JustifierJfTest.compressSubnormalUnderflowShrinkZero", "org.tiqian.layout.JustifierJfTest.compressSubnormalUnderflowShrinkZero", || {
        JustifierJfTestSupport::justifier_jf_test_support_sec(UStr::new(&[99,111,109,112,114,101,115,115,83,117,98,110,111,114,109,97,108,85,110,100,101,114,102,108,111,119,83,104,114,105,110,107,90,101,114,111]));
        let p = Justifier::new(Some(0.5), Some(0.25)).compress(1e-300f64, &vec![
    (ShrinkOpportunity::new(0u32, 1u32, 1e300f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(1e-300f64, p.surplus_before, None).unwrap();
    });
}

#[test]
fn compression_with_zero_surplus_and_zero_capacity() {
    testlib::run("org.tiqian.layout.JustifierJfTest.compressionWithZeroSurplusAndZeroCapacity", "org.tiqian.layout.JustifierJfTest.compressionWithZeroSurplusAndZeroCapacity", || {
        JustifierJfTestSupport::justifier_jf_test_support_sec(UStr::new(&[99,111,109,112,114,101,115,115,105,111,110,87,105,116,104,90,101,114,111,83,117,114,112,108,117,115,65,110,100,90,101,114,111,67,97,112,97,99,105,116,121]));
        let j = Justifier::new(Some(0.5), Some(0.25));
        let a = j.compress(0 as f64, &vec![]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, a.surplus_before, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, a.unfilled_surplus, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((a.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
        let b = j.compress(10 as f64, &vec![
    (ShrinkOpportunity::new(0u32, 1u32, 0 as f64 as f64, ShrinkChannel::TrailingGlue, Some(false))).clone(),
]).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10 as f64, b.surplus_before, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(10 as f64, b.unfilled_surplus, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from((b.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, None).unwrap();
    });
}

#[test]
fn empty_line_cluster_range_skips_uniform_space_loop() {
    testlib::run("org.tiqian.layout.JustifierJfTest.emptyLineClusterRangeSkipsUniformSpaceLoop", "org.tiqian.layout.JustifierJfTest.emptyLineClusterRangeSkipsUniformSpaceLoop", || {
        JustifierJfTestSupport::justifier_jf_test_support_sec(UStr::new(&[101,109,112,116,121,76,105,110,101,67,108,117,115,116,101,114,82,97,110,103,101,83,107,105,112,115,85,110,105,102,111,114,109,83,112,97,99,101,76,111,111,112]));
        let c = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[20013]), 0, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[25991]), 1, None, None).unwrap()).clone(),
];
        let r = vec![FontRole::CjkText, FontRole::CjkText];
        let e = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
];
        let p = JustifierJfTestSupport::justifier_jf_test_support_justify(&c, &r, &e, IntRange::new(1u32, 0u32), 50 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(50 as f64, p.unfilled_deficit, None).unwrap();
    });
}

#[test]
fn preferred_inline_object_boundary_out_of_bounds() {
    testlib::run("org.tiqian.layout.JustifierJfTest.preferredInlineObjectBoundaryOutOfBounds", "org.tiqian.layout.JustifierJfTest.preferredInlineObjectBoundaryOutOfBounds", || {
        JustifierJfTestSupport::justifier_jf_test_support_sec(UStr::new(&[112,114,101,102,101,114,114,101,100,73,110,108,105,110,101,79,98,106,101,99,116,66,111,117,110,100,97,114,121,79,117,116,79,102,66,111,117,110,100,115]));
        let c = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[20013]), 0, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[25991]), 1, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[23383]), 2, None, None).unwrap()).clone(),
];
        let r = vec![FontRole::CjkText, FontRole::CjkText, FontRole::CjkText];
        let e = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
];
        let mut b: SortedMapTableBuilder<u32, InlineObjectPreferredStretch> = SortedTable::sorted_table_map_builder::<u32,
InlineObjectPreferredStretch>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        let ks = vec![4294967295u32, 2, 5];
        let mut i2 = 0u32;
        while (i32::from_ne_bytes(((i2) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((ks.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            b.put(&(ks[usize::try_from(i2).unwrap_or(0)]), &(InlineObjectPreferredStretch::new(InlineObjectPreferredStretchKind::Relation, 0 as f64 as f64, 4 as f64 as f64).unwrap()));
            i2 = u32::wrapping_add(i2, 1);
        }
        let p = JustifierJfTestSupport::justifier_jf_test_support_justify(&c, &r, &e, IntRange::new(0u32, 2u32), 60 as f64, Some(16 as f64 as f64), Some(false), None.clone(), Some(true), Some(0.25f64), Some(0.5f64), None, None, None, None, None, None, None, Some(b.clone().build()), None, None, None).unwrap();
        let mut none = true;
        let mut i3 = 0u32;
        while (i32::from_ne_bytes(((i3) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if p.allocations[usize::try_from(i3).unwrap_or(0)].kind == GlueKind::InlineObjectRelation {
                none = false;
            }
            i3 = u32::wrapping_add(i3, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn single_cluster_range_produces_no_opportunities() {
    testlib::run("org.tiqian.layout.JustifierJfTest.singleClusterRangeProducesNoOpportunities", "org.tiqian.layout.JustifierJfTest.singleClusterRangeProducesNoOpportunities", || {
        JustifierJfTestSupport::justifier_jf_test_support_sec(UStr::new(&[115,105,110,103,108,101,67,108,117,115,116,101,114,82,97,110,103,101,80,114,111,100,117,99,101,115,78,111,79,112,112,111,114,116,117,110,105,116,105,101,115]));
        let p = JustifierJfTestSupport::justifier_jf_test_support_justify(&vec![
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[20013]), 0, None, None).unwrap()).clone(),
], &vec![FontRole::CjkText], &vec![
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
], IntRange::new(0u32, 0u32), 30 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(0, u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    });
}

#[test]
fn typed_space_and_word_space_predicate_edge_conditions() {
    testlib::run("org.tiqian.layout.JustifierJfTest.typedSpaceAndWordSpacePredicateEdgeConditions", "org.tiqian.layout.JustifierJfTest.typedSpaceAndWordSpacePredicateEdgeConditions", || {
        JustifierJfTestSupport::justifier_jf_test_support_sec(UStr::new(&[116,121,112,101,100,83,112,97,99,101,65,110,100,87,111,114,100,83,112,97,99,101,80,114,101,100,105,99,97,116,101,69,100,103,101,67,111,110,100,105,116,105,111,110,115]));
        let c = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[]), 0, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[32]), 1, Some(4 as f64 as f64), None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[32]), 2, Some(4 as f64 as f64), None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[97,98,99]), 3, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[120,121,122]), 4, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[23383]), 5, None, None).unwrap()).clone(),
];
        let r = vec![
    FontRole::LatinText,
    FontRole::LatinText,
    FontRole::LatinText,
    FontRole::LatinText,
    FontRole::LatinText,
    FontRole::CjkText,
];
        let e = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_e(None, None, None)).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(None, None, None)).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(None, None, None)).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
];
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes(((u32::try_from((JustifierJfTestSupport::justifier_jf_test_support_justify(&c, &r, &e, IntRange::new(0u32, 5u32), 150 as f64, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).unwrap().allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) > (0), None).unwrap();
    });
}

#[test]
fn virtual_non_sino_western_boundary_when_allow_sino_western_gap_stretch_is_false() {
    testlib::run("org.tiqian.layout.JustifierJfTest.virtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalse", "org.tiqian.layout.JustifierJfTest.virtualNonSinoWesternBoundaryWhenAllowSinoWesternGapStretchIsFalse", || {
        JustifierJfTestSupport::justifier_jf_test_support_sec(UStr::new(&[118,105,114,116,117,97,108,78,111,110,83,105,110,111,87,101,115,116,101,114,110,66,111,117,110,100,97,114,121,87,104,101,110,65,108,108,111,119,83,105,110,111,87,101,115,116,101,114,110,71,97,112,83,116,114,101,116,99,104,73,115,70,97,108,115,101]));
        let c = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[20013]), 0, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[91]), 1, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[49]), 2, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[93]), 3, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[25991]), 4, None, None).unwrap()).clone(),
];
        let r = vec![FontRole::CjkText, FontRole::CjkText, FontRole::CjkText, FontRole::CjkText, FontRole::CjkText];
        let e = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(None, None, None)).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(None, None, None)).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(None, None, None)).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
];
        let p = JustifierJfTestSupport::justifier_jf_test_support_justify(&c, &r, &e, IntRange::new(0u32, 4u32), 100 as f64, Some(16 as f64 as f64), Some(false), None.clone(), Some(false), Some(0.25f64), Some(0.5f64), None, None, None, None, Some(JustifierJfTestSupport::justifier_jf_test_support_int_map(&vec![3], &vec![0])), None, None, None, None, None, None).unwrap();
        let mut ok = false;
        let mut i4 = 0u32;
        while (i32::from_ne_bytes(((i4) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if p.allocations[usize::try_from(i4).unwrap_or(0)].target_cluster_index == 3 && ((p.allocations[usize::try_from(i4).unwrap_or(0)]).clone().reason).to_ustring() == UString::from("AttachedInlineVirtualInterChar") {
                ok = true;
            }
            i4 = u32::wrapping_add(i4, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, None).unwrap();
    });
}

#[test]
fn virtual_sino_western_gap_when_allow_sino_western_gap_stretch_is_false() {
    testlib::run("org.tiqian.layout.JustifierJfTest.virtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalse", "org.tiqian.layout.JustifierJfTest.virtualSinoWesternGapWhenAllowSinoWesternGapStretchIsFalse", || {
        JustifierJfTestSupport::justifier_jf_test_support_sec(UStr::new(&[118,105,114,116,117,97,108,83,105,110,111,87,101,115,116,101,114,110,71,97,112,87,104,101,110,65,108,108,111,119,83,105,110,111,87,101,115,116,101,114,110,71,97,112,83,116,114,101,116,99,104,73,115,70,97,108,115,101]));
        let c = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[20013]), 0, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[91]), 1, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[49]), 2, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[93]), 3, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[97]), 4, None, None).unwrap()).clone(),
];
        let r = vec![
    FontRole::CjkText,
    FontRole::CjkText,
    FontRole::CjkText,
    FontRole::CjkText,
    FontRole::LatinText,
];
        let e = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(None, None, None)).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(None, None, None)).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(None, None, None)).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
];
        let p = JustifierJfTestSupport::justifier_jf_test_support_justify(&c, &r, &e, IntRange::new(0u32, 4u32), 100 as f64, Some(16 as f64 as f64), Some(false), None.clone(), Some(false), Some(0.25f64), Some(0.5f64), None, None, None, None, Some(JustifierJfTestSupport::justifier_jf_test_support_int_map(&vec![3], &vec![0])), Some(JustifierJfTestSupport::justifier_jf_test_support_set(&vec![3])), None, None, None, None, None).unwrap();
        let mut ok = true;
        let mut i5 = 0u32;
        while (i32::from_ne_bytes(((i5) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if p.allocations[usize::try_from(i5).unwrap_or(0)].target_cluster_index == 3 {
                ok = false;
            }
            i5 = u32::wrapping_add(i5, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, None).unwrap();
    });
}

#[test]
fn zero_cjk_latin_headroom_produces_no_opportunities() {
    testlib::run("org.tiqian.layout.JustifierJfTest.zeroCjkLatinHeadroomProducesNoOpportunities", "org.tiqian.layout.JustifierJfTest.zeroCjkLatinHeadroomProducesNoOpportunities", || {
        JustifierJfTestSupport::justifier_jf_test_support_sec(UStr::new(&[122,101,114,111,67,106,107,76,97,116,105,110,72,101,97,100,114,111,111,109,80,114,111,100,117,99,101,115,78,111,79,112,112,111,114,116,117,110,105,116,105,101,115]));
        let c = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[20013]), 0, None, None).unwrap()).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_c(UStr::new(&[97]), 1, None, None).unwrap()).clone(),
];
        let r = vec![FontRole::CjkText, FontRole::LatinText];
        let e = vec![
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Wide), Some(EastAsianSpacingValue::Wide), Some(true))).clone(),
    (JustifierJfTestSupport::justifier_jf_test_support_e(Some(EastAsianSpacingValue::Narrow), Some(EastAsianSpacingValue::Narrow), None)).clone(),
];
        let p = JustifierJfTestSupport::justifier_jf_test_support_justify(&c, &r, &e, IntRange::new(0u32, 1u32), 40 as f64, Some(16 as f64 as f64), Some(false), None.clone(), Some(true), Some(0.5f64), Some(0.5f64), None, None, None, None, None, None, None, None, None, None, None).unwrap();
        let mut ok = true;
        let mut i6 = 0u32;
        while (i32::from_ne_bytes(((i6) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((p.allocations.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if p.allocations[usize::try_from(i6).unwrap_or(0)].kind != GlueKind::CjkInterChar {
                ok = false;
            }
            i6 = u32::wrapping_add(i6, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(ok, None).unwrap();
    });
}
